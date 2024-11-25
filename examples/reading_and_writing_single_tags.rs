use tokio_labjack_lib::{ETHERNET_MAC, TEST, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32};
use tokio_modbus::prelude::*;

#[tokio::main()]
async fn main() {
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await.unwrap();

    // Ensure the test value is always 0x00112233
    let value = TEST.read(&mut ctx).await.unwrap();
    assert!(value == 0x00112233);

    // Test reading and writing an f32
    let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();
    println!("TEST_FLOAT32: {value:?}");
    TEST_FLOAT32.write(&mut ctx, 43.2).await.unwrap();
    let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();
    assert!(value == 43.2);

    // test reading and writing an i32
    let value = TEST_INT32.read(&mut ctx).await.unwrap();
    println!("TEST_INT32: {value:?}");
    TEST_INT32.write(&mut ctx, -1234567).await.unwrap();
    let value = TEST_INT32.read(&mut ctx).await.unwrap();
    assert!(value == -1234567);

    // test reading a u64
    let value = ETHERNET_MAC.read(&mut ctx).await.unwrap();
    println!("ETHERNET_MAC: {value:?}");

    // test reading and writing a u32
    let value = TEST_UINT32.read(&mut ctx).await.unwrap();
    println!("TEST_UINT32: {value:?}");
    TEST_UINT32.write(&mut ctx, 4294967295).await.unwrap();
    let value = TEST_UINT32.read(&mut ctx).await.unwrap();
    assert!(value == 4294967295);

    // test reading and writing a u16
    let value = TEST_UINT16.read(&mut ctx).await.unwrap();
    println!("TEST_UINT16: {value:?}");
    TEST_UINT16.write(&mut ctx, 64000).await.unwrap();
    let value = TEST_UINT16.read(&mut ctx).await.unwrap();
    assert!(value == 64000);

    println!("Disconnecting");
    ctx.disconnect().await.unwrap();
}
