//! Streams a 32-bit value (SYSTEM_TIMER_20HZ) using the STREAM_DATA_CAPTURE_16 register to capture
//! the upper 16 bits.
//!
//! Streams at 1Hz, so we ensure that each SYSTEM_TIMER_20HZ value increases by 20.

use tokio::time::{sleep, Duration};
use async_labjack::client::LabjackClient;
use async_labjack::client::LabjackInteractions;
use async_labjack::labjack::StreamConfigBuilder;
use async_labjack::{STREAM_DATA_CAPTURE_16, SYSTEM_TIMER_20HZ};

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    // stop any currently running stream.
    client.stop_stream().await.unwrap();

    const NUM_SCANS: u32 = 5;
    const NUM_TAGS: u32 = 2;
    const TOTAL_SAMPLES_EXPECTED: u32 = NUM_SCANS * NUM_TAGS;

    // command response mode (auto_target = 16) sends data to the STREAM_DATA_CR tag
    // Burst mode (num_scans > 0) ends the scan after that number of scans is produced
    let new_stream_config = StreamConfigBuilder::default()
        .num_addresses(2)
        .scan_rate(1.0)
        .num_scans(NUM_SCANS)
        .auto_target(16)
        .build()
        .unwrap();

    // Reading a 32 bit value from stream. In order to get the MSB of the data you need to place
    // STREAM_DATA_CAPTURE_16 after the intended 32 bit register. See
    // https://support.labjack.com/docs/3-2-stream-mode-t-series-datasheet#id-3.2StreamMode[T-SeriesDatasheet]-16-bitor32-bitData
    // We're reading the 20Hz system timer, at a rate of 1Hz, so each read value should increase
    // by ~20.
    client
        .start_stream(
            &new_stream_config,
            vec![SYSTEM_TIMER_20HZ.into(), STREAM_DATA_CAPTURE_16.into()],
        )
        .await
        .unwrap();

    sleep(Duration::from_secs(5)).await;

    // read the data from STREAM_DATA_CR
    let data = client
        .read_stream_cr(TOTAL_SAMPLES_EXPECTED as u16)
        .await
        .unwrap();
    assert_eq!(data.len(), TOTAL_SAMPLES_EXPECTED as usize);

    let mut start = true;
    let mut prev_val: u32 = 0;
    const EXPECTED_TIME_DIFF: u32 = 20;
    for val in data.chunks_exact(2) {
        let (msb, lsb) = (val[1], val[0]);
        let converted_32_bit_val: u32 = ((msb as u32) << 16) + lsb as u32;
        println!("system timer 20Hz: {converted_32_bit_val:?}");

        if start {
            prev_val = converted_32_bit_val;
            start = false;
        } else {
            assert!(converted_32_bit_val > prev_val);
            let time_diff = converted_32_bit_val - prev_val;

            // allow some wiggle room in the time diff in case there is jitter
            assert!((EXPECTED_TIME_DIFF - 1..=EXPECTED_TIME_DIFF + 1).contains(&time_diff));
            prev_val = converted_32_bit_val;
        }
    }

    println!("All values from the stream were consumed and as expected.");

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
