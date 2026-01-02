//! Command opcodes and implementations for SX126x

/// NOP command for SPI
pub const NOP: u8 = 0x00;

/// Command opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Operational modes
    SetSleep = 0x84,
    SetStandby = 0x80,
    SetFs = 0xC1,
    SetTx = 0x83,
    SetRx = 0x82,
    SetStopTimerOnPreamble = 0x9F,
    SetRxDutyCycle = 0x94,
    SetCad = 0xC5,
    SetTxContinuousWave = 0xD1,
    SetTxInfinitePreamble = 0xD2,
    SetRegulatorMode = 0x96,
    Calibrate = 0x89,
    CalibrateImage = 0x98,
    SetPaCfg = 0x95,
    SetRxTxFallbackMode = 0x93,
    
    // Registers and buffer access
    WriteRegister = 0x0D,
    ReadRegister = 0x1D,
    WriteBuffer = 0x0E,
    ReadBuffer = 0x1E,
    
    // DIO and IRQ control
    SetDioIrqParams = 0x08,
    GetIrqStatus = 0x12,
    ClrIrqStatus = 0x02,
    SetDio2AsRfSwitchCtrl = 0x9D,
    SetDio3AsTcxoCtrl = 0x97,
    
    // RF modulation and packet-related
    SetRfFrequency = 0x86,
    SetPktType = 0x8A,
    GetPktType = 0x11,
    SetTxParams = 0x8E,
    SetModulationParams = 0x8B,
    SetPktParams = 0x8C,
    SetCadParams = 0x88,
    SetBufferBaseAddress = 0x8F,
    SetLoraSymbNumTimeout = 0xA0,
    
    // Communication status
    GetStatus = 0xC0,
    GetRxBufferStatus = 0x13,
    GetPktStatus = 0x14,
    GetRssiInst = 0x15,
    GetStats = 0x10,
    ResetStats = 0x00,
    
    // Miscellaneous
    GetDeviceErrors = 0x17,
    ClrDeviceErrors = 0x07,
}

use crate::{hal::Hal, status::Error, types::*, SX126x};

