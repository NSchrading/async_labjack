use log::{debug, error, info, log_enabled, Level};
use std::ffi::CString;
use tokio_labjack_lib::{
    AIN1, FILE_IO_DIR_CURRENT, FILE_IO_DIR_FIRST, FILE_IO_OPEN, FILE_IO_PATH_READ,
    FILE_IO_PATH_READ_LEN_BYTES, FILE_IO_PATH_WRITE, FILE_IO_PATH_WRITE_LEN_BYTES, FILE_IO_READ,
    FILE_IO_SIZE_BYTES, TEST, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await?;

    println!("Fetching the coupler ID");

    // // fetch the data, it is returned in big endian
    // let data: Vec<u16> = ctx.read_input_registers(0x2, 2).await??;
    // // Combine the two u16s into a single u32 in big endian
    // let combined_value = (u32::from(data[0]) << 16) | u32::from(data[1]);
    // // Convert the u32 to f32
    // let value = f32::from_bits(combined_value);

    let value = AIN1.read(&mut ctx).await;

    println!("The data is {value:?}");

    // let value = TEST.read(&mut ctx).await;

    // println!("TEST: {value:?}");

    let value = TEST_FLOAT32.read(&mut ctx).await;

    println!("TEST_FLOAT32: {value:?}");

    TEST_FLOAT32.write(&mut ctx, 43.2).await;

    let value = TEST_FLOAT32.read(&mut ctx).await;

    println!("TEST_FLOAT32: {value:?}");

    let value = TEST_INT32.read(&mut ctx).await;

    println!("TEST_INT32: {value:?}");

    let value = TEST_UINT16.read(&mut ctx).await;

    println!("TEST_UINT16: {value:?}");

    let value = TEST_UINT32.read(&mut ctx).await;

    println!("TEST_UINT32: {value:?}");

    //test get cwd
    FILE_IO_DIR_CURRENT.write(&mut ctx, 1).await;
    let num_bytes = FILE_IO_PATH_READ_LEN_BYTES.read(&mut ctx).await;
    println!("reading num_bytes: {num_bytes:?}");
    let cwd = FILE_IO_PATH_READ.read_string(&mut ctx, num_bytes).await;
    println!("cwd: {cwd:?}");

    // test read cwd
    FILE_IO_DIR_FIRST.write(&mut ctx, 1).await;
    let num_bytes = FILE_IO_PATH_READ_LEN_BYTES.read(&mut ctx).await;
    println!("reading num_bytes: {num_bytes:?}");
    let path = FILE_IO_PATH_READ.read_string(&mut ctx, num_bytes).await;
    let num_file_content_bytes = FILE_IO_SIZE_BYTES.read(&mut ctx).await;
    println!("path: {path:?}");
    println!("file byte size: {num_file_content_bytes:?}");

    // test read a file
    let mut filename: Vec<u8> = "/log1.csv".as_bytes().to_vec();
    filename.push(0);
    let fname_num_bytes = filename.len();
    println!("fname bytes: {filename:?}");
    FILE_IO_PATH_WRITE_LEN_BYTES
        .write(&mut ctx, fname_num_bytes as u32)
        .await;
    println!("wrote fname_num_bytes: {fname_num_bytes:?}");
    FILE_IO_PATH_WRITE.write(&mut ctx, filename).await;
    println!("wrote filename to FILE_IO_PATH_WRITE");
    FILE_IO_OPEN.write(&mut ctx, 1).await;
    println!("wrote 1 to FILE_IO_OPEN");
    let num_file_content_bytes = FILE_IO_SIZE_BYTES.read(&mut ctx).await;
    println!("number of bytes to read from file: {num_file_content_bytes:?}");
    let file_data = FILE_IO_READ
        .read_file(&mut ctx, num_file_content_bytes)
        .await;
    println!("file_data: {file_data:?}");

    println!("Disconnecting");
    ctx.disconnect().await?;

    Ok(())
}
