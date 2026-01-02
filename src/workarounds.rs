//! Workarounds for SX126x chip errata

use crate::{hal::Hal, status::Error, types::*, SX126x};

impl<H: Hal> SX126x<H> {
    /// GFSK workaround for 600 bps, 800 Hz deviation, 4.8 kHz BW
    ///
    /// This workaround must be applied after setting GFSK modulation and packet parameters
    /// when using specific modulation settings.
    pub fn workaround_gfsk_0_6_kbps(&mut self) -> Result<(), Error> {
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_1, &[0x0F])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_2, &[0x00])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_3, &[0x00])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_4, &[0x0F, 0xB0])
    }

    /// GFSK workaround for 1200 bps, 5000 Hz deviation, 19.5 kHz BW
    ///
    /// This workaround must be applied after setting GFSK modulation and packet parameters
    /// when using specific modulation settings.
    pub fn workaround_gfsk_1_2_kbps(&mut self) -> Result<(), Error> {
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_1, &[0x0F])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_2, &[0x00])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_3, &[0x00])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_4, &[0x1E, 0xC1])
    }

    /// Reset GFSK workarounds
    ///
    /// Must be called before configuring a different GFSK modulation after
    /// applying workarounds.
    pub fn workaround_gfsk_reset(&mut self) -> Result<(), Error> {
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_1, &[0x00])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_2, &[0x40])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_3, &[0x00])?;
        self.write_register(crate::regs::REG_GFSK_WORKAROUND_4, &[0x29, 0x0F])
    }

    /// TX modulation workaround (Section 15.1.2 of datasheet)
    ///
    /// Before any packet transmission, this configures the TX modulation register
    /// based on packet type and LoRa bandwidth.
    pub fn tx_modulation_workaround(
        &mut self,
        pkt_type: PacketType,
        lora_bw: Option<LoraBw>,
    ) -> Result<(), Error> {
        let mut reg = [0u8];
        self.read_register(crate::regs::REG_TX_MODULATION, &mut reg)?;
        
        let value = if pkt_type == PacketType::Lora {
            if let Some(bw) = lora_bw {
                if bw == LoraBw::Bw500 {
                    reg[0] & !0x04 // Clear bit 2 for 500 kHz
                } else {
                    reg[0] | 0x04 // Set bit 2 for other bandwidths
                }
            } else {
                reg[0] | 0x04 // Set bit 2 by default
            }
        } else {
            reg[0] | 0x04 // Set bit 2 for non-LoRa modulations
        };
        
        self.write_register(crate::regs::REG_TX_MODULATION, &[value])
    }

    /// IQ polarity workaround for inverted IQ
    ///
    /// Optimizes inverted IQ operation as per datasheet section 15.4
    pub fn workaround_inverted_iq(&mut self, enable: bool) -> Result<(), Error> {
        let mut reg = [0u8];
        self.read_register(crate::regs::REG_IQ_POLARITY, &mut reg)?;
        
        let value = if enable {
            0x46 // Inverted IQ
        } else {
            0x44 // Standard IQ
        };
        
        self.write_register(crate::regs::REG_IQ_POLARITY, &[value])
    }

    /// Better resistance to antenna mismatch (SX1262)
    ///
    /// Workaround from datasheet section 15.2 - optimizes PA clamping threshold
    /// for SX1262. Should be called after power-on reset or wake from cold start.
    pub fn workaround_antenna_mismatch(&mut self) -> Result<(), Error> {
        self.cfg_tx_clamp()
    }
}
