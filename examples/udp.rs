use async_labjack::client::LabjackClient;
use async_labjack::client::LabjackInteractions;
use async_labjack::labjack::StreamConfigBuilder;
use async_labjack::STREAM_DEBUG_GET_SELF_INDEX;
use async_labjack::{
    ETHERNET_MAC, ETHERNET_UDP_DISCOVERY_ONLY_DEFAULT, LAST_ERR_DETAIL, LAST_ERR_TRANSACTION_ID,
    TEST, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32,
};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io;
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:52362".parse().unwrap();

    let client = &mut LabjackClient::connect_socket_udp(socket_addr)
        .await
        .unwrap();

    // Ensure the test value is always 0x00112233
    let value = ETHERNET_UDP_DISCOVERY_ONLY_DEFAULT
        .read(client)
        .await
        .unwrap();
    println!("{:?}", value);

    TEST_UINT32.write(client, 8675309).await.unwrap();
    let value = TEST_UINT32.read(client).await.unwrap();
    println!("{:?}", value);

    // let mut bytes = BytesMut::with_capacity(12);
    // bytes.put_u16(1); // transaction id
    // bytes.put_u16(0); // protocol id
    // bytes.put_u16(6); // request size (mbfb request below + 1)
    // bytes.put_u8(255); // unit id
    // bytes.put_u8(3); // function

    // // mbfb request
    // bytes.put_u16(TEST_UINT32.address);
    // bytes.put_u16(2);
    // let bytes = bytes.freeze();

    // let sock = UdpSocket::bind("0.0.0.0:52362").await.unwrap();

    // let remote_addr = "192.168.42.100:52362";
    // //sock.connect(remote_addr).await.unwrap();

    // let len = sock.send_to(&bytes, remote_addr).await.unwrap(); // Specify destination each time
    // println!("{:?} bytes sent to {}", len, remote_addr);

    // loop {
    //     let mut buf = [0; 20];
    //     match sock.try_recv_from(&mut buf) {
    //         // Or use blocking recv_from with a timeout
    //         Ok((len, src_addr)) => {
    //             println!("{:?} bytes received from {:?}", len, src_addr);
    //             println!("{:?}", &buf);
    //             println!(
    //                 "{:?}",
    //                 u32::from_be_bytes([buf[9], buf[10], buf[11], buf[12]])
    //             );
    //             // break; // Or continue looping
    //         }
    //         Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
    //             // No data received yet, can add a small delay or continue
    //             tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    //             continue;
    //         }
    //         Err(e) => {
    //             eprintln!("Error receiving data: {:?}", e);
    //             break;
    //         }
    //     }
    // }

    // let value = LAST_ERR_DETAIL.read(client).await.unwrap();
    // println!("{:?}", value);
    // let client = &mut LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
    //     .await
    //     .unwrap();

    // // stop any currently running stream.
    // client.stop_stream().await.unwrap();

    // // command response mode (auto_target = 16) sends data to the STREAM_DATA_CR tag
    // // Burst mode (num_scans > 0) ends the scan after that number of scans is produced
    // // Scan 300 times, buffering them into packets of 100 samples each.
    // // This will produce a total of 1200 values because we are going to stream 4 registers
    // const NUM_SCANS: u32 = 300;
    // const NUM_TAGS: u32 = 4;
    // const TOTAL_SAMPLES_EXPECTED: u32 = NUM_SCANS * NUM_TAGS;
    // let new_stream_config = StreamConfigBuilder::default()
    //     .num_addresses(NUM_TAGS)
    //     .scan_rate(1000.0)
    //     .num_scans(NUM_SCANS)
    //     .auto_target(16)
    //     .build()
    //     .unwrap();

    // client
    //     .start_stream(
    //         &new_stream_config,
    //         vec![
    //             STREAM_DEBUG_GET_SELF_INDEX.into(),
    //             STREAM_DEBUG_GET_SELF_INDEX.into(),
    //             STREAM_DEBUG_GET_SELF_INDEX.into(),
    //             STREAM_DEBUG_GET_SELF_INDEX.into(),
    //         ],
    //     )
    //     .await
    //     .unwrap();

    // // sleep some time to wait for data to populate in STREAM_DATA_CR
    // // we're sampling at 1kHz so this should be able to do ~1000 scans and NUM_SCANS is less
    // // than that
    // sleep(Duration::from_secs(1)).await;

    // // read the data from STREAM_DATA_CR
    // let data = client
    //     .read_stream_cr(TOTAL_SAMPLES_EXPECTED as u16)
    //     .await
    //     .unwrap();

    // assert_eq!(data.len(), TOTAL_SAMPLES_EXPECTED as usize);
    // for values in data.chunks_exact(NUM_TAGS as usize) {
    //     let (idx_0, idx_1, idx_2, idx_3) = (values[0], values[1], values[2], values[3]);
    //     assert_eq!(idx_0, 0);
    //     assert_eq!(idx_1, 1);
    //     assert_eq!(idx_2, 2);
    //     assert_eq!(idx_3, 3);
    // }

    // println!("All values from the stream were consumed and as expected.");

    // println!("Success! Disconnecting...");
    // client.disconnect().await.unwrap();
}
