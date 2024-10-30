use crate::helpers::bit_manipulation::{u16_to_u8_vec, u8_to_u16_vec};
use crate::labjack_tag::labjack_tag::{Hydratable, HydratedTag, LabjackTag};
use bytes::{BufMut, BytesMut};
use std::borrow::Cow;
use std::io::Cursor;
use std::io::Read;
use std::iter::zip;
use tokio_modbus::client::{Client, Context};
use tokio_modbus::prelude::{Request, Response};
use tokio_modbus::Address;
use tokio_modbus::Result;

pub struct ModbusFeedbackFrame<'a> {
    read_addresses: &'a [Address],
    write_addresses: &'a [Address],
    read_counts: &'a [u8],
    write_counts: &'a [u8],
    write_data: &'a [u8],
}

impl<'a> ModbusFeedbackFrame<'a> {
    pub fn new(
        read_addresses: &'a [Address],
        write_addresses: &'a [Address],
        read_counts: &'a [u8],
        write_counts: &'a [u8],
        write_data: &'a [u8],
    ) -> Self {
        Self {
            read_addresses: read_addresses,
            write_addresses: write_addresses,
            read_counts: read_counts,
            write_counts: write_counts,
            write_data: write_data,
        }
    }

    pub fn new_read_frame(read_addresses: &'a [Address], read_counts: &'a [u8]) -> Self {
        Self {
            read_addresses: read_addresses,
            write_addresses: &[],
            read_counts: read_counts,
            write_counts: &[],
            write_data: &[],
        }
    }

    pub fn new_write_frame(
        write_addresses: &'a [Address],
        write_counts: &'a [u8],
        write_data: &'a [u8],
    ) -> Self {
        Self {
            read_addresses: &[],
            write_addresses: write_addresses,
            read_counts: &[],
            write_counts: write_counts,
            write_data: write_data,
        }
    }

    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(
            self.read_addresses.len() * 3 + self.write_addresses.len() * 3 + self.write_data.len(),
        );

        // Add write bytes before read bytes. This allows for single operations that write to data
        // and then return their newly written values as reads.
        // If a frame generates an error, no further frames are executed. Frames before the error frames are executed (e.g. outputs are set),
        // but no data is returned for those frames (since you just get the standard Modbus error response).

        let mut data_bytes_cursor = Cursor::new(self.write_data);
        for (address, num_registers) in zip(self.write_addresses, self.write_counts) {
            bytes.put_u8(1);
            bytes.put_slice(&address.to_be_bytes());
            bytes.put_u8(*num_registers);
            match *num_registers {
                1 => {
                    let mut data_for_address = [0; 2];
                    data_bytes_cursor.read_exact(&mut data_for_address).unwrap();
                    bytes.put_slice(&data_for_address);
                }
                2 => {
                    let mut data_for_address = [0; 4];
                    data_bytes_cursor.read_exact(&mut data_for_address).unwrap();
                    bytes.put_slice(&data_for_address);
                }
                _ => unreachable!(
                    "There should never be a tag with a register count not equal to 1 or 2."
                ),
            }
        }

        for (address, count) in zip(self.read_addresses, self.read_counts) {
            bytes.put_u8(0);
            bytes.put_slice(&address.to_be_bytes());
            bytes.put_u8(*count);
        }

        bytes
    }
}

pub trait CustomReader: Client {
    /// Read multiple frames using custom MBFB modbus function implemented in labjacks (0x4C)
    /// https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76)
    async fn read_write_frame_bytes(&mut self, bytes: &[u8]) -> Result<Vec<u8>>;
    async fn read_mbfb(&mut self, mbfb: &ModbusFeedbackFrame) -> Result<Vec<u8>>;
    async fn read_tags(&mut self, tags: &[&dyn Hydratable]) -> Result<Vec<HydratedTag>>;

    async fn read_write_mbfb(&mut self, mbfb: &ModbusFeedbackFrame) -> Result<Vec<u8>>;
    async fn read_write_tags(
        &mut self,
        read_tags: &[&dyn Hydratable],
        write_tags: &[&dyn Hydratable],
        tag_values: &[HydratedTag],
    ) -> Result<Vec<HydratedTag>>;
}

fn bytes_to_hydrated_tags(bytes: &[u8], read_tags: &[&dyn Hydratable]) -> Vec<HydratedTag> {
    let mut hydrated_result = Vec::new();
    let mut byte_cursor = Cursor::new(bytes);
    for tag in read_tags {
        match tag.register_count() {
            1 => {
                let mut bytes = [0; 2];
                byte_cursor.read_exact(&mut bytes).unwrap();
                hydrated_result.push(tag.hydrate(&bytes))
            }
            2 => {
                let mut bytes = [0; 4];
                byte_cursor.read_exact(&mut bytes).unwrap();
                hydrated_result.push(tag.hydrate(&bytes))
            }
            _ => unreachable!(
                "There should never be a tag with a register count not equal to 1 or 2."
            ),
        }
    }
    hydrated_result
}

