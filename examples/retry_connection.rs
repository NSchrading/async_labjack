//! Reads the TEST register in a loop, reconnecting to the labjack if disconnections occur.
//! To test, try unplugging the ethernet cable from the labjack and then reconnecting.
//!
//! Tries to connect to the labjack up to 20 times. If not fails and exits.
//!
//! Waits for ctrl+c to end otherwise.

use tokio::time::{sleep, Duration};
use async_labjack::client::LabjackClient;
use async_labjack::TEST;

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut client = LabjackClient::connect_with_retries(socket_addr, Duration::from_secs(1), 20)
        .await
        .unwrap();
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
                    client = LabjackClient::connect_with_retries(
                        socket_addr,
                        Duration::from_secs(1),
                        20,
                    )
                    .await
                    .unwrap();
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            // If ending streaming or other cleanup required, you may want to use notifications to
            // detect when ctrl+c was hit and notify the read_task to clean up. See
            // streaming_ain_with_graceful_cleanup as an example.
            println!("Got ctrl+c, ending...");
        },
        _ = read_task => {
            eprintln!("something caused read_task to end early!");
        },
    }
}
