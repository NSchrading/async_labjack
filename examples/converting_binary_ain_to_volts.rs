//! Only valid for T7-Pro! Change to other calibration values to make this work for T7, T4 or T8.
//!
//! Reads values from AIN1. Configures it to the +/- 10V range with ground negative channel (single
//! ended) and default resolution index.
//!
//! You may wish to connect AIN1 to a known voltage source, for example, wire DAC0 to AIN1.
//! Then, while the example runs, change the voltage source and see that the AIN value printed
//! changes to the expected value.
//!
//! Note that DAC0 is 0-5V and the top range may be lower depending on your USB voltage.
//! See https://support.labjack.com/docs/9-0-vs-power-supply-t-series-datasheet for more details.

use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio_labjack_lib::client::LabjackClient;
use tokio_labjack_lib::client::LabjackInteractions;
use tokio_labjack_lib::helpers::calibrations::t7_ain_binary_to_volts;
use tokio_labjack_lib::helpers::calibrations::Calibrations;
use tokio_labjack_lib::helpers::stream::process_stream;
use tokio_labjack_lib::labjack_tag::HydratedTagValue;
use tokio_labjack_lib::labjack_tag::StreamConfigBuilder;
use tokio_labjack_lib::{AIN1, AIN1_BINARY, AIN1_NEGATIVE_CH, AIN1_RANGE, AIN1_RESOLUTION_INDEX};

#[tokio::main()]
async fn main() {
    env_logger::init();

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

    // We need the calibration constants in order to convert the binary values to volts.
    let t7_cal = match client.read_calibrations().await.unwrap() {
        Calibrations::T7Calibrations(cal) => cal,
        _ => panic!("Unexpected calibration!"),
    };
    println!("Calibration constants: {t7_cal:?}");

    // First let's read both the 24-bit binary value and the converted value from the labjack
    let readings = client
        .read_tags(&[AIN1_BINARY.into(), AIN1.into()])
        .await
        .unwrap();

    let raw_24_bit_ain1_binary = match readings[0] {
        HydratedTagValue::U32(val) => val,
        _ => {
            panic!("unexpected tag value: {:?}", readings[0])
        }
    };
    // If using a T7-pro, the default resolution index is 9 for command-response reads.
    // This makes use of the high-resolution 24-bit ADC, so we should use the high resolution (hr)
    // calibration constants.
    // If using a T7, change this to t7_cal.hs_gain_1_ain_cal.
    // See https://support.labjack.com/docs/a-3-2-2-t7-noise-and-resolution-t-series-datasheet
    let volt_value = t7_ain_binary_to_volts(raw_24_bit_ain1_binary, &t7_cal.hr_gain_1_ain_cal);

    let ain_volt_converted_on_device = match readings[1] {
        HydratedTagValue::F32(val) => val,
        _ => {
            panic!("unexpected tag value: {:?}", readings[1])
        }
    };

    println!("Raw 24-bit value: {raw_24_bit_ain1_binary:?}");
    println!("Our conversion: {volt_value:?}V, on device: {ain_volt_converted_on_device:?}V");

    // Now let's stream some 16-bit AIN values:

    // Spontaneous mode (auto_target = 1) sends data to port 702
    // Burst mode (num_scans > 0) ends the scan after that number of scans is produced
    // Scan 300 times, buffering them into packets of 1 sample each for min latency.
    // This will produce a total of 300 values over 30 seconds because we are going to stream 1
    // tag 300 times with a scan rate of 10Hz.
    const NUM_SCANS: u32 = 300;
    const NUM_TAGS: u32 = 1;
    const TOTAL_SAMPLES_EXPECTED: u32 = NUM_SCANS * NUM_TAGS;
    let new_stream_config = StreamConfigBuilder::default()
        .num_addresses(NUM_TAGS)
        .scan_rate(10.0)
        .num_scans(NUM_SCANS)
        .auto_target(1)
        .samples_per_packet(1)
        .build()
        .unwrap();

    let stream = TcpStream::connect("192.168.42.100:702").await.unwrap();
    client
        .start_stream(new_stream_config, vec![AIN1.into()])
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
    let mut total_count = 0;
    while let Some(value) = rx.recv().await {
        // Process the received values
        // Note that we use hs instead of hr calibrations here. Streaming always uses the lower
        // precision 16-bit ADCs and always returns u16s.
        // See https://support.labjack.com/docs/a-3-2-2-t7-noise-and-resolution-t-series-datasheet
        let volt_value = t7_ain_binary_to_volts(value as u32, &t7_cal.hs_gain_1_ain_cal);
        println!("{volt_value:?}V");
        total_count += 1;
    }

    assert!(total_count == TOTAL_SAMPLES_EXPECTED);
    println!("All values from the stream were consumed and as expected.");

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
