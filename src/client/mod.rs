//! Additional traits for the tokio modbus Client / Context for reading and writing LabjackTags or
//! ModbusFeedbackFrames.
use crate::helpers::bit_manipulation::u8_to_u16_vec;
use crate::helpers::calibrations::{
    AinCalibrationBuilder, DacCalibrationBuilder, T7Calibrations, T7CalibrationsBuilder,
    TemperatureCalibrationBuilder,
};
use crate::labjack_tag::{
    Addressable, HydratedTagValue, Readable, ReadableLabjackTag, WritableLabjackTag,
};
use crate::labjack_tag::{StreamConfig, StreamConfigBuilder};
use crate::modbus_feedback::mbfb::ModbusFeedbackFrame;
use crate::{
    LabjackError, INTERNAL_FLASH_READ, INTERNAL_FLASH_READ_POINTER, LAST_ERR_DETAIL,
    STREAM_AUTO_TARGET, STREAM_BUFFER_SIZE_BYTES, STREAM_DATATYPE, STREAM_DATA_CR, STREAM_ENABLE,
    STREAM_NUM_ADDRESSES, STREAM_NUM_SCANS, STREAM_RESOLUTION_INDEX, STREAM_SAMPLES_PER_PACKET,
    STREAM_SCANLIST_ADDRESS0, STREAM_SCANRATE_HZ, STREAM_SETTLING_US,
};
use crate::{Result, TokioLabjackError};
use anyhow;
use anyhow::bail;
use async_trait::async_trait;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::borrow::Cow;
use std::cmp;
use std::io;
use std::iter::zip;
use std::net::SocketAddr;
use tokio::net::TcpSocket;
use tokio::time::timeout;
use tokio::time::Duration;
use tokio_modbus;
use tokio_modbus::client::{Client, Context};
use tokio_modbus::prelude::tcp;
use tokio_modbus::prelude::Writer;
use tokio_modbus::prelude::{Request, Response};

#[derive(Debug)]
pub struct LabjackClient {
    pub context: Context,
    address: SocketAddr,
    pub command_response_timeout: Duration,
}

impl LabjackClient {
    pub async fn connect(socket_addr: SocketAddr) -> anyhow::Result<Self> {
        let address = socket_addr;

        // since we're constructing the socket and then connecting, we could add
        // socket configuration
        let socket = TcpSocket::new_v4()?;
        let transport = socket.connect(address).await?;
        let context = tcp::attach(transport);

        let labjack_client = LabjackClient {
            context,
            address,
            command_response_timeout: Duration::from_secs(5),
        };
        Ok(labjack_client)
    }

    pub async fn connect_with_timeout(
        socket_addr: SocketAddr,
        timeout_duration: Duration,
    ) -> anyhow::Result<Self> {
        timeout(timeout_duration, Self::connect(socket_addr)).await?
    }

    pub async fn disconnect(&mut self) -> io::Result<()> {
        self.context.disconnect().await
    }
}

