use bytes::Buf;
use bytes::Bytes;
use tokio::time::Duration;
use tokio_labjack_lib::client::LabjackClient;
use tokio_labjack_lib::client::LabjackInteractions;
use tokio_labjack_lib::modbus_feedback::mbfb::ModbusFeedbackFrame;
use tokio_labjack_lib::{AIN0, TEST_FLOAT32, TEST_INT32};

#[tokio::main()]
async fn main() {
    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    // We're going to read 6 registers starting from address 0 (AIN0).
    // This means we'll overrun AIN0 (2 registers) and get AIN1 and AIN2 as well.
    let mut mbfb = ModbusFeedbackFrame::new_read_frame(&[AIN0.address], &[6]);
    let mut bytes_response = client.read_mbfb(&mut mbfb).await.unwrap();

    assert!(bytes_response.len() == 12);

    println!("AIN0: {}", bytes_response.get_f32());
    println!("AIN1: {}", bytes_response.get_f32());
    println!("AIN2: {}", bytes_response.get_f32());

    println!("writing 5.4321 to TEST_FLOAT32 and -314 to TEST_INT32");
    let mut bytes_vec = 5.4321_f32.to_be_bytes().to_vec();
    bytes_vec.extend((-314_i32).to_be_bytes());
    let mut mbfb = ModbusFeedbackFrame::new_write_frame(
        &[TEST_FLOAT32.address, TEST_INT32.address],
        &[2, 2],
        Bytes::from(bytes_vec),
    );
    client.write_mbfb(&mut mbfb).await.unwrap();

    let value = TEST_FLOAT32.read(&mut client).await.unwrap();
    assert!(value == 5.4321);

    let value = TEST_INT32.read(&mut client).await.unwrap();
    assert!(value == -314);

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
