//! Type definitions for SX126x driver
//!
//! This module contains all the enums, structs, and constants used by the driver.

/// Maximum timeout value in RTC steps
pub const MAX_TIMEOUT_IN_RTC_STEP: u32 = 0x00FF_FFFE;

/// Maximum timeout in milliseconds
pub const MAX_TIMEOUT_IN_MS: u32 = MAX_TIMEOUT_IN_RTC_STEP / 64;

/// RX single mode - stays in RX until reception occurs
pub const RX_SINGLE_MODE: u32 = 0x0000_0000;

/// RX continuous mode
pub const RX_CONTINUOUS: u32 = 0x00FF_FFFF;

/// Over-current protection value for 60mA
pub const OCP_PARAM_VALUE_60_MA: u8 = 0x18;

/// Over-current protection value for 140mA
pub const OCP_PARAM_VALUE_140_MA: u8 = 0x38;

/// Default trimming capacitor value in standby XOSC mode
pub const XTAL_TRIMMING_CAPACITOR_DEFAULT_VALUE_STDBY_XOSC: u8 = 0x12;

/// Maximum LoRa symbol number timeout
pub const MAX_LORA_SYMB_NUM_TIMEOUT: u8 = 248;

/// Maximum number of registers in retention list
pub const MAX_NB_REG_IN_RETENTION: u8 = 4;

/// Image calibration step in MHz
pub const IMAGE_CALIBRATION_STEP_IN_MHZ: u8 = 4;

/// Internal crystal frequency
pub const XTAL_FREQ: u32 = 32_000_000;

/// RTC frequency in Hz
pub const RTC_FREQ_IN_HZ: u32 = 64_000;

/// Sleep mode configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SleepConfig {
    /// Cold start - configuration is lost
    ColdStart = 0 << 2,
    /// Warm start - configuration retained
    WarmStart = 1 << 2,
}

/// Standby mode configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StandbyConfig {
    /// Standby with RC oscillator
    Rc = 0x00,
    /// Standby with XOSC
    Xosc = 0x01,
}

/// Regulator mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegMode {
    /// LDO regulator (default)
    Ldo = 0x00,
    /// DC-DC regulator
    Dcdc = 0x01,
}

/// Power amplifier configuration parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaCfgParams {
    /// PA duty cycle
    pub pa_duty_cycle: u8,
    /// High power maximum
    pub hp_max: u8,
    /// Device selection
    pub device_sel: u8,
    /// PA LUT
    pub pa_lut: u8,
}

/// Fallback mode after TX or RX
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FallbackMode {
    /// Standby with RC oscillator
    StandbyRc = 0x20,
    /// Standby with XOSC
    StandbyXosc = 0x30,
    /// Frequency synthesis mode
    Fs = 0x40,
}

/// IRQ mask flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IrqMask(pub u16);

impl IrqMask {
    /// No interrupt
    pub const NONE: u16 = 0;
    /// TX done
    pub const TX_DONE: u16 = 1 << 0;
    /// RX done
    pub const RX_DONE: u16 = 1 << 1;
    /// Preamble detected
    pub const PREAMBLE_DETECTED: u16 = 1 << 2;
    /// Sync word valid
    pub const SYNC_WORD_VALID: u16 = 1 << 3;
    /// Header valid
    pub const HEADER_VALID: u16 = 1 << 4;
    /// Header error
    pub const HEADER_ERROR: u16 = 1 << 5;
    /// CRC error
    pub const CRC_ERROR: u16 = 1 << 6;
    /// CAD done
    pub const CAD_DONE: u16 = 1 << 7;
    /// CAD detected
    pub const CAD_DETECTED: u16 = 1 << 8;
    /// Timeout
    pub const TIMEOUT: u16 = 1 << 9;
    /// LR-FHSS hop
    pub const LR_FHSS_HOP: u16 = 1 << 14;
    /// All interrupts
    pub const ALL: u16 = Self::TX_DONE
        | Self::RX_DONE
        | Self::PREAMBLE_DETECTED
        | Self::SYNC_WORD_VALID
        | Self::HEADER_VALID
        | Self::HEADER_ERROR
        | Self::CRC_ERROR
        | Self::CAD_DONE
        | Self::CAD_DETECTED
        | Self::TIMEOUT
        | Self::LR_FHSS_HOP;
}

/// Calibration mask flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalMask(pub u8);

