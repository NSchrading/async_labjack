//! Additional traits for the tokio modbus Client / Context for reading and writing LabjackTags or
//! ModbusFeedbackFrames.
use crate::helpers::bit_manipulation::u8_to_u16_vec;
use crate::helpers::calibrations::{
    Calibrations, T4AinHVCalibrationBuilder, T4AinLVCalibrationBuilder, T4Calibrations,
    T4CalibrationsBuilder, T4DacCalibrationBuilder, T4SpecVCalibrationBuilder,
    T7AinCalibrationBuilder, T7Calibrations, T7CalibrationsBuilder, T7DacCalibrationBuilder,
    TemperatureCalibrationBuilder, CAL_CONST_STARTING_ADDRESS,
};
use crate::labjack_tag::{
    Addressable, HydratedTagValue, Readable, ReadableLabjackTag, WritableLabjackTag,
};
use crate::labjack_tag::{StreamConfig, StreamConfigBuilder};
use crate::modbus_feedback::mbfb::ModbusFeedbackFrame;
use crate::modbus_feedback::MBFB_FUNCTION_CODE;
use crate::{
    LabjackError, INTERNAL_FLASH_READ, INTERNAL_FLASH_READ_POINTER, LAST_ERR_DETAIL, PRODUCT_ID,
    STREAM_AUTO_TARGET, STREAM_BUFFER_SIZE_BYTES, STREAM_DATATYPE, STREAM_DATA_CR, STREAM_ENABLE,
    STREAM_NUM_ADDRESSES, STREAM_NUM_SCANS, STREAM_RESOLUTION_INDEX, STREAM_SAMPLES_PER_PACKET,
    STREAM_SCANLIST_ADDRESS0, STREAM_SCANRATE_HZ, STREAM_SETTLING_US,
};
use crate::{Result, TokioLabjackError};
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
use tokio_modbus::prelude::{ExceptionCode, Request, Response};

/// The kind of labjack this device is, based on the PRODUCT_ID register.
#[derive(Debug)]
pub enum LabjackKind {
    T4,
    T7,
    T8,
    Digit,
}

/// The client used to interact with the labjack. To use, first call [`LabjackClient::connect`] or
/// [`LabjackClient::connect_with_timeout`] to construct a [`LabjackClient`].
///
/// context: The underlying [`tokio_modbus::client::Context`] handling modbus interactions.
///
/// address: The IP address of the labjack.
///
/// command_response_timeout: The duration to wait for a response from the labjack when
///     calling command response functions on it. Defaults to 5 seconds.
///
/// labjack_kind: The kind of labjack that this is. See [`LabjackKind`].
///
/// # Examples
///
/// ```no_run
/// use tokio_labjack_lib::client::LabjackClient;
/// use tokio::time::Duration;
/// use tokio_labjack_lib::AIN0;
///
/// #[tokio::main()]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let labjack_address = "192.168.0.100:502".parse()?;
///     let mut client = LabjackClient::connect_with_timeout(labjack_address, Duration::from_millis(3000))
///         .await?;
///     let val = AIN0.read(&mut client).await?;
///     client.disconnect().await?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct LabjackClient {
    pub context: Context,
    pub address: SocketAddr,
    pub command_response_timeout: Duration,
    pub labjack_kind: LabjackKind,
}

