//! Functions for constructing and interacting with the custom
//! [Labjack Modbus Feedback function](https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76))
use crate::{Result, TokioLabjackError};
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
    /// Constructs a new [`ModbusFeedbackFrame`].
    ///
    /// # Examples
    /// ```
    /// use tokio_labjack::{AIN0, AIN1, TEST_FLOAT32};
    /// use bytes::Bytes;
    /// use tokio_labjack::modbus_feedback::mbfb::{ModbusFeedbackFrame};
    ///
    /// // A feedback frame that will read AIN0 and AIN1 and write 5.4321 to TEST_FLOAT32.
    /// let mut mbfb = ModbusFeedbackFrame::new(
    ///     &[AIN0.address, AIN1.address],
    ///     &[TEST_FLOAT32.address],
    ///     &[2, 2],
    ///     &[2],
    ///     Bytes::from_iter(5.4321_f32.to_be_bytes().into_iter())
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

    /// Constructs a new [`ModbusFeedbackFrame`] for reading only.
    ///
    /// # Examples
    /// ```
    /// use tokio_labjack::{AIN0, AIN1};
    /// use tokio_labjack::modbus_feedback::mbfb::{ModbusFeedbackFrame};
    ///
    /// // A feedback frame that will read AIN0 and AIN1.
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

    /// Constructs a new [`ModbusFeedbackFrame`] for writing only.
    ///
    /// # Examples
    /// ```
    /// use tokio_labjack::{TEST_FLOAT32, TEST_INT32};
    /// use bytes::Bytes;
    /// use tokio_labjack::modbus_feedback::mbfb::{ModbusFeedbackFrame};
    ///
    /// // A feedback frame that will write 5.4321 to TEST_FLOAT32 and -314 to TEST_INT32.
    /// let mut mbfb = ModbusFeedbackFrame::new_write_frame(
    ///     &[TEST_FLOAT32.address, TEST_INT32.address],
    ///     &[2, 2],
    ///     Bytes::from_iter(
    ///        [5.4321_f32.to_be_bytes(), (-314_i32).to_be_bytes()]
    ///            .into_iter()
    ///            .flatten(),
    ///     )
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

    /// Convert the [`ModbusFeedbackFrame`] to [`Bytes`]. Mutable because it affects the underlying
    /// `write_data` [`Bytes`] in the [`ModbusFeedbackFrame`].
    ///
    /// This function adds write bytes before read bytes. This allows for single operations that
    /// write to data and then return their newly written values as reads.
    pub fn to_bytes_mut(&mut self) -> Result<Bytes> {
        let mut bytes = BytesMut::with_capacity(
            self.read_addresses.len() * 4 + self.write_addresses.len() * 4 + self.write_data.len(),
        );
        for (address, num_registers) in zip(self.write_addresses, self.write_counts) {
            bytes.put_u8(1);
            bytes.put_u16(*address);
            bytes.put_u8(*num_registers);
            match *num_registers {
                1 => bytes.put_u16(self.write_data.get_u16()),
                2 => bytes.put_u32(self.write_data.get_u32()),
                _ => {
                    return Err(TokioLabjackError::Other(
                        "There should never be a writeable tag with a register count not equal to 1 or 2."
                            .into(),
                    ))
                }
            }
        }

        for (address, count) in zip(self.read_addresses, self.read_counts) {
            bytes.put_u8(0);
            bytes.put_u16(*address);
            bytes.put_u8(*count);
        }
        tracing::debug!("Raw bytes of mbfb: {bytes:?}");

        Ok(bytes.freeze())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbfb_to_bytes_mut_read_only() {
        let read_addresses = [1, 2, 3];
        let read_counts = [1, 2, 4];

        let mut mbfb = ModbusFeedbackFrame::new_read_frame(&read_addresses, &read_counts);
        let bytes = mbfb.to_bytes_mut().unwrap();

        // Expected bytes:
        // 0x00 0x00 0x01 0x01  (read address 1, count 1)
        // 0x00 0x00 0x02 0x02  (read address 2, count 2)
        // 0x00 0x00 0x03 0x04  (read address 3, count 4)
        let expected: Vec<u8> = vec![
            0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x03, 0x04,
        ];
        assert_eq!(bytes, Bytes::from(expected));
    }

    #[test]
    fn test_mbfb_to_bytes_mut_write_only() {
        let write_addresses = [1, 2];
        let write_counts = [1, 2];
        let write_data = Bytes::from(vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]);

        let mut mbfb =
            ModbusFeedbackFrame::new_write_frame(&write_addresses, &write_counts, write_data);
        let bytes = mbfb.to_bytes_mut().unwrap();

        // Expected bytes:
        // 0x01 0x00 0x01 0x01 0x12 0x34  (write address 1, count 1, data 0x1234)
        // 0x01 0x00 0x02 0x02 0x56 0x78 0x9A 0xBC  (write address 2, count 2, data 0x56789ABC)
        let expected: Vec<u8> = vec![
            0x01, 0x00, 0x01, 0x01, 0x12, 0x34, 0x01, 0x00, 0x02, 0x02, 0x56, 0x78, 0x9A, 0xBC,
        ];
        assert_eq!(bytes, Bytes::from(expected));
    }

    #[test]
    fn test_mbfb_to_bytes_mut_read_write() {
        let read_addresses = [1, 2];
        let write_addresses = [3, 4];
        let read_counts = [1, 2];
        let write_counts = [2, 1];
        let write_data = Bytes::from(vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]);

        let mut mbfb = ModbusFeedbackFrame::new(
            &read_addresses,
            &write_addresses,
            &read_counts,
            &write_counts,
            write_data,
        );
        let bytes = mbfb.to_bytes_mut().unwrap();

        // Expected bytes:
        // 0x01 0x00 0x03 0x02 0x12 0x34 0x56 0x78  (write address 3, count 2, data 0x12345678)
        // 0x01 0x00 0x04 0x01 0x9A 0xBC            (write address 4, count 1, data 0x9ABC)
        // 0x00 0x00 0x01 0x01                      (read address 1, count 1)
        // 0x00 0x00 0x02 0x02                      (read address 2, count 2)
        let expected: Vec<u8> = vec![
            0x01, 0x00, 0x03, 0x02, 0x12, 0x34, 0x56, 0x78, 0x01, 0x00, 0x04, 0x01, 0x9A, 0xBC,
            0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x02, 0x02,
        ];
        assert_eq!(bytes, Bytes::from(expected));
    }

    #[test]
    fn test_invalid_mbfb() {
        let write_addresses = [1];
        let write_counts = [3];
        let write_data = Bytes::from(vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]);

        let mut mbfb =
            ModbusFeedbackFrame::new_write_frame(&write_addresses, &write_counts, write_data);

        let res = mbfb.to_bytes_mut();
        match res {
            Err(e) => {
                assert_eq!(e.to_string(), "There should never be a writeable tag with a register count not equal to 1 or 2.")
            }
            Ok(_) => {
                panic!("Expected to receive an error result from to_bytes_mut!")
            }
        };
    }
}
