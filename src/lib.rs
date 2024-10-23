use std::io::Read;
use std::marker::PhantomData;
use tokio_modbus::client::{Context, Writer};
use tokio_modbus::prelude::Reader;
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

pub const AIN0: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(0);
pub const TEST: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(55100);
pub const TEST_UINT16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(55110);
pub const TEST_UINT32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(55120);
pub const TEST_INT32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(55122);
pub const TEST_FLOAT32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(55124);