impl LabjackClient {
    /// Connect to the labjack at address socket_addr. If the labjack is not connectable,
    /// this function may hang until the socket is finally closed. If you want a timeout,
    /// on the connection attempt, use [`LabjackClient::connect_with_timeout`].
    pub async fn connect(socket_addr: SocketAddr) -> Result<Self> {
        let address = socket_addr;

        // todo: since we're constructing the socket and then connecting, we could add
        // socket configuration
        let socket = match TcpSocket::new_v4() {
            Ok(res) => res,
            // converting the io error to our TokioLabjackError
            Err(e) => {
                return Err(TokioLabjackError::TokioModbusError(
                    tokio_modbus::Error::Transport(e),
                ));
            }
        };
        let transport = match socket.connect(address).await {
            Ok(res) => res,
            // converting the io error to our TokioLabjackError
            Err(e) => {
                return Err(TokioLabjackError::TokioModbusError(
                    tokio_modbus::Error::Transport(e),
                ));
            }
        };
        let context = tcp::attach(transport);

        let mut labjack_client = LabjackClient {
            context,
            address,
            command_response_timeout: Duration::from_secs(5),
            labjack_kind: LabjackKind::T7, // temporarily assign a kind defaulted to T7
        };

        let actual_kind = labjack_client.get_labjack_kind().await?;
        labjack_client.labjack_kind = actual_kind;

        Ok(labjack_client)
    }

    /// Connect to the labjack at address `socket_addr`, waiting for the `timeout_duration` to
    /// connect. If unable to connect within `timeout_duration`, then returns an error.
    pub async fn connect_with_timeout(
        socket_addr: SocketAddr,
        timeout_duration: Duration,
    ) -> Result<Self> {
        timeout(timeout_duration, Self::connect(socket_addr)).await?
    }

    /// Disconnect from the labjack. On disconnection, will attempt to stop streaming if a stream
    /// is running.
    pub async fn disconnect(&mut self) -> io::Result<()> {
        if let Err(e) = self.stop_stream().await {
            match e {
                TokioLabjackError::TokioModbusError(tokio_modbus::Error::Transport(e))
                    if e.kind() == io::ErrorKind::NotConnected =>
                {
                    // We're already disconnected - no need to call it again.
                    return Ok(());
                }
                _ => {
                    // Nothing to do but note the error and attempt disconnect. User will
                    // have to try stopping stream via powercycle or command.
                    log::error!("Unable to stop stream before disconnect: {e}");
                }
            }
        }
        self.context.disconnect().await
    }

    /// Get the kind of labjack from PRODUCT_ID
    async fn get_labjack_kind(&mut self) -> Result<LabjackKind> {
        let product_id = PRODUCT_ID.read(self).await?;
        match product_id {
            4.0 => Ok(LabjackKind::T4),
            7.0 => Ok(LabjackKind::T7),
            8.0 => Ok(LabjackKind::T8),
            200.0 => Ok(LabjackKind::Digit),
            _ => Err(TokioLabjackError::Other(format!(
                "Unknown labjack product id: {product_id:?}"
            ))),
        }
    }
}

// todo - unsure if I want this. It's probably fine to block the thread to perform the disconnect
// but I need to bring in the futures crate just for this.
// impl Drop for LabjackClient {
//     fn drop(&mut self) {
//         if let Err(e) = futures::executor::block_on(self.disconnect()) {
//             log::error!("Unable to disconnect properly during client drop: {e}");
//         }
//     }
// }

/// An extra trait for the tokio_modbus Client, allowing for reads and writes of higher
/// level [`ModbusFeedbackFrame`]s or [`ReadableLabjackTag`]s / [`WritableLabjackTag`]s.
#[async_trait]
pub trait LabjackInteractions {
    /// Read and/or write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a [`Bytes`] representation of a well-formed MBFB packet and returns a future of the
    /// resulting [`Bytes`].
    /// This is not atomic, meaning if an error occurs mid-way through the operation, some writes
    /// or reads may take effect but the remaining operations will not.
    async fn read_write_frame_bytes(&mut self, bytes: Bytes) -> Result<Bytes>;

    /// Read asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a [`ModbusFeedbackFrame`] and returns a future of the resulting Bytes.
    async fn read_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes>;

    /// Read asynchronously from the labjack via the Modbus Feedback function.
    /// Takes ReadableLabjackTags and returns a future of the resulting [`HydratedTagValue`]s.
    async fn read_tags(&mut self, tags: &[ReadableLabjackTag]) -> Result<Vec<HydratedTagValue>>;

