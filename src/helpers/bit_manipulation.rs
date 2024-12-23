//! Helper functions for bit manipulation.

/// Takes a big-endian 4-byte array and converts it to a big-endian array of 2 16-bit values.
///
/// # Examples
///
/// ```
/// use async_labjack::helpers::bit_manipulation::be_bytes_to_u16_array;
///
/// let bytes = [0x98, 0x76, 0x54, 0x32];
/// assert_eq!(be_bytes_to_u16_array(bytes), [0x9876, 0x5432]);
/// ```
pub fn be_bytes_to_u16_array(bytes: [u8; 4]) -> [u16; 2] {
    [
        u16::from_be_bytes([bytes[0], bytes[1]]),
        u16::from_be_bytes([bytes[2], bytes[3]]),
    ]
}

/// Takes a big-endian array of even length and converts it to a big-endian array of 16-bit values.
///
/// If the input array is not even, then the final u8 will be placed in the MSB of the resulting u16
/// value, with the LSB filled with 0s.
///
/// # Examples
///
/// ```
/// use async_labjack::helpers::bit_manipulation::u8_to_u16_vec;
///
/// let bytes = [0xA1, 0xB2];
/// assert_eq!(u8_to_u16_vec(&bytes), [0xA1B2]);
/// ```
pub fn u8_to_u16_vec(input: &[u8]) -> Vec<u16> {
    input
        .chunks(2)
        .map(|chunk| {
            if chunk.len() == 2 {
                u16::from_be_bytes([chunk[0], chunk[1]])
            } else {
                u16::from_be_bytes([chunk[0], 0])
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_be_bytes_to_u16_array() {
        let bytes = [0, 0, 0, 0];
        assert_eq!(be_bytes_to_u16_array(bytes), [0, 0]);

        let bytes = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(be_bytes_to_u16_array(bytes), [0xFFFF, 0xFFFF]);

        let bytes = [0xAB, 0xCD, 0x01, 0x21];
        assert_eq!(be_bytes_to_u16_array(bytes), [0xABCD, 0x0121]);

        let bytes = [0x10, 0x01, 0xA0, 0x00];
        assert_eq!(be_bytes_to_u16_array(bytes), [0x1001, 0xA000]);
    }

    #[test]
    fn test_u8_to_u16_vec() {
        let bytes = [0, 0, 0, 0, 0, 0];
        assert_eq!(u8_to_u16_vec(&bytes), [0, 0, 0]);

        let bytes = [0x12, 0x34, 0x56];
        assert_eq!(u8_to_u16_vec(&bytes), [0x1234, 0x5600]);

        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(u8_to_u16_vec(&bytes), [0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF]);

        let bytes = [0xAB, 0xCD, 0x01, 0x21];
        assert_eq!(u8_to_u16_vec(&bytes), [0xABCD, 0x0121]);

        let bytes = [0x10, 0x01, 0xA0, 0x00];
        assert_eq!(u8_to_u16_vec(&bytes), [0x1001, 0xA000]);
    }
}
