//! Only valid for T7-Pro! Change to other calibration values to make this work for T7, T4 or T8.
//!
//! Reads values from AIN1 until interruption (ctrl+c). Gracefully cleans up.
//! Configures AIN1 to the +/- 10V range with ground negative channel (single ended)
//! and default resolution index.
//!
//! You may wish to connect AIN1 to a known voltage source, for example, wire DAC0 to AIN1.
//! Then, while the example runs, change the voltage source and see that the AIN value printed
//! changes to the expected value.
//!
//! Note that DAC0 is 0-5V and the top range may be lower depending on your USB voltage.
//! See https://support.labjack.com/docs/9-0-vs-power-supply-t-series-datasheet for more details.

use tokio::net::TcpStream;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout, Duration};
use tokio_labjack::client::LabjackClient;
use tokio_labjack::client::LabjackInteractions;
use tokio_labjack::helpers::calibrations::t7_ain_binary_to_volts;
use tokio_labjack::helpers::calibrations::{T7AinCalibration, T7Calibrations};
use tokio_labjack::helpers::stream::process_stream;
use tokio_labjack::labjack_tag::StreamConfigBuilder;
use tokio_labjack::TokioLabjackError;
use tokio_labjack::{AIN1, AIN1_BINARY, AIN1_NEGATIVE_CH, AIN1_RANGE, AIN1_RESOLUTION_INDEX};
use tokio_util::sync::CancellationToken;

/// Process the received values and print to console.
/// Note that we use hs instead of hr calibrations here. Streaming always uses the lower
/// precision 16-bit ADCs and always returns u16s.
/// See https://support.labjack.com/docs/a-3-2-2-t7-noise-and-resolution-t-series-datasheet
fn print_converted_stream_val(value: u16, ain_calibration: &T7AinCalibration) {
    let volt_value = t7_ain_binary_to_volts(value as u32, ain_calibration);
    println!("{volt_value:?}V");
}

async fn connect_with_retries(
    socket_addr: std::net::SocketAddr,
    retry_interval: Duration,
) -> TcpStream {
    loop {
        match timeout(retry_interval, TcpStream::connect(socket_addr)).await {
            Ok(Ok(stream)) => {
                println!("Connected to labjack stream port 702!");
                return stream;
            }
            e => {
                println!("Error connecting to {socket_addr:?}: {e:?}");
                println!("Retrying in {retry_interval:?}...");
            }
        }
    }
}

async fn connect_ljc_with_retries(
    socket_addr: std::net::SocketAddr,
    retry_interval: Duration,
) -> LabjackClient {
    // todo: maybe add this to client api
    loop {
        match LabjackClient::connect_with_timeout(socket_addr, retry_interval).await {
            Ok(client) => {
                println!("Connected to LabjackClient!");
                return client;
            }
            Err(e) => {
                println!("Error connecting to LabjackClient: {:?}", e);
            }
        }
    }
}