impl CalMask {
    /// RC 64kHz calibration
    pub const RC64K: u8 = 1 << 0;
    /// RC 13MHz calibration
    pub const RC13M: u8 = 1 << 1;
    /// PLL calibration
    pub const PLL: u8 = 1 << 2;
    /// ADC pulse calibration
    pub const ADC_PULSE: u8 = 1 << 3;
    /// ADC bulk N calibration
    pub const ADC_BULK_N: u8 = 1 << 4;
    /// ADC bulk P calibration
    pub const ADC_BULK_P: u8 = 1 << 5;
    /// Image calibration
    pub const IMAGE: u8 = 1 << 6;
    /// All calibrations
    pub const ALL: u8 = Self::RC64K
        | Self::RC13M
        | Self::PLL
        | Self::ADC_PULSE
        | Self::ADC_BULK_N
        | Self::ADC_BULK_P
        | Self::IMAGE;
}

/// TCXO control voltage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TcxoCtrlVoltage {
    /// 1.6V
    V1_6 = 0x00,
    /// 1.7V
    V1_7 = 0x01,
    /// 1.8V
    V1_8 = 0x02,
    /// 2.2V
    V2_2 = 0x03,
    /// 2.4V
    V2_4 = 0x04,
    /// 2.7V
    V2_7 = 0x05,
    /// 3.0V
    V3_0 = 0x06,
    /// 3.3V
    V3_3 = 0x07,
}

/// Packet type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    /// GFSK modulation
    Gfsk = 0x00,
    /// LoRa modulation
    Lora = 0x01,
    /// BPSK modulation
    Bpsk = 0x02,
    /// LR-FHSS modulation
    LrFhss = 0x03,
}

/// PA ramp time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RampTime {
    /// 10 microseconds
    Ramp10Us = 0x00,
    /// 20 microseconds
    Ramp20Us = 0x01,
    /// 40 microseconds
    Ramp40Us = 0x02,
    /// 80 microseconds
    Ramp80Us = 0x03,
    /// 200 microseconds
    Ramp200Us = 0x04,
    /// 800 microseconds
    Ramp800Us = 0x05,
    /// 1700 microseconds
    Ramp1700Us = 0x06,
    /// 3400 microseconds
    Ramp3400Us = 0x07,
}

/// GFSK pulse shape
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GfskPulseShape {
    /// No filtering
    Off = 0x00,
    /// Gaussian BT 0.3
    Bt03 = 0x08,
    /// Gaussian BT 0.5
    Bt05 = 0x09,
    /// Gaussian BT 0.7
    Bt07 = 0x0A,
    /// Gaussian BT 1.0
    Bt1 = 0x0B,
}

/// GFSK bandwidth
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GfskBw {
    /// 4.8 kHz
    Bw4800 = 0x1F,
    /// 5.8 kHz
    Bw5800 = 0x17,
    /// 7.3 kHz
    Bw7300 = 0x0F,
    /// 9.7 kHz
    Bw9700 = 0x1E,
    /// 11.7 kHz
    Bw11700 = 0x16,
    /// 14.6 kHz
    Bw14600 = 0x0E,
    /// 19.5 kHz
    Bw19500 = 0x1D,
    /// 23.4 kHz
    Bw23400 = 0x15,
    /// 29.3 kHz
    Bw29300 = 0x0D,
    /// 39.0 kHz
    Bw39000 = 0x1C,
    /// 46.9 kHz
    Bw46900 = 0x14,
    /// 58.6 kHz
    Bw58600 = 0x0C,
    /// 78.2 kHz
    Bw78200 = 0x1B,
    /// 93.8 kHz
    Bw93800 = 0x13,
    /// 117.3 kHz
    Bw117300 = 0x0B,
    /// 156.2 kHz
    Bw156200 = 0x1A,
    /// 187.2 kHz
    Bw187200 = 0x12,
    /// 234.3 kHz
    Bw234300 = 0x0A,
    /// 312.0 kHz
    Bw312000 = 0x19,
    /// 373.6 kHz
    Bw373600 = 0x11,
    /// 467.0 kHz
    Bw467000 = 0x09,
}

/// GFSK modulation parameters
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GfskModParams {
    /// Bit rate in bps
    pub br_in_bps: u32,
    /// Frequency deviation in Hz
    pub fdev_in_hz: u32,
    /// Pulse shaping
    pub pulse_shape: GfskPulseShape,
    /// Bandwidth (DSB)
    pub bw_dsb_param: GfskBw,
}

