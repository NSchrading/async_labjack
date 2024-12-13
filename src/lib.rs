pub use crate::labjack::all_tags::*;
pub use crate::labjack::errors::LabjackError;
use thiserror::Error;

pub mod client;
pub mod helpers;
pub mod labjack;
pub mod modbus_feedback;
pub mod prelude;

#[derive(Debug, Error)]
pub enum TokioLabjackError {
    #[error(transparent)]
    LabjackError(#[from] LabjackError),
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
pub type Result<T> = std::result::Result<T, TokioLabjackError>;
