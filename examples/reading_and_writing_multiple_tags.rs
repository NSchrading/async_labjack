use tokio::time::Duration;
use tokio_labjack_lib::client::LabjackClient;
use tokio_labjack_lib::client::LabjackInteractions;
use tokio_labjack_lib::labjack_tag::HydratedTagValue;
use tokio_labjack_lib::{ETHERNET_MAC, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32};

#[tokio::main()]
async fn main() {
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    // Write to a bunch of tags at once
    client
        .write_tags(
            &[
                TEST_FLOAT32.into(),
                TEST_INT32.into(),
                TEST_UINT32.into(),
                TEST_UINT16.into(),
            ],
            &[
                HydratedTagValue::F32(123.456),
                HydratedTagValue::I32(123456),
                HydratedTagValue::U32(123456),
                HydratedTagValue::U16(12345),
            ],
        )
        .await
        .unwrap();

    // Read a bunch of tags at once
    let results = client
        .read_tags(&[
            TEST_FLOAT32.into(),
            TEST_INT32.into(),
            ETHERNET_MAC.into(),
            TEST_UINT32.into(),
            TEST_UINT16.into(),
        ])
        .await
        .unwrap();

    let float32_val: f32 = (&results[0]).try_into().unwrap();
    assert!(float32_val == 123.456);

    let int32_val: i32 = (&results[1]).try_into().unwrap();
    assert!(int32_val == 123456);

    let ethernet_mac_val: u64 = (&results[2]).try_into().unwrap();
    println!("mac address: {ethernet_mac_val:?}");

    let uint32_val: u32 = (&results[3]).try_into().unwrap();
    assert!(uint32_val == 123456);

    let uint16_val: u16 = (&results[4]).try_into().unwrap();
    assert!(uint16_val == 12345);

    // Read and write all at once. Writing occurs first so you can see the change from the reads.
    let results = client
        .read_write_tags(
            &[
                TEST_FLOAT32.into(),
                TEST_INT32.into(),
                TEST_UINT32.into(),
                TEST_UINT16.into(),
            ],
            &[
                TEST_FLOAT32.into(),
                TEST_INT32.into(),
                TEST_UINT32.into(),
                TEST_UINT16.into(),
            ],
            &[
                HydratedTagValue::F32(-98765.43),
                HydratedTagValue::I32(-987654),
                HydratedTagValue::U32(987654),
                HydratedTagValue::U16(9876),
            ],
        )
        .await
        .unwrap();

    let float32_val: f32 = (&results[0]).try_into().unwrap();
    assert!(float32_val == -98765.43);

    let int32_val: i32 = (&results[1]).try_into().unwrap();
    assert!(int32_val == -987654);

    let uint32_val: u32 = (&results[2]).try_into().unwrap();
    assert!(uint32_val == 987654);

    let uint16_val: u16 = (&results[3]).try_into().unwrap();
    assert!(uint16_val == 9876);

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
