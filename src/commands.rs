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
