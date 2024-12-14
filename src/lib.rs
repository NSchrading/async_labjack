/*!
This crate is a pure rust async library to communicate with LabJack T-series devices. It is
completely standalone and does not require [LJM](https://support.labjack.com/docs/ljm-library-overview).

It differentiates itself from `LJM` and other available labjack crates in the rust ecosystem in the
following ways:

* Pure rust. This is not an FFI binding to `LJM`. Instead it uses the
  [direct modbus TCP](https://support.labjack.com/docs/protocol-details-direct-modbus-tcp) interface
  of the LabJack.

* Does not require `LJM` installed on your system. As long as you can establish a tcp connection to
  your LabJack, you can use this library to work with your LabJack.

* Asynchronous.

* Strongly-typed. All registers (tags) that are available on the LabJack have types and read/write
  specifications in this library. The rust compiler will prevent issues where, for example, you may
  be attempting to read a write-only register or get a floating point value from a u32 register.
  This will prevent issues at compile time, rather than waiting to get back errors from the LabJack
  response at runtime.

* All labjack registers and error codes are defined as constants in this library.
  See [`crate::labjack::all_tags`] and [`crate::labjack::errors`]. When a labjack error occurs,
  all functions in this library will attempt to return the descriptive error from the
  `LAST_ERR_DETAIL` register, which you can match on via the `LabjackErrorCode` enum.

* TCP-only. This library does not support USB connections to the LabJack.

## Example

Many complete examples, including for streaming, can be found in the examples/ directory.

```ignore
use async_labjack::client::LabjackClient;
use async_labjack::{TEST, TEST_FLOAT32, TEST_INT32};

// Change to the address of your labjack
let socket_addr = "192.168.42.100:502".parse().unwrap();

let client = &mut LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
    .await
    .unwrap();

// Read a single LabJack tag.
// Ensure the test value is always 0x00112233
let value = TEST.read(client).await.unwrap();
assert_eq!(value, 0x00112233);

// Read and write multiple tags at once.
// This is done in a single modbus function call, and writes occur before reads,
// so you can write to tags and immediately get back the newly written values.
let results = client
  .read_write_tags(
      // The tags to read
      &[
          TEST_FLOAT32.into(),
          TEST_INT32.into(),
      ],
      // The tags to write to
      &[
          TEST_FLOAT32.into(),
          TEST_INT32.into(),
      ],
      // The values to write to the tags
      &[
          HydratedTagValue::F32(-98765.43),
          HydratedTagValue::I32(-987654),
      ],
  )
  .await
  .unwrap();

let float32_val: f32 = (&results[0]).try_into().unwrap();
assert_eq!(float32_val, -98765.43);

let int32_val: i32 = (&results[1]).try_into().unwrap();
assert_eq!(int32_val, -987654);

client.disconnect().await.unwrap();
```
*/

pub use crate::labjack::all_tags::*;
pub use crate::labjack::errors::LabjackErrorCode;
use thiserror::Error;

pub mod client;
pub mod helpers;
pub mod labjack;
pub mod modbus_feedback;
pub mod prelude;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    LabjackErrorCode(#[from] LabjackErrorCode),
    #[error(transparent)]
    TokioModbusExceptionCode(#[from] tokio_modbus::ExceptionCode),
    #[error("Unknown status code for enum: {0}")]
    UnknownStatusCode(u16),
    #[error(transparent)]
    TokioModbusError(#[from] tokio_modbus::Error),
    #[error(transparent)]
    TimeElapsed(#[from] tokio::time::error::Elapsed),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    ProcessStreamSendError(#[from] tokio::sync::mpsc::error::SendError<u16>),
    #[error("{0}")]
    Other(String),
}

/// Specialized [`std::result::Result`] type
pub type Result<T> = std::result::Result<T, Error>;
