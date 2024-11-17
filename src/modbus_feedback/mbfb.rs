//! Functions for constructing and interacting with the custom [Labjack Modbus Feedback function](https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76))
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::iter::zip;
use tokio_modbus::Address;

/// A struct holding all of the information needed to build a Modbus Feedback Frame
/// and convert it to bytes for communication with the mbfb function.
pub struct ModbusFeedbackFrame<'a> {
    /// The addresses to read from.
    pub read_addresses: &'a [Address],
    /// The addresses to write to.
    pub write_addresses: &'a [Address],
    /// The number of registers to read per read_address, length must match the length of
    /// read_addresses.
    pub read_counts: &'a [u8],
    /// The number of registers to write per write_address, length must match the length of
    /// write_addresses.
    pub write_counts: &'a [u8],
    /// The data to be written to the write_addresses.
    pub write_data: Bytes,
}

impl<'a> ModbusFeedbackFrame<'a> {
    /// Constructs a new `ModbusFeedbackFrame`.
    ///
    /// # Examples
    /// ```
    /// use tokio_labjack_lib::{AIN0, AIN1, TEST_FLOAT32};
    /// use bytes::Bytes;
    /// use tokio_labjack_lib::modbus_feedback::mbfb::{ModbusFeedbackFrame};
    /// // A feedback frame that will read AIN0 and AIN1 and write 5.4321 to TEST_FLOAT32.
    /// let mut mbfb = ModbusFeedbackFrame::new(
    ///     &[AIN0.address, AIN1.address],
    ///     &[TEST_FLOAT32.address],
    ///     &[2, 2],
    ///     &[2],
    ///     Bytes::from(&[0x40_u8, 0xad, 0xd3, 0xc3][..])
    /// );
    /// ```
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

    /// Constructs a new `ModbusFeedbackFrame` for reading only.
    ///
    /// # Examples
    /// ```
    /// use tokio_labjack_lib::{AIN0, AIN1};
    /// use tokio_labjack_lib::modbus_feedback::mbfb::{ModbusFeedbackFrame};
    /// // A feedback frame that will read AIN0 and AIN1 and write 5.4321 to TEST_FLOAT32.
    /// let mut mbfb = ModbusFeedbackFrame::new_read_frame(
    ///     &[AIN0.address, AIN1.address],
    ///     &[2, 2],
    /// );
    /// ```
    pub fn new_read_frame(read_addresses: &'a [Address], read_counts: &'a [u8]) -> Self {
        Self {
            read_addresses,
            write_addresses: &[],
            read_counts,
            write_counts: &[],
            write_data: Bytes::new(),
        }
    }

    /// Constructs a new `ModbusFeedbackFrame` for writing only.
    ///
    /// # Examples
    /// ```
    /// use tokio_labjack_lib::{TEST_FLOAT32, TEST_INT32};
    /// use bytes::Bytes;
    /// use tokio_labjack_lib::modbus_feedback::mbfb::{ModbusFeedbackFrame};
    /// // A feedback frame that will write 5.4321 to TEST_FLOAT32 and -314 to TEST_INT32.
    /// let mut bytes_vec = 5.4321_f32.to_be_bytes().to_vec();
    /// bytes_vec.extend((-314_i32).to_be_bytes());
    /// let mut mbfb = ModbusFeedbackFrame::new_write_frame(
    ///     &[TEST_FLOAT32.address, TEST_INT32.address],
    ///     &[2, 2],
    ///     Bytes::from(bytes_vec),
    /// );
    /// ```
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

    /// Convert the `ModbusFeedbackFrame` to `Bytes`. Mutable because it affects the underlying
    /// `write_data` Bytes in the `ModbusFeedbackFrame`.
    ///
    /// This function adds write bytes before read bytes. This allows for single operations that
    /// write to data and then return their newly written values as reads.
    pub fn to_bytes_mut(&mut self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(
            self.read_addresses.len() * 4 + self.write_addresses.len() * 4 + self.write_data.len(),
        );
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
                    "There should never be a writeable tag with a register count not equal to 1
                    or 2."
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
