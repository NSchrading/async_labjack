use crate::labjack_tag::{
    Addressable, HydratedTagValue, Readable, ReadableLabjackTag, WritableLabjackTag,
};
use async_trait::async_trait;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::borrow::Cow;
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
    write_data: Bytes,
}

impl<'a> ModbusFeedbackFrame<'a> {
    pub fn new(
        read_addresses: &'a [Address],
        write_addresses: &'a [Address],
        read_counts: &'a [u8],
        write_counts: &'a [u8],
        write_data: Bytes,
    ) -> Self {
        Self {
            read_addresses,
            write_addresses,
            read_counts,
            write_counts,
            write_data,
        }
    }

    pub fn new_read_frame(read_addresses: &'a [Address], read_counts: &'a [u8]) -> Self {
        Self {
            read_addresses,
            write_addresses: &[],
            read_counts,
            write_counts: &[],
            write_data: Bytes::new(),
        }
    }

    pub fn new_write_frame(
        write_addresses: &'a [Address],
        write_counts: &'a [u8],
        write_data: Bytes,
    ) -> Self {
        Self {
            read_addresses: &[],
            write_addresses,
            read_counts: &[],
            write_counts,
            write_data,
        }
    }

    fn to_bytes_mut(&mut self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(
            self.read_addresses.len() * 4 + self.write_addresses.len() * 4 + self.write_data.len(),
        );

        // Add write bytes before read bytes. This allows for single operations that write to data
        // and then return their newly written values as reads.
        // If a frame generates an error, no further frames are executed.
        // Frames before the error frames are executed (e.g. outputs are set), but no data is
        // returned for those frames (since you just get the standard Modbus error response).
        for (address, num_registers) in zip(self.write_addresses, self.write_counts) {
            bytes.put_u8(1);
            bytes.put_u16(*address);
            bytes.put_u8(*num_registers);
            match *num_registers {
                1 => {
                    bytes.put_u16(self.write_data.get_u16());
                }
                2 => {
                    bytes.put_u32(self.write_data.get_u32());
                }
                _ => unreachable!(
                    "There should never be a tag with a register count not equal to 1 or 2."
                ),
            }
        }

        for (address, count) in zip(self.read_addresses, self.read_counts) {
            bytes.put_u8(0);
            bytes.put_u16(*address);
            bytes.put_u8(*count);
        }

        bytes.freeze()
    }
}

#[async_trait]
pub trait CustomReader: Client {
    /// Read multiple frames using custom MBFB modbus function implemented in labjacks (0x4C)
    /// https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76)
    async fn read_write_frame_bytes(&mut self, bytes: Bytes) -> Result<Bytes>;
    async fn read_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes>;
    async fn read_tags(&mut self, tags: &[ReadableLabjackTag]) -> Result<Vec<HydratedTagValue>>;

    async fn read_write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes>;
    async fn read_write_tags<const N: usize>(
        &mut self,
        read_tags: &[ReadableLabjackTag],
        write_tags: &[WritableLabjackTag; N],
        tag_values: &[HydratedTagValue; N],
    ) -> Result<Vec<HydratedTagValue>>;
}

fn bytes_to_hydrated_tags(
    bytes: &mut Bytes,
    read_tags: &[ReadableLabjackTag],
) -> Vec<HydratedTagValue> {
    let mut hydrated_result = Vec::new();
    for tag in read_tags {
        match tag.register_count() {
            1 => hydrated_result.push(tag.hydrate(&mut bytes.copy_to_bytes(2))),
            2 => hydrated_result.push(tag.hydrate(&mut bytes.copy_to_bytes(4))),
            _ => unreachable!(
                "There should never be a tag with a register count not equal to 1 or 2."
            ),
        }
    }
    hydrated_result
}

fn write_tags_to_bytes<const N: usize>(
    tags: &[WritableLabjackTag; N],
    tag_values: &[HydratedTagValue; N],
) -> Bytes {
    // overestimate capacity in the case of u16 values, but usually each tag_value is going to be
    // 4 bytes (u32, f32, etc).
    // This could be more accurate by looping through tag_values and matching on the type to get
    // the exact sizes, but probably not necessary.
    let mut bytes = BytesMut::with_capacity(tags.len() * 4 + tag_values.len() * 4);
    for (tag, tag_value) in zip(tags, tag_values) {
        bytes.put_u8(1);
        bytes.put_u16(tag.address());
        bytes.put_u8(tag.register_count());

        match tag_value {
            HydratedTagValue::F32(value) => bytes.put_f32(*value),
            HydratedTagValue::I32(value) => bytes.put_i32(*value),
            HydratedTagValue::U32(value) => bytes.put_u32(*value),
            HydratedTagValue::U16(value) => bytes.put_u16(*value),
        }
    }
    bytes.freeze()
}

