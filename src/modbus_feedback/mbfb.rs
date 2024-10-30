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
    pub read_addresses: &'a [Address],
    pub write_addresses: &'a [Address],
    pub read_counts: &'a [u8],
    pub write_counts: &'a [u8],
    pub write_data: &'a [u8],
}

impl ModbusFeedbackFrame<'_> {
    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(
            self.read_addresses.len() * 3 + self.write_addresses.len() * 3 + self.write_data.len(),
        );

        for (address, count) in zip(self.read_addresses, self.read_counts) {
            bytes.put_u8(0);
            bytes.put_slice(&address.to_be_bytes());
            bytes.put_u8(*count);
        }

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

        bytes
    }
}

pub trait CustomReader: Client {
    /// Read multiple frames using custom MBFB modbus function implemented in labjacks (0x4C)
    /// https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76)
    async fn read_frame_bytes(&mut self, mbfb: &ModbusFeedbackFrame) -> Result<Vec<u8>>;
    async fn read_frames(&mut self, mbfb: &ModbusFeedbackFrame) -> Result<Vec<u16>>;
    async fn read_tags(&mut self, tags: &[&dyn Hydratable]) -> Result<Vec<HydratedTag>>;
}

impl CustomReader for Context {
    async fn read_frame_bytes(&mut self, mbfb: &ModbusFeedbackFrame<'_>) -> Result<Vec<u8>> {
        let bytes = mbfb.to_bytes();

        println!("bytes: {bytes:?}");

        self.call(Request::Custom(0x4C, Cow::Borrowed(&bytes)))
            .await
            .map(|result| {
                result.map_err(Into::into).map(|response| match response {
                    Response::Custom(function_code, words) => {
                        debug_assert_eq!(function_code, 0x4C);
                        debug_assert_eq!(
                            words.len(),
                            mbfb.read_counts.iter().map(|&val| (val as usize) * 2).sum()
                        );
                        words.as_ref().to_vec()
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })
    }

    async fn read_frames(&mut self, mbfb: &ModbusFeedbackFrame<'_>) -> Result<Vec<u16>> {
        self.read_frame_bytes(mbfb)
            .await
            .map(|result| result.map(|response| u8_to_u16_vec(&response)))
    }

    async fn read_tags(&mut self, tags: &[&dyn Hydratable]) -> Result<Vec<HydratedTag>> {
        let mut addresses = Vec::new();
        let mut counts: Vec<u8> = Vec::new();
        for tag in tags {
            addresses.push(tag.address());
            counts.push(tag.register_count());
        }

        let mbfb = ModbusFeedbackFrame {
            read_addresses: &addresses,
            read_counts: &counts,
            write_addresses: &[],
            write_counts: &[],
            write_data: &[],
        };
        let result = self
            .read_frame_bytes(&mbfb)
            .await
            .map(|result| {
                result.map(|response| {
                    let mut hydrated_result = Vec::new();
                    let mut byte_cursor = Cursor::new(response);
                    for tag in tags {
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
                })
            });
        result
    }
}

pub trait CustomWriter: Client {
    /// Write multiple frames using custom MBFB modbus function implemented in labjacks (0x4C)
    /// https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76)
    async fn write_frame_bytes(
        &mut self,
        addr: &[Address],
        registers_per_address: &[u8],
        bytes: &[u8],
    ) -> Result<()>;

    async fn write_frames(
        &mut self,
        addr: &[Address],
        registers_per_address: &[u8],
        frames: &[u16],
    ) -> Result<()>;

    async fn write_tags(
        &mut self,
        tags: &[&dyn Hydratable],
        tag_values: &[HydratedTag],
    ) -> Result<()>;
}

impl CustomWriter for Context {
    async fn write_frame_bytes(
        &mut self,
        addr: &[Address],
        registers_per_address: &[u8],
        bytes: &[u8],
    ) -> Result<()> {
        let mut request_bytes = Vec::new();
        let mut data_bytes_cursor = Cursor::new(bytes);

        for (address, num_registers) in zip(addr, registers_per_address) {
            request_bytes.push(1);
            request_bytes.extend(address.to_be_bytes());
            request_bytes.push(*num_registers);
            match *num_registers {
                1 => {
                    let mut data_for_address = [0; 2];
                    data_bytes_cursor.read_exact(&mut data_for_address).unwrap();
                    request_bytes.extend(data_for_address);
                }
                2 => {
                    let mut data_for_address = [0; 4];
                    data_bytes_cursor.read_exact(&mut data_for_address).unwrap();
                    request_bytes.extend(data_for_address);
                }
                _ => unreachable!(
                    "There should never be a tag with a register count not equal to 1 or 2."
                ),
            }
        }

        println!("bytes: {request_bytes:?}");

        self.call(Request::Custom(0x4C, Cow::Borrowed(&request_bytes)))
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

    async fn write_frames(
        &mut self,
        addr: &[Address],
        registers_per_address: &[u8],
        frames: &[u16],
    ) -> Result<()> {
        self.write_frame_bytes(addr, registers_per_address, &u16_to_u8_vec(&frames))
            .await
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

        self.write_frame_bytes(&addresses, &registers_per_address, &bytes)
            .await
    }
}
