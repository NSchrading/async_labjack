//! Additional traits for the tokio modbus Client / Context for reading and writing LabjackTags or
//! ModbusFeedbackFrames.
use crate::helpers::bit_manipulation::{be_bytes_to_u16_array, u8_to_u16_vec};
use crate::labjack_tag::{
    Addressable, HydratedTagValue, Readable, ReadableLabjackTag, WritableLabjackTag,
};
use crate::labjack_tag::{StreamConfig, StreamConfigBuilder};
use crate::modbus_feedback::mbfb::ModbusFeedbackFrame;
use crate::{
    AIN0, AIN1, STREAM_AUTO_TARGET, STREAM_BUFFER_SIZE_BYTES, STREAM_DATATYPE,
    STREAM_DEBUG_GET_SELF_INDEX, STREAM_ENABLE, STREAM_NUM_ADDRESSES, STREAM_NUM_SCANS,
    STREAM_RESOLUTION_INDEX, STREAM_SAMPLES_PER_PACKET, STREAM_SCANLIST_ADDRESS0,
    STREAM_SCANLIST_ADDRESS1, STREAM_SCANRATE_HZ, STREAM_SETTLING_US,
};
use anyhow::bail;
use async_trait::async_trait;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::borrow::Cow;
use std::iter::zip;
use tokio_modbus::client::{Client, Context};
use tokio_modbus::prelude::Writer;
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

    async fn read_stream_config(&mut self) -> Result<StreamConfig>;
    async fn start_stream(
        &mut self,
        config: StreamConfig,
        tags: Vec<ReadableLabjackTag>,
    ) -> Result<()>;
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

    async fn read_stream_config(&mut self) -> Result<StreamConfig> {
        self.read_tags(&[
            STREAM_SCANRATE_HZ.into(),
            STREAM_NUM_ADDRESSES.into(),
            STREAM_SAMPLES_PER_PACKET.into(),
            STREAM_SETTLING_US.into(),
            STREAM_RESOLUTION_INDEX.into(),
            STREAM_BUFFER_SIZE_BYTES.into(),
            STREAM_AUTO_TARGET.into(),
            STREAM_NUM_SCANS.into(),
        ])
        .await
        .map(|result| {
            result.map(|response| {
                assert!(response.len() == 8);
                let scan_rate = match response[0] {
                    HydratedTagValue::F32(val) => val,
                    _ => panic!("scan_rate must be an F32"),
                };

                let num_addresses = match response[1] {
                    HydratedTagValue::U32(val) => val,
                    _ => panic!("num_addresses must be a U32"),
                };

                let samples_per_packet = match response[2] {
                    HydratedTagValue::U32(val) => val,
                    _ => panic!("samples_per_packet must be a U32"),
                };

                let settling_us = match response[3] {
                    HydratedTagValue::F32(val) => val,
                    _ => panic!("settling_us must be an F32"),
                };

                let resolution_index = match response[4] {
                    HydratedTagValue::U32(val) => val,
                    _ => panic!("resolution_index must be a U32"),
                };

                let buffer_size_bytes = match response[5] {
                    HydratedTagValue::U32(val) => val,
                    _ => panic!("buffer_size_bytes must be a U32"),
                };

                let auto_target = match response[6] {
                    HydratedTagValue::U32(val) => val,
                    _ => panic!("auto_target must be a U32"),
                };

                let num_scans = match response[7] {
                    HydratedTagValue::U32(val) => val,
                    _ => panic!("num_scans must be a U32"),
                };

                let mut config = StreamConfigBuilder::default();
                config
                    .scan_rate(scan_rate)
                    .num_addresses(num_addresses)
                    .samples_per_packet(samples_per_packet)
                    .settling_us(settling_us)
                    .resolution_index(resolution_index)
                    .buffer_size_bytes(buffer_size_bytes)
                    .auto_target(auto_target)
                    .num_scans(num_scans);
                config.build().unwrap()
            })
        })
    }

    async fn start_stream(
        &mut self,
        config: StreamConfig,
        tags: Vec<ReadableLabjackTag>,
    ) -> Result<()> {
        let num_registers = config.num_addresses * 2;

        // assert!(tags.len() == config.num_addresses as usize);
        // assert!(num_registers <= 255);

        self.read_write_tags(
            &[
                STREAM_SCANRATE_HZ.into(),
                STREAM_NUM_ADDRESSES.into(),
                STREAM_SAMPLES_PER_PACKET.into(),
                STREAM_SETTLING_US.into(),
                STREAM_RESOLUTION_INDEX.into(),
                STREAM_BUFFER_SIZE_BYTES.into(),
                STREAM_AUTO_TARGET.into(),
                STREAM_NUM_SCANS.into(),
            ],
            &[
                STREAM_SCANRATE_HZ.into(),
                STREAM_NUM_ADDRESSES.into(),
                STREAM_SAMPLES_PER_PACKET.into(),
                STREAM_SETTLING_US.into(),
                STREAM_RESOLUTION_INDEX.into(),
                STREAM_BUFFER_SIZE_BYTES.into(),
                STREAM_AUTO_TARGET.into(),
                STREAM_NUM_SCANS.into(),
                STREAM_DATATYPE.into(),
            ],
            &[
                HydratedTagValue::F32(config.scan_rate),
                HydratedTagValue::U32(config.num_addresses),
                HydratedTagValue::U32(config.samples_per_packet),
                HydratedTagValue::F32(config.settling_us),
                HydratedTagValue::U32(config.resolution_index),
                HydratedTagValue::U32(config.buffer_size_bytes),
                HydratedTagValue::U32(config.auto_target),
                HydratedTagValue::U32(config.num_scans),
                HydratedTagValue::U32(0),
            ],
        )
        .await;

        let mut bytes_vec = Vec::new();
        for tag in tags {
            bytes_vec.extend((tag.address() as u32).to_be_bytes());
        }
        let data_bytes = Bytes::from(bytes_vec);

        self.write_multiple_registers(
            STREAM_SCANLIST_ADDRESS0.address,
            &u8_to_u16_vec(&data_bytes).unwrap(),
        )
        .await

        // let mut bytes_vec = Vec::new();
        // for tag in tags {
        //     bytes_vec.extend((tag.address() as u32).to_be_bytes());
        // }
        // let data_bytes = Bytes::from(bytes_vec);
        // println!("{:?}", data_bytes);

        // let mut bytes = BytesMut::with_capacity(4 + data_bytes.len());
        // bytes.put_u8(1);
        // bytes.put_u16(STREAM_SCANLIST_ADDRESS0.address);
        // bytes.extend(data_bytes);
        // println!("{:?}", bytes);
        // self.read_write_frame_bytes(bytes.freeze()).await
    }

    // //Starting address is 4016.
    // if(readMultipleRegistersTCP(sock, 4016, 2, data) < 0)
    // 	return -1;
    // bytesToUint32(data, autoTarget); //Address = 4016 (STREAM_AUTO_TARGET)

    // //Starting address is 4020.
    // if(readMultipleRegistersTCP(sock, 4020, 2, data) < 0)
    // 	return -1;
    // bytesToUint32(data, numScans); //Address = 4020 (STREAM_NUM_SCANS)
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
