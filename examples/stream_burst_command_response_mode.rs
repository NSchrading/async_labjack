use tokio::time::{sleep, Duration};
use tokio_labjack_lib::client::CustomReader;
use tokio_labjack_lib::labjack_tag::StreamConfigBuilder;
use tokio_labjack_lib::STREAM_DEBUG_GET_SELF_INDEX;
use tokio_modbus::prelude::*;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await?;

    // command response mode (auto_target = 16) sends data to the STREAM_DATA_CR tag
    // Burst mode (num_scans > 0) ends the scan after that number of scans is produced
    // Scan 300 times, buffering them into packets of 100 samples each.
    // This will produce a total of 600 values because we are going to stream 2 registers
    const NUM_SCANS: u32 = 300;
    const NUM_TAGS: u32 = 4;
    const TOTAL_SAMPLES_EXPECTED: u32 = NUM_SCANS * NUM_TAGS;
    let new_stream_config = StreamConfigBuilder::default()
        .num_addresses(NUM_TAGS)
        .scan_rate(1000.0)
        .num_scans(NUM_SCANS)
        .auto_target(16)
        .build()
        .unwrap();

    ctx.start_stream(
        new_stream_config,
        vec![
            STREAM_DEBUG_GET_SELF_INDEX.into(),
            STREAM_DEBUG_GET_SELF_INDEX.into(),
            STREAM_DEBUG_GET_SELF_INDEX.into(),
            STREAM_DEBUG_GET_SELF_INDEX.into(),
        ],
    )
    .await
    .unwrap();

    // sleep some time to wait for data to populate in STREAM_DATA_CR
    // we're sampling at 1kHz so this should be able to do ~1000 scans and NUM_SCANS is less
    // than that
    sleep(Duration::from_secs(1)).await;

    // read the data from STREAM_DATA_CR
    let data = ctx
        .read_stream_cr(TOTAL_SAMPLES_EXPECTED as u16)
        .await
        .unwrap();

    assert!(data.len() == TOTAL_SAMPLES_EXPECTED as usize);
    for values in data.chunks_exact(NUM_TAGS as usize) {
        let (idx_0, idx_1, idx_2, idx_3) = (values[0], values[1], values[2], values[3]);
        assert!(idx_0 == 0);
        assert!(idx_1 == 1);
        assert!(idx_2 == 2);
        assert!(idx_3 == 3);
    }

    println!("All values from the stream were consumed and as expected.");

    // Stream burst ends the stream automatically, but if this was not a stream burst example
    // you should stop the stream before disconnecting.

    println!("Disconnecting");
    ctx.disconnect().await?;

    Ok(())
}
