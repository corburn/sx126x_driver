//! BPSK modulation support for SX126x
//!
//! This module provides Binary Phase Shift Keying (BPSK) modulation support.
//! Enable with the `bpsk` feature flag.

use crate::{hal::Hal, status::Error, SX126x};

/// BPSK packet parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BpskPktParams {
    /// Payload length in bytes
    pub pld_len_in_bytes: u8,
}

impl<H: Hal> SX126x<H> {
    /// Set BPSK packet parameters
    ///
    /// Note: Full BPSK implementation depends on chip variant and firmware
    pub fn set_bpsk_pkt_params(&mut self, _params: &BpskPktParams) -> Result<(), Error> {
        // BPSK implementation would go here
        // This is a placeholder for the feature
        Err(Error::InvalidParameter)
    }
}
