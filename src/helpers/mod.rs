//! Helper functions.

pub mod bit_manipulation;
pub mod calibrations;
pub mod macros;
pub mod stream;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Unknown error code: {0}")]
    UnknownErrorCode(u16),
}