impl CustomReader for Context {
    async fn read_write_frame_bytes(&mut self, bytes: &[u8]) -> Result<Vec<u8>> {
        println!("bytes: {bytes:?}");

        self.call(Request::Custom(0x4C, Cow::Borrowed(&bytes)))
            .await
            .map(|result| {
                result.map_err(Into::into).map(|response| match response {
                    Response::Custom(function_code, words) => {
                        debug_assert_eq!(function_code, 0x4C);
                        words.as_ref().to_vec()
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })
    }

    async fn read_mbfb(&mut self, mbfb: &ModbusFeedbackFrame<'_>) -> Result<Vec<u8>> {
        assert!(mbfb.write_addresses.len() == 0);
        assert!(mbfb.write_counts.len() == 0);
        assert!(mbfb.write_data.len() == 0);

        let bytes = mbfb.to_bytes();
        self.read_write_frame_bytes(&bytes).await
    }

    async fn read_tags(&mut self, tags: &[&dyn Hydratable]) -> Result<Vec<HydratedTag>> {
        let mut addresses = Vec::new();
        let mut counts: Vec<u8> = Vec::new();
        for tag in tags {
            addresses.push(tag.address());
            counts.push(tag.register_count());
        }

        let mbfb = ModbusFeedbackFrame::new_read_frame(&addresses, &counts);
        let result = self
            .read_mbfb(&mbfb)
            .await
            .map(|result| result.map(|response| bytes_to_hydrated_tags(&response, tags)));
        result
    }

    async fn read_write_mbfb(&mut self, mbfb: &ModbusFeedbackFrame<'_>) -> Result<Vec<u8>> {
        let bytes = mbfb.to_bytes();
        self.read_write_frame_bytes(&bytes).await
    }

    async fn read_write_tags(
        &mut self,
        read_tags: &[&dyn Hydratable],
        write_tags: &[&dyn Hydratable],
        tag_values: &[HydratedTag],
    ) -> Result<Vec<HydratedTag>> {
        let mut read_addresses = Vec::new();
        let mut read_counts: Vec<u8> = Vec::new();
        for tag in read_tags {
            read_addresses.push(tag.address());
            read_counts.push(tag.register_count());
        }

        let mut write_addresses = Vec::new();
        let mut registers_per_address = Vec::new();
        let mut write_bytes = Vec::new();

        for (tag, tag_value) in zip(write_tags, tag_values) {
            write_addresses.push(tag.address());
            registers_per_address.push(tag.register_count());

            match tag_value {
                HydratedTag::F32(value) => write_bytes.extend(value.to_be_bytes()),
                HydratedTag::I32(value) => write_bytes.extend(value.to_be_bytes()),
                HydratedTag::U32(value) => write_bytes.extend(value.to_be_bytes()),
                HydratedTag::U16(value) => write_bytes.extend(value.to_be_bytes()),
            }
        }

        let mbfb = ModbusFeedbackFrame::new(
            &read_addresses,
            &write_addresses,
            &read_counts,
            &registers_per_address,
            &write_bytes,
        );
        let result = self
            .read_write_mbfb(&mbfb)
            .await
            .map(|result| result.map(|response| bytes_to_hydrated_tags(&response, read_tags)));
        result
    }
}

pub trait CustomWriter: Client {
    /// Write multiple frames using custom MBFB modbus function implemented in labjacks (0x4C)
    /// https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76)
    async fn write_frame_bytes(&mut self, mbfb: &ModbusFeedbackFrame<'_>) -> Result<()>;

    async fn write_tags(
        &mut self,
        tags: &[&dyn Hydratable],
        tag_values: &[HydratedTag],
    ) -> Result<()>;
}

impl CustomWriter for Context {
    async fn write_frame_bytes(&mut self, mbfb: &ModbusFeedbackFrame<'_>) -> Result<()> {
        assert!(mbfb.read_addresses.len() == 0);
        assert!(mbfb.read_counts.len() == 0);

        let bytes = mbfb.to_bytes();

        println!("bytes: {bytes:?}");

        self.call(Request::Custom(0x4C, Cow::Borrowed(&bytes)))
            .await
            .map(|result| {
                result.map_err(Into::into).map(|response| match response {
                    Response::Custom(function_code, words) => {
                        debug_assert_eq!(function_code, 0x4C);
                        debug_assert_eq!(words.len(), 0);
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })
    }

    async fn write_tags(
        &mut self,
        tags: &[&dyn Hydratable],
        tag_values: &[HydratedTag],
    ) -> Result<()> {
        let mut addresses = Vec::new();
        let mut registers_per_address = Vec::new();
        let mut bytes = Vec::new();

        for (tag, tag_value) in zip(tags, tag_values) {
            addresses.push(tag.address());
            registers_per_address.push(tag.register_count());

            match tag_value {
                HydratedTag::F32(value) => bytes.extend(value.to_be_bytes()),
                HydratedTag::I32(value) => bytes.extend(value.to_be_bytes()),
                HydratedTag::U32(value) => bytes.extend(value.to_be_bytes()),
                HydratedTag::U16(value) => bytes.extend(value.to_be_bytes()),
            }
        }

        let mbfb = ModbusFeedbackFrame::new_write_frame(&addresses, &registers_per_address, &bytes);

        self.write_frame_bytes(&mbfb).await
    }
}
