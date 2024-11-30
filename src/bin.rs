use bytes::Buf;
use bytes::Bytes;
use std::io;
use std::{thread, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::{TcpSocket, TcpStream};
use tokio::sync::mpsc;
use tokio::time::timeout;
use tokio::time::{sleep, Duration};
use tokio_labjack_lib::client::{LabjackClient, LabjackInteractions};
use tokio_labjack_lib::helpers::calibrations::AinCalibrationBuilder;
use tokio_labjack_lib::helpers::calibrations::{ain_binary_to_volts, AinCalibration};
use tokio_labjack_lib::helpers::stream::process_stream;
use tokio_labjack_lib::labjack_tag::{HydratedTagValue, StreamConfigBuilder};
use tokio_labjack_lib::modbus_feedback::mbfb::ModbusFeedbackFrame;
use tokio_labjack_lib::{
    AIN0, AIN0_BINARY, AIN1, AIN1_BINARY, AIN2, AIN2_BINARY, DAC0, ETHERNET_IP_DEFAULT,
    ETHERNET_MAC, FILE_IO_DIR_CURRENT, FILE_IO_DIR_FIRST, FILE_IO_OPEN, FILE_IO_PATH_READ,
    FILE_IO_PATH_READ_LEN_BYTES, FILE_IO_PATH_WRITE, FILE_IO_PATH_WRITE_LEN_BYTES, FILE_IO_READ,
    FILE_IO_SIZE_BYTES, FIO0, FIO1, INTERNAL_FLASH_READ, INTERNAL_FLASH_READ_POINTER,
    STREAM_AUTO_TARGET, STREAM_BUFFER_SIZE_BYTES, STREAM_DATATYPE, STREAM_DATA_CAPTURE_16,
    STREAM_DATA_CR, STREAM_DEBUG_GET_SELF_INDEX, STREAM_ENABLE, STREAM_NUM_ADDRESSES,
    STREAM_NUM_SCANS, STREAM_RESOLUTION_INDEX, STREAM_SAMPLES_PER_PACKET, STREAM_SCANRATE_HZ,
    STREAM_SETTLING_US, SYSTEM_TIMER_20HZ, TEST, TEST_FLOAT32, TEST_INT32, TEST_UINT16,
    TEST_UINT32, WIFI_MAC, WIFI_SSID_DEFAULT,
};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    if let Err(e) = client.stop_stream().await {
        // Usually this is just because the stream is already stopped. That's fine.
        println!("Unable to stop stream before disconnect: {e}");
    }

    if let Err(e) = client.stop_stream().await {
        // Usually this is just because the stream is already stopped. That's fine.
        println!("Unable to stop stream before disconnect: {e}");
    }

    if let Err(e) = client.stop_stream().await {
        // Usually this is just because the stream is already stopped. That's fine.
        println!("Unable to stop stream before disconnect: {e}");
    }

    println!("connected");

    // let mut i = 1.1;
    // loop {
    //     println!("Connecting...");
    //     let mut result =
    //         LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000)).await;
    //     while let Err(e) = result {
    //         println!("Error connecting: {e:?}");
    //         sleep(Duration::from_millis(1000)).await;
    //         println!("Connecting...");
    //         result =
    //             LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000)).await;
    //     }
    //     println!("Succesfully connected.");
    //     let mut ljc = result.unwrap();

    //     ljc.command_response_timeout = Duration::from_millis(100);

    //     loop {
    //         if TEST_FLOAT32.write(&mut ljc, i).await.is_err() {
    //             println!(
    //                 "failed to get write response within {:?}",
    //                 ljc.command_response_timeout
    //             );
    //             break;
    //         }
    //         let value = timeout(Duration::from_millis(100), TEST_FLOAT32.read(&mut ljc)).await;
    //         if value.is_err() {
    //             println!("did not receive value within 100 ms");
    //             break;
    //         }
    //         println!("TEST_FLOAT32: {:?}", value.unwrap().unwrap());
    //         i += 5.0;
    //         sleep(Duration::from_millis(10)).await;
    //     }
    //     println!("Disconnecting");
    //     ljc.disconnect().await;
    // }

    println!("Disconnecting");
    client.disconnect().await?;

    Ok(())
}
