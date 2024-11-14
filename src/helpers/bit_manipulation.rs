//! Helper functions for bit manipulation.

use anyhow::{bail, Result};

/// Takes a big-endian 4-byte array and converts it to a big-endian array of 2 16-bit values.
///
/// # Examples
///
/// ```
/// use tokio_labjack_lib::helpers::bit_manipulation::be_bytes_to_u16_array;
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
/// Returns an error if the input array is not even.
///
/// # Examples
///
/// ```
/// use tokio_labjack_lib::helpers::bit_manipulation::u8_to_u16_vec;
/// let bytes = [0xA1, 0xB2];
/// assert_eq!(u8_to_u16_vec(&bytes).unwrap(), [0xA1B2]);
/// ```
pub fn u8_to_u16_vec(input: &[u8]) -> Result<Vec<u16>> {
    if input.len() % 2 != 0 {
        bail!(
            "Input vector must have an even length, but length was {}",
            input.len()
        );
    }
    Ok(input
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect())
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
        assert_eq!(u8_to_u16_vec(&bytes).unwrap(), [0, 0, 0]);

        let bytes = [1, 2, 3];
        assert_eq!(
            u8_to_u16_vec(&bytes).unwrap_err().to_string(),
            format!(
                "Input vector must have an even length, but length was {}",
                bytes.len()
            )
        );

        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(
            u8_to_u16_vec(&bytes).unwrap(),
            [0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF]
        );

        let bytes = [0xAB, 0xCD, 0x01, 0x21];
        assert_eq!(u8_to_u16_vec(&bytes).unwrap(), [0xABCD, 0x0121]);

        let bytes = [0x10, 0x01, 0xA0, 0x00];
        assert_eq!(u8_to_u16_vec(&bytes).unwrap(), [0x1001, 0xA000]);
    }
}
