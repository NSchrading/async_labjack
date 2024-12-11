//! Only valid for T7-Pro! Change to other calibration values to make this work for T7, T4 or T8.
//!
//! Streaming AIN values gives you raw binary, so you need to convert to voltage if that is
//! the kind of reading you intend to get.
//!
//! Reads values from AIN1 until interruption (ctrl+c). Gracefully cleans up and handles
//! loss of ethernet connection by attempting to re-establish connection.
//!
//! Configures AIN1 to the +/- 10V range with ground negative channel (single ended)
//! and default resolution index.
//!
//! You may wish to connect AIN1 to a known voltage source, for example, wire DAC0 to AIN1.
//! Then, while the example runs, change the voltage source and see that the AIN value printed
//! changes to the expected value.
//!
//! Note that DAC0 is 0-5V and the top range may be lower depending on your USB voltage.
//! See https://support.labjack.com/docs/9-0-vs-power-supply-t-series-datasheet for more details.

use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::Notify;
use tokio::time::{sleep, timeout, Duration};
use tokio_labjack::client::LabjackClient;
use tokio_labjack::client::LabjackInteractions;
use tokio_labjack::helpers::calibrations::t7_ain_binary_to_volts;
use tokio_labjack::helpers::calibrations::{T7AinCalibration, T7Calibrations};
use tokio_labjack::helpers::stream::process_stream;
use tokio_labjack::labjack_tag::StreamConfigBuilder;
use tokio_labjack::TokioLabjackError;
use tokio_labjack::{AIN1, AIN1_NEGATIVE_CH, AIN1_RANGE, AIN1_RESOLUTION_INDEX, TEST};
use tokio_util::sync::CancellationToken;

/// Process the received values and print to console.
/// Note that we use hs instead of hr calibrations here. Streaming always uses the lower
/// precision 16-bit ADCs and always returns u16s.
/// See https://support.labjack.com/docs/a-3-2-2-t7-noise-and-resolution-t-series-datasheet
fn print_converted_stream_val(value: u16, ain_calibration: &T7AinCalibration) {
    let volt_value = t7_ain_binary_to_volts(value as u32, ain_calibration);
    println!("{volt_value:?}V");
}

/// Continuously try to connect to the labjack stream port
async fn connect_to_stream_with_retries(
    socket_addr: std::net::SocketAddr,
    retry_interval: Duration,
) -> TcpStream {
    loop {
        match timeout(retry_interval, TcpStream::connect(socket_addr)).await {
            Ok(Ok(stream)) => {
                println!("Connected to labjack stream port!");
                return stream;
            }
            e => {
                println!("Error connecting to {socket_addr:?}: {e:?}");
                println!("Retrying in {retry_interval:?}...");
            }
        }
    }
}

