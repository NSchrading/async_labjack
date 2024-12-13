//! Demonstrating that you can customize the socket before connecting to the labjack using
//! [`LabjackClient::connect_socket_with_timeout`] or [`LabjackClient::connect_socket`]

use tokio::net::TcpSocket;
use tokio::time::Duration;
use async_labjack::client::LabjackClient;
use async_labjack::TEST;

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:502".parse().unwrap();
    let custom_socket = TcpSocket::new_v4().unwrap();

    // setting arbitrary values to demonstrate custom socket options
    custom_socket.set_recv_buffer_size(1024).unwrap();

    let mut client = LabjackClient::connect_socket_with_timeout(
        custom_socket,
        socket_addr,
        Duration::from_millis(3000),
    )
    .await
    .unwrap();

    // Ensure the test value is always 0x00112233
    let value = TEST.read(&mut client).await.unwrap();
    assert_eq!(value, 0x00112233);

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
