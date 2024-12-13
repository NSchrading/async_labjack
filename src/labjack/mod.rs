//! structs and traits for Labjack Tags.
pub mod all_tags;
pub mod errors;

use crate::client::LabjackClient;
use crate::client::LabjackInteractions;
use crate::helpers::bit_manipulation::{be_bytes_to_u16_array, u8_to_u16_vec};
use crate::modbus_feedback::mbfb::ModbusFeedbackFrame;
use crate::Result;
use crate::Error;
use bytes::{Buf, Bytes, BytesMut};
use derive_builder::Builder;
use enum_dispatch::enum_dispatch;
use std::cmp;
use std::marker::PhantomData;
use std::str;
use tokio::time::timeout;
use tokio_modbus::client::Writer;
use tokio_modbus::prelude::Reader;

/// Tags with this type can be read and will have `read` implementations.
pub struct CanRead;

/// Tags with this type can be written and will have `write` implementations.
pub struct CanWrite;

/// Tags with this type cannot be read, but may be writeable if they have `CanWrite`.
pub struct CannotRead;

/// Tags with this type cannot be written, but may be readable if they have `CanRead`.
pub struct CannotWrite;
/// A generic struct holding the address of a labjack tag (register). Because
/// register is an overloaded term that could mean an individual modbus register, we refer to
/// the complete value containing 1 or more underlying modbus registers as a tag.
/// `T` is the type of data that the tag holds. Currently one of `u16`, `u32`, `u64`, `i32`, `f32`,
/// or `Bytes`.
///
/// `R` is `CanRead` or `CannotRead`
///
/// `W` is `CanWrite` or `CannotWrite`
pub struct LabjackTag<T, R, W> {
    pub address: u16,
    _phantom_data: PhantomData<(T, R, W)>, // To differentiate types at compile time
}

impl<T, R, W> LabjackTag<T, R, W> {
    /// Constructs a new [`LabjackTag<T, R, W>`] with an address. Normally you will not need
    /// to construct your own tag, and can instead use the pre-defined tags imported from
    /// async_labjack_lib.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_labjack::labjack::{CanRead, CannotWrite, LabjackTag};
    ///
    /// let AIN0: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(0);
    /// ```
    pub const fn new(address: u16) -> LabjackTag<T, R, W> {
        LabjackTag::<T, R, W> {
            address,
            _phantom_data: PhantomData,
        }
    }
}