    /// Read and/or write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a [`ModbusFeedbackFrame`] and returns a future of the resulting [`Bytes`].
    /// Writes are done before reads. This allows for single operations that
    /// write to data and then return their newly written values as reads.
    /// This is not atomic, meaning if an error occurs mid-way through the operation, some writes
    /// or reads may take effect but the remaining operations will not.
    async fn read_write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes>;

    /// Read and/or write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes [`ReadableLabjackTag`]s, [`WritableLabjackTag`]s, and the [`HydratedTagValue`]s to
    /// write for each [`WritableLabjackTag`] and returns a future of the resulting
    /// [`HydratedTagValue`]s. Writes are done before reads. This allows for single operations that
    /// write to data and then return their newly written values as reads.
    /// This is not atomic, meaning if an error occurs mid-way through the operation, some writes
    /// or reads may take effect but the remaining operations will not.
    async fn read_write_tags<const N: usize>(
        &mut self,
        read_tags: &[ReadableLabjackTag],
        write_tags: &[WritableLabjackTag; N],
        tag_values: &[HydratedTagValue; N],
    ) -> Result<Vec<HydratedTagValue>>;

    /// Read the currently configured `STREAM_` values from the labjack and return them as a
    /// [`StreamConfig`]. See [Labjack documentation](https://support.labjack.com/docs/3-2-4-low-level-stream-t-series-datasheet)
    /// for more details on streaming.
    async fn read_stream_config(&mut self) -> Result<StreamConfig>;

    /// Start a stream using the provided [`StreamConfig`] values. If using command-response
    /// streaming (auto_target = 16), then you can make use of the [`LabjackClient::read_stream_cr`]
    /// function to read the stream values. If using spontaneous stream mode, then you can make use
    /// of the [`crate::helpers::stream::process_stream`] function to asynchronously process the stream
    /// values sent to port 702.
    ///
    /// See the stream examples in the examples directory.
    ///
    /// See [Labjack documentation](https://support.labjack.com/docs/3-2-4-low-level-stream-t-series-datasheet)
    /// for more details on streaming.
    ///
    /// You should take care to stop the stream before ending a program, otherwise it may continue
    /// to run on your device.
    async fn start_stream(
        &mut self,
        config: StreamConfig,
        tags: Vec<ReadableLabjackTag>,
    ) -> Result<()>;

    /// Stop streaming on the labjack.
    async fn stop_stream(&mut self) -> Result<()>;

    /// Attempt to read num_samples stream samples from the [`STREAM_DATA_CR`] stream buffer.
    ///
    /// If there are fewer samples in the buffer than num_samples, you will get back
    /// that smaller amount. If there are more samples in the buffer than num_samples, you
    /// will get back exactly num_samples and the remaining samples in the buffer can be read
    /// later.
    ///
    /// The values in the returned vector are interleaved according to the current
    /// streaming configuration. For example, if you are streaming [`crate::AIN0`] and
    /// [`crate::AIN1`], the first value will be the first sample of [`crate::AIN0`], the second
    /// will be the first sample for [`crate::AIN1`], the third will be the second sample of
    /// [`crate::AIN0`], etc.
    ///
    /// Some streamable registers (e.g. [`crate::DIO4_EF_READ_A`]) have 32-bit data.
    /// When streaming a register that produces 32-bit data, the lower 16 bits (LSW) will be
    /// returned and the upper 16 bits (MSW) will be saved in [`crate::STREAM_DATA_CAPTURE_16`].
    /// To get the full 32-bit value, add  [`crate::STREAM_DATA_CAPTURE_16`] to the stream scan
    /// list after any applicable 32-bit register, then combine the two values in software
    /// (LSW + 65536*MSW). Note that  [`crate::STREAM_DATA_CAPTURE_16`] may be placed in multiple
    /// locations in the scan list to stream multiple 32 bit tags.
    async fn read_stream_cr(&mut self, num_samples: u16) -> Result<Vec<u16>>;

    /// Read calibration constants from T7 internal flash. T7-only.
    /// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
    async fn read_t7_calibrations(&mut self) -> Result<T7Calibrations>;

