use crate::helpers::bit_manipulation::{be_bytes_to_u16_array, u8_to_u16_vec};
use crate::modbus_feedback::mbfb::{CustomReader, ModbusFeedbackFrame};
use bytes::{Buf, Bytes, BytesMut};
use std::cmp;
use std::marker::PhantomData;
use tokio_modbus::client::{Context, Writer};
use tokio_modbus::prelude::Reader;
pub struct CanRead;
pub struct CanWrite;
pub struct CannotRead;
pub struct CannotWrite;
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use std::str;
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

impl<R> LabjackTag<f32, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: f32) -> Result<()> {
        Ok(context
            .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes()))
            .await??)
    }
}

impl<W> LabjackTag<f32, CanRead, W> {
    pub async fn read(self, context: &mut Context) -> Result<f32> {
        // fetch the data, it is returned in big endian
        let data: Vec<u16> = context.read_input_registers(self.address, 2).await??;
        // Combine the two u16s into a single u32 in big endian
        let combined_value = (u32::from(data[0]) << 16) | u32::from(data[1]);
        // Convert the u32 to f32
        Ok(f32::from_bits(combined_value))
    }
}

impl<R> LabjackTag<i32, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: i32) -> Result<()> {
        // fetch the data, it is returned in big endian
        Ok(context
            .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes()))
            .await??)
    }
}

impl<W> LabjackTag<i32, CanRead, W> {
    pub async fn read(self, context: &mut Context) -> Result<i32> {
        // fetch the data, it is returned in big endian
        let data: Vec<u16> = context.read_input_registers(self.address, 2).await??;
        // Combine the two u16s into a single u32 in big endian
        let combined_value = (u32::from(data[0]) << 16) | u32::from(data[1]);
        // Convert the u32 to i32
        Ok(i32::from_be_bytes(combined_value.to_be_bytes()))
    }
}

impl<R> LabjackTag<u32, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: u32) -> Result<()> {
        // fetch the data, it is returned in big endian
        Ok(context
            .write_multiple_registers(self.address, &be_bytes_to_u16_array(val.to_be_bytes()))
            .await??)
    }
}

impl<W> LabjackTag<u32, CanRead, W> {
    pub async fn read(self, context: &mut Context) -> Result<u32> {
        // fetch the data, it is returned in big endian
        let data: Vec<u16> = context.read_input_registers(self.address, 2).await??;
        // Combine the two u16s into a single u32 in big endian
        Ok((u32::from(data[0]) << 16) | u32::from(data[1]))
    }
}

impl<W> LabjackTag<u16, CanRead, W> {
    pub async fn read(self, context: &mut Context) -> Result<u16> {
        // fetch the data, it is returned in big endian
        let data: Vec<u16> = context.read_input_registers(self.address, 1).await??;
        Ok(u16::from(data[0]))
    }
}

impl<R> LabjackTag<u16, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: u16) -> Result<()> {
        // fetch the data, it is returned in big endian
        Ok(context.write_single_register(self.address, val).await??)
    }
}

impl<W> LabjackTag<Bytes, CanRead, W> {
    pub async fn read(self, context: &mut Context, num_bytes: u32) -> Result<Bytes> {
        let MAX_BYTES_PER_CALL = 1020; // Max ethernet packet size is 1040 bytes, keeping this at 1020 bytes accounts for overhead from the MBFB response packet
        let mut num_bytes_to_read = num_bytes;
        let mut data_bytes = BytesMut::with_capacity(num_bytes as usize);

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

            let mut mbfb = ModbusFeedbackFrame::new_read_frame(&addresses, &register_counts);

            let result = context.read_mbfb(&mut mbfb).await??;
            println!("num bytes read: {:?}", result.len());
            num_bytes_to_read = num_bytes_to_read.saturating_sub(result.len() as u32);
            println!("need to read: {num_bytes_to_read:?} bytes");

            data_bytes.extend(result);
        }

        // If we want a non-even number of bytes, we need to pop off the last byte because read_frame_bytes will always return a whole number of 16-bit registers.
        if num_bytes % 2 != 0 {
            data_bytes.truncate(data_bytes.len() - 1);
        }
        Ok(data_bytes.freeze())
    }

    pub async fn read_string(self, context: &mut Context, len: u32) -> Result<String> {
        let mut bytes = self.read(context, len).await?;
        // The bytes returned will have a null byte (c-string)
        bytes.truncate(bytes.len() - 1);
        let str_slice = str::from_utf8(&bytes)?;
        Ok(str_slice.to_string())
    }

    pub async fn read_file(self, context: &mut Context, len: u32) -> Result<String> {
        let bytes = self.read(context, len).await?;
        let str_slice = str::from_utf8(&bytes)?;
        Ok(str_slice.to_string())
    }
}

impl<R> LabjackTag<Bytes, R, CanWrite> {
    pub async fn write(self, context: &mut Context, val: Bytes) -> Result<()> {
        // fetch the data, it is returned in big endian
        Ok(context
            .write_multiple_registers(self.address, &u8_to_u16_vec(&val))
            .await??)
    }
}

#[enum_dispatch]
pub trait Addressable {
    fn register_count(&self) -> u8;
    fn address(&self) -> u16;
}

#[enum_dispatch]
pub trait Readable: Addressable {
    fn hydrate(&self, bytes: &mut Bytes) -> HydratedTagValue;
}

#[enum_dispatch]
pub trait Writable: Addressable {}

impl<R, W> Addressable for LabjackTag<f32, R, W> {
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

impl<R> Writable for LabjackTag<f32, R, CanWrite> {}

impl<R, W> Addressable for LabjackTag<i32, R, W> {
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

impl<R> Writable for LabjackTag<i32, R, CanWrite> {}

impl<R, W> Addressable for LabjackTag<u32, R, W> {
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

impl<R> Writable for LabjackTag<u32, R, CanWrite> {}

impl<R, W> Addressable for LabjackTag<u16, R, W> {
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

impl<R> Writable for LabjackTag<u16, R, CanWrite> {}

#[derive(Debug)]
pub enum HydratedTagValue {
    F32(f32),
    I32(i32),
    U32(u32),
    U16(u16),
}

#[enum_dispatch(Addressable)]
#[enum_dispatch(Writable)]
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

#[enum_dispatch(Addressable)]
#[enum_dispatch(Readable)]
pub enum ReadableLabjackTag {
    F32ReadOnly(LabjackTag<f32, CanRead, CannotWrite>),
    F32ReadWrite(LabjackTag<f32, CanRead, CanWrite>),

    I32ReadOnly(LabjackTag<i32, CanRead, CannotWrite>),
    I32ReadWrite(LabjackTag<i32, CanRead, CanWrite>),

    U32ReadOnly(LabjackTag<u32, CanRead, CannotWrite>),
    U32ReadWrite(LabjackTag<u32, CanRead, CanWrite>),

    U16ReadOnly(LabjackTag<u16, CanRead, CannotWrite>),
    U16ReadWrite(LabjackTag<u16, CanRead, CanWrite>),
}
