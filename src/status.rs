//! Status types and error handling for the SX126x driver

/// Status result type for SX126x operations
pub type Status = Result<(), Error>;

/// Errors that can occur during SX126x operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// HAL layer error
    Hal,
    /// Invalid parameter provided
    InvalidParameter,
    /// Command timeout
    Timeout,
    /// Device is busy
    Busy,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Hal => write!(f, "HAL error"),
            Error::InvalidParameter => write!(f, "Invalid parameter"),
            Error::Timeout => write!(f, "Timeout"),
            Error::Busy => write!(f, "Device busy"),
        }
    }
}