impl<H: Hal> SX126x<H> {
    /// Set the chip in sleep mode
    pub fn set_sleep(&mut self, cfg: SleepConfig) -> Result<(), Error> {
        let buf = [OpCode::SetSleep as u8, cfg as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set the chip in standby mode
    pub fn set_standby(&mut self, cfg: StandbyConfig) -> Result<(), Error> {
        let buf = [OpCode::SetStandby as u8, cfg as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set the chip in frequency synthesis mode
    pub fn set_fs(&mut self) -> Result<(), Error> {
        let buf = [OpCode::SetFs as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set the chip in TX mode
    pub fn set_tx(&mut self, timeout_in_ms: u32) -> Result<(), Error> {
        if timeout_in_ms > MAX_TIMEOUT_IN_MS {
            return Err(Error::InvalidParameter);
        }
        let timeout_rtc = timeout_in_ms * 64;
        self.set_tx_with_timeout_in_rtc_step(timeout_rtc)
    }

    /// Set the chip in TX mode with timeout in RTC steps
    pub fn set_tx_with_timeout_in_rtc_step(&mut self, timeout: u32) -> Result<(), Error> {
        if timeout > MAX_TIMEOUT_IN_RTC_STEP {
            return Err(Error::InvalidParameter);
        }
        let buf = [
            OpCode::SetTx as u8,
            ((timeout >> 16) & 0xFF) as u8,
            ((timeout >> 8) & 0xFF) as u8,
            (timeout & 0xFF) as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set the chip in RX mode
    pub fn set_rx(&mut self, timeout_in_ms: u32) -> Result<(), Error> {
        if timeout_in_ms > MAX_TIMEOUT_IN_MS {
            return Err(Error::InvalidParameter);
        }
        let timeout_rtc = timeout_in_ms * 64;
        self.set_rx_with_timeout_in_rtc_step(timeout_rtc)
    }

    /// Set the chip in RX mode with timeout in RTC steps
    pub fn set_rx_with_timeout_in_rtc_step(&mut self, timeout: u32) -> Result<(), Error> {
        if timeout > MAX_TIMEOUT_IN_RTC_STEP && timeout != RX_CONTINUOUS {
            return Err(Error::InvalidParameter);
        }
        let buf = [
            OpCode::SetRx as u8,
            ((timeout >> 16) & 0xFF) as u8,
            ((timeout >> 8) & 0xFF) as u8,
            (timeout & 0xFF) as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Stop timer on preamble detection
    pub fn stop_timer_on_preamble(&mut self, enable: bool) -> Result<(), Error> {
        let buf = [OpCode::SetStopTimerOnPreamble as u8, enable as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set RX duty cycle mode
    pub fn set_rx_duty_cycle(&mut self, rx_time_in_ms: u32, sleep_time_in_ms: u32) -> Result<(), Error> {
        let rx_time_rtc = rx_time_in_ms * 64;
        let sleep_time_rtc = sleep_time_in_ms * 64;
        self.set_rx_duty_cycle_with_timings_in_rtc_step(rx_time_rtc, sleep_time_rtc)
    }

    /// Set RX duty cycle mode with timings in RTC steps
    pub fn set_rx_duty_cycle_with_timings_in_rtc_step(
        &mut self,
        rx_time: u32,
        sleep_time: u32,
    ) -> Result<(), Error> {
        let buf = [
            OpCode::SetRxDutyCycle as u8,
            ((rx_time >> 16) & 0xFF) as u8,
            ((rx_time >> 8) & 0xFF) as u8,
            (rx_time & 0xFF) as u8,
            ((sleep_time >> 16) & 0xFF) as u8,
            ((sleep_time >> 8) & 0xFF) as u8,
            (sleep_time & 0xFF) as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set CAD mode
    pub fn set_cad(&mut self) -> Result<(), Error> {
        let buf = [OpCode::SetCad as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set TX continuous wave mode
    pub fn set_tx_cw(&mut self) -> Result<(), Error> {
        let buf = [OpCode::SetTxContinuousWave as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set TX infinite preamble mode
    pub fn set_tx_infinite_preamble(&mut self) -> Result<(), Error> {
        let buf = [OpCode::SetTxInfinitePreamble as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set regulator mode
    pub fn set_reg_mode(&mut self, mode: RegMode) -> Result<(), Error> {
        let buf = [OpCode::SetRegulatorMode as u8, mode as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Perform calibration
    pub fn calibrate(&mut self, mask: u8) -> Result<(), Error> {
        let buf = [OpCode::Calibrate as u8, mask];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Calibrate image
    pub fn calibrate_image(&mut self, freq1: u8, freq2: u8) -> Result<(), Error> {
        let buf = [OpCode::CalibrateImage as u8, freq1, freq2];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Calibrate image in MHz
    pub fn calibrate_image_in_mhz(&mut self, freq1_mhz: u16, freq2_mhz: u16) -> Result<(), Error> {
        let freq1 = (freq1_mhz / IMAGE_CALIBRATION_STEP_IN_MHZ as u16) as u8;
        let freq2 = (freq2_mhz / IMAGE_CALIBRATION_STEP_IN_MHZ as u16) as u8;
        self.calibrate_image(freq1, freq2)
    }

    /// Set PA configuration
    pub fn set_pa_cfg(&mut self, params: &PaCfgParams) -> Result<(), Error> {
        let buf = [
            OpCode::SetPaCfg as u8,
            params.pa_duty_cycle,
            params.hp_max,
            params.device_sel,
            params.pa_lut,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set RX/TX fallback mode
    pub fn set_rx_tx_fallback_mode(&mut self, mode: FallbackMode) -> Result<(), Error> {
        let buf = [OpCode::SetRxTxFallbackMode as u8, mode as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Write to register
    pub fn write_register(&mut self, address: u16, buffer: &[u8]) -> Result<(), Error> {
        let mut cmd = [0u8; 4];
        cmd[0] = OpCode::WriteRegister as u8;
        cmd[1] = ((address >> 8) & 0xFF) as u8;
        cmd[2] = (address & 0xFF) as u8;
        cmd[3..4 + buffer.len().min(252)].copy_from_slice(&buffer[..buffer.len().min(252)]);
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write(&cmd[..3]).map_err(|_| Error::Hal)?;
        self.hal.write(buffer).map_err(|_| Error::Hal)
    }

    /// Read from register
    pub fn read_register(&mut self, address: u16, buffer: &mut [u8]) -> Result<(), Error> {
        let cmd = [
            OpCode::ReadRegister as u8,
            ((address >> 8) & 0xFF) as u8,
            (address & 0xFF) as u8,
            NOP,
        ];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, buffer).map_err(|_| Error::Hal)
    }

    /// Write to buffer
    pub fn write_buffer(&mut self, offset: u8, buffer: &[u8]) -> Result<(), Error> {
        let cmd = [OpCode::WriteBuffer as u8, offset];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write(&cmd).map_err(|_| Error::Hal)?;
        self.hal.write(buffer).map_err(|_| Error::Hal)
    }

    /// Read from buffer
    pub fn read_buffer(&mut self, offset: u8, buffer: &mut [u8]) -> Result<(), Error> {
        let cmd = [OpCode::ReadBuffer as u8, offset, NOP];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, buffer).map_err(|_| Error::Hal)
    }

    /// Set DIO IRQ parameters
    pub fn set_dio_irq_params(
        &mut self,
        irq_mask: u16,
        dio1_mask: u16,
        dio2_mask: u16,
        dio3_mask: u16,
    ) -> Result<(), Error> {
        let buf = [
            OpCode::SetDioIrqParams as u8,
            ((irq_mask >> 8) & 0xFF) as u8,
            (irq_mask & 0xFF) as u8,
            ((dio1_mask >> 8) & 0xFF) as u8,
            (dio1_mask & 0xFF) as u8,
            ((dio2_mask >> 8) & 0xFF) as u8,
            (dio2_mask & 0xFF) as u8,
            ((dio3_mask >> 8) & 0xFF) as u8,
            (dio3_mask & 0xFF) as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Get IRQ status
    pub fn get_irq_status(&mut self) -> Result<u16, Error> {
        let cmd = [OpCode::GetIrqStatus as u8, NOP, NOP, NOP];
        let mut resp = [0u8; 3];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        Ok(((resp[1] as u16) << 8) | (resp[2] as u16))
    }

    /// Clear IRQ status
    pub fn clear_irq_status(&mut self, irq_mask: u16) -> Result<(), Error> {
        let buf = [
            OpCode::ClrIrqStatus as u8,
            ((irq_mask >> 8) & 0xFF) as u8,
            (irq_mask & 0xFF) as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Get and clear IRQ status
    pub fn get_and_clear_irq_status(&mut self) -> Result<u16, Error> {
        let irq = self.get_irq_status()?;
        if irq != 0 {
            self.clear_irq_status(irq)?;
        }
        Ok(irq)
    }

    /// Set DIO2 as RF switch control
    pub fn set_dio2_as_rf_sw_ctrl(&mut self, enable: bool) -> Result<(), Error> {
        let buf = [OpCode::SetDio2AsRfSwitchCtrl as u8, enable as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set DIO3 as TCXO control
    pub fn set_dio3_as_tcxo_ctrl(&mut self, voltage: TcxoCtrlVoltage, timeout: u32) -> Result<(), Error> {
        let buf = [
            OpCode::SetDio3AsTcxoCtrl as u8,
            voltage as u8,
            ((timeout >> 16) & 0xFF) as u8,
            ((timeout >> 8) & 0xFF) as u8,
            (timeout & 0xFF) as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set RF frequency in Hz
    pub fn set_rf_freq(&mut self, freq_in_hz: u32) -> Result<(), Error> {
        let freq_pll = convert_freq_in_hz_to_pll_step(freq_in_hz);
        self.set_rf_freq_in_pll_steps(freq_pll)
    }

    /// Set RF frequency in PLL steps
    pub fn set_rf_freq_in_pll_steps(&mut self, freq: u32) -> Result<(), Error> {
        let buf = [
            OpCode::SetRfFrequency as u8,
            ((freq >> 24) & 0xFF) as u8,
            ((freq >> 16) & 0xFF) as u8,
            ((freq >> 8) & 0xFF) as u8,
            (freq & 0xFF) as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set packet type
    pub fn set_pkt_type(&mut self, pkt_type: PacketType) -> Result<(), Error> {
        let buf = [OpCode::SetPktType as u8, pkt_type as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Get packet type
    pub fn get_pkt_type(&mut self) -> Result<PacketType, Error> {
        let cmd = [OpCode::GetPktType as u8, NOP, NOP];
        let mut resp = [0u8; 2];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        match resp[1] {
            0x00 => Ok(PacketType::Gfsk),
            0x01 => Ok(PacketType::Lora),
            0x02 => Ok(PacketType::Bpsk),
            0x03 => Ok(PacketType::LrFhss),
            _ => Err(Error::InvalidParameter),
        }
    }

    /// Set TX parameters
    pub fn set_tx_params(&mut self, power_dbm: i8, ramp_time: RampTime) -> Result<(), Error> {
        let buf = [OpCode::SetTxParams as u8, power_dbm as u8, ramp_time as u8];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Reset the radio
    pub fn reset(&mut self) -> Result<(), Error> {
        self.hal.reset().map_err(|_| Error::Hal)
    }

    /// Wake up from sleep
    pub fn wakeup(&mut self) -> Result<(), Error> {
        let buf = [OpCode::GetStatus as u8, NOP];
        self.hal.write(&buf).map_err(|_| Error::Hal)
    }

    /// Set buffer base addresses
    pub fn set_buffer_base_address(&mut self, tx_base: u8, rx_base: u8) -> Result<(), Error> {
        let buf = [OpCode::SetBufferBaseAddress as u8, tx_base, rx_base];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set LoRa symbol number timeout
    pub fn set_lora_symb_nb_timeout(&mut self, nb_of_symbs: u8) -> Result<(), Error> {
        if nb_of_symbs > MAX_LORA_SYMB_NUM_TIMEOUT {
            return Err(Error::InvalidParameter);
        }
        
        if nb_of_symbs == 0 {
            // Disable timeout
            let mut reg = [0u8];
            self.read_register(crate::regs::REG_LR_SYNCH_TIMEOUT, &mut reg)?;
            reg[0] &= 0xF8;
            self.write_register(crate::regs::REG_LR_SYNCH_TIMEOUT, &reg)?;
            return Ok(());
        }
        
        // Calculate exp and mant
        let mut exp = 0u8;
        let mut mant = nb_of_symbs;
        
        while mant > 31 {
            mant = (mant + 1) >> 1;
            exp += 1;
        }
        
        let reg = [exp + (mant << 3)];
        self.write_register(crate::regs::REG_LR_SYNCH_TIMEOUT, &reg)
    }

    /// Get chip status
    pub fn get_status(&mut self) -> Result<ChipStatus, Error> {
        let cmd = [OpCode::GetStatus as u8];
        let mut resp = [0u8; 1];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        let cmd_status_val = (resp[0] >> 1) & 0x07;
        let chip_mode_val = (resp[0] >> 4) & 0x07;
        
        let cmd_status = match cmd_status_val {
            0 => CommandStatus::Reserved,
            1 => CommandStatus::Rfu,
            2 => CommandStatus::DataAvailable,
            3 => CommandStatus::CommandTimeout,
            4 => CommandStatus::CommandProcessError,
            5 => CommandStatus::CommandExecFailure,
            6 => CommandStatus::CommandTxDone,
            _ => CommandStatus::Reserved,
        };
        
        let chip_mode = match chip_mode_val {
            0 => ChipMode::Unused,
            1 => ChipMode::Rfu,
            2 => ChipMode::StandbyRc,
            3 => ChipMode::StandbyXosc,
            4 => ChipMode::Fs,
            5 => ChipMode::Rx,
            6 => ChipMode::Tx,
            _ => ChipMode::Unused,
        };
        
        Ok(ChipStatus { cmd_status, chip_mode })
    }

    /// Get RX buffer status
    pub fn get_rx_buffer_status(&mut self) -> Result<RxBufferStatus, Error> {
        let cmd = [OpCode::GetRxBufferStatus as u8, NOP, NOP];
        let mut resp = [0u8; 2];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        Ok(RxBufferStatus {
            pld_len_in_bytes: resp[0],
            buffer_start_pointer: resp[1],
        })
    }

    /// Get GFSK packet status
    pub fn get_gfsk_pkt_status(&mut self) -> Result<GfskPktStatus, Error> {
        let cmd = [OpCode::GetPktStatus as u8, NOP, NOP, NOP];
        let mut resp = [0u8; 3];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        let rx_status = GfskRxStatus {
            pkt_sent: (resp[0] & 0x01) != 0,
            pkt_received: (resp[0] & 0x02) != 0,
            abort_error: (resp[0] & 0x04) != 0,
            length_error: (resp[0] & 0x08) != 0,
            crc_error: (resp[0] & 0x10) != 0,
            adrs_error: (resp[0] & 0x20) != 0,
        };
        
        Ok(GfskPktStatus {
            rx_status,
            rssi_sync: -((resp[1] as i8) >> 1),
            rssi_avg: -((resp[2] as i8) >> 1),
        })
    }

    /// Get LoRa packet status
    pub fn get_lora_pkt_status(&mut self) -> Result<LoraPktStatus, Error> {
        let cmd = [OpCode::GetPktStatus as u8, NOP, NOP, NOP];
        let mut resp = [0u8; 3];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        let rssi_pkt = -(resp[0] as i8) / 2;
        let snr_pkt = (resp[1] as i8) / 4;
        let signal_rssi = -(resp[2] as i8) / 2;
        
        Ok(LoraPktStatus {
            rssi_pkt_in_dbm: rssi_pkt,
            snr_pkt_in_db: snr_pkt,
            signal_rssi_pkt_in_dbm: signal_rssi,
        })
    }

    /// Get instantaneous RSSI
    pub fn get_rssi_inst(&mut self) -> Result<i16, Error> {
        let cmd = [OpCode::GetRssiInst as u8, NOP, NOP];
        let mut resp = [0u8; 2];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        Ok(-(resp[1] as i16) / 2)
    }

    /// Get GFSK statistics
    pub fn get_gfsk_stats(&mut self) -> Result<GfskStats, Error> {
        let cmd = [OpCode::GetStats as u8, NOP, NOP, NOP, NOP, NOP, NOP, NOP];
        let mut resp = [0u8; 6];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        Ok(GfskStats {
            nb_pkt_received: ((resp[0] as u16) << 8) | (resp[1] as u16),
            nb_pkt_crc_error: ((resp[2] as u16) << 8) | (resp[3] as u16),
            nb_pkt_len_error: ((resp[4] as u16) << 8) | (resp[5] as u16),
        })
    }

    /// Get LoRa statistics
    pub fn get_lora_stats(&mut self) -> Result<LoraStats, Error> {
        let cmd = [OpCode::GetStats as u8, NOP, NOP, NOP, NOP, NOP, NOP, NOP];
        let mut resp = [0u8; 6];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        Ok(LoraStats {
            nb_pkt_received: ((resp[0] as u16) << 8) | (resp[1] as u16),
            nb_pkt_crc_error: ((resp[2] as u16) << 8) | (resp[3] as u16),
            nb_pkt_header_error: ((resp[4] as u16) << 8) | (resp[5] as u16),
        })
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) -> Result<(), Error> {
        let buf = [OpCode::ResetStats as u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Get device errors
    pub fn get_device_errors(&mut self) -> Result<u16, Error> {
        let cmd = [OpCode::GetDeviceErrors as u8, NOP, NOP, NOP];
        let mut resp = [0u8; 2];
        
        self.hal.wait_on_busy().map_err(|_| Error::Hal)?;
        self.hal.write_read(&cmd, &mut resp).map_err(|_| Error::Hal)?;
        
        Ok(((resp[0] as u16) << 8) | (resp[1] as u16))
    }

    /// Clear device errors
    pub fn clear_device_errors(&mut self) -> Result<(), Error> {
        let buf = [OpCode::ClrDeviceErrors as u8, 0x00, 0x00];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set GFSK modulation parameters
    pub fn set_gfsk_mod_params(&mut self, params: &GfskModParams) -> Result<(), Error> {
        let br_bytes = params.br_in_bps.to_be_bytes();
        let fdev_bytes = params.fdev_in_hz.to_be_bytes();
        
        let buf = [
            OpCode::SetModulationParams as u8,
            br_bytes[1], br_bytes[2], br_bytes[3],
            params.pulse_shape as u8,
            params.bw_dsb_param as u8,
            fdev_bytes[1], fdev_bytes[2], fdev_bytes[3],
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set LoRa modulation parameters
    pub fn set_lora_mod_params(&mut self, params: &LoraModParams) -> Result<(), Error> {
        let buf = [
            OpCode::SetModulationParams as u8,
            params.sf as u8,
            params.bw as u8,
            params.cr as u8,
            params.ldro,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set GFSK packet parameters
    pub fn set_gfsk_pkt_params(&mut self, params: &GfskPktParams) -> Result<(), Error> {
        let preamble_bytes = params.preamble_len_in_bits.to_be_bytes();
        
        let buf = [
            OpCode::SetPktParams as u8,
            preamble_bytes[0], preamble_bytes[1],
            params.preamble_detector as u8,
            params.sync_word_len_in_bits,
            params.address_filtering as u8,
            params.header_type as u8,
            params.pld_len_in_bytes,
            params.crc_type as u8,
            params.dc_free as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set LoRa packet parameters
    pub fn set_lora_pkt_params(&mut self, params: &LoraPktParams) -> Result<(), Error> {
        let preamble_bytes = params.preamble_len_in_symb.to_be_bytes();
        
        let buf = [
            OpCode::SetPktParams as u8,
            preamble_bytes[0], preamble_bytes[1],
            params.header_type as u8,
            params.pld_len_in_bytes,
            params.crc_is_on as u8,
            params.invert_iq_is_on as u8,
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set CAD parameters
    pub fn set_cad_params(&mut self, params: &CadParams) -> Result<(), Error> {
        let timeout_bytes = params.cad_timeout.to_be_bytes();
        
        let buf = [
            OpCode::SetCadParams as u8,
            params.cad_symb_nb as u8,
            params.cad_detect_peak,
            params.cad_detect_min,
            params.cad_exit_mode as u8,
            timeout_bytes[1], timeout_bytes[2], timeout_bytes[3],
        ];
        self.write_command(&buf).map_err(|_| Error::Hal)
    }

    /// Set GFSK packet address
    pub fn set_gfsk_pkt_address(&mut self, node_address: u8, broadcast_address: u8) -> Result<(), Error> {
        self.write_register(crate::regs::REG_SYNC_WORD_BASE_ADDRESS + 8, &[node_address])?;
        self.write_register(crate::regs::REG_SYNC_WORD_BASE_ADDRESS + 9, &[broadcast_address])
    }

    /// Generate random numbers
    pub fn get_random_numbers(&mut self, numbers: &mut [u32]) -> Result<(), Error> {
        for num in numbers.iter_mut() {
            let mut bytes = [0u8; 4];
            self.read_register(crate::regs::REG_RNG_BASE_ADDRESS, &mut bytes)?;
            *num = u32::from_be_bytes(bytes);
        }
        Ok(())
    }

    /// Handle RX done (stop RTC timer)
    pub fn handle_rx_done(&mut self) -> Result<(), Error> {
        // Stop RTC and clear timeout event
        self.stop_rtc()
    }

    /// Stop RTC timer (workaround)
    pub fn stop_rtc(&mut self) -> Result<(), Error> {
        let mut reg = [0u8];
        self.read_register(crate::regs::REG_RTC_CTRL, &mut reg)?;
        reg[0] |= 0x01;
        self.write_register(crate::regs::REG_RTC_CTRL, &reg)?;
        
        self.read_register(crate::regs::REG_EVT_CLR, &mut reg)?;
        reg[0] |= crate::regs::REG_EVT_CLR_TIMEOUT_MASK;
        self.write_register(crate::regs::REG_EVT_CLR, &reg)
    }

    /// Configure RX boosted mode
    pub fn cfg_rx_boosted(&mut self, enable: bool) -> Result<(), Error> {
        let value = if enable { 0x96 } else { 0x94 };
        self.write_register(crate::regs::REG_RX_GAIN, &[value])
    }

    /// Set GFSK sync word
    pub fn set_gfsk_sync_word(&mut self, sync_word: &[u8]) -> Result<(), Error> {
        if sync_word.len() > 8 {
            return Err(Error::InvalidParameter);
        }
        self.write_register(crate::regs::REG_SYNC_WORD_BASE_ADDRESS, sync_word)
    }

    /// Set LoRa sync word
    pub fn set_lora_sync_word(&mut self, sync_word: u8) -> Result<(), Error> {
        let msb = ((sync_word & 0xF0) | 0x04) & 0xF4;
        let lsb = ((sync_word & 0x0F) | 0x04) & 0x4F;
        self.write_register(crate::regs::REG_LR_SYNC_WORD, &[msb, lsb])
    }

    /// Set GFSK CRC seed
    pub fn set_gfsk_crc_seed(&mut self, seed: u16) -> Result<(), Error> {
        let bytes = seed.to_be_bytes();
        self.write_register(crate::regs::REG_CRC_SEED_BASE_ADDRESS, &bytes)
    }

    /// Set GFSK CRC polynomial
    pub fn set_gfsk_crc_polynomial(&mut self, polynomial: u16) -> Result<(), Error> {
        let bytes = polynomial.to_be_bytes();
        self.write_register(crate::regs::REG_CRC_POLY_BASE_ADDRESS, &bytes)
    }

    /// Set GFSK whitening seed
    pub fn set_gfsk_whitening_seed(&mut self, seed: u16) -> Result<(), Error> {
        let bytes = seed.to_be_bytes();
        self.write_register(crate::regs::REG_WHIT_SEED_BASE_ADDRESS, &bytes)
    }

    /// Configure TX clamp (workaround for SX1262)
    pub fn cfg_tx_clamp(&mut self) -> Result<(), Error> {
        let mut reg = [0u8];
        self.read_register(crate::regs::REG_TX_CLAMP_CFG, &mut reg)?;
        reg[0] = (reg[0] & !crate::regs::REG_TX_CLAMP_CFG_MASK) | 0x1E;
        self.write_register(crate::regs::REG_TX_CLAMP_CFG, &reg)
    }

    /// Set over-current protection value
    pub fn set_ocp_value(&mut self, ocp_in_step_of_2_5_ma: u8) -> Result<(), Error> {
        if ocp_in_step_of_2_5_ma > 63 {
            return Err(Error::InvalidParameter);
        }
        self.write_register(crate::regs::REG_OCP, &[ocp_in_step_of_2_5_ma])
    }

    /// Set trimming capacitor values
    pub fn set_trimming_capacitor_values(&mut self, trim_xta: u8, trim_xtb: u8) -> Result<(), Error> {
        let value = ((trim_xtb & 0x0F) << 4) | (trim_xta & 0x0F);
        self.write_register(crate::regs::REG_XTA_TRIM, &[value])
    }

    /// Add registers to retention list
    pub fn add_registers_to_retention_list(&mut self, addresses: &[u16]) -> Result<(), Error> {
        if addresses.len() > MAX_NB_REG_IN_RETENTION as usize {
            return Err(Error::InvalidParameter);
        }
        
        for addr in addresses {
            let bytes = addr.to_be_bytes();
            // This would need to interact with the retention list register
            // Implementation depends on specific chip behavior
            self.write_register(0x09CD, &bytes)?;
        }
        Ok(())
    }

    /// Initialize retention list with workaround registers
    pub fn init_retention_list(&mut self) -> Result<(), Error> {
        let registers = [
            crate::regs::REG_RX_GAIN,
            crate::regs::REG_TX_MODULATION,
            crate::regs::REG_IQ_POLARITY,
        ];
        self.add_registers_to_retention_list(&registers)
    }

    /// Get LoRa parameters from received header
    pub fn get_lora_params_from_header(&mut self) -> Result<(LoraCr, bool), Error> {
        let mut reg = [0u8];
        
        // Read CR
        self.read_register(crate::regs::REG_LR_HEADER_CR, &mut reg)?;
        let cr_val = (reg[0] >> crate::regs::REG_LR_HEADER_CR_POS) & 0x07;
        let cr = match cr_val {
            1 => LoraCr::Cr45,
            2 => LoraCr::Cr46,
            3 => LoraCr::Cr47,
            4 => LoraCr::Cr48,
            _ => return Err(Error::InvalidParameter),
        };
        
        // Read CRC
        self.read_register(crate::regs::REG_LR_HEADER_CRC, &mut reg)?;
        let crc_is_on = ((reg[0] >> crate::regs::REG_LR_HEADER_CRC_POS) & 0x01) != 0;
        
        Ok((cr, crc_is_on))
    }

    // Helper function to write command
    fn write_command(&mut self, data: &[u8]) -> Result<(), H::Error> {
        self.hal.wait_on_busy()?;
        self.hal.write(data)
    }
}

/// Convert frequency in Hz to PLL steps
pub fn convert_freq_in_hz_to_pll_step(freq_in_hz: u32) -> u32 {
    // Formula: freq_pll = (freq_hz * 2^25) / XTAL_FREQ
    let freq_in_hz_64 = freq_in_hz as u64;
    let pll_step = (freq_in_hz_64 << 25) / (XTAL_FREQ as u64);
    pll_step as u32
}

/// Convert timeout in ms to RTC steps
pub fn convert_timeout_in_ms_to_rtc_step(timeout_in_ms: u32) -> u32 {
    timeout_in_ms * 64
}
