//! Only valid for T7-Pro! Change to other calibration values to make this work for T7, T4 or T8.
//!
//! Reads binary value from AIN1 and converts it to voltage using T7-pro's high resolution ADC
//! conversion contants.
//! Configures AIN1 to the +/- 10V range with ground negative channel (single ended)
//! and default resolution index.
//!
//! You may wish to connect AIN1 to a known voltage source, for example, wire DAC0 to AIN1 to
//! verify proper readings.
//!
//! Note that DAC0 is 0-5V and the top range may be lower depending on your USB voltage.
//! See https://support.labjack.com/docs/9-0-vs-power-supply-t-series-datasheet for more details.

use tokio::time::Duration;
use tokio_labjack::client::LabjackClient;
use tokio_labjack::client::LabjackInteractions;
use tokio_labjack::helpers::calibrations::t7_ain_binary_to_volts;
use tokio_labjack::helpers::calibrations::T7Calibrations;
use tokio_labjack::{AIN1, AIN1_BINARY, AIN1_NEGATIVE_CH, AIN1_RANGE, AIN1_RESOLUTION_INDEX};

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

    // // We need the calibration constants in order to convert the binary values to volts.
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

    // These won't match exactly, because they are sampled at different times. T7 does not have
    // simultaneous sampling. If using T8, they may match.
    println!("Our conversion: {volt_value:?}V, on device: {ain_volt_converted_on_device:?}V");

    println!("Success!");
}
