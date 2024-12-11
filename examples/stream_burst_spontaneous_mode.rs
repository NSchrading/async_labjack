use tokio::net::TcpStream;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio_labjack::client::LabjackClient;
use tokio_labjack::client::LabjackInteractions;
use tokio_labjack::helpers::stream::process_stream;
use tokio_labjack::labjack_tag::StreamConfigBuilder;
use tokio_labjack::STREAM_DEBUG_GET_SELF_INDEX;

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
            &new_stream_config,
            vec![
                STREAM_DEBUG_GET_SELF_INDEX.into(),
                STREAM_DEBUG_GET_SELF_INDEX.into(),
            ],
        )
        .await
        .unwrap();

    // just as an example, you can read the stream config before or after you set it to
    // confirm expectations.
    let current_stream_config = client.read_stream_config().await.unwrap();
    println!("Current stream config: {current_stream_config:?}");

    // As the data streams in, we need to parse it from the Modbus Feedback Spontaneous
    // Packet Protocol to the data bytes. We do this in a background async task.
    let (tx, mut rx) = mpsc::channel(1024);
    let stream_processing_task = tokio::spawn(async move {
        if let Err(e) = process_stream(stream, &tx, Duration::from_secs(3)).await {
            println!("Stream processing ended in failure: {e}");
        } else {
            println!("Stream processing ended nominally.");
        }
    });

    let receive_task = tokio::spawn(async move {
        // process_stream will put the raw u16 stream values in the Receiver
        let mut idx_0 = true;
        let mut total_count = 0;
        while let Some(value) = rx.recv().await {
            // Process the received values
            if idx_0 {
                assert_eq!(value, 0);
            } else {
                assert_eq!(value, 1);
            }
            idx_0 = !idx_0;
            total_count += 1;
        }
        total_count
    });

    let joined_tasks = tokio::spawn(async move {
        let (processing_result, receiver_result) =
            tokio::join!(stream_processing_task, receive_task);
        // check the processing task didn't break
        processing_result.unwrap();
        let total_count = receiver_result.unwrap();
        assert_eq!(total_count, TOTAL_SAMPLES_EXPECTED);
        println!("All values from the stream were consumed and as expected.");
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Captured ctrl+c, ending program gracefully.");
        },
        _ = joined_tasks => {
            println!("Success!");
        },
    }

    println!("Disconnecting...");
    client.disconnect().await.unwrap();
}
