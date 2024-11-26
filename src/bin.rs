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
use tokio_labjack_lib::client::LabjackClient;
use tokio_labjack_lib::client::{CustomReader, CustomWriter};
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
    use tokio_modbus::prelude::*;
    env_logger::init();

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    // if let Err(e) = STREAM_ENABLE.write(&mut ctx, 0).await {
    //     log::debug!("{e}");
    // }

    let mut i = 1.1;
    loop {
        //let mut ctx = tcp::connect(socket_addr).await?;

        println!("Connecting...");
        let mut result =
            LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000)).await;
        while let Err(e) = result {
            println!("Error connecting: {e:?}");
            sleep(Duration::from_millis(1000)).await;
            println!("Connecting...");
            result =
                LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000)).await;
        }
        println!("Succesfully connected.");
        let mut ljc = result.unwrap();

        ljc.command_response_timeout = Duration::from_millis(100);
        //let ctx = &mut ljc.context;

        loop {
            if TEST_FLOAT32.write(&mut ljc, i).await.is_err() {
                println!(
                    "failed to get write response within {:?}",
                    ljc.command_response_timeout
                );
                break;
            }
            let value = timeout(Duration::from_millis(100), TEST_FLOAT32.read(&mut ljc)).await;
            if value.is_err() {
                println!("did not receive value within 100 ms");
                break;
            }
            println!("TEST_FLOAT32: {:?}", value.unwrap().unwrap());
            i += 5.0;
            sleep(Duration::from_millis(10)).await;
        }
        println!("Disconnecting");
        ljc.disconnect().await;
    }

    // DAC0.write(&mut ctx, 3.333).await.unwrap();

    // let value = AIN0_BINARY.read(&mut ctx).await.unwrap();
    // println!("AIN0_BINARY: {value:?}");

    // let value = AIN0.read(&mut ctx).await.unwrap();
    // println!("AIN0: {value:?}");

    // let value = AIN1.read(&mut ctx).await.unwrap();
    // println!("AIN1: {value:?}");

    // let value = AIN1_BINARY.read(&mut ctx).await.unwrap();
    // println!("AIN1_BINARY: {value:?}");

    // let value = AIN2_BINARY.read(&mut ctx).await.unwrap();
    // println!("AIN2_BINARY: {value:?}");

    // let value = AIN2.read(&mut ctx).await.unwrap();
    // println!("AIN2: {value:?}");

    // let value = TEST.read(&mut ctx).await.unwrap();
    // println!("TEST: {value:?}");

    // let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();

    // println!("TEST_FLOAT32: {value:?}");

    // TEST_FLOAT32.write(&mut ctx, 43.2).await.unwrap();

    // let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();

    // println!("TEST_FLOAT32: {value:?}");

    // let value = TEST_INT32.read(&mut ctx).await.unwrap();

    // println!("TEST_INT32: {value:?}");

    // let value = TEST_UINT16.read(&mut ctx).await.unwrap();

    // println!("TEST_UINT16: {value:?}");

    // let value = TEST_UINT32.read(&mut ctx).await.unwrap();

    // println!("TEST_UINT32: {value:?}");

    // let value = ETHERNET_MAC.read(&mut ctx).await.unwrap();
    // println!("ETHERNET_MAC: {value:?}");

    // let value = ETHERNET_IP_DEFAULT.read(&mut ctx).await.unwrap();
    // println!("ETHERNET_IP_DEFAULT: {value:?}");

    // let value = WIFI_MAC.read(&mut ctx).await.unwrap();
    // println!("WIFI_MAC: {value:?}");

    // let value = WIFI_SSID_DEFAULT.read_string(&mut ctx, 20).await.unwrap();
    // println!("WIFI_SSID_DEFAULT: {value:?}");

    // //test get cwd
    // FILE_IO_DIR_CURRENT.write(&mut ctx, 1).await.unwrap();
    // let num_bytes = FILE_IO_PATH_READ_LEN_BYTES.read(&mut ctx).await.unwrap();
    // println!("reading num_bytes: {num_bytes:?}");
    // let cwd = FILE_IO_PATH_READ
    //     .read_string(&mut ctx, num_bytes)
    //     .await
    //     .unwrap();
    // println!("cwd: {cwd:?}");

    // // test read cwd
    // FILE_IO_DIR_FIRST.write(&mut ctx, 1).await.unwrap();
    // let num_bytes = FILE_IO_PATH_READ_LEN_BYTES.read(&mut ctx).await.unwrap();
    // println!("reading num_bytes: {num_bytes:?}");
    // let path = FILE_IO_PATH_READ
    //     .read_string(&mut ctx, num_bytes)
    //     .await
    //     .unwrap();
    // let num_file_content_bytes = FILE_IO_SIZE_BYTES.read(&mut ctx).await.unwrap();
    // println!("path: {path:?}");
    // println!("file byte size: {num_file_content_bytes:?}");

    // // // test read a file
    // // let filename = Bytes::from_static(b"/log1.csv\0");
    // // let fname_num_bytes = filename.len();
    // // println!("fname bytes: {filename:?}");
    // // FILE_IO_PATH_WRITE_LEN_BYTES
    // //     .write(&mut ctx, fname_num_bytes as u32)
    // //     .await
    // //     .unwrap();
    // // println!("wrote fname_num_bytes: {fname_num_bytes:?}");
    // // FILE_IO_PATH_WRITE.write(&mut ctx, filename).await.unwrap();
    // // println!("wrote filename to FILE_IO_PATH_WRITE");
    // // FILE_IO_OPEN.write(&mut ctx, 1).await.unwrap();
    // // println!("wrote 1 to FILE_IO_OPEN");
    // // let num_file_content_bytes = FILE_IO_SIZE_BYTES.read(&mut ctx).await.unwrap();
    // // println!("number of bytes to read from file: {num_file_content_bytes:?}");
    // // let file_data = FILE_IO_READ
    // //     .read_file(&mut ctx, num_file_content_bytes)
    // //     .await
    // //     .unwrap();
    // // println!("file_data: {file_data:?}");

    // let mut mbfb = ModbusFeedbackFrame::new_read_frame(&[0], &[254]);
    // let rsp = ctx.read_mbfb(&mut mbfb).await.unwrap().unwrap();
    // println!("multi data: {:?}", rsp);

    // let results = ctx
    //     .read_tags(&[
    //         AIN1.into(),
    //         TEST_FLOAT32.into(),
    //         TEST_INT32.into(),
    //         TEST_UINT16.into(),
    //         TEST_UINT32.into(),
    //         ETHERNET_MAC.into(),
    //     ])
    //     .await
    //     .unwrap()
    //     .unwrap();
    // println!("weird tag results: {results:?}");

    // println!("writing 5.4321 to TEST_FLOAT32 and -314 to TEST_INT32");

    // let mut bytes_vec = 5.4321_f32.to_be_bytes().to_vec();
    // bytes_vec.extend((-314_i32).to_be_bytes());
    // let mut mbfb = ModbusFeedbackFrame::new_write_frame(
    //     &[TEST_FLOAT32.address, TEST_INT32.address],
    //     &[2, 2],
    //     Bytes::from(bytes_vec),
    // );
    // ctx.write_mbfb(&mut mbfb).await.unwrap().unwrap();

    // let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();

    // println!("TEST_FLOAT32: {value:?}");

    // let value = TEST_INT32.read(&mut ctx).await.unwrap();

    // println!("TEST_INT32: {value:?}");

    // println!("writing 123.432 to TEST_FLOAT32, 347382 to TEST_INT32, and 65000 to TEST_UINT16");
    // ctx.write_tags(
    //     &[TEST_FLOAT32.into(), TEST_INT32.into(), TEST_UINT16.into()],
    //     &[
    //         HydratedTagValue::F32(123.432),
    //         HydratedTagValue::I32(347382),
    //         HydratedTagValue::U16(65000),
    //     ],
    // )
    // .await
    // .unwrap()
    // .unwrap();

    // let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();

    // println!("TEST_FLOAT32: {value:?}");

    // let value = TEST_INT32.read(&mut ctx).await.unwrap();

    // println!("TEST_INT32: {value:?}");

    // let value = TEST_UINT16.read(&mut ctx).await.unwrap();

    // println!("TEST_UINT16: {value:?}");

    // println!("writing 999.999 to TEST_FLOAT32, 1234 to TEST_INT32, and 4321 to TEST_UINT16");
    // let results = ctx
    //     .read_write_tags(
    //         &[TEST_FLOAT32.into(), TEST_INT32.into(), TEST_UINT16.into()],
    //         &[TEST_FLOAT32.into(), TEST_INT32.into(), TEST_UINT16.into()],
    //         &[
    //             HydratedTagValue::F32(999.999),
    //             HydratedTagValue::I32(1234),
    //             HydratedTagValue::U16(4321),
    //         ],
    //     )
    //     .await
    //     .unwrap()
    //     .unwrap();
    // println!("weird tag results: {results:?}");

    // let results = ctx.read_stream_config().await.unwrap().unwrap();

    // // Example: 200 sample stream burst writing to command response buffer
    // // reading AIN values, need to convert to voltages
    // let new_stream_config = StreamConfigBuilder::default()
    //     .num_addresses(2)
    //     .scan_rate(100.0)
    //     .num_scans(200)
    //     .auto_target(16)
    //     .buffer_size_bytes(4096)
    //     .build()
    //     .unwrap();

    // ctx.start_stream(new_stream_config, vec![AIN0.into(), AIN1.into()])
    //     .await
    //     .unwrap();

    // sleep(Duration::from_secs(5)).await;

    // let data = ctx.read_stream_cr(2000).await.unwrap();
    // for ain_value in data {
    //     println!(
    //         "{:?}",
    //         ain_binary_to_volts(ain_value, &t7_cal.hs_gain_1_ain_cal)
    //     );
    // }

    // STREAM_ENABLE.write(&mut ctx, 0).await.unwrap();
    // sleep(Duration::from_secs(1)).await;

    // // Example: continuous stream 2 samples out to port 702 spontaneous mode
    // // with 2 samples per packet for min latency
    // let new_stream_config = StreamConfigBuilder::default()
    //     .num_addresses(2)
    //     .scan_rate(50000.0)
    //     .auto_target(1)
    //     .buffer_size_bytes(32768)
    //     .samples_per_packet(2)
    //     .build()
    //     .unwrap();

    // let addr = "192.168.42.100:702".parse().unwrap();
    // let socket = TcpSocket::new_v4()?;
    // //socket.set_recv_buffer_size(4096).unwrap();
    // let stream = socket.connect(addr).await?;

    // ctx.start_stream(new_stream_config, vec![AIN0.into(), AIN1.into()])
    //     .await
    //     .unwrap();

    // let (tx, mut rx) = mpsc::channel(100);
    // tokio::spawn(async move {
    //     if let Err(e) = process_stream(stream, tx).await {
    //         println!("Stream processing ended in failure: {e}");
    //     } else {
    //         println!("Stream processing ended nominally");
    //     }
    // });
    // while let Some(value) = rx.recv().await {
    //     // Process the received values
    //     // Note that value of 65535 usually indicate a gap in data due to buffer overflow
    //     // Most values are firmware protected to never return this value, so this is safe to use
    //     // as an indicator of buffer overflow. Check debug logs for messages about auto recovery.
    //     // See https://support.labjack.com/docs/ljm-stream-configs
    //     println!(
    //         "Received value: {}",
    //         ain_binary_to_volts(value, &t7_cal.hs_gain_1_ain_cal)
    //     );
    // }

    // if let Err(e) = STREAM_ENABLE.write(&mut ctx, 0).await {
    //     log::debug!("{e}");
    // }

    // println!("Disconnecting");
    // ctx.disconnect().await?;

    Ok(())
}
