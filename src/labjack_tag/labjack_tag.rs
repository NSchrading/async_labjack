use crate::helpers::bit_manipulation::{be_bytes_to_u16_array, u16_to_u8_vec, u8_to_u16_vec};
use crate::modbus_feedback::mbfb::CustomReader;
use std::cmp;
use std::marker::PhantomData;
use tokio_modbus::client::{Context, Writer};
use tokio_modbus::prelude::Reader;
pub struct CanRead;
pub struct CanWrite;
pub struct CannotRead;
pub struct CannotWrite;
use std::ffi::CString;

pub struct LabjackTag<T, R, W> {
    pub address: u16,
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

impl<R, W> LabjackTag<f32, R, W> {
    pub fn register_count(self) -> u8 {
        2
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

impl<R, W> LabjackTag<i32, R, W> {
    pub fn register_count(self) -> u8 {
        2
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

impl<R, W> LabjackTag<u32, R, W> {
    pub fn register_count(self) -> u8 {
        2
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

impl<R, W> LabjackTag<u16, R, W> {
    pub fn register_count(self) -> u8 {
        1
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
    pub async fn read(self, context: &mut Context, num_bytes: u32) -> Vec<u8> {
        let MAX_BYTES_PER_CALL = 1020; // Max ethernet packet size is 1040 bytes, keeping this at 1020 bytes accounts for overhead from the MBFB response packet
        let mut num_bytes_to_read = num_bytes;
        let mut data_bytes: Vec<u8> = Vec::new();

        while num_bytes_to_read > 0 {
            let limited_num_bytes_to_read = cmp::min(num_bytes_to_read, MAX_BYTES_PER_CALL);

            // fetch the data, it is returned in big endian
            let mut register_count = ((limited_num_bytes_to_read + 1) / 2) as u16;

            let MAX_REGISTERS = 255;
            let mut addresses: Vec<u16> = Vec::new();
            let mut register_counts: Vec<u8> = Vec::new();

            while register_count > 0 {
                let num_registers_to_read = cmp::min(register_count, MAX_REGISTERS);
                addresses.push(self.address);
                register_counts.push(num_registers_to_read as u8);
                register_count -= num_registers_to_read;
            }

            let result = context
                .read_frame_bytes(&addresses, &register_counts)
                .await
                .unwrap()
                .unwrap();
            println!("num bytes read: {:?}", result.len());
            num_bytes_to_read = num_bytes_to_read.saturating_sub(result.len() as u32);
            println!("need to read: {num_bytes_to_read:?} bytes");

            data_bytes.extend(result);
        }

        // If we want a non-even number of bytes, we need to pop off the last byte because read_frame_bytes will always return a whole number of 16-bit registers.
        if num_bytes % 2 != 0 {
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

pub trait Hydratable {
    fn hydrate(&self, bytes: &[u8]) -> HydratedTag;
    fn register_count(&self) -> u8;
    fn address(&self) -> u16;
}

impl<W> Hydratable for LabjackTag<f32, CanRead, W> {
    fn hydrate(&self, bytes: &[u8]) -> HydratedTag {
        // Convert the u32 to f32
        HydratedTag::F32(f32::from_be_bytes(bytes.try_into().unwrap()))
    }
    fn register_count(&self) -> u8 {
        2
    }
    fn address(&self) -> u16 {
        self.address
    }
}

impl<W> Hydratable for LabjackTag<i32, CanRead, W> {
    fn hydrate(&self, bytes: &[u8]) -> HydratedTag {
        HydratedTag::I32(i32::from_be_bytes(bytes.try_into().unwrap()))
    }
    fn register_count(&self) -> u8 {
        2
    }
    fn address(&self) -> u16 {
        self.address
    }
}

impl<W> Hydratable for LabjackTag<u32, CanRead, W> {
    fn hydrate(&self, bytes: &[u8]) -> HydratedTag {
        HydratedTag::U32(u32::from_be_bytes(bytes.try_into().unwrap()))
    }
    fn register_count(&self) -> u8 {
        2
    }
    fn address(&self) -> u16 {
        self.address
    }
}

impl<W> Hydratable for LabjackTag<u16, CanRead, W> {
    fn hydrate(&self, bytes: &[u8]) -> HydratedTag {
        HydratedTag::U16(u16::from_be_bytes(bytes.try_into().unwrap()))
    }
    fn register_count(&self) -> u8 {
        1
    }
    fn address(&self) -> u16 {
        self.address
    }
}

#[derive(Debug)]
pub enum HydratedTag {
    F32(f32),
    I32(i32),
    U32(u32),
    U16(u16),
}