    /// Read calibration constants from T4 internal flash. T4-only.
    /// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
    async fn read_t4_calibrations(&mut self) -> Result<T4Calibrations>;

    /// Read calibration constants from internal flash.
    async fn read_calibrations(&mut self) -> Result<Calibrations>;

    /// Write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a [`ModbusFeedbackFrame`] and returns a future of the result.
    async fn write_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<()>;

    /// Write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes a Bytes of a well-formed MBFB packet and returns a future of the result.
    async fn write_bytes(&mut self, bytes: Bytes) -> Result<()>;

    /// Write asynchronously from the labjack via the Modbus Feedback function.
    /// Takes [`WritableLabjackTag`]s and the [`HydratedTagValue`]s to write and returns a
    /// future of the result.
    async fn write_tags<const N: usize>(
        &mut self,
        tags: &[WritableLabjackTag; N],
        tag_values: &[HydratedTagValue; N],
    ) -> Result<()>;

    /// If an error occurs when interacting with the labjack, this function can return additional
    /// error details via the [`LAST_ERR_DETAIL`] register. Returns the error as a [`LabjackError`]
    /// enum.
    async fn get_last_error_details(&mut self) -> Result<LabjackError>;

    /// Attempt to read [`LAST_ERR_DETAIL`], returning a [`TokioLabjackError`] with more error
    /// details if possible. If [`LAST_ERR_DETAIL`] is [`LabjackError::LjSuccess`], then returns the
    /// original error that prompted wanting more details. If an error occurs while trying to
    /// read [`LAST_ERR_DETAIL`], then this also returns the original error that prompted
    /// wanting more details.
    async fn detailed_error_from_exception_code(
        &mut self,
        error: ExceptionCode,
    ) -> TokioLabjackError;
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
            // 16 bit registers
            1 => hydrated_result.push(tag.hydrate(&mut bytes.copy_to_bytes(2))),
            // 32 bit registers
            2 => hydrated_result.push(tag.hydrate(&mut bytes.copy_to_bytes(4))),
            // 64 bit registers
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
        match self
            .context
            .call(Request::Custom(MBFB_FUNCTION_CODE, Cow::Borrowed(&bytes)))
            .await
        {
            // Convert tokio_modbus::Error to TokioLabjackError
            Ok(Ok(response)) => match response {
                Response::Custom(function_code, words) => {
                    debug_assert_eq!(function_code, MBFB_FUNCTION_CODE);
                    Ok(words)
                }
                _ => unreachable!("call() should reject mismatching responses"),
            },
            // got tokio_modbus::ExceptionCode
            Ok(Err(e)) => Err(self.detailed_error_from_exception_code(e).await),
            // got tokio_modbus::Error
            Err(e) => Err(e.into()),
        }
    }

    async fn read_mbfb(&mut self, mbfb: &mut ModbusFeedbackFrame<'_>) -> Result<Bytes> {
        if !mbfb.write_addresses.is_empty() {
            return Err(TokioLabjackError::Other(
                "Write addresses should be empty in a read mbfb".into(),
            ));
        }

        if !mbfb.write_counts.is_empty() {
            return Err(TokioLabjackError::Other(
                "Write counts should be empty in a read mbfb".into(),
            ));
        }

        if !mbfb.write_data.is_empty() {
            return Err(TokioLabjackError::Other(
                "Write data should be empty in a read mbfb".into(),
            ));
        }

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
        let tags_to_read = [
            STREAM_SCANRATE_HZ.into(),
            STREAM_NUM_ADDRESSES.into(),
            STREAM_SAMPLES_PER_PACKET.into(),
            STREAM_SETTLING_US.into(),
            STREAM_RESOLUTION_INDEX.into(),
            STREAM_BUFFER_SIZE_BYTES.into(),
            STREAM_AUTO_TARGET.into(),
            STREAM_NUM_SCANS.into(),
        ];

        let result = self.read_tags(&tags_to_read).await?;

        let result_len = result.len();
        let expected_len = tags_to_read.len();
        if result_len != expected_len {
            return Err(TokioLabjackError::Other(format!(
                "Unexpected response length from read_tags: {}. Expected length of {}",
                result_len, expected_len
            )));
        }

        let scan_rate = match result[0] {
            HydratedTagValue::F32(val) => val,
            _ => panic!("scan_rate must be an F32"),
        };

        let num_addresses = match result[1] {
            HydratedTagValue::U32(val) => val,
            _ => panic!("num_addresses must be a U32"),
        };

        let samples_per_packet = match result[2] {
            HydratedTagValue::U32(val) => val,
            _ => panic!("samples_per_packet must be a U32"),
        };

        let settling_us = match result[3] {
            HydratedTagValue::F32(val) => val,
            _ => panic!("settling_us must be an F32"),
        };

        let resolution_index = match result[4] {
            HydratedTagValue::U32(val) => val,
            _ => panic!("resolution_index must be a U32"),
        };

        let buffer_size_bytes = match result[5] {
            HydratedTagValue::U32(val) => val,
            _ => panic!("buffer_size_bytes must be a U32"),
        };

        let auto_target = match result[6] {
            HydratedTagValue::U32(val) => val,
            _ => panic!("auto_target must be a U32"),
        };

        let num_scans = match result[7] {
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
        let result = config.build();
        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(TokioLabjackError::Other(format!(
                "Unable to build stream config: {}",
                e
            ))),
        }
    }

    async fn start_stream(
        &mut self,
        config: StreamConfig,
        tags: Vec<ReadableLabjackTag>,
    ) -> Result<()> {
        if tags.len() != config.num_addresses as usize {
            return Err(TokioLabjackError::Other("The number of provided tags to stream should equal num_addresses in stream config.".into()));
        }

        // Each tag address is 32 bits, which is 2 registers
        let num_registers = config.num_addresses * 2;
        // Max number of registers to write in a single modbus function 16 call is 123.
        // TODO: we can improve this by calling write_multiple_registers multiple times as
        // many times as needed. The actual limit is 128 tags streaming at once.
        if num_registers > 123 {
            return Err(TokioLabjackError::Other(format!("Max number of registers we can write to is 123, but desired is {}. Reduce number of tags to stream.", num_registers)));
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
                &u8_to_u16_vec(&data_bytes),
            )
            .await??;

        // start the stream - check that it was set to 1.
        match self
            .read_write_tags(
                &[STREAM_ENABLE.into()],
                &[STREAM_ENABLE.into()],
                &[HydratedTagValue::U32(1)],
            )
            .await
        {
            Ok(res) => {
                if res.len() != 1 {
                    Err(TokioLabjackError::Other(format!(
                        "Unexpected result from starting stream: {:?}",
                        res
                    )))
                } else if let HydratedTagValue::U32(val) = res[0] {
                    if val != 1 {
                        Err(TokioLabjackError::Other("Unable to start stream!".into()))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(TokioLabjackError::Other(format!(
                        "Unexpected result from starting stream: {:?}",
                        res[0]
                    )))
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn stop_stream(&mut self) -> Result<()> {
        // end the stream - check that it was set to 0.
        let stream_disable_result = self
            .read_write_tags(
                &[STREAM_ENABLE.into()],
                &[STREAM_ENABLE.into()],
                &[HydratedTagValue::U32(0)],
            )
            .await;

        match stream_disable_result {
            Ok(res) => {
                if res.len() != 1 {
                    Err(TokioLabjackError::Other(format!(
                        "Unexpected result from ending stream: {:?}",
                        res
                    )))
                } else if let HydratedTagValue::U32(val) = res[0] {
                    if val != 0 {
                        Err(TokioLabjackError::Other("Unable to end stream!".into()))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(TokioLabjackError::Other(format!(
                        "Unexpected result from ending stream: {:?}",
                        res[0]
                    )))
                }
            }
            Err(e) => match e {
                TokioLabjackError::LabjackError(LabjackError::StreamNotRunning) => {
                    log::debug!("Stream was already stopped, no need to stop again.");
                    Ok(())
                }
                _ => Err(e),
            },
        }
    }

    async fn read_calibrations(&mut self) -> Result<Calibrations> {
        match self.labjack_kind {
            LabjackKind::T4 => Ok(self.read_t4_calibrations().await?.into()),
            LabjackKind::T7 => Ok(self.read_t7_calibrations().await?.into()),
            _ => unimplemented!("Only t4 and t7 calibrations are currently supported."),
        }
    }

    async fn read_t4_calibrations(&mut self) -> Result<T4Calibrations> {
        match self.labjack_kind {
            LabjackKind::T4 => {}
            _ => {
                return Err(TokioLabjackError::Other(format!(
                    "{:?} not valid, expected {:?}",
                    self.labjack_kind,
                    LabjackKind::T4
                )));
            }
        }

        // Write the calibration constant starting address to INTERNAL_FLASH_READ_POINTER
        // then read all 38 registers (76 bytes) of calibration constants.
        let mut mbfb = ModbusFeedbackFrame::new(
            &[INTERNAL_FLASH_READ.address],
            &[INTERNAL_FLASH_READ_POINTER.address],
            &[38],
            &[2],
            Bytes::from(CAL_CONST_STARTING_ADDRESS.to_be_bytes().to_vec()),
        );
        let mut result = self.read_write_mbfb(&mut mbfb).await?;
        let result_len = result.len();
        let expected_len = 76;

        if result_len != expected_len {
            return Err(TokioLabjackError::Other(format!(
                "Expected to receive {} bytes of data, but received {} bytes instead.",
                expected_len, result_len
            )));
        }

        let mut t4_cals = T4CalibrationsBuilder::default().build().unwrap();

        for cal_idx in 0..4 {
            let slope = result.get_f32();
            let offset = result.get_f32();
            let ain_cal = T4AinHVCalibrationBuilder::default()
                .slope(slope)
                .offset(offset)
                .build()
                .unwrap();
            match cal_idx {
                0 => t4_cals.ain0_cal = ain_cal,
                1 => t4_cals.ain1_cal = ain_cal,
                2 => t4_cals.ain2_cal = ain_cal,
                3 => t4_cals.ain3_cal = ain_cal,
                _ => unreachable!("cal_idx should max out at 3"),
            }
        }

        let slope = result.get_f32();
        let offset = result.get_f32();
        let lv_cal = T4AinLVCalibrationBuilder::default()
            .slope(slope)
            .offset(offset)
            .build()
            .unwrap();
        t4_cals.lv_cal = lv_cal;

        let slope = result.get_f32();
        let offset = result.get_f32();
        let spec_v_cal = T4SpecVCalibrationBuilder::default()
            .slope(slope)
            .offset(offset)
            .build()
            .unwrap();
        t4_cals.spec_v_cal = spec_v_cal;

        for cal_idx in 0..2 {
            let slope = result.get_f32();
            let offset = result.get_f32();
            let dac_cal = T4DacCalibrationBuilder::default()
                .slope(slope)
                .offset(offset)
                .build()
                .unwrap();
            match cal_idx {
                0 => t4_cals.dac0_cal = dac_cal,
                1 => t4_cals.dac1_cal = dac_cal,
                _ => unreachable!("cal_idx should max out at 1"),
            }
        }

        let t_slope = result.get_f32();
        let t_offset = result.get_f32();
        let temperature_cal = TemperatureCalibrationBuilder::default()
            .slope(t_slope)
            .offset(t_offset)
            .build()
            .unwrap();
        t4_cals.temperature_cal = temperature_cal;
        t4_cals.ain_bias_current = result.get_f32();

        Ok(t4_cals)
    }

    async fn read_t7_calibrations(&mut self) -> Result<T7Calibrations> {
        match self.labjack_kind {
            LabjackKind::T7 => {}
            _ => {
                return Err(TokioLabjackError::Other(format!(
                    "{:?} not valid, expected {:?}",
                    self.labjack_kind,
                    LabjackKind::T7
                )));
            }
        }

        // Write the calibration constant starting address to INTERNAL_FLASH_READ_POINTER
        // then read all 82 registers (164 bytes) of calibration constants.
        let mut mbfb = ModbusFeedbackFrame::new(
            &[INTERNAL_FLASH_READ.address],
            &[INTERNAL_FLASH_READ_POINTER.address],
            &[82],
            &[2],
            Bytes::from(CAL_CONST_STARTING_ADDRESS.to_be_bytes().to_vec()),
        );
        let mut result = self.read_write_mbfb(&mut mbfb).await?;

        let result_len = result.len();
        let expected_len = 164;

        if result_len != expected_len {
            return Err(TokioLabjackError::Other(format!(
                "Expected to receive {} bytes of data, but received {} bytes instead.",
                expected_len, result_len
            )));
        }

        let mut t7_cals = T7CalibrationsBuilder::default().build().unwrap();

        for cal_idx in 0..8 {
            let positive_slope = result.get_f32();
            let negative_slope = result.get_f32();
            let binary_center = result.get_f32();
            let voltage_offset = result.get_f32();
            let ain_cal = T7AinCalibrationBuilder::default()
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
            let slope = result.get_f32();
            let offset = result.get_f32();
            let dac_cal = T7DacCalibrationBuilder::default()
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

        let t_slope = result.get_f32();
        let t_offset = result.get_f32();
        let temperature_cal = TemperatureCalibrationBuilder::default()
            .slope(t_slope)
            .offset(t_offset)
            .build()
            .unwrap();
        t7_cals.temperature_cal = temperature_cal;
        t7_cals.i_source_10u = result.get_f32();
        t7_cals.i_source_200u = result.get_f32();
        t7_cals.ain_bias_current = result.get_f32();

        Ok(t7_cals)
    }

    async fn read_stream_cr(&mut self, num_samples: u16) -> Result<Vec<u16>> {
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
            log::debug!("raw resultant bytes: {resultant_bytes:?}");

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
        if !mbfb.read_addresses.is_empty() {
            return Err(TokioLabjackError::Other(
                "Read addresses should be empty in a write mbfb".into(),
            ));
        }

        if !mbfb.read_counts.is_empty() {
            return Err(TokioLabjackError::Other(
                "Read counts should be empty in a write mbfb".into(),
            ));
        }

        let bytes = mbfb.to_bytes_mut();
        self.write_bytes(bytes).await
    }

    async fn write_bytes(&mut self, bytes: Bytes) -> Result<()> {
        let result = self
            .context
            .call(Request::Custom(MBFB_FUNCTION_CODE, Cow::Borrowed(&bytes)))
            .await
            .map(|result| {
                result.map(|response| match response {
                    Response::Custom(function_code, words) => {
                        debug_assert_eq!(function_code, MBFB_FUNCTION_CODE);
                        debug_assert_eq!(words.len(), 0);
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })?;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(self.detailed_error_from_exception_code(e).await),
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

    async fn get_last_error_details(&mut self) -> Result<LabjackError> {
        let error_code = LAST_ERR_DETAIL.read(self).await?;
        Ok(error_code.try_into()?)
    }

    async fn detailed_error_from_exception_code(
        &mut self,
        error: ExceptionCode,
    ) -> TokioLabjackError {
        if let Ok(better_error) = self.get_last_error_details().await {
            match better_error {
                // sometimes the error details aren't filled in, in which case
                // you get LjSuccess. In this case, we should fall back to the original
                // error returned from tokio modbus
                LabjackError::LjSuccess => TokioLabjackError::from(error),
                _ => TokioLabjackError::from(better_error),
            }
        } else {
            TokioLabjackError::from(error)
        }
    }
}
