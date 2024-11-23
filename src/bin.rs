use bytes::Buf;
use bytes::Bytes;
use std::io;
use std::{thread, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use tokio_labjack_lib::client::{CustomReader, CustomWriter};
use tokio_labjack_lib::helpers::calibrations::AinCalibrationBuilder;
use tokio_labjack_lib::helpers::calibrations::{ain_binary_to_volts, AinCalibration};
use tokio_labjack_lib::labjack_tag::{HydratedTagValue, StreamConfigBuilder};
use tokio_labjack_lib::modbus_feedback::mbfb::ModbusFeedbackFrame;
use tokio_labjack_lib::{
    AIN0, AIN0_BINARY, AIN1, AIN1_BINARY, AIN2, AIN2_BINARY, DAC0, ETHERNET_MAC,
    FILE_IO_DIR_CURRENT, FILE_IO_DIR_FIRST, FILE_IO_OPEN, FILE_IO_PATH_READ,
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

    let mut ctx = tcp::connect(socket_addr).await?;

    if let Err(e) = STREAM_ENABLE.write(&mut ctx, 0).await {
        log::debug!("{e}");
    }

    DAC0.write(&mut ctx, 3.333).await.unwrap();

    let value = AIN0_BINARY.read(&mut ctx).await.unwrap();
    println!("AIN0_BINARY: {value:?}");

    let value = AIN0.read(&mut ctx).await.unwrap();
    println!("AIN0: {value:?}");

    let value = AIN1.read(&mut ctx).await.unwrap();
    println!("AIN1: {value:?}");

    let value = AIN1_BINARY.read(&mut ctx).await.unwrap();
    println!("AIN1_BINARY: {value:?}");

    let value = AIN2_BINARY.read(&mut ctx).await.unwrap();
    println!("AIN2_BINARY: {value:?}");

    let value = AIN2.read(&mut ctx).await.unwrap();
    println!("AIN2: {value:?}");

    let value = TEST.read(&mut ctx).await.unwrap();
    println!("TEST: {value:?}");

    let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();

    println!("TEST_FLOAT32: {value:?}");

    TEST_FLOAT32.write(&mut ctx, 43.2).await.unwrap();

    let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();

    println!("TEST_FLOAT32: {value:?}");

    let value = TEST_INT32.read(&mut ctx).await.unwrap();

    println!("TEST_INT32: {value:?}");

    let value = TEST_UINT16.read(&mut ctx).await.unwrap();

    println!("TEST_UINT16: {value:?}");

    let value = TEST_UINT32.read(&mut ctx).await.unwrap();

    println!("TEST_UINT32: {value:?}");

    let value = ETHERNET_MAC.read(&mut ctx).await.unwrap();
    println!("ETHERNET_MAC: {value:?}");

    let value = WIFI_MAC.read(&mut ctx).await.unwrap();
    println!("WIFI_MAC: {value:?}");

    let value = WIFI_SSID_DEFAULT.read_string(&mut ctx, 20).await.unwrap();
    println!("WIFI_SSID_DEFAULT: {value:?}");

    //test get cwd
    FILE_IO_DIR_CURRENT.write(&mut ctx, 1).await.unwrap();
    let num_bytes = FILE_IO_PATH_READ_LEN_BYTES.read(&mut ctx).await.unwrap();
    println!("reading num_bytes: {num_bytes:?}");
    let cwd = FILE_IO_PATH_READ
        .read_string(&mut ctx, num_bytes)
        .await
        .unwrap();
    println!("cwd: {cwd:?}");

    // test read cwd
    FILE_IO_DIR_FIRST.write(&mut ctx, 1).await.unwrap();
    let num_bytes = FILE_IO_PATH_READ_LEN_BYTES.read(&mut ctx).await.unwrap();
    println!("reading num_bytes: {num_bytes:?}");
    let path = FILE_IO_PATH_READ
        .read_string(&mut ctx, num_bytes)
        .await
        .unwrap();
    let num_file_content_bytes = FILE_IO_SIZE_BYTES.read(&mut ctx).await.unwrap();
    println!("path: {path:?}");
    println!("file byte size: {num_file_content_bytes:?}");

    // // test read a file
    // let filename = Bytes::from_static(b"/log1.csv\0");
    // let fname_num_bytes = filename.len();
    // println!("fname bytes: {filename:?}");
    // FILE_IO_PATH_WRITE_LEN_BYTES
    //     .write(&mut ctx, fname_num_bytes as u32)
    //     .await
    //     .unwrap();
    // println!("wrote fname_num_bytes: {fname_num_bytes:?}");
    // FILE_IO_PATH_WRITE.write(&mut ctx, filename).await.unwrap();
    // println!("wrote filename to FILE_IO_PATH_WRITE");
    // FILE_IO_OPEN.write(&mut ctx, 1).await.unwrap();
    // println!("wrote 1 to FILE_IO_OPEN");
    // let num_file_content_bytes = FILE_IO_SIZE_BYTES.read(&mut ctx).await.unwrap();
    // println!("number of bytes to read from file: {num_file_content_bytes:?}");
    // let file_data = FILE_IO_READ
    //     .read_file(&mut ctx, num_file_content_bytes)
    //     .await
    //     .unwrap();
    // println!("file_data: {file_data:?}");

    let mut mbfb = ModbusFeedbackFrame::new_read_frame(&[0], &[254]);
    let rsp = ctx.read_mbfb(&mut mbfb).await.unwrap().unwrap();
    println!("multi data: {:?}", rsp);

    let results = ctx
        .read_tags(&[
            AIN1.into(),
            TEST_FLOAT32.into(),
            TEST_INT32.into(),
            TEST_UINT16.into(),
            TEST_UINT32.into(),
            ETHERNET_MAC.into(),
        ])
        .await
        .unwrap()
        .unwrap();
    println!("weird tag results: {results:?}");

    println!("writing 5.4321 to TEST_FLOAT32 and -314 to TEST_INT32");

    let mut bytes_vec = 5.4321_f32.to_be_bytes().to_vec();
    bytes_vec.extend((-314_i32).to_be_bytes());
    let mut mbfb = ModbusFeedbackFrame::new_write_frame(
        &[TEST_FLOAT32.address, TEST_INT32.address],
        &[2, 2],
        Bytes::from(bytes_vec),
    );
    ctx.write_mbfb(&mut mbfb).await.unwrap().unwrap();

    let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();

    println!("TEST_FLOAT32: {value:?}");

    let value = TEST_INT32.read(&mut ctx).await.unwrap();

    println!("TEST_INT32: {value:?}");

    println!("writing 123.432 to TEST_FLOAT32, 347382 to TEST_INT32, and 65000 to TEST_UINT16");
    ctx.write_tags(
        &[TEST_FLOAT32.into(), TEST_INT32.into(), TEST_UINT16.into()],
        &[
            HydratedTagValue::F32(123.432),
            HydratedTagValue::I32(347382),
            HydratedTagValue::U16(65000),
        ],
    )
    .await
    .unwrap()
    .unwrap();

    let value = TEST_FLOAT32.read(&mut ctx).await.unwrap();

    println!("TEST_FLOAT32: {value:?}");

    let value = TEST_INT32.read(&mut ctx).await.unwrap();

    println!("TEST_INT32: {value:?}");

    let value = TEST_UINT16.read(&mut ctx).await.unwrap();

    println!("TEST_UINT16: {value:?}");

    println!("writing 999.999 to TEST_FLOAT32, 1234 to TEST_INT32, and 4321 to TEST_UINT16");
    let results = ctx
        .read_write_tags(
            &[TEST_FLOAT32.into(), TEST_INT32.into(), TEST_UINT16.into()],
            &[TEST_FLOAT32.into(), TEST_INT32.into(), TEST_UINT16.into()],
            &[
                HydratedTagValue::F32(999.999),
                HydratedTagValue::I32(1234),
                HydratedTagValue::U16(4321),
            ],
        )
        .await
        .unwrap()
        .unwrap();
    println!("weird tag results: {results:?}");

    let results = ctx.read_stream_config().await.unwrap().unwrap();

    // scanRate = 1000.0f; //Scans per second. Samples per second = scanRate * numAddresses
    // numAddresses = NUM_ADDRESSES;
    // samplesPerPacket = STREAM_MAX_SAMPLES_PER_PACKET_TCP;  //Max is 512. For better throughput set this to high values.
    // settling = 10.0; //10 microseconds
    // resolutionIndex = 0; //Default
    // bufferSizeBytes = 0; //Default
    // autoTarget = STREAM_TARGET_ETHERNET; //Stream target is Ethernet.
    // numScans = 0; //0 = Run continuously.

    println!("stream config: {results:?}");

    // get t7 calibrations
    let t7_cal = ctx.read_calibrations().await.unwrap().unwrap();
    println!("{t7_cal:?}");

    // Example: 200 sample stream burst writing to command response buffer
    // reading AIN values, need to convert to voltages
    let new_stream_config = StreamConfigBuilder::default()
        .num_addresses(2)
        .scan_rate(100.0)
        .num_scans(200)
        .auto_target(16)
        .buffer_size_bytes(4096)
        .build()
        .unwrap();

    ctx.start_stream(new_stream_config, vec![AIN0.into(), AIN1.into()])
        .await
        .unwrap()
        .unwrap();

    STREAM_ENABLE.write(&mut ctx, 1).await.unwrap();

    sleep(Duration::from_secs(5)).await;

    let data = ctx.read_stream_cr(2000).await.unwrap();
    for ain_value in data {
        println!(
            "{:?}",
            ain_binary_to_volts(ain_value, &t7_cal.hs_gain_1_ain_cal)
        );
    }

    STREAM_ENABLE.write(&mut ctx, 0).await.unwrap();
    sleep(Duration::from_secs(1)).await;

    // Example: infinite stream writing 2 samples out to port 702 spontaneous mode
    let new_stream_config = StreamConfigBuilder::default()
        .num_addresses(2)
        .scan_rate(100.0)
        .num_scans(0)
        .auto_target(1)
        .buffer_size_bytes(4096)
        .samples_per_packet(2)
        .build()
        .unwrap();
    let mut stream = TcpStream::connect("192.168.42.100:702").await?;

    ctx.start_stream(new_stream_config, vec![AIN0.into(), AIN1.into()])
        .await
        .unwrap()
        .unwrap();

    STREAM_ENABLE.write(&mut ctx, 1).await.unwrap();

    tokio::spawn(async move {
        let mut header_buf = [0; 16];
        loop {
            let result = stream.read_exact(&mut header_buf).await;
            if let Err(e) = result {
                println!("error reading header: {e:?}");
            } else {
                println!("header: {:?}", header_buf);
                // subtract off the 10 remaining header bytes, the bytes left are all data
                let num_bytes: u32 = (((header_buf[4] as u32) << 16) + header_buf[5] as u32) - 10;
                println!("num_bytes: {:?}", num_bytes);
                let mut data_buf = vec![0; num_bytes as usize];
                let data_result = stream.read_exact(&mut data_buf).await;
                if let Err(e) = data_result {
                    println!("error reading data: {e:?}");
                } else {
                    let val_0 = u16::from_be_bytes([data_buf[0], data_buf[1]]);
                    let val_1 = u16::from_be_bytes([data_buf[2], data_buf[3]]);
                    println!(
                        "{:?}",
                        ain_binary_to_volts(val_0, &t7_cal.hs_gain_1_ain_cal)
                    );
                    println!(
                        "{:?}",
                        ain_binary_to_volts(val_1, &t7_cal.hs_gain_1_ain_cal)
                    );
                    //println!("Received: {:?}", data_buf);
                }
            }
        }
    });

    println!("sleeping to allow for data to stream in...");
    sleep(Duration::from_secs(2)).await;

    println!("done");
    STREAM_ENABLE.write(&mut ctx, 0).await.unwrap();
    // sleep(Duration::from_secs(1)).await;

    // // Example: reading a 32 bit value from stream
    // // we're reading the 20Hz system timer, at a rate of 1Hz, so each read should
    // // increase by 20
    // let new_stream_config = StreamConfigBuilder::default()
    //     .num_addresses(2)
    //     .scan_rate(1.0)
    //     .num_scans(5)
    //     .auto_target(16)
    //     .buffer_size_bytes(4096)
    //     .build()
    //     .unwrap();

    // ctx.start_stream(
    //     new_stream_config,
    //     vec![SYSTEM_TIMER_20HZ.into(), STREAM_DATA_CAPTURE_16.into()],
    // )
    // .await
    // .unwrap()
    // .unwrap();

    // STREAM_ENABLE.write(&mut ctx, 1).await.unwrap();

    // sleep(Duration::from_secs(5)).await;

    // let data = ctx.read_stream_cr(2000).await.unwrap();
    // for val in data.chunks_exact(2) {
    //     let (msb, lsb) = (val[1], val[0]);
    //     let converted: u32 = ((msb as u32) << 16) + lsb as u32;
    //     println!("{:?}", converted);
    // }

    // STREAM_ENABLE.write(&mut ctx, 0).await.unwrap();

    println!("Disconnecting");
    ctx.disconnect().await?;

    Ok(())
}