/// LoRa spreading factor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LoraSf {
    /// Spreading factor 5
    Sf5 = 0x05,
    /// Spreading factor 6
    Sf6 = 0x06,
    /// Spreading factor 7
    Sf7 = 0x07,
    /// Spreading factor 8
    Sf8 = 0x08,
    /// Spreading factor 9
    Sf9 = 0x09,
    /// Spreading factor 10
    Sf10 = 0x0A,
    /// Spreading factor 11
    Sf11 = 0x0B,
    /// Spreading factor 12
    Sf12 = 0x0C,
}

/// LoRa bandwidth
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LoraBw {
    /// 500 kHz
    Bw500 = 6,
    /// 250 kHz
    Bw250 = 5,
    /// 125 kHz
    Bw125 = 4,
    /// 62.5 kHz
    Bw062 = 3,
    /// 41.7 kHz
    Bw041 = 10,
    /// 31.25 kHz
    Bw031 = 2,
    /// 20.8 kHz
    Bw020 = 9,
    /// 15.6 kHz
    Bw015 = 1,
    /// 10.4 kHz
    Bw010 = 8,
    /// 7.8 kHz
    Bw007 = 0,
}

impl LoraBw {
    /// Get bandwidth in Hz
    pub fn to_hz(self) -> u32 {
        match self {
            LoraBw::Bw007 => 7_800,
            LoraBw::Bw010 => 10_400,
            LoraBw::Bw015 => 15_600,
            LoraBw::Bw020 => 20_800,
            LoraBw::Bw031 => 31_250,
            LoraBw::Bw041 => 41_700,
            LoraBw::Bw062 => 62_500,
            LoraBw::Bw125 => 125_000,
            LoraBw::Bw250 => 250_000,
            LoraBw::Bw500 => 500_000,
        }
    }
}

/// LoRa coding rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LoraCr {
    /// 4/5 coding rate
    Cr45 = 0x01,
    /// 4/6 coding rate
    Cr46 = 0x02,
    /// 4/7 coding rate
    Cr47 = 0x03,
    /// 4/8 coding rate
    Cr48 = 0x04,
}

/// LoRa modulation parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoraModParams {
    /// Spreading factor
    pub sf: LoraSf,
    /// Bandwidth
    pub bw: LoraBw,
    /// Coding rate
    pub cr: LoraCr,
    /// Low data rate optimization (0 or 1)
    pub ldro: u8,
}

/// GFSK preamble detector length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GfskPreambleDetector {
    /// Preamble detector off
    Off = 0x00,
    /// 8 bits minimum
    Min8Bits = 0x04,
    /// 16 bits minimum
    Min16Bits = 0x05,
    /// 24 bits minimum
    Min24Bits = 0x06,
    /// 32 bits minimum
    Min32Bits = 0x07,
}

/// GFSK address filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GfskAddressFiltering {
    /// Disabled
    Disable = 0x00,
    /// Node address only
    NodeAddress = 0x01,
    /// Node and broadcast addresses
    NodeAndBroadcastAddresses = 0x02,
}

/// GFSK packet length mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GfskPktLenMode {
    /// Fixed length
    Fixed = 0x00,
    /// Variable length
    Variable = 0x01,
}

/// GFSK CRC type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GfskCrcType {
    /// CRC off
    Off = 0x01,
    /// 1 byte CRC
    Byte1 = 0x00,
    /// 2 bytes CRC
    Bytes2 = 0x02,
    /// 1 byte inverted CRC
    Byte1Inv = 0x04,
    /// 2 bytes inverted CRC
    Bytes2Inv = 0x06,
}

/// GFSK DC-free encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GfskDcFree {
    /// Off
    Off = 0x00,
    /// Whitening
    Whitening = 0x01,
}

/// LoRa packet length mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LoraPktLenMode {
    /// Explicit header
    Explicit = 0x00,
    /// Implicit header
    Implicit = 0x01,
}

/// LoRa packet parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoraPktParams {
    /// Preamble length in symbols
    pub preamble_len_in_symb: u16,
    /// Header type
    pub header_type: LoraPktLenMode,
    /// Payload length in bytes
    pub pld_len_in_bytes: u8,
    /// CRC enabled
    pub crc_is_on: bool,
    /// IQ inversion
    pub invert_iq_is_on: bool,
}

/// GFSK packet parameters
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GfskPktParams {
    /// Preamble length in bits
    pub preamble_len_in_bits: u16,
    /// Preamble detector
    pub preamble_detector: GfskPreambleDetector,
    /// Sync word length in bits
    pub sync_word_len_in_bits: u8,
    /// Address filtering
    pub address_filtering: GfskAddressFiltering,
    /// Header type
    pub header_type: GfskPktLenMode,
    /// Payload length in bytes
    pub pld_len_in_bytes: u8,
    /// CRC type
    pub crc_type: GfskCrcType,
    /// DC-free encoding
    pub dc_free: GfskDcFree,
}

