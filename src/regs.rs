//! Register address definitions for SX126x

/// CRC seed base address
pub const REG_CRC_SEED_BASE_ADDRESS: u16 = 0x06BC;

/// CRC polynomial base address
pub const REG_CRC_POLY_BASE_ADDRESS: u16 = 0x06BE;

/// Whitening seed base address
pub const REG_WHIT_SEED_BASE_ADDRESS: u16 = 0x06B8;

/// Sync word base address
pub const REG_SYNC_WORD_BASE_ADDRESS: u16 = 0x06C0;

/// LoRa sync word (0x1424 for private, 0x3444 for public LoRaWAN)
pub const REG_LR_SYNC_WORD: u16 = 0x0740;

/// LoRa header coding rate
pub const REG_LR_HEADER_CR: u16 = 0x0749;
pub const REG_LR_HEADER_CR_POS: u8 = 4;
pub const REG_LR_HEADER_CR_MASK: u8 = 0x07 << REG_LR_HEADER_CR_POS;

/// LoRa header CRC
pub const REG_LR_HEADER_CRC: u16 = 0x076B;
pub const REG_LR_HEADER_CRC_POS: u8 = 4;
pub const REG_LR_HEADER_CRC_MASK: u8 = 0x01 << REG_LR_HEADER_CRC_POS;

/// Random number generator base address
pub const REG_RNG_BASE_ADDRESS: u16 = 0x0819;

/// Analog LNA
pub const REG_ANA_LNA: u16 = 0x08E2;

/// Analog mixer
pub const REG_ANA_MIXER: u16 = 0x08E5;

/// RX gain (0x94 power saving, 0x96 boosted)
pub const REG_RX_GAIN: u16 = 0x08AC;

/// Crystal trimming capacitor
pub const REG_XTA_TRIM: u16 = 0x0911;

/// Over-current protection
pub const REG_OCP: u16 = 0x08E7;

/// IQ polarity (workaround)
pub const REG_IQ_POLARITY: u16 = 0x0736;

/// TX modulation (workaround)
pub const REG_TX_MODULATION: u16 = 0x0889;

/// TX clamp configuration (workaround)
pub const REG_TX_CLAMP_CFG: u16 = 0x08D8;
pub const REG_TX_CLAMP_CFG_POS: u8 = 1;
pub const REG_TX_CLAMP_CFG_MASK: u8 = 0x0F << REG_TX_CLAMP_CFG_POS;

/// RTC control
pub const REG_RTC_CTRL: u16 = 0x0902;

/// Event clear
pub const REG_EVT_CLR: u16 = 0x0944;
pub const REG_EVT_CLR_TIMEOUT_POS: u8 = 1;
pub const REG_EVT_CLR_TIMEOUT_MASK: u8 = 0x01 << REG_EVT_CLR_TIMEOUT_POS;

/// RX address pointer
pub const REG_RX_ADDRESS_POINTER: u16 = 0x0803;

/// RX/TX payload length
pub const REG_RXTX_PAYLOAD_LEN: u16 = 0x06BB;

/// Output disable register
pub const REG_OUT_DIS_REG: u16 = 0x0580;
pub const REG_OUT_DIS_REG_DIO3_POS: u8 = 3;
pub const REG_OUT_DIS_REG_DIO3_MASK: u8 = 0x01 << REG_OUT_DIS_REG_DIO3_POS;

/// Input enable register
pub const REG_IN_EN_REG: u16 = 0x0583;
pub const REG_IN_EN_REG_DIO3_POS: u8 = 3;
pub const REG_IN_EN_REG_DIO3_MASK: u8 = 0x01 << REG_IN_EN_REG_DIO3_POS;

/// TX bitbang A register
pub const REG_BITBANG_A_REG: u16 = 0x0680;
pub const REG_BITBANG_A_REG_ENABLE_POS: u8 = 4;
pub const REG_BITBANG_A_REG_ENABLE_MASK: u8 = 0x07 << REG_BITBANG_A_REG_ENABLE_POS;
pub const REG_BITBANG_A_REG_ENABLE_VAL: u8 = 0x01 << REG_BITBANG_A_REG_ENABLE_POS;

/// TX bitbang B register
pub const REG_BITBANG_B_REG: u16 = 0x0587;
pub const REG_BITBANG_B_REG_ENABLE_POS: u8 = 0;
pub const REG_BITBANG_B_REG_ENABLE_MASK: u8 = 0x0F << REG_BITBANG_B_REG_ENABLE_POS;
pub const REG_BITBANG_B_REG_ENABLE_VAL: u8 = 0x0C << REG_BITBANG_B_REG_ENABLE_POS;

/// LoRa sync timeout
pub const REG_LR_SYNCH_TIMEOUT: u16 = 0x0706;

// GFSK workaround registers
pub const REG_GFSK_WORKAROUND_1: u16 = 0x06D1;
pub const REG_GFSK_WORKAROUND_2: u16 = 0x089B;
pub const REG_GFSK_WORKAROUND_3: u16 = 0x08B8;
pub const REG_GFSK_WORKAROUND_4: u16 = 0x06AC;