impl<W> LabjackTag<u64, CanRead, W> {
    /// Read the tag asynchronously and return a future holding a `Result<u64>`.
    pub async fn read(self, client: &mut LabjackClient) -> Result<u64> {
        // fetch the data, it is returned in big endian
        let result = timeout(
            client.command_response_timeout,
            client.context.read_input_registers(self.address, 4),
        )
        .await??;

        match result {
            Ok(data) => {
                // Combine the four u16s into a single u64 in big endian
                Ok((u64::from(data[0]) << 48)
                    | (u64::from(data[1]) << 32)
                    | (u64::from(data[2]) << 16)
                    | u64::from(data[3]))
            }
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<R> LabjackTag<f32, R, CanWrite> {
    /// Write an f32 to the tag asynchronously and return a future holding a `Result<()>`.
    pub async fn write(self, client: &mut LabjackClient, val: f32) -> Result<()> {
        let result = timeout(
            client.command_response_timeout,
            client
                .context
                .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes())),
        )
        .await??;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<W> LabjackTag<f32, CanRead, W> {
    /// Read the tag asynchronously and return a future holding a `Result<f32>`.
    pub async fn read(self, client: &mut LabjackClient) -> Result<f32> {
        // fetch the data, it is returned in big endian
        let result = timeout(
            client.command_response_timeout,
            client.context.read_input_registers(self.address, 2),
        )
        .await??;

        match result {
            Ok(data) => {
                // Combine the two u16s into a single u32 in big endian
                let combined_value = (u32::from(data[0]) << 16) | u32::from(data[1]);
                // Convert the u32 to f32
                Ok(f32::from_bits(combined_value))
            }
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<R> LabjackTag<i32, R, CanWrite> {
    /// Write an i32 to the tag asynchronously and return a future holding a `Result<()>`.
    pub async fn write(self, client: &mut LabjackClient, val: i32) -> Result<()> {
        let result = timeout(
            client.command_response_timeout,
            client
                .context
                .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes())),
        )
        .await??;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<W> LabjackTag<i32, CanRead, W> {
    /// Read the tag asynchronously and return a future holding a `Result<i32>`.
    pub async fn read(self, client: &mut LabjackClient) -> Result<i32> {
        // fetch the data, it is returned in big endian
        let result = timeout(
            client.command_response_timeout,
            client.context.read_input_registers(self.address, 2),
        )
        .await??;

        match result {
            Ok(data) => {
                // Combine the two u16s into a single u32 in big endian
                let combined_value = (u32::from(data[0]) << 16) | u32::from(data[1]);
                // Convert the u32 to i32
                Ok(i32::from_be_bytes(combined_value.to_be_bytes()))
            }
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<R> LabjackTag<u32, R, CanWrite> {
    /// Write a u32 to the tag asynchronously and return a future holding a `Result<()>`.
    pub async fn write(self, client: &mut LabjackClient, val: u32) -> Result<()> {
        let result = timeout(
            client.command_response_timeout,
            client
                .context
                .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes())),
        )
        .await??;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<W> LabjackTag<u32, CanRead, W> {
    /// Read the tag asynchronously and return a future holding a `Result<u32>`.
    pub async fn read(self, client: &mut LabjackClient) -> Result<u32> {
        // fetch the data, it is returned in big endian
        let result = timeout(
            client.command_response_timeout,
            client.context.read_input_registers(self.address, 2),
        )
        .await??;

        match result {
            Ok(data) => {
                // Combine the two u16s into a single u32 in big endian
                Ok((u32::from(data[0]) << 16) | u32::from(data[1]))
            }
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<W> LabjackTag<u16, CanRead, W> {
    /// Read the tag asynchronously and return a future holding a `Result<u16>`.
    pub async fn read(self, client: &mut LabjackClient) -> Result<u16> {
        // fetch the data, it is returned in big endian
        let result = timeout(
            client.command_response_timeout,
            client.context.read_input_registers(self.address, 1),
        )
        .await??;

        match result {
            Ok(data) => Ok(data[0]),
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<R> LabjackTag<u16, R, CanWrite> {
    /// Write a u16 to the tag asynchronously and return a future holding a `Result<()>`.
    pub async fn write(self, client: &mut LabjackClient, val: u16) -> Result<()> {
        let result = timeout(
            client.command_response_timeout,
            client.context.write_single_register(self.address, val),
        )
        .await??;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

impl<W> LabjackTag<Bytes, CanRead, W> {
    /// Read a specified number of bytes from the tag asynchronously and return a future holding a
    /// `Result<Bytes>`. This is valid for Labjack [Buffer Registers](https://support.labjack.com/docs/3-1-modbus-map-t-series-datasheet#id-3.1ModbusMap[T-SeriesDatasheet]-BufferRegisters).
    /// Note: This may incur multiple read calls to the underlying Labjack device depending on how
    /// many bytes need to be read.
    pub async fn read(self, client: &mut LabjackClient, num_bytes: u32) -> Result<Bytes> {
        // Max ethernet packet size is 1040 bytes, keeping this at 1020 bytes accounts for overhead
        // from the MBFB response packet
        const MAX_BYTES_PER_CALL: u16 = 1020;

        // Maximum number of registers you can read from a single starting address in an mbfb frame
        const MAX_REGISTERS: u8 = 255;

        let mut total_bytes_to_read = num_bytes;
        let mut data_bytes = BytesMut::with_capacity(num_bytes as usize);

        while total_bytes_to_read > 0 {
            // as u16 will always be safe here because it's limited by MAX_BYTES_PER_CALL
            let cur_num_bytes_to_read =
                cmp::min(total_bytes_to_read, MAX_BYTES_PER_CALL.into()) as u16;
            // each register is 16 bits (2-bytes)
            let mut total_registers_to_read = (cur_num_bytes_to_read + 1) / 2;
            let mut addresses: Vec<u16> = Vec::new();
            let mut register_counts: Vec<u8> = Vec::new();

            // You can only read a max of 255 registers from a single starting address, which would
            // be 510 bytes. But you can get a total of ~1020 bytes in a single mbfb response.
            // This means you can construct a frame with the starting address specified twice and
            // both times reading up to 255 registers for a total of 1020 bytes, e.g.:
            //
            // Frame Byte 0: 0 (Frame Type)
            // Frame Byte 1-2: <starting address>
            // Frame Byte 3: 255
            // Frame Byte 0: 0 (Frame Type)
            // Frame Byte 1-2: <starting address>
            // Frame Byte 3: 255
            while total_registers_to_read > 0 {
                let cur_num_registers_to_read =
                    cmp::min(total_registers_to_read, MAX_REGISTERS.into());
                addresses.push(self.address);
                // as u8 will always be safe here because it's limited by MAX_REGISTERS
                register_counts.push(cur_num_registers_to_read as u8);
                total_registers_to_read -= cur_num_registers_to_read;
            }

            let mut mbfb = ModbusFeedbackFrame::new_read_frame(&addresses, &register_counts);

            let result =
                timeout(client.command_response_timeout, client.read_mbfb(&mut mbfb)).await??;
            tracing::debug!("total num bytes read from read_mbfb: {}", result.len());
            total_bytes_to_read = total_bytes_to_read.saturating_sub(result.len() as u32);
            tracing::debug!("Still need to read {total_bytes_to_read} bytes");

            data_bytes.extend_from_slice(&result);
        }

        // If we want a non-even number of bytes, we need to pop off the last byte because
        // read_frame_bytes will always return a whole number of 16-bit registers.
        if num_bytes % 2 != 0 {
            data_bytes.truncate(data_bytes.len() - 1);
        }
        Ok(data_bytes.freeze())
    }

    /// Read a specified number of bytes from the tag asynchronously and return a future holding a
    /// `Result<String>`. The resultant String must be valid utf8.
    /// This is valid for Labjack [Buffer Registers](https://support.labjack.com/docs/3-1-modbus-map-t-series-datasheet#id-3.1ModbusMap[T-SeriesDatasheet]-BufferRegisters).
    /// Note: This may incur multiple read calls to the underlying Labjack device depending on how
    /// many bytes need to be read.
    pub async fn read_string(self, client: &mut LabjackClient, len: u32) -> Result<String> {
        let mut bytes = self.read(client, len).await?;
        // The bytes returned will have a null byte (c-string)
        bytes.truncate(bytes.len() - 1);
        let str_slice = str::from_utf8(&bytes)?;
        Ok(str_slice.to_string())
    }

    /// Read a specified number of bytes from the file tag asynchronously and return a future holding a
    /// `Result<String>`. The resultant String must be valid utf8.
    /// This is valid for Labjack [Buffer Registers](https://support.labjack.com/docs/3-1-modbus-map-t-series-datasheet#id-3.1ModbusMap[T-SeriesDatasheet]-BufferRegisters).
    /// Note: This may incur multiple read calls to the underlying Labjack device depending on how
    /// many bytes need to be read.
    pub async fn read_file(self, client: &mut LabjackClient, len: u32) -> Result<String> {
        let bytes = self.read(client, len).await?;
        let str_slice = str::from_utf8(&bytes)?;
        Ok(str_slice.to_string())
    }
}

impl<R> LabjackTag<Bytes, R, CanWrite> {
    /// Write the specified Bytes to the tag asynchronously and return a future holding a Result.
    pub async fn write(self, client: &mut LabjackClient, val: Bytes) -> Result<()> {
        let result = timeout(
            client.command_response_timeout,
            client
                .context
                .write_multiple_registers(self.address, &u8_to_u16_vec(&val)),
        )
        .await??;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(client.detailed_error_from_exception_code(e).await),
        }
    }
}

/// All labjack tags are Addressable, meaning they have an address and a certain number of
/// registers holding their value. These traits exist so that we can pass a dynamic vector
/// of varying labjack tags to [`LabjackClient::read_tags`] or [`LabjackClient::write_tags`] and
/// then either return the appropriate [`HydratedTagValue`] or write the appropriate values
/// to the correct registers.
#[enum_dispatch]
pub trait Addressable {
    /// Return the register count expected of the Addressable labjack tag.
    fn register_count(&self) -> u8;

    /// Return the address expected of the Addressable labjack tag.
    fn address(&self) -> u16;
}

/// Any Readable labjack tag is capable of being read and turned into a HydratedTagValue.
#[enum_dispatch]
pub trait Readable: Addressable {
    /// Take bytes and convert them into the type specified by the Readable labjack tag,
    /// returning a HydratedTagValue.
    fn hydrate(&self, bytes: &mut Bytes) -> HydratedTagValue;
}

impl<R, W> Addressable for LabjackTag<u64, R, W> {
    /// 64 bit labjack tags have 4 16-bit registers.
    fn register_count(&self) -> u8 {
        4
    }
    fn address(&self) -> u16 {
        self.address
    }
}

impl<W> Readable for LabjackTag<u64, CanRead, W> {
    fn hydrate(&self, bytes: &mut Bytes) -> HydratedTagValue {
        HydratedTagValue::U64(bytes.get_u64())
    }
}

impl<R, W> Addressable for LabjackTag<f32, R, W> {
    /// 32 bit labjack tags have 2 16-bit registers.
    fn register_count(&self) -> u8 {
        2
    }
    fn address(&self) -> u16 {
        self.address
    }
}

impl<W> Readable for LabjackTag<f32, CanRead, W> {
    fn hydrate(&self, bytes: &mut Bytes) -> HydratedTagValue {
        HydratedTagValue::F32(bytes.get_f32())
    }
}

impl<R, W> Addressable for LabjackTag<i32, R, W> {
    /// 32 bit labjack tags have 2 16-bit registers.
    fn register_count(&self) -> u8 {
        2
    }
    fn address(&self) -> u16 {
        self.address
    }
}

impl<W> Readable for LabjackTag<i32, CanRead, W> {
    fn hydrate(&self, bytes: &mut Bytes) -> HydratedTagValue {
        HydratedTagValue::I32(bytes.get_i32())
    }
}

impl<R, W> Addressable for LabjackTag<u32, R, W> {
    /// 32 bit labjack tags have 2 16-bit registers.
    fn register_count(&self) -> u8 {
        2
    }
    fn address(&self) -> u16 {
        self.address
    }
}

impl<W> Readable for LabjackTag<u32, CanRead, W> {
    fn hydrate(&self, bytes: &mut Bytes) -> HydratedTagValue {
        HydratedTagValue::U32(bytes.get_u32())
    }
}

impl<R, W> Addressable for LabjackTag<u16, R, W> {
    /// 16 bit labjack tags have 1 16-bit register.
    fn register_count(&self) -> u8 {
        1
    }
    fn address(&self) -> u16 {
        self.address
    }
}

impl<W> Readable for LabjackTag<u16, CanRead, W> {
    fn hydrate(&self, bytes: &mut Bytes) -> HydratedTagValue {
        HydratedTagValue::U16(bytes.get_u16())
    }
}

/// A HydratedTagValue simply holds the underlying value for a tag, e.g. a u32 for a
/// `LabjackTag<u32, CanRead, W>`
#[derive(Debug, PartialEq)]
pub enum HydratedTagValue {
    U64(u64),
    F32(f32),
    I32(i32),
    U32(u32),
    U16(u16),
}

impl TryInto<f32> for &HydratedTagValue {
    type Error = Error;

    fn try_into(self) -> Result<f32> {
        match self {
            &HydratedTagValue::F32(val) => Ok(val),
            _ => Err(Error::Other(format!(
                "Expected F32, got {:?}",
                self
            ))),
        }
    }
}

impl TryInto<i32> for &HydratedTagValue {
    type Error = Error;

    fn try_into(self) -> Result<i32> {
        match self {
            &HydratedTagValue::I32(val) => Ok(val),
            _ => Err(Error::Other(format!(
                "Expected I32, got {:?}",
                self
            ))),
        }
    }
}

impl TryInto<u64> for &HydratedTagValue {
    type Error = Error;

    fn try_into(self) -> Result<u64> {
        match self {
            &HydratedTagValue::U64(val) => Ok(val),
            _ => Err(Error::Other(format!(
                "Expected U64, got {:?}",
                self
            ))),
        }
    }
}

impl TryInto<u32> for &HydratedTagValue {
    type Error = Error;

    fn try_into(self) -> Result<u32> {
        match self {
            &HydratedTagValue::U32(val) => Ok(val),
            _ => Err(Error::Other(format!(
                "Expected U32, got {:?}",
                self
            ))),
        }
    }
}

impl TryInto<u16> for &HydratedTagValue {
    type Error = Error;

    fn try_into(self) -> Result<u16> {
        match self {
            &HydratedTagValue::U16(val) => Ok(val),
            _ => Err(Error::Other(format!(
                "Expected U16, got {:?}",
                self
            ))),
        }
    }
}

/// An enum of all possible writable LabjackTags. This is used in the dynamic
/// [`LabjackClient::write_tags`] implementation. Note that this uses enum_dispatch
/// so that we avoid dynamic dispatch with dyn.
#[enum_dispatch(Addressable)]
pub enum WritableLabjackTag {
    F32WriteOnly(LabjackTag<f32, CannotRead, CanWrite>),
    F32ReadWrite(LabjackTag<f32, CanRead, CanWrite>),

    I32WriteOnly(LabjackTag<i32, CannotRead, CanWrite>),
    I32ReadWrite(LabjackTag<i32, CanRead, CanWrite>),

    U32WriteOnly(LabjackTag<u32, CannotRead, CanWrite>),
    U32ReadWrite(LabjackTag<u32, CanRead, CanWrite>),

    U16WriteOnly(LabjackTag<u16, CannotRead, CanWrite>),
    U16ReadWrite(LabjackTag<u16, CanRead, CanWrite>),
}

/// An enum of all possible readable LabjackTags. This is used in the dynamic
/// [`LabjackClient::read_tags`] implementation. Note that this uses enum_dispatch
/// so that we avoid dynamic dispatch with dyn.
#[enum_dispatch(Addressable)]
#[enum_dispatch(Readable)]
pub enum ReadableLabjackTag {
    U64ReadOnly(LabjackTag<u64, CanRead, CannotWrite>),
    U64ReadWrite(LabjackTag<u64, CanRead, CanWrite>),

    F32ReadOnly(LabjackTag<f32, CanRead, CannotWrite>),
    F32ReadWrite(LabjackTag<f32, CanRead, CanWrite>),

    I32ReadOnly(LabjackTag<i32, CanRead, CannotWrite>),
    I32ReadWrite(LabjackTag<i32, CanRead, CanWrite>),

    U32ReadOnly(LabjackTag<u32, CanRead, CannotWrite>),
    U32ReadWrite(LabjackTag<u32, CanRead, CanWrite>),

    U16ReadOnly(LabjackTag<u16, CanRead, CannotWrite>),
    U16ReadWrite(LabjackTag<u16, CanRead, CanWrite>),
}

/// A struct holding stream configuration information. This is passed to
/// [`LabjackClient::start_stream`] to configure stream mode properly on the labjack.
#[derive(Builder, Debug, PartialEq)]
pub struct StreamConfig {
    /// Scans per second. Samples per second = scanRate * numAddresses
    #[builder(default = 1000.0)]
    pub scan_rate: f32,
    /// Required field. Number of addresses to include in the scan list.
    pub num_addresses: u32,
    /// Number of samples sent in each packet from the labjack. Max is 512.
    /// The labjack will buffer the data internally until this is reached.
    #[builder(default = 512)]
    pub samples_per_packet: u32,
    /// Time in microseconds to allow signals to settle after switching the mux.
    /// Does not apply to the 1st channel in the scan list, as that settling is
    /// controlled by scan rate (the time from the last channel until the start of the next scan).
    /// Default = 0. When set to less than 1, automatic settling will be used.
    /// The automatic settling behavior varies by device. Ignored for T8.
    #[builder(default = 0.0)]
    pub settling_us: f32,
    /// How accurately to sample when streaming.
    ///
    /// Valid values:
    /// - T8: 0-16. 0 will use the best resolution for the specified data rate.
    /// - T7: 0-8. Default value of 0 corresponds to an index of 1.
    /// - T4: 0-5. Default value of 0 corresponds to an index of 1.
    #[builder(default = 0)]
    pub resolution_index: u32,
    /// Size of the stream data buffer in bytes. A value of 0 equates to the default value.
    /// Must be a power of 2. Size in samples is STREAM_BUFFER_SIZE_BYTES / 2.
    /// Size in scans is (STREAM_BUFFER_SIZE_BYTES / 2) / STREAM_NUM_ADDRESSES.
    /// Changes while stream is running do not affect the currently running stream.
    /// T8: Max size is 262144. Default size is 4096.
    /// T7: Max size is 32768. Default size is 4096.
    /// T4: Max size is 32768. Default size is 8192.
    #[builder(default = 0)]
    pub buffer_size_bytes: u32,
    /// Controls where data will be sent. Value is a bitmask.
    /// bit 0: 1 = Send to Ethernet 702 sockets,
    /// bit 1: 1 = Send to USB,
    /// bit 4: 1 = Command-Response mode.
    /// All other bits are reserved.
    #[builder(default = 1)]
    pub auto_target: u32,
    /// How many scans to perform before ending. This is 'stream burst mode' if > 0. If 0, the
    /// scan runs continuously.
    #[builder(default = 0)]
    pub num_scans: u32,
}