/// CAD number of symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CadSymbols {
    /// 1 symbol
    Symb1 = 0x00,
    /// 2 symbols
    Symb2 = 0x01,
    /// 4 symbols
    Symb4 = 0x02,
    /// 8 symbols
    Symb8 = 0x03,
    /// 16 symbols
    Symb16 = 0x04,
}

/// CAD exit mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CadExitMode {
    /// CAD only
    CadOnly = 0x00,
    /// Switch to RX
    CadRx = 0x01,
    /// Listen before talk
    CadLbt = 0x10,
}

/// CAD parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CadParams {
    /// Number of symbols
    pub cad_symb_nb: CadSymbols,
    /// Peak detection threshold
    pub cad_detect_peak: u8,
    /// Minimum detection threshold
    pub cad_detect_min: u8,
    /// Exit mode
    pub cad_exit_mode: CadExitMode,
    /// Timeout
    pub cad_timeout: u32,
}

/// Chip mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipMode {
    /// Unused
    Unused = 0,
    /// Reserved for future use
    Rfu = 1,
    /// Standby RC
    StandbyRc = 2,
    /// Standby XOSC
    StandbyXosc = 3,
    /// Frequency synthesis
    Fs = 4,
    /// RX mode
    Rx = 5,
    /// TX mode
    Tx = 6,
}

/// Command status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CommandStatus {
    /// Reserved
    Reserved = 0,
    /// Reserved for future use
    Rfu = 1,
    /// Data available
    DataAvailable = 2,
    /// Command timeout
    CommandTimeout = 3,
    /// Command processing error
    CommandProcessError = 4,
    /// Command execution failure
    CommandExecFailure = 5,
    /// TX done
    CommandTxDone = 6,
}

/// Chip status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChipStatus {
    /// Command status
    pub cmd_status: CommandStatus,
    /// Chip mode
    pub chip_mode: ChipMode,
}

/// RX buffer status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RxBufferStatus {
    /// Payload length in bytes
    pub pld_len_in_bytes: u8,
    /// Buffer start pointer
    pub buffer_start_pointer: u8,
}

/// GFSK RX status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GfskRxStatus {
    /// Packet sent
    pub pkt_sent: bool,
    /// Packet received
    pub pkt_received: bool,
    /// Abort error
    pub abort_error: bool,
    /// Length error
    pub length_error: bool,
    /// CRC error
    pub crc_error: bool,
    /// Address error
    pub adrs_error: bool,
}

/// GFSK packet status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GfskPktStatus {
    /// RX status
    pub rx_status: GfskRxStatus,
    /// RSSI sync
    pub rssi_sync: i8,
    /// RSSI average
    pub rssi_avg: i8,
}

/// LoRa packet status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoraPktStatus {
    /// RSSI of packet in dBm
    pub rssi_pkt_in_dbm: i8,
    /// SNR of packet in dB
    pub snr_pkt_in_db: i8,
    /// Signal RSSI of packet in dBm
    pub signal_rssi_pkt_in_dbm: i8,
}

/// GFSK statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GfskStats {
    /// Number of packets received
    pub nb_pkt_received: u16,
    /// Number of CRC errors
    pub nb_pkt_crc_error: u16,
    /// Number of length errors
    pub nb_pkt_len_error: u16,
}

/// LoRa statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoraStats {
    /// Number of packets received
    pub nb_pkt_received: u16,
    /// Number of CRC errors
    pub nb_pkt_crc_error: u16,
    /// Number of header errors
    pub nb_pkt_header_error: u16,
}

/// Error flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorMask(pub u16);

impl ErrorMask {
    /// RC 64kHz calibration error
    pub const RC64K_CALIBRATION: u16 = 1 << 0;
    /// RC 13MHz calibration error
    pub const RC13M_CALIBRATION: u16 = 1 << 1;
    /// PLL calibration error
    pub const PLL_CALIBRATION: u16 = 1 << 2;
    /// ADC calibration error
    pub const ADC_CALIBRATION: u16 = 1 << 3;
    /// Image calibration error
    pub const IMG_CALIBRATION: u16 = 1 << 4;
    /// XOSC start error
    pub const XOSC_START: u16 = 1 << 5;
    /// PLL lock error
    pub const PLL_LOCK: u16 = 1 << 6;
    /// PA ramp error
    pub const PA_RAMP: u16 = 1 << 8;
}