/// Start the AIN stream. Detect disconnection events and reconnect if possible, starting
/// the stream up again if connection is lost.
async fn maintain_stream(socket_addr: std::net::SocketAddr, client: &mut LabjackClient) {
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

    // max number of times we'll try reconnecting to the labjack if we lose connection.
    const MAX_CONNECTION_ATTEMPTS: usize = 20;

    loop {
        // stop any currently running stream.
        client.stop_stream().await.unwrap();
        client
            .start_stream(&new_stream_config, vec![AIN1.into()])
            .await
            .unwrap();
        loop {
            // Send a command-response every second so that we don't trigger the watchdog restart.
            // Also serves as a way to detect disconnections within this loop
            match TEST.read(client).await {
                // Production code may want to make a distinction between error types, e.g. only
                // reconnect on timeouts.
                Err(e) => {
                    eprintln!("client encountered error: {e:?}");
                    break;
                }
                Ok(val) => {
                    if val != 0x00112233 {
                        eprintln!("client got unexpected value: {val:?}");
                        break;
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
        // This is using unwrap because the only thing left to do would be to call disconnect
        // to stop the stream, but since we're not connected, we can't. Production environments
        // likely wouldn't want to use unwrap, but this is an example.
        *client = LabjackClient::connect_with_retries(
            socket_addr,
            Duration::from_millis(3000),
            MAX_CONNECTION_ATTEMPTS,
        )
        .await
        .unwrap();
    }
}

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    // Make sure AIN1 range is +/- 10V. We use this calibration mode when converting binary to volts
    AIN1_RANGE.write(&mut client, 0.0).await.unwrap();
    // Make sure AIN1 negative channel is ground (single ended)
    AIN1_NEGATIVE_CH.write(&mut client, 199).await.unwrap();
    // Make sure resolution index is default
    AIN1_RESOLUTION_INDEX.write(&mut client, 0).await.unwrap();

    // For extra durability in production use cases, enable the labjack watchdog.
    // This will ensure that the labjack will reboot itself if it doesn't receive any
    // communication from the host application within a timeout.
    // See https://support.labjack.com/docs/23-0-watchdog-t-series-datasheet for more details.
    // Note: Don't write to these registers often, as it can wear out system flash.
    // I have found that in some cases when ethernet communication is lost, the labjack
    // gets into a state where no further connection attempts can succeed until a powercycle.
    // This watchdog timer detects and corrects that issue by powercycling the labjack.
    // // disable watchdog while making changes.
    // tokio_labjack::WATCHDOG_ENABLE_DEFAULT
    //     .write(&mut client, 0)
    //     .await
    //     .unwrap();
    // // set watchdog to 10s timeout
    // tokio_labjack::WATCHDOG_TIMEOUT_S_DEFAULT
    //     .write(&mut client, 10)
    //     .await
    //     .unwrap();
    // // enable power reset on timeout
    // tokio_labjack::WATCHDOG_RESET_ENABLE_DEFAULT
    //     .write(&mut client, 1)
    //     .await
    //     .unwrap();
    // // set the watchdog startup delay to 10s
    // tokio_labjack::WATCHDOG_STARTUP_DELAY_S_DEFAULT
    //     .write(&mut client, 10)
    //     .await
    //     .unwrap();
    // // enable watchdog
    // tokio_labjack::WATCHDOG_ENABLE_DEFAULT
    //     .write(&mut client, 1)
    //     .await
    //     .unwrap();
    // // do an initial clear to start the watchdog timer
    // tokio_labjack::WATCHDOG_STRICT_CLEAR
    //     .write(&mut client, 1)
    //     .await
    //     .unwrap();

    // We need the calibration constants in order to convert the binary values to volts.
    let t7_cal: T7Calibrations = client
        .read_calibrations()
        .await
        .unwrap()
        .try_into()
        .unwrap();
    println!("Calibration constants: {t7_cal:?}");

    // We'll use this channel to send parsed stream values from the process_stream task
    // to the receiving task. Change the buffer capacity depending on stream rate.
    let (tx, mut rx) = mpsc::channel(1024);

    // We use this channel to send results from the receive task to the main task when
    // ctrl+c is encountered. We can't join on the receive task after the select! because
    // JoinHandles are not cloneable and it is consumed in the select!
    let (tx_oneshot, rx_oneshot) = tokio::sync::oneshot::channel();

    // We use this notication to let the main task know that the stream task is done cleaning
    // up after ctrl+c.
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    // A token to indicate to the receive channel to end and clean up after getting ctrl+c
    let token = CancellationToken::new();
    let receive_task_token = token.clone();
    let stream_task_token = token.clone();

    // Let's stream some 16-bit AIN values
    let stream_task = tokio::spawn(async move {
        tokio::select! {
            _ = stream_task_token.cancelled() => {
                println!("Stream task ended via ctrl+c, cleaning up.");
                client.disconnect().await.unwrap();
                notify2.notify_one();
            }
            _ = maintain_stream(socket_addr, &mut client) => {
                // this shouldn't be possible, maintain_stream is an infinite loop
                eprintln!("Stream task ended unexpectedly.");
                client.disconnect().await.unwrap();
            }
        }
    });

    // process_stream will put the raw u16 stream values in the Receiver
    let receive_task = tokio::spawn(async move {
        let mut total_count = 0;

        loop {
            tokio::select! {
                _ = receive_task_token.cancelled() => {
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
        let mut stream = connect_to_stream_with_retries(
            "192.168.42.100:702".parse().unwrap(),
            Duration::from_secs(3),
        )
        .await;

        loop {
            if let Err(e) = process_stream(stream, &tx, Duration::from_secs(3)).await {
                match e {
                    // Occurs when the receive end closes before the transmit end. Print the last
                    // value and end.
                    TokioLabjackError::ProcessStreamSendError(
                        tokio::sync::mpsc::error::SendError(val),
                    ) => {
                        // The value that the process_stream failed to send is returned in the SendError
                        let volt_value =
                            t7_ain_binary_to_volts(val as u32, &t7_cal.hs_gain_1_ain_cal);
                        eprintln!("Process stream send failed: {e:?}, lost value: {volt_value}V");
                        return;
                    }
                    // If we get a TimeElapsed error, we probably lost connection to the labjack.
                    // In this case, we want to reattempt connection and keep processing.
                    TokioLabjackError::TimeElapsed(_) => {
                        stream = connect_to_stream_with_retries(
                            "192.168.42.100:702".parse().unwrap(),
                            Duration::from_secs(3),
                        )
                        .await;
                    }
                    // Unknown errors, end stream.
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

            // send the cancellation notification to the tasks
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

            // wait for the stream task to finish disconnecting client.
            notify.notified().await;
            println!("Graceful cleanup done.");
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
        _ = stream_task => {
            eprintln!("something caused stream_task to end early!");
        }
    }

    println!("Success!");
}