fn read_tags_to_bytes(tags: &[ReadableLabjackTag]) -> Bytes {
    let mut bytes = BytesMut::with_capacity(tags.len() * 4);
    for tag in tags {
        bytes.put_u8(0);
        bytes.put_u16(tag.address());
        bytes.put_u8(tag.register_count());
    }
    bytes.freeze()
}

#[async_trait]
impl CustomReader for Context {
    async fn read_write_frame_bytes(&mut self, bytes: Bytes) -> Result<Bytes> {
        self.call(Request::Custom(0x4C, Cow::Borrowed(&bytes)))
            .await
            .map(|result| {
                result.map_err(Into::into).map(|response| match response {
                    Response::Custom(function_code, words) => {
                        debug_assert_eq!(function_code, 0x4C);
                        words
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })
    }

    async fn read_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes> {
        assert!(mbfb.write_addresses.is_empty());
        assert!(mbfb.write_counts.is_empty());
        assert!(mbfb.write_data.is_empty());

        let bytes = mbfb.to_bytes_mut();
        self.read_write_frame_bytes(bytes).await
    }

    async fn read_tags(&mut self, tags: &[ReadableLabjackTag]) -> Result<Vec<HydratedTagValue>> {
        let mut addresses = Vec::new();
        let mut counts: Vec<u8> = Vec::new();
        for tag in tags {
            addresses.push(tag.address());
            counts.push(tag.register_count());
        }

        let mut mbfb = ModbusFeedbackFrame::new_read_frame(&addresses, &counts);
        self.read_mbfb(&mut mbfb)
            .await
            .map(|result| result.map(|mut response| bytes_to_hydrated_tags(&mut response, tags)))
    }

    async fn read_write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes> {
        let bytes = mbfb.to_bytes_mut();
        self.read_write_frame_bytes(bytes).await
    }

    async fn read_write_tags<const N: usize>(
        &mut self,
        read_tags: &[ReadableLabjackTag],
        write_tags: &[WritableLabjackTag; N],
        tag_values: &[HydratedTagValue; N],
    ) -> Result<Vec<HydratedTagValue>> {
        let mut read_addresses = Vec::new();
        let mut read_counts: Vec<u8> = Vec::new();
        for tag in read_tags {
            read_addresses.push(tag.address());
            read_counts.push(tag.register_count());
        }

        // Add write bytes before read bytes. This allows for single operations that write to data
        // and then return their newly written values as reads.
        let write_bytes = write_tags_to_bytes(write_tags, tag_values);
        let read_bytes = read_tags_to_bytes(read_tags);
        let total_len = write_bytes.len() + read_bytes.len();
        let bytes = write_bytes.chain(read_bytes).copy_to_bytes(total_len);

        self.read_write_frame_bytes(bytes).await.map(|result| {
            result.map(|mut response| bytes_to_hydrated_tags(&mut response, read_tags))
        })
    }
}

#[async_trait]
pub trait CustomWriter: Client {
    /// Write multiple frames using custom MBFB modbus function implemented in labjacks (0x4C)
    /// https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76)
    async fn write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<()>;
    async fn write_bytes(&mut self, bytes: Bytes) -> Result<()>;

    async fn write_tags<const N: usize>(
        &mut self,
        tags: &[WritableLabjackTag; N],
        tag_values: &[HydratedTagValue; N],
    ) -> Result<()>;
}

#[async_trait]
impl CustomWriter for Context {
    async fn write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<()> {
        assert!(mbfb.read_addresses.is_empty());
        assert!(mbfb.read_counts.is_empty());

        let bytes = mbfb.to_bytes_mut();
        log::debug!("Raw bytes of mbfb: {bytes:?}");

        self.write_bytes(bytes).await
    }

    async fn write_bytes(&mut self, bytes: Bytes) -> Result<()> {
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

    async fn write_tags<const N: usize>(
        &mut self,
        tags: &[WritableLabjackTag; N],
        tag_values: &[HydratedTagValue; N],
    ) -> Result<()> {
        self.write_bytes(write_tags_to_bytes(tags, tag_values))
            .await
    }
}
