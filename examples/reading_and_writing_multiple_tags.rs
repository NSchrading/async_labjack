use tokio_labjack_lib::client::{CustomReader, CustomWriter};
use tokio_labjack_lib::labjack_tag::HydratedTagValue;
use tokio_labjack_lib::{ETHERNET_MAC, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32};
use tokio_modbus::prelude::*;

#[tokio::main()]
async fn main() {
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await.unwrap();

    // Write to a bunch of tags at once
    ctx.write_tags(
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
    .unwrap()
    .unwrap();

    // Read a bunch of tags at once
    let results = ctx
        .read_tags(&[
            TEST_FLOAT32.into(),
            TEST_INT32.into(),
            ETHERNET_MAC.into(),
            TEST_UINT32.into(),
            TEST_UINT16.into(),
        ])
        .await
        .unwrap()
        .unwrap();

    if let HydratedTagValue::F32(val) = results[0] {
        assert!(val == 123.456)
    } else {
        panic!("Unexpected tag value")
    };

    if let HydratedTagValue::I32(val) = results[1] {
        assert!(val == 123456)
    } else {
        panic!("Unexpected tag value")
    };

    if let HydratedTagValue::U64(val) = results[2] {
        // the mac address could be anything and it isn't writeable
        println!("mac address: {val:?}");
    } else {
        panic!("Unexpected tag value")
    };

    if let HydratedTagValue::U32(val) = results[3] {
        assert!(val == 123456)
    } else {
        panic!("Unexpected tag value")
    };

    if let HydratedTagValue::U16(val) = results[4] {
        assert!(val == 12345)
    } else {
        panic!("Unexpected tag value")
    };

    // Read and write all at once. Writing occurs first so you can see the change from the reads.
    let results = ctx
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
        .unwrap()
        .unwrap();

    if let HydratedTagValue::F32(val) = results[0] {
        assert!(val == -98765.43)
    } else {
        panic!("Unexpected tag value")
    };

    if let HydratedTagValue::I32(val) = results[1] {
        assert!(val == -987654)
    } else {
        panic!("Unexpected tag value")
    };

    if let HydratedTagValue::U32(val) = results[2] {
        assert!(val == 987654)
    } else {
        panic!("Unexpected tag value")
    };

    if let HydratedTagValue::U16(val) = results[3] {
        assert!(val == 9876)
    } else {
        panic!("Unexpected tag value")
    };

    println!("Disconnecting");
    ctx.disconnect().await.unwrap();
}
