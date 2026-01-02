//! # SX126x Radio Driver
//!
//! This crate provides a Rust driver for the Semtech SX126x series of LoRa transceivers.
//!
//! ## Features
//!
//! - `bpsk`: Enable BPSK modulation support
//! - `lr-fhss`: Enable LR-FHSS (Long Range Frequency Hopping Spread Spectrum) support
//!
//! ## Hardware Support
//!
//! This driver is designed to work with:
//! - SX1261
//! - SX1262
//! - SX1268
//!
//! ## Usage
//!
//! The driver uses the `embedded-hal` traits for platform abstraction. Users must
//! implement the HAL trait to provide platform-specific SPI and GPIO functionality.
//!
//! ```rust,no_run
//! # use sx126x::{SX126x, hal::Hal};
//! # struct MyHal;
//! # impl Hal for MyHal {
//! #     type Error = ();
//! #     fn reset(&mut self) -> Result<(), Self::Error> { Ok(()) }
//! #     fn wait_on_busy(&mut self) -> Result<(), Self::Error> { Ok(()) }
//! #     fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> { Ok(()) }
//! #     fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error> { Ok(()) }
//! #     fn write_read(&mut self, write_data: &[u8], read_data: &mut [u8]) -> Result<(), Self::Error> { Ok(()) }
//! # }
//! # let mut hal = MyHal;
//! // Create a new SX126x instance with your HAL implementation
//! let mut radio = SX126x::new(hal);
//!
//! // Configure and use the radio
//! // radio.set_standby(StandbyConfig::Rc)?;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod hal;
pub mod status;
pub mod types;
pub mod commands;
pub mod regs;

#[cfg(feature = "bpsk")]
pub mod bpsk;

#[cfg(feature = "lr-fhss")]
pub mod lr_fhss;

use hal::Hal;
pub use status::Status;
pub use types::*;

/// SX126x radio driver
pub struct SX126x<H: Hal> {
    hal: H,
}

impl<H: Hal> SX126x<H> {
    /// Create a new SX126x driver instance
    pub fn new(hal: H) -> Self {
        Self { hal }
    }

    /// Get a reference to the HAL
    pub fn hal(&self) -> &H {
        &self.hal
    }

    /// Get a mutable reference to the HAL
    pub fn hal_mut(&mut self) -> &mut H {
        &mut self.hal
    }
}