// todo: complicated idea - labjack allows a second connection on port 502
// this second client could be a heartbeat client that just attempts to write to a test
// register and read it.
// If it detects an issue, it could send a signal so that the other client triggers a reconnect.

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

    // Make sure AIN1 range is +/- 10V. We use this calibration mode when converting binary to volts
    AIN1_RANGE.write(&mut client, 0.0).await.unwrap();
    // Make sure AIN1 negative channel is ground (single ended)
    AIN1_NEGATIVE_CH.write(&mut client, 199).await.unwrap();
    // Make sure resolution index is default
    AIN1_RESOLUTION_INDEX.write(&mut client, 0).await.unwrap();

    // We need the calibration constants in order to convert the binary values to volts.
    let t7_cal: T7Calibrations = client
        .read_calibrations()
        .await
        .unwrap()
        .try_into()
        .unwrap();
    println!("Calibration constants: {t7_cal:?}");

    // First let's read both the 24-bit binary value and the converted value from the labjack
    let readings = client
        .read_tags(&[AIN1_BINARY.into(), AIN1.into()])
        .await
        .unwrap();

    let raw_24_bit_ain1_binary = (&readings[0]).try_into().unwrap();

    // If using a T7-pro, the default resolution index is 9 for command-response reads.
    // This makes use of the high-resolution 24-bit ADC, so we should use the high resolution (hr)
    // calibration constants.
    // If using a T7, change this to t7_cal.hs_gain_1_ain_cal.
    // See https://support.labjack.com/docs/a-3-2-2-t7-noise-and-resolution-t-series-datasheet
    let volt_value = t7_ain_binary_to_volts(raw_24_bit_ain1_binary, &t7_cal.hr_gain_1_ain_cal);

    let ain_volt_converted_on_device: f32 = (&readings[1]).try_into().unwrap();

    println!("Raw 24-bit value: {raw_24_bit_ain1_binary:?}");
    println!("Our conversion: {volt_value:?}V, on device: {ain_volt_converted_on_device:?}V");

    // Now let's stream some 16-bit AIN values

    // Spontaneous mode (auto_target = 1) sends data to port 702
    // num_scans defaults to 0 (runs forever)
    // Buffer data into packets of 1 sample each for min latency between sampling and processng.
    // Samples at 10Hz.
    // Note that if scan_rate is much higher and samples_per_packet is still small,
    // you may run into buffer issues where the processing can't keep up. To address,
    // increasing samples_per_packet should help, at the cost of increasing the time between
    // sampling and processing.
    const NUM_TAGS: u32 = 1;
    let new_stream_config = StreamConfigBuilder::default()
        .num_addresses(NUM_TAGS)
        .scan_rate(10.0)
        .auto_target(1)
        .samples_per_packet(1)
        .build()
        .unwrap();

    client
        .start_stream(new_stream_config, vec![AIN1.into()])
        .await
        .unwrap();

    // We'll use this channel to send parsed stream values from the process_stream task
    // to the receiving task
    let (tx, mut rx) = mpsc::channel(1024);

    // We use this channel to send results from the receive task to the main task when
    // ctrl+c is encountered. We can't join on the receive task after the select! because
    // JoinHandles are not cloneable and it is consumed in the select!
    let (tx_oneshot, rx_oneshot) = tokio::sync::oneshot::channel();

    // A token to indicate to the receive channel to end and clean up after getting ctrl+c
    let token = CancellationToken::new();
    let cloned_token = token.clone();

    // process_stream will put the raw u16 stream values in the Receiver
    let receive_task = tokio::spawn(async move {
        let mut total_count = 0;

        loop {
            tokio::select! {
                _ = cloned_token.cancelled() => {
                    // Time to end. Close the receiving end and consume values until they stop
                    rx.close();
                    while let Some(value) = rx.recv().await {
                        print_converted_stream_val(value, &t7_cal.hs_gain_1_ain_cal);
                        total_count += 1;
                    }
                    break;

                }
                Some(value) = rx.recv() => {
                    // Process the received values
                    print_converted_stream_val(value, &t7_cal.hs_gain_1_ain_cal);
                    total_count += 1;
                }
            }
        }

        if let Err(e) = tx_oneshot.send(total_count) {
            eprintln!("Unable to send result!: {e:?}");
        }
        total_count
    });

    // As the data streams in, we need to parse it from the Modbus Feedback Spontaneous
    // Packet Protocol to the data bytes. We do this in a background async task.
    let stream_processing_task = tokio::spawn(async move {
        let mut stream = connect_with_retries(
            "192.168.42.100:702".parse().unwrap(),
            Duration::from_secs(3),
        )
        .await;

        loop {
            if let Err(e) = process_stream(stream, &tx, Duration::from_secs(3)).await {
                match e {
                    TokioLabjackError::ProcessStreamSendError(
                        tokio::sync::mpsc::error::SendError(val),
                    ) => {
                        // The value that the process_stream failed to send is returned in the SendError
                        let volt_value =
                            t7_ain_binary_to_volts(val as u32, &t7_cal.hs_gain_1_ain_cal);
                        eprintln!("Process stream send failed: {e:?}, lost value: {volt_value}V");
                        return;
                    }
                    // todo: may need to restart stream somehow
                    TokioLabjackError::TimeElapsed(_) => {
                        stream = connect_with_retries(
                            "192.168.42.100:702".parse().unwrap(),
                            Duration::from_secs(3),
                        )
                        .await;
                    }
                    _ => {
                        eprintln!("Stream processing ended in error: {e:?}");
                        return;
                    }
                }
            } else {
                println!("Stream processing ended nominally.");
                return;
            }
        }
    });

    // Wait for ctrl+c to end the program, which is the nominal way to end.
    // If either of the other tasks end, that's unexpected and we'll log that as
    // an error.
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Got ctrl+c, ending gracefully.");

            // send the cancellation notification to the receive task
            token.cancel();

            // await the results from the receive task
            match rx_oneshot.await {
                Ok(total_count) => {
                    println!("Received {total_count} total values.");
                }
                Err(e) => {
                    eprintln!("Stream processing task was cancelled: {e:?}");
                }
            }
        },
        _ = stream_processing_task => {
            eprintln!("something caused stream_processing_task to end early!");

            // send the cancellation notification to the receive task
            token.cancel();

            // await the results from the receive task
            match rx_oneshot.await {
                Ok(total_processed) => {
                    eprintln!("Received {total_processed} total values before failing.");
                }
                Err(e) => {
                    eprintln!("Stream processing task was cancelled: {e:?}");
                }
            }

        },
        result = receive_task => {
            eprintln!("something caused receive_task to end early!");
            match result {
                Ok(total_processed) => {
                    eprintln!("Received {total_processed} total values before failing.");
                }
                Err(e) => {
                    eprintln!("Unexpected return from receive_task: {e:?}");
                }
            }
        },
    }

    println!("Success!");
    loop {
        // this is silly but if the labjack lost connection, we need to re-attempt connection
        // to stop the stream, then end.
        println!("Disconnecting...");
        if (client.disconnect().await).is_err() {
            client = connect_ljc_with_retries(socket_addr, Duration::from_secs(3)).await;
        } else {
            println!("Successfully disconnected");
            break;
        }
    }
}
