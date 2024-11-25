use anyhow::bail;
use anyhow::Result;
use std::cmp::min;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::time::{sleep, Duration};

#[repr(u16)]
pub enum StreamStatus {
    Nominal = 0,
    AutoRecoverActive = 2940,
    AutoRecoverEnd = 2941, //Additional Info. = # scans skipped
    ScanOverlap = 2942,
    AutoRecoverEndOverflow = 2943,
    BurstComplete = 2944,
}

impl TryFrom<u16> for StreamStatus {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == StreamStatus::Nominal as u16 => Ok(StreamStatus::Nominal),
            x if x == StreamStatus::AutoRecoverActive as u16 => Ok(StreamStatus::AutoRecoverActive),
            x if x == StreamStatus::AutoRecoverEnd as u16 => Ok(StreamStatus::AutoRecoverEnd),
            x if x == StreamStatus::ScanOverlap as u16 => Ok(StreamStatus::ScanOverlap),
            x if x == StreamStatus::AutoRecoverEndOverflow as u16 => {
                Ok(StreamStatus::AutoRecoverEndOverflow)
            }
            x if x == StreamStatus::BurstComplete as u16 => Ok(StreamStatus::BurstComplete),
            _ => Err(()),
        }
    }
}

pub async fn process_stream(mut stream: TcpStream, tx: Sender<u16>) -> Result<()> {
    let mut header_buf = [0; 16];

    loop {
        stream.read_exact(&mut header_buf).await?;
        log::debug!("header: {:?}", header_buf);
        let function_code = header_buf[7];

        // sanity checks
        if function_code != 76 {
            bail!(
                "Unexpected function_code: {}, expected to be 76.",
                function_code
            );
        }

        // todo: could check on the transaction id, making sure it's incrementing

        let backlog_bytes = ((header_buf[10] as u16) << 8) + header_buf[11] as u16;
        let status_code = ((header_buf[12] as u16) << 8) + header_buf[13] as u16;
        match status_code.try_into() {
            Ok(StreamStatus::Nominal) => {}
            Ok(StreamStatus::AutoRecoverActive) => {
                log::debug!(
                    "Stream buffer overload occured. In auto recovery mode, but continuing scan. 
                    Number of backlog bytes = {}",
                    backlog_bytes,
                );
            }
            Ok(StreamStatus::AutoRecoverEnd) => {
                let num_scans_skipped = ((header_buf[14] as u16) << 8) + header_buf[15] as u16;
                log::debug!(
                    "Auto recover mode has ended. The number of skipped scans = {}.",
                    num_scans_skipped,
                );
            }
            Ok(StreamStatus::ScanOverlap) => {
                bail!(
                    "Stream scan overlap occured. This usually indicates the scan rate is too fast 
                    for the stream configuration. Number of backlog bytes = {}",
                    backlog_bytes,
                );
            }
            Ok(StreamStatus::AutoRecoverEndOverflow) => {
                bail!(
                    "During auto recovery the skipped samples counter (16-bit) overflowed. 
                    Stopping the stream because of unknown amount of skipped samples.
                    Number of backlog bytes = {}",
                    backlog_bytes,
                );
            }
            Ok(StreamStatus::BurstComplete) => {
                let num_samples_remaining = ((header_buf[14] as u16) << 8) + header_buf[15] as u16;
                log::debug!(
                    "Burst stream mode ended successfully. Remaining samples to read: {}",
                    num_samples_remaining
                );
                let num_bytes_remaining = num_samples_remaining * 2;
                let mut data_buf = vec![0; num_bytes_remaining as usize];
                stream.read_exact(&mut data_buf).await?;
                // Parse data_bytes into u16 values
                for data_byte in data_buf
                    .chunks_exact(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                {
                    tx.send(data_byte).await?;
                }
                return Ok(());
            }
            _ => {
                bail!("Unknown status code: {}, ending stream", status_code,);
            }
        }
        // The 4th and 5th byte in the header indicate the length of the subsequent packet
        // but there are other bytes in the packet after length that are not data.
        // Subtract off the 10 remaining header bytes, the bytes left are all data.
        let num_bytes = (((header_buf[4] as u16) << 8) + header_buf[5] as u16) - 10;
        log::debug!("num_bytes: {:?}", num_bytes);
        let mut data_buf = vec![0; num_bytes as usize];
        stream.read_exact(&mut data_buf).await?;

        // Parse data_bytes into u16 values
        for data_byte in data_buf
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        {
            tx.send(data_byte).await?;
        }
    }
}
