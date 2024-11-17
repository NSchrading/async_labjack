//! Additional traits for the tokio modbus Client / Context for reading and writing LabjackTags or
//! ModbusFeedbackFrames.
use crate::labjack_tag::{
    Addressable, HydratedTagValue, Readable, ReadableLabjackTag, WritableLabjackTag,
};
use crate::modbus_feedback::mbfb::ModbusFeedbackFrame;
use async_trait::async_trait;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::borrow::Cow;
use std::iter::zip;
use tokio_modbus::client::{Client, Context};
use tokio_modbus::prelude::{Request, Response};
use tokio_modbus::Result;

/// An extra trait for the tokio_modbus Client, allowing for reads (or reads and writes) of higher
/// level ModbusFeedbackFrames or ReadableLabjackTags / WritableLabjackTags.
#[async_trait]
pub trait CustomReader: Client {
    /// Read and/or write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a Bytes representation of a well-formed MBFB packet and returns a future of the
    /// resulting Bytes.
    /// This is not atomic, meaning if an error occurs mid-way through the operation, some writes
    /// or reads may take effect but the remaining operations will not.
    async fn read_write_frame_bytes(&mut self, bytes: Bytes) -> Result<Bytes>;

    /// Read asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a ModbusFeedbackFrame and returns a future of the resulting Bytes.
    async fn read_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes>;

    /// Read asynchronously from the labjack via the Modbus Feedback function.
    /// Takes ReadableLabjackTags and returns a future of the resulting HydratedTagValues.
    async fn read_tags(&mut self, tags: &[ReadableLabjackTag]) -> Result<Vec<HydratedTagValue>>;

    /// Read and/or write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a ModbusFeedbackFrame and returns a future of the resulting Bytes.
    /// Writes are done before reads. This allows for single operations that
    /// write to data and then return their newly written values as reads.
    /// This is not atomic, meaning if an error occurs mid-way through the operation, some writes
    /// or reads may take effect but the remaining operations will not.
    async fn read_write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes>;

    /// Read and/or write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes ReadableLabjackTags, WritableLabjackTags, and the HydratedTagValues to write for each
    /// WritableLabjackTag and returns a future of the resulting HydratedTagValues.
    /// Writes are done before reads. This allows for single operations that
    /// write to data and then return their newly written values as reads.
    /// This is not atomic, meaning if an error occurs mid-way through the operation, some writes
    /// or reads may take effect but the remaining operations will not.
    async fn read_write_tags<const N: usize>(
        &mut self,
        read_tags: &[ReadableLabjackTag],
        write_tags: &[WritableLabjackTag; N],
        tag_values: &[HydratedTagValue; N],
    ) -> Result<Vec<HydratedTagValue>>;
}

/// Take the given Bytes and convert them to HydratedTagValue based on the provided
/// ReadableLabjackTags.
fn bytes_to_hydrated_tags(
    bytes: &mut Bytes,
    read_tags: &[ReadableLabjackTag],
) -> Vec<HydratedTagValue> {
    let mut hydrated_result = Vec::with_capacity(read_tags.len());
    for tag in read_tags {
        match tag.register_count() {
            1 => hydrated_result.push(tag.hydrate(&mut bytes.copy_to_bytes(2))),
            2 => hydrated_result.push(tag.hydrate(&mut bytes.copy_to_bytes(4))),
            4 => hydrated_result.push(tag.hydrate(&mut bytes.copy_to_bytes(8))),
            _ => unreachable!(
                "There should never be a tag with a register count not equal to 1, 2, or 4."
            ),
        }
    }
    hydrated_result
}

/// Convert the WritableLabjackTags and their values to Write Frame Bytes in the MBFB format.
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
            HydratedTagValue::U64(value) => bytes.put_u64(*value),
            HydratedTagValue::F32(value) => bytes.put_f32(*value),
            HydratedTagValue::I32(value) => bytes.put_i32(*value),
            HydratedTagValue::U32(value) => bytes.put_u32(*value),
            HydratedTagValue::U16(value) => bytes.put_u16(*value),
        }
    }
    bytes.freeze()
}

/// Convert the ReadableLabjackTags to Read Frame Bytes in the MBFB format.
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

/// An extra trait for the tokio_modbus Client, allowing for writes of higher
/// level ModbusFeedbackFrames or WritableLabjackTags.
#[async_trait]
pub trait CustomWriter: Client {
    /// Write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a ModbusFeedbackFrame and returns a future of the result.
    async fn write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<()>;

    /// Write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a Bytes of a well-formed MBFB packet and returns a future of the result.
    async fn write_bytes(&mut self, bytes: Bytes) -> Result<()>;

    /// Write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes WritableLabjackTags and the HydratedTagValues to write and returns a future of
    /// the result.
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
