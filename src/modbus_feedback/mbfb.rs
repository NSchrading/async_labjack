use crate::helpers::bit_manipulation::u8_to_u16_vec;
use std::borrow::Cow;
use std::iter::zip;
use tokio_modbus::client::{Client, Context};
use tokio_modbus::prelude::{Request, Response};
use tokio_modbus::Address;
use tokio_modbus::Result;

pub trait CustomReader: Client {
    /// Read multiple frames using custom MBFB modbus function implemented in labjacks (0x4C)
    /// https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76)
    async fn read_frame_bytes(&mut self, addr: &[Address], cnt: &[u8]) -> Result<Vec<u8>>;
    async fn read_frames(&mut self, addr: &[Address], cnt: &[u8]) -> Result<Vec<u16>>;
}

impl CustomReader for Context {
    async fn read_frame_bytes(&mut self, addr: &[Address], cnt: &[u8]) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        for (address, count) in zip(addr, cnt) {
            bytes.push(0);
            bytes.extend(address.to_be_bytes());
            bytes.push(*count);
        }

        println!("bytes: {bytes:?}");

        self.call(Request::Custom(0x4C, Cow::Borrowed(&bytes)))
            .await
            .map(|result| {
                result.map_err(Into::into).map(|response| match response {
                    Response::Custom(function_code, words) => {
                        debug_assert_eq!(function_code, 0x4C);
                        debug_assert_eq!(
                            words.len(),
                            cnt.iter().map(|&val| (val as usize) * 2).sum()
                        );
                        words.as_ref().to_vec()
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })
    }

    async fn read_frames(&mut self, addr: &[Address], cnt: &[u8]) -> Result<Vec<u16>> {
        self.read_frame_bytes(addr, cnt)
            .await
            .map(|result| result.map(|response| u8_to_u16_vec(&response)))
    }
}
