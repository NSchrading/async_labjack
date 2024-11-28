use tokio::time::Duration;
use tokio_labjack_lib::client::LabjackClient;
use tokio_labjack_lib::{ETHERNET_MAC, TEST, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32};

#[tokio::main()]
async fn main() {
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    // Ensure the test value is always 0x00112233
    let value = TEST.read(&mut client).await.unwrap();
    assert!(value == 0x00112233);

    // Test reading and writing an f32
    let value = TEST_FLOAT32.read(&mut client).await.unwrap();
    println!("TEST_FLOAT32: {value:?}");
    TEST_FLOAT32.write(&mut client, 43.2).await.unwrap();
    let value = TEST_FLOAT32.read(&mut client).await.unwrap();
    assert!(value == 43.2);

    // test reading and writing an i32
    let value = TEST_INT32.read(&mut client).await.unwrap();
    println!("TEST_INT32: {value:?}");
    TEST_INT32.write(&mut client, -1234567).await.unwrap();
    let value = TEST_INT32.read(&mut client).await.unwrap();
    assert!(value == -1234567);

    // test reading a u64
    let value = ETHERNET_MAC.read(&mut client).await.unwrap();
    println!("ETHERNET_MAC: {value:?}");

    // test reading and writing a u32
    let value = TEST_UINT32.read(&mut client).await.unwrap();
    println!("TEST_UINT32: {value:?}");
    TEST_UINT32.write(&mut client, 4294967295).await.unwrap();
    let value = TEST_UINT32.read(&mut client).await.unwrap();
    assert!(value == 4294967295);

    // test reading and writing a u16
    let value = TEST_UINT16.read(&mut client).await.unwrap();
    println!("TEST_UINT16: {value:?}");
    TEST_UINT16.write(&mut client, 64000).await.unwrap();
    let value = TEST_UINT16.read(&mut client).await.unwrap();
    assert!(value == 64000);

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}