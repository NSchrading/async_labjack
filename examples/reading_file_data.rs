//! Requires that your labjack has an SD card installed
//! <https://support.labjack.com/docs/21-0-sd-card-t7-only-t-series-datasheet>
//!
//! Switches current working directory to / and reads the first file it finds.
//! If there are no files in /, this will fail.
//!
//! To populate your labjack with a file in /, try running a lua script to write
//! data to a file. Follow instructions here:
//! <https://support.labjack.com/docs/21-0-sd-card-t7-only-t-series-datasheet#id-21.0SDCard(T7Only)[T-SeriesDatasheet]-TestingthemicroSDCard>

use async_labjack::client::LabjackClient;
use async_labjack::{
    Error, LabjackErrorCode, FILE_IO_CLOSE, FILE_IO_DIR_CHANGE, FILE_IO_DIR_CURRENT,
    FILE_IO_DIR_FIRST, FILE_IO_DIR_NEXT, FILE_IO_OPEN, FILE_IO_PATH_READ,
    FILE_IO_PATH_READ_LEN_BYTES, FILE_IO_PATH_WRITE, FILE_IO_PATH_WRITE_LEN_BYTES, FILE_IO_READ,
    FILE_IO_SIZE_BYTES,
};
use bytes::{BufMut, Bytes, BytesMut};
use tokio::time::Duration;

#[tokio::main()]
async fn main() {
    env_logger::init();

    // Change to the address of your labjack
    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let client = &mut LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
        .await
        .unwrap();

    let current_working_directory = Bytes::from_static(b"/\0");
    // First, we want to switch to the cwd of /. We need to write the number of bytes
    // of that path to FILE_IO_PATH_WRITE_LEN_BYTES.
    FILE_IO_PATH_WRITE_LEN_BYTES
        .write(client, current_working_directory.len() as u32)
        .await
        .unwrap();
    // Then we need to write that path to FILE_IO_PATH_WRITE
    FILE_IO_PATH_WRITE
        .write(client, current_working_directory)
        .await
        .unwrap();
    // Then we trigger the switch to that directory by writing to FILE_IO_DIR_CHANGE
    FILE_IO_DIR_CHANGE.write(client, 1).await.unwrap();

    // Get current working directory
    FILE_IO_DIR_CURRENT.write(client, 1).await.unwrap();
    let num_bytes = FILE_IO_PATH_READ_LEN_BYTES.read(client).await.unwrap();
    println!("reading num_bytes: {num_bytes:?}");
    let cwd = FILE_IO_PATH_READ
        .read_string(client, num_bytes)
        .await
        .unwrap();
    println!("The current working directory is '{cwd:?}'");
    assert_eq!(cwd, "/");

    // Read the current working directory
    FILE_IO_DIR_FIRST.write(client, 1).await.unwrap();
    let num_bytes = FILE_IO_PATH_READ_LEN_BYTES.read(client).await.unwrap();
    println!("reading num_bytes: {num_bytes:?}");
    let path = FILE_IO_PATH_READ
        .read_string(client, num_bytes)
        .await
        .unwrap();
    let num_file_content_bytes = FILE_IO_SIZE_BYTES.read(client).await.unwrap();
    println!("path: {path:?}");
    println!("file byte size: {num_file_content_bytes:?}");

    // test read a file
    let file_path = format!("{cwd}{path}");
    let mut filename = BytesMut::from(&file_path[..]);

    // Filename must be a c-string (null-terminated)
    filename.put_u8(0);
    let filename = filename.freeze();

    let fname_num_bytes = filename.len();
    println!("fname bytes: {filename:?}");

    // First, write the number of bytes of the file path you wish to access
    FILE_IO_PATH_WRITE_LEN_BYTES
        .write(client, fname_num_bytes as u32)
        .await
        .unwrap();
    println!("wrote fname_num_bytes: {fname_num_bytes:?}");

    // Then write the file path to FILE_IO_PATH_WRITE
    FILE_IO_PATH_WRITE.write(client, filename).await.unwrap();
    println!("wrote filename to FILE_IO_PATH_WRITE");

    // Then write a value to FILE_IO_OPEN to open the file
    FILE_IO_OPEN.write(client, 1).await.unwrap();
    println!("wrote 1 to FILE_IO_OPEN");

    // Then read from FILE_IO_SIZE_BYTES to get how many bytes the file is
    let num_file_content_bytes = FILE_IO_SIZE_BYTES.read(client).await.unwrap();
    println!("number of bytes to read from file: {num_file_content_bytes:?}");

    // Read the data from the file
    let file_data = FILE_IO_READ
        .read_file(client, num_file_content_bytes)
        .await
        .unwrap();
    println!("file_data: {file_data:?}");

    // Finally, close the file.
    FILE_IO_CLOSE.write(client, 1).await.unwrap();

    // Instead of doing the above where we read the first file we found,
    // if you want to iterate until you find the file you wish to read, you must
    // write to FILE_IO_DIR_NEXT to go to the next path, then read FILE_IO_PATH_READ again. You can
    // do this repeatedly until it returns a FileIoEndOfCwd error.
    loop {
        match FILE_IO_DIR_NEXT.write(client, 1).await {
            Ok(_) => {
                // read FILE_IO_PATH_READ
            }
            Err(e) => {
                assert!(matches!(
                    e,
                    Error::LabjackErrorCode(LabjackErrorCode::FileIoEndOfCwd)
                ));
                break; // Exit the loop
            }
        }
    }

    println!("Success! Disconnecting...");
    client.disconnect().await.unwrap();
}
