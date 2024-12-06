use tokio::time::{sleep, Duration};
use tokio_labjack::client::LabjackClient;
use tokio_labjack::TEST;

/// Tries to connect to the LabjackClient at the given address, retrying indefinitely
/// until a connection is established.
///
/// # Arguments
///
/// * `socket_addr` - The socket address of the Labjack device.
/// * `retry_interval` - The time interval between connection attempts.
///
/// # Returns
///
/// A `LabjackClient` containing the connected client if successful
async fn connect_with_retries(
    socket_addr: std::net::SocketAddr,
    retry_interval: Duration,
) -> LabjackClient {
    // todo: maybe add this to client api
    loop {
        match LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000)).await {
            Ok(client) => {
                println!("Connected to LabjackClient!");
                return client;
            }
            Err(e) => {
                println!("Error connecting to LabjackClient: {:?}", e);
                println!("Retrying in {:?}...", retry_interval);
                sleep(retry_interval).await;
            }
        }
    }
}

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = connect_with_retries(socket_addr, Duration::from_secs(1)).await;
    client.command_response_timeout = Duration::from_millis(500);

    let read_task = tokio::spawn(async move {
        loop {
            match TEST.read(&mut client).await {
                Ok(value) => {
                    assert_eq!(value, 0x00112233);
                    println!("Got {value:?}");
                }
                Err(e) => {
                    println!("Error occurred reading: {e}");
                    client = connect_with_retries(socket_addr, Duration::from_secs(1)).await;
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Got ctrl+c, ending gracefully.");
        },
        _ = read_task => {
            eprintln!("something caused read_task to end early!");
        },
    }
}