/// An extra trait for the tokio_modbus Client, allowing for reads (or reads and writes) of higher
/// level ModbusFeedbackFrames or ReadableLabjackTags / WritableLabjackTags.
#[async_trait]
pub trait LabjackInteractions {
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
    ) -> anyhow::Result<()>;
    async fn stop_stream(&mut self) -> anyhow::Result<()>;

    async fn read_calibrations(&mut self) -> Result<T7Calibrations>;

    /// Attempt to read num_samples stream samples from the STREAM_DATA_CR stream buffer.
    /// If there are fewer samples in the buffer than num_samples, you will get back
    /// that smaller amount. If there are more samples in the buffer than num_samples, you
    /// will get back exactly num_samples and the remaining samples in the buffer can be read
    /// later. The values in the returned vector are interleaved according to the current
    /// streaming configuration. For example, if you are streaming AIN0 and AIN1, the first
    /// value will be the first sample of AIN0, the second will be the first sample for AIN1,
    /// the third will be the second sample of AIN0, etc.
    /// Some streamable registers (e.g. DIO4_EF_READ_A) have 32-bit data.
    /// When streaming a register that produces 32-bit data, the lower 16 bits (LSW) will be
    /// returned and the upper 16 bits (MSW) will be saved in STREAM_DATA_CAPTURE_16.
    /// To get the full 32-bit value, add STREAM_DATA_CAPTURE_16 to the stream scan list after any
    /// applicable 32-bit register, then combine the two values in software
    /// (LSW + 65536*MSW). Note that STREAM_DATA_CAPTURE_16 may be placed in multiple locations in
    /// the scan list.
    async fn read_stream_cr(&mut self, num_samples: u16) -> anyhow::Result<Vec<u16>>;

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

    async fn get_last_error_details(&mut self) -> anyhow::Result<LabjackError>;
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
impl LabjackInteractions for LabjackClient {
    async fn read_write_frame_bytes(&mut self, bytes: Bytes) -> Result<Bytes> {
        let result = self
            .context
            .call(Request::Custom(0x4C, Cow::Borrowed(&bytes)))
            .await
            .map(|result| {
                result.map(|response| match response {
                    Response::Custom(function_code, words) => {
                        debug_assert_eq!(function_code, 0x4C);
                        words
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })?;
        match result {
            Ok(res) => Ok(res),
            Err(e) => {
                if let Ok(better_error) = self.get_last_error_details().await {
                    match better_error {
                        // sometimes the error details aren't filled in, in which case
                        // you get LjSuccess. In this case, we should fall back to the original
                        // error returned from tokio modbus
                        LabjackError::LjSuccess => Err(TokioLabjackError::from(e)),
                        _ => Err(TokioLabjackError::from(better_error)),
                    }
                } else {
                    Err(TokioLabjackError::from(e))
                }
            }
        }
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
            .map(|mut response| bytes_to_hydrated_tags(&mut response, tags))
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

        self.read_write_frame_bytes(bytes)
            .await
            .map(|mut response| bytes_to_hydrated_tags(&mut response, read_tags))
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
        .map(|response| {
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
    }

    async fn start_stream(
        &mut self,
        config: StreamConfig,
        tags: Vec<ReadableLabjackTag>,
    ) -> anyhow::Result<()> {
        if tags.len() != config.num_addresses as usize {
            bail!(
                "The number of provided tags to stream should equal num_addresses in stream config."
            )
        }

        // Each tag address is 32 bits, which is 2 registers
        let num_registers = config.num_addresses * 2;
        // Max number of registers to write in a single modbus function 16 call is 123.
        // TODO: we can improve this by calling write_multiple_registers multiple times as
        // many times as needed. The actual limit is 128 tags streaming at once.
        if num_registers > 123 {
            bail!(
                "Max number of registers we can write to is 123, but desired is {}. Reduce number of tags to stream.", num_registers
            )
        }

        // write the config values
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
        .await?;

        let mut bytes_vec = Vec::new();
        for tag in tags {
            bytes_vec.extend((tag.address() as u32).to_be_bytes());
        }
        let data_bytes = Bytes::from(bytes_vec);

        // write the addresses that should be streamed to STREAM_SCANLIST_ADDRESS<N>
        self.context
            .write_multiple_registers(
                STREAM_SCANLIST_ADDRESS0.address,
                &u8_to_u16_vec(&data_bytes).unwrap(),
            )
            .await??;

        // start the stream - check that it was set to 1.
        let stream_enable_result = self
            .read_write_tags(
                &[STREAM_ENABLE.into()],
                &[STREAM_ENABLE.into()],
                &[HydratedTagValue::U32(1)],
            )
            .await;

        if let Ok(res) = stream_enable_result {
            if res.len() != 1 {
                bail!("Unexpected result from starting stream: {:?}", res);
            }
            if let HydratedTagValue::U32(val) = res[0] {
                if val != 1 {
                    bail!("Unable to start stream!");
                }
            } else {
                bail!("Unexpected result from starting stream: {:?}", res[0]);
            }
        }
        Ok(())
    }

    async fn stop_stream(&mut self) -> anyhow::Result<()> {
        // end the stream - check that it was set to 0.
        let stream_disable_result = self
            .read_write_tags(
                &[STREAM_ENABLE.into()],
                &[STREAM_ENABLE.into()],
                &[HydratedTagValue::U32(0)],
            )
            .await;

        if let Ok(res) = stream_disable_result {
            if res.len() != 1 {
                bail!("Unexpected result from ending stream: {:?}", res);
            }
            if let HydratedTagValue::U32(val) = res[0] {
                if val != 0 {
                    bail!("Unable to end stream!");
                }
            } else {
                bail!("Unexpected result from ending stream: {:?}", res[0]);
            }
        }
        Ok(())
    }

    async fn read_calibrations(&mut self) -> Result<T7Calibrations> {
        let cal_constant_starting_address: u32 = 0x3C4000;

        // Write the calibration constant starting address to INTERNAL_FLASH_READ_POINTER
        // then read all 82 registers (164 bytes) of calibration constants.
        let mut mbfb = ModbusFeedbackFrame::new(
            &[INTERNAL_FLASH_READ.address],
            &[INTERNAL_FLASH_READ_POINTER.address],
            &[82],
            &[2],
            Bytes::from(cal_constant_starting_address.to_be_bytes().to_vec()),
        );
        self.read_write_mbfb(&mut mbfb).await.map(|mut response| {
            assert!(
                response.len() == 164,
                "Expected to receive 164 bytes of data, but received {} bytes instead.",
                response.len()
            );

            let mut t7_cals = T7CalibrationsBuilder::default().build().unwrap();

            for cal_idx in 0..8 {
                let positive_slope = response.get_f32();
                let negative_slope = response.get_f32();
                let binary_center = response.get_f32();
                let voltage_offset = response.get_f32();
                let ain_cal = AinCalibrationBuilder::default()
                    .binary_center(binary_center)
                    .positive_slope(positive_slope)
                    .negative_slope(negative_slope)
                    .voltage_offset(voltage_offset)
                    .build()
                    .unwrap();
                match cal_idx {
                    0 => t7_cals.hs_gain_1_ain_cal = ain_cal,
                    1 => t7_cals.hs_gain_10_ain_cal = ain_cal,
                    2 => t7_cals.hs_gain_100_ain_cal = ain_cal,
                    3 => t7_cals.hs_gain_1000_ain_cal = ain_cal,
                    4 => t7_cals.hr_gain_1_ain_cal = ain_cal,
                    5 => t7_cals.hr_gain_10_ain_cal = ain_cal,
                    6 => t7_cals.hr_gain_100_ain_cal = ain_cal,
                    7 => t7_cals.hr_gain_1000_ain_cal = ain_cal,
                    _ => unreachable!("cal_idx should max out at 7"),
                }
            }

            for cal_idx in 0..2 {
                let slope = response.get_f32();
                let offset = response.get_f32();
                let dac_cal = DacCalibrationBuilder::default()
                    .slope(slope)
                    .offset(offset)
                    .build()
                    .unwrap();
                match cal_idx {
                    0 => t7_cals.dac0_cal = dac_cal,
                    1 => t7_cals.dac1_cal = dac_cal,
                    _ => unreachable!("cal_idx should max out at 1"),
                }
            }

            let t_slope = response.get_f32();
            let t_offset = response.get_f32();
            let temperature_cal = TemperatureCalibrationBuilder::default()
                .slope(t_slope)
                .offset(t_offset)
                .build()
                .unwrap();
            t7_cals.temperature_cal = temperature_cal;
            t7_cals.i_source_10u = response.get_f32();
            t7_cals.i_source_200u = response.get_f32();
            t7_cals.ain_bias_current = response.get_f32();

            t7_cals
        })
    }

    async fn read_stream_cr(&mut self, num_samples: u16) -> anyhow::Result<Vec<u16>> {
        // First 4 registers are:
        // Bytes 8-9: Number of samples in this read
        // Bytes 10-11: Backlog Bytes
        // Bytes 12-13: Status Code
        // Byte 14-15: Additional status information

        let mut data = Vec::with_capacity(num_samples as usize);
        let mut num_samples_to_read = num_samples;
        let mut more_data_available = true;

        while num_samples_to_read > 0 && more_data_available {
            let num_registers_to_read = cmp::min(4 + num_samples_to_read, 255) as u8;
            let num_registers_to_read_ref = &[num_registers_to_read];

            let mut mbfb = ModbusFeedbackFrame::new_read_frame(
                &[STREAM_DATA_CR.address],
                num_registers_to_read_ref,
            );
            let mut resultant_bytes = self.read_mbfb(&mut mbfb).await?;
            let num_samples_returned = resultant_bytes.get_u16();
            let backlog_bytes = resultant_bytes.get_u16();
            let status_code = resultant_bytes.get_u16();
            let additional_status_info = resultant_bytes.get_u16();

            log::debug!(
                "num_samples_returned: {num_samples_returned:?}, backlog_bytes: {backlog_bytes:?}, status_code: {status_code:?}, additional_status_info: {additional_status_info:?}"
            );

            for _ in 0..num_samples_returned {
                data.push(resultant_bytes.get_u16());
            }
            more_data_available = backlog_bytes > 0;

            num_samples_to_read = num_samples_to_read.saturating_sub(num_samples_returned);
            log::debug!("remaining desired sample reads: {num_samples_to_read:?}, more_data_available: {more_data_available:?}");
        }

        Ok(data)
    }

    async fn write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<()> {
        assert!(mbfb.read_addresses.is_empty());
        assert!(mbfb.read_counts.is_empty());

        let bytes = mbfb.to_bytes_mut();
        log::debug!("Raw bytes of mbfb: {bytes:?}");

        self.write_bytes(bytes).await
    }

    async fn write_bytes(&mut self, bytes: Bytes) -> Result<()> {
        let result = self
            .context
            .call(Request::Custom(0x4C, Cow::Borrowed(&bytes)))
            .await
            .map(|result| {
                result.map(|response| match response {
                    Response::Custom(function_code, words) => {
                        debug_assert_eq!(function_code, 0x4C);
                        debug_assert_eq!(words.len(), 0);
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })?;

        match result {
            Ok(res) => Ok(res),
            Err(e) => {
                if let Ok(better_error) = self.get_last_error_details().await {
                    match better_error {
                        // sometimes the error details aren't filled in, in which case
                        // you get LjSuccess. In this case, we should fall back to the original
                        // error returned from tokio modbus
                        LabjackError::LjSuccess => Err(TokioLabjackError::from(e)),
                        _ => Err(TokioLabjackError::from(better_error)),
                    }
                } else {
                    Err(TokioLabjackError::from(e))
                }
            }
        }
    }

    async fn write_tags<const N: usize>(
        &mut self,
        tags: &[WritableLabjackTag; N],
        tag_values: &[HydratedTagValue; N],
    ) -> Result<()> {
        self.write_bytes(write_tags_to_bytes(tags, tag_values))
            .await
    }

    async fn get_last_error_details(&mut self) -> anyhow::Result<LabjackError> {
        let error_code = LAST_ERR_DETAIL.read(self).await?;
        Ok(error_code.try_into()?)
    }
}
