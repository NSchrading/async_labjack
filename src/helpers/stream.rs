//! Helper functions for interacting with stream data.
//! See [Labjack documentation](https://support.labjack.com/docs/3-2-4-low-level-stream-t-series-datasheet)

use crate::{LabjackError, Result, TokioLabjackError};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::time::{timeout, Duration};

/// Given a [`TcpStream`] receiving stream data, parse the data into u16s and
/// transmit them to the provided tx [`tokio::sync::mpsc::Sender`]. If stream burst was configured
/// then this function will eventually end once the [`LabjackError::StreamBurstComplete`] signal
/// is sent from the labjack. Otherwise this may run forever or until the receive end closes or an
/// unexpected error occurs. This function will handle stream auto recovery if necessary,
/// and end if [`LabjackError::StreamScanOverlap`], [`LabjackError::StreamAutoRecoverEndOverflow`],
/// [`LabjackError::StreamBufferFull`], or a Timeout occurs. `timeout_duration` is used to set
/// how long to wait for new data in the stream. If that duration elapses, the processing will end.
pub async fn process_stream(
    mut stream: TcpStream,
    tx: &Sender<u16>,
    timeout_duration: Duration,
) -> Result<()> {
    // See https://support.labjack.com/docs/3-2-4-low-level-stream-t-series-datasheet#id-3.2.4Low-LevelStream[T-SeriesDatasheet]-DataCollection
    // for information on the packet format
    let mut header_buf = [0; 16];

    loop {
        if let Err(e) = timeout(timeout_duration, stream.read_exact(&mut header_buf)).await? {
            return Err(TokioLabjackError::TokioModbusError(
                tokio_modbus::Error::Transport(e),
            ));
        }
        #[cfg(debug_assertions)]
        {
            tracing::debug!("stream header: {:?}", header_buf);
        }
        let function_code = header_buf[7];

        // sanity checks
        if function_code != 76 {
            return Err(TokioLabjackError::Other(format!(
                "Unexpected function_code: {}, expected to be 76.",
                function_code
            )));
        }

        // todo: could check on the transaction id, making sure it's incrementing
        let status_code = u16::from_be_bytes([header_buf[12], header_buf[13]]);
        match status_code.try_into() {
            Ok(LabjackError::LjSuccess) => {}
            Ok(LabjackError::StreamAutoRecoverActive) => {
                #[cfg(debug_assertions)]
                {
                    let backlog_bytes = u16::from_be_bytes([header_buf[10], header_buf[11]]);
                    tracing::debug!(
                        "Stream buffer overload occured. In auto recovery mode, but continuing scan. 
                        Number of backlog bytes = {}",
                        backlog_bytes,
                    );
                }
            }
            Ok(LabjackError::StreamAutoRecoverEnd) => {
                #[cfg(debug_assertions)]
                {
                    let num_scans_skipped = u16::from_be_bytes([header_buf[14], header_buf[15]]);
                    tracing::debug!(
                        "Auto recover mode has ended. The number of skipped scans = {}.",
                        num_scans_skipped,
                    );
                }
            }
            Ok(LabjackError::StreamScanOverlap) => {
                return Err(TokioLabjackError::from(LabjackError::StreamScanOverlap));
            }
            Ok(LabjackError::StreamAutoRecoverEndOverflow) => {
                return Err(TokioLabjackError::from(
                    LabjackError::StreamAutoRecoverEndOverflow,
                ));
            }
            Ok(LabjackError::StreamBurstComplete) => {
                let num_samples_remaining = u16::from_be_bytes([header_buf[14], header_buf[15]]);
                tracing::debug!(
                    "Burst stream mode ended successfully. Remaining samples to read: {}",
                    num_samples_remaining
                );
                let num_bytes_remaining = num_samples_remaining * 2;
                let mut data_buf = vec![0; num_bytes_remaining as usize];

                if let Err(e) = timeout(timeout_duration, stream.read_exact(&mut data_buf)).await? {
                    return Err(TokioLabjackError::TokioModbusError(
                        tokio_modbus::Error::Transport(e),
                    ));
                }

                // Parse data_bytes into u16 values
                for data_byte in data_buf
                    .chunks_exact(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                {
                    tx.send(data_byte).await?;
                }
                return Ok(());
            }
            Ok(LabjackError::StreamBufferFull) => {
                return Err(TokioLabjackError::from(LabjackError::StreamBufferFull));
            }
            // If we get a different status_code that is parseable into a LabjackError,
            // this is an unexpected error. We only expect to get the STREAM_* status codes here
            Ok(_) => {
                tracing::debug!(
                    "Received unexpected status code from stream: {}",
                    status_code
                );
                return Err(TokioLabjackError::UnknownStatusCode(status_code));
            }
            Err(e) => {
                return Err(e);
            }
        }
        // The 4th and 5th byte in the header indicate the length of the subsequent packet
        // but there are other bytes in the packet after length that are not data.
        // Subtract off the 10 remaining header bytes, the bytes left are all data.
        let num_bytes = u16::from_be_bytes([header_buf[4], header_buf[5]]) - 10;
        #[cfg(debug_assertions)]
        {
            tracing::debug!("num bytes to read in stream data: {:?}", num_bytes);
        }
        let mut data_buf = vec![0; num_bytes as usize];

        if let Err(e) = timeout(timeout_duration, stream.read_exact(&mut data_buf)).await? {
            return Err(TokioLabjackError::TokioModbusError(
                tokio_modbus::Error::Transport(e),
            ));
        }

        // Parse data_bytes into u16 values
        for data_byte in data_buf
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        {
            tx.send(data_byte).await?;
        }
    }
}
