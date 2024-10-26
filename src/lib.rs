use std::borrow::Cow;
use std::cmp;
use std::ffi::CString;
use std::io::Read;
use std::iter::zip;
use std::marker::PhantomData;
use tokio_modbus::client::{Client, Context, Writer};
use tokio_modbus::prelude::Reader;
use tokio_modbus::prelude::Request;
use tokio_modbus::prelude::Response;
use tokio_modbus::Address;
use tokio_modbus::Quantity;
use tokio_modbus::Result;

pub enum LabjackType {
    UINT16 = 0,
    UINT32 = 1,
    INT32 = 2,
    FLOAT32 = 3,
    LJM_STRING = 98,
    LJM_BYTE = 99,
}

fn be_bytes_to_u16_array(bytes: [u8; 4]) -> [u16; 2] {
    [
        u16::from_be_bytes([bytes[0], bytes[1]]),
        u16::from_be_bytes([bytes[2], bytes[3]]),
    ]
}

pub fn u16_to_u8_vec(input: &[u16]) -> Vec<u8> {
    input
        .iter()
        .flat_map(|x| x.to_be_bytes().to_vec())
        .collect()
}

fn u8_to_u16_vec(input: &[u8]) -> Vec<u16> {
    input
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect()
}

pub struct CanRead;
pub struct CanWrite;
pub struct CannotRead;
pub struct CannotWrite;

pub struct LabjackTag<T, R, W> {
    address: u16,
    _phantom_data: PhantomData<(T, R, W)>, // To differentiate types at compile time
}

impl<T, R, W> LabjackTag<T, R, W> {
    pub const fn new(address: u16) -> LabjackTag<T, R, W> {
        LabjackTag::<T, R, W> {
            address: address,
            _phantom_data: PhantomData,
        }
    }
}

impl<R> LabjackTag<f32, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: f32) -> () {
        context
            .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes()))
            .await
            .unwrap()
            .unwrap();
    }
}

impl<W> LabjackTag<f32, CanRead, W> {
    pub async fn read(self, context: &mut Context) -> f32 {
        // fetch the data, it is returned in big endian
        let data: Vec<u16> = context
            .read_input_registers(self.address, 2)
            .await
            .unwrap()
            .unwrap();
        // Combine the two u16s into a single u32 in big endian
        let combined_value = (u32::from(data[0]) << 16) | u32::from(data[1]);
        // Convert the u32 to f32
        f32::from_bits(combined_value)
    }
}

impl<R> LabjackTag<i32, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: i32) -> () {
        // fetch the data, it is returned in big endian
        context
            .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes()))
            .await
            .unwrap()
            .unwrap();
    }
}

impl<W> LabjackTag<i32, CanRead, W> {
    pub async fn read(self, context: &mut Context) -> i32 {
        // fetch the data, it is returned in big endian
        let data: Vec<u16> = context
            .read_input_registers(self.address, 2)
            .await
            .unwrap()
            .unwrap();
        // Combine the two u16s into a single u32 in big endian
        let combined_value = (u32::from(data[0]) << 16) | u32::from(data[1]);
        // Convert the u32 to i32
        i32::from_be_bytes(combined_value.to_be_bytes())
    }
}

impl<R> LabjackTag<u32, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: u32) -> () {
        // fetch the data, it is returned in big endian
        context
            .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes()))
            .await
            .unwrap()
            .unwrap();
    }
}

impl<W> LabjackTag<u32, CanRead, W> {
    pub async fn read(self, context: &mut Context) -> u32 {
        // fetch the data, it is returned in big endian
        let data: Vec<u16> = context
            .read_input_registers(self.address, 2)
            .await
            .unwrap()
            .unwrap();
        // Combine the two u16s into a single u32 in big endian
        (u32::from(data[0]) << 16) | u32::from(data[1])
    }
}

impl<W> LabjackTag<u16, CanRead, W> {
    pub async fn read(self, context: &mut Context) -> u16 {
        // fetch the data, it is returned in big endian
        let data: Vec<u16> = context
            .read_input_registers(self.address, 1)
            .await
            .unwrap()
            .unwrap();
        u16::from(data[0])
    }
}

impl<R> LabjackTag<u16, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: u16) -> () {
        // fetch the data, it is returned in big endian
        context
            .write_single_register(self.address, val)
            .await
            .unwrap()
            .unwrap();
    }
}

impl<W> LabjackTag<Vec<u8>, CanRead, W> {
    pub async fn read(self, context: &mut Context, len: u32) -> Vec<u8> {
        // fetch the data, it is returned in big endian
        let mut register_count = ((len + 1) / 2) as u16;

        let MAX_REGISTERS = 127;
        let mut data_bytes: Vec<u8> = Vec::new();

        while register_count > 0 {
            let num_registers_to_read = cmp::min(register_count, MAX_REGISTERS);
            let data: Vec<u16> = context
                .read_holding_registers(self.address, num_registers_to_read)
                .await
                .unwrap()
                .unwrap();
            data_bytes.extend_from_slice(&u16_to_u8_vec(&data));
            register_count -= num_registers_to_read;
        }
        if len % 2 != 0 {
            data_bytes.pop();
        }
        data_bytes
    }

    pub async fn read_string(self, context: &mut Context, len: u32) -> String {
        let bytes = self.read(context, len).await;
        CString::from_vec_with_nul(bytes)
            .unwrap()
            .to_str()
            .unwrap()
            .into()
    }

    pub async fn read_file(self, context: &mut Context, len: u32) -> String {
        let bytes = self.read(context, len).await;
        String::from_utf8(bytes).unwrap()
    }
}

impl<R> LabjackTag<Vec<u8>, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: Vec<u8>) -> () {
        // fetch the data, it is returned in big endian
        context
            .write_multiple_registers(self.address, &u8_to_u16_vec(&val))
            .await
            .unwrap()
            .unwrap();
    }
}

pub trait CustomReader: Client {
    /// Read multiple frames (0x4C)
    async fn read_frames(&mut self, addr: &[Address], cnt: &[u8]) -> Result<Vec<u16>>;
}

impl CustomReader for Context {
    async fn read_frames(&mut self, addr: &[Address], cnt: &[u8]) -> Result<Vec<u16>> {
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
                        u8_to_u16_vec(words.as_ref())
                    }
                    _ => unreachable!("call() should reject mismatching responses"),
                })
            })
    }
}

pub const AIN0: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(0);
pub const AIN1: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(2);
pub const TEST: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(55100);
pub const TEST_UINT16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(55110);
pub const TEST_UINT32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(55120);
pub const TEST_INT32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(55122);
pub const TEST_FLOAT32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(55124);

pub const FILE_IO_DIR_CURRENT: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60601);
pub const FILE_IO_PATH_READ_LEN_BYTES: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(60642);
pub const FILE_IO_PATH_READ: LabjackTag<Vec<u8>, CanRead, CannotWrite> = LabjackTag::new(60652);

pub const FILE_IO_PATH_WRITE_LEN_BYTES: LabjackTag<u32, CannotRead, CanWrite> =
    LabjackTag::new(60640);
pub const FILE_IO_PATH_WRITE: LabjackTag<Vec<u8>, CannotRead, CanWrite> = LabjackTag::new(60650);
pub const FILE_IO_OPEN: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60620);
pub const FILE_IO_READ: LabjackTag<Vec<u8>, CanRead, CannotWrite> = LabjackTag::new(60656);
pub const FILE_IO_SIZE_BYTES: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(60628);
pub const FILE_IO_CLOSE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60621);

pub const FILE_IO_DIR_FIRST: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60610);
