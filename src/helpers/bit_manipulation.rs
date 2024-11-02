pub fn be_bytes_to_u16_array(bytes: [u8; 4]) -> [u16; 2] {
    [
        u16::from_be_bytes([bytes[0], bytes[1]]),
        u16::from_be_bytes([bytes[2], bytes[3]]),
    ]
}

pub fn u8_to_u16_vec(input: &[u8]) -> Vec<u16> {
    input
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect()
}
