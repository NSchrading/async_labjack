use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio_labjack_lib::client::LabjackClient;
use tokio_labjack_lib::client::LabjackInteractions;
use tokio_labjack_lib::helpers::stream::process_stream;
use tokio_labjack_lib::labjack_tag::StreamConfigBuilder;
use tokio_labjack_lib::STREAM_DEBUG_GET_SELF_INDEX;

#[tokio::main()]
async fn main() {
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    // Spontaneous mode (auto_target = 1) sends data to port 702
    // Burst mode (num_scans > 0) ends the scan after that number of scans is produced
    // Scan 300 times, buffering them into packets of 100 samples each.
    // This will produce a total of 600 values because we are going to stream 2 registers
    const NUM_SCANS: u32 = 300;
    const NUM_TAGS: u32 = 2;
    const TOTAL_SAMPLES_EXPECTED: u32 = NUM_SCANS * NUM_TAGS;
    let new_stream_config = StreamConfigBuilder::default()
        .num_addresses(NUM_TAGS)
        .scan_rate(100.0)
        .num_scans(NUM_SCANS)
        .auto_target(1)
        .samples_per_packet(100)
        .build()
        .unwrap();

    let stream = TcpStream::connect("192.168.42.100:702").await.unwrap();
    client
        .start_stream(
            new_stream_config,
            vec![
                STREAM_DEBUG_GET_SELF_INDEX.into(),
                STREAM_DEBUG_GET_SELF_INDEX.into(),
            ],
        )
        .await
        .unwrap();

    // As the data streams in, we need to parse it from the Modbus Feedback Spontaneous
    // Packet Protocol to the data bytes. We do this in a background async task.
    let (tx, mut rx) = mpsc::channel(1024);
    tokio::spawn(async move {
        if let Err(e) = process_stream(stream, tx).await {
            println!("Stream processing ended in failure: {e}");
        } else {
            println!("Stream processing ended nominally.");
        }
    });

    // process_stream will put the raw u16 stream values in the Receiver
    let mut idx_0 = true;
    let mut total_count = 0;
    while let Some(value) = rx.recv().await {
        // Process the received values
        if idx_0 {
            assert!(value == 0);
        } else {
            assert!(value == 1);
        }
        idx_0 = !idx_0;
        total_count += 1;
    }

    assert!(total_count == TOTAL_SAMPLES_EXPECTED);
    println!("All values from the stream were consumed and as expected.");

    // Stream burst ends the stream automatically, but if this was not a stream burst example
    // you should stop the stream before disconnecting.

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
