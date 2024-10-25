use async_labjack::client::LabjackClient;
use async_labjack::{ETHERNET_MAC, TEST, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32};
use tokio::time::Duration;

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let client = &mut LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    // Ensure the test value is always 0x00112233
    let value = TEST.read(client).await.unwrap();
    assert_eq!(value, 0x00112233);

    // Test reading and writing an f32
    let value = TEST_FLOAT32.read(client).await.unwrap();
    println!("TEST_FLOAT32: {value:?}");
    TEST_FLOAT32.write(client, 43.2).await.unwrap();
    let value = TEST_FLOAT32.read(client).await.unwrap();
    assert_eq!(value, 43.2);

    // test reading and writing an i32
    let value = TEST_INT32.read(client).await.unwrap();
    println!("TEST_INT32: {value:?}");
    TEST_INT32.write(client, -1234567).await.unwrap();
    let value = TEST_INT32.read(client).await.unwrap();
    assert_eq!(value, -1234567);

    // test reading a u64
    let value = ETHERNET_MAC.read(client).await.unwrap();
    println!("ETHERNET_MAC: {value:?}");

    // test reading and writing a u32
    let value = TEST_UINT32.read(client).await.unwrap();
    println!("TEST_UINT32: {value:?}");
    TEST_UINT32.write(client, 4294967295).await.unwrap();
    let value = TEST_UINT32.read(client).await.unwrap();
    assert_eq!(value, 4294967295);

    // test reading and writing a u16
    let value = TEST_UINT16.read(client).await.unwrap();
    println!("TEST_UINT16: {value:?}");
    TEST_UINT16.write(client, 64000).await.unwrap();
    let value = TEST_UINT16.read(client).await.unwrap();
    assert_eq!(value, 64000);

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
