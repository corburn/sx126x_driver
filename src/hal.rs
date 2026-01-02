//! Hardware Abstraction Layer trait for SX126x
//!
//! This module defines the HAL trait that must be implemented by users
//! to provide platform-specific functionality.

use crate::status::Error;

/// Hardware Abstraction Layer trait for SX126x communication
///
/// Users must implement this trait to provide platform-specific SPI and GPIO
/// functionality for communicating with the SX126x radio.
pub trait Hal {
    /// HAL-specific error type
    type Error;

    /// Reset the radio
    ///
    /// This should toggle the NRST pin low then high to reset the radio.
    fn reset(&mut self) -> Result<(), Self::Error>;

    /// Wait for the BUSY pin to go low
    ///
    /// The SX126x asserts BUSY during command processing. This function
    /// should block until BUSY is deasserted.
    fn wait_on_busy(&mut self) -> Result<(), Self::Error>;

    /// Write data to the radio via SPI
    ///
    /// # Arguments
    /// * `data` - Data to write over SPI
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;

    /// Read data from the radio via SPI
    ///
    /// # Arguments
    /// * `data` - Buffer to read data into
    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error>;

    /// Write then read data over SPI
    ///
    /// # Arguments
    /// * `write_data` - Data to write
    /// * `read_data` - Buffer to read response into
    fn write_read(&mut self, write_data: &[u8], read_data: &mut [u8]) -> Result<(), Self::Error>;
}

/// Convert HAL errors to driver errors
pub(crate) fn hal_error_to_status<E>(_e: E) -> Error {
    Error::Hal
}
