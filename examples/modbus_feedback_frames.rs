use async_labjack::client::LabjackClient;
use async_labjack::client::LabjackInteractions;
use async_labjack::modbus_feedback::mbfb::ModbusFeedbackFrame;
use async_labjack::{AIN0, TEST_FLOAT32, TEST_INT32};
use bytes::Buf;
use bytes::Bytes;
use tokio::time::Duration;

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let client = &mut LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    // We're going to read 6 registers starting from address 0 (AIN0).
    // This means we'll overrun AIN0 (2 registers) and get AIN1 and AIN2 as well.
    let mbfb = &mut ModbusFeedbackFrame::new_read_frame(&[AIN0.address], &[6]);
    let mut bytes_response = client.read_mbfb(mbfb).await.unwrap();

    assert_eq!(bytes_response.len(), 12);

    println!("AIN0: {}", bytes_response.get_f32());
    println!("AIN1: {}", bytes_response.get_f32());
    println!("AIN2: {}", bytes_response.get_f32());

    println!("writing 5.4321 to TEST_FLOAT32 and -314 to TEST_INT32");
    let mbfb = &mut ModbusFeedbackFrame::new_write_frame(
        &[TEST_FLOAT32.address, TEST_INT32.address],
        &[2, 2],
        Bytes::from_iter(
            [5.4321_f32.to_be_bytes(), (-314_i32).to_be_bytes()]
                .into_iter()
                .flatten(),
        ),
    );
    client.write_mbfb(mbfb).await.unwrap();

    let value = TEST_FLOAT32.read(client).await.unwrap();
    assert_eq!(value, 5.4321);

    let value = TEST_INT32.read(client).await.unwrap();
    assert_eq!(value, -314);

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
