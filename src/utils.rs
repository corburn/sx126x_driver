//! Utility functions and helpers for SX126x driver

use crate::types::*;

/// Get GFSK bandwidth parameter for a given bandwidth in Hz
pub fn get_gfsk_bw_param(bw_hz: u32) -> Option<GfskBw> {
    match bw_hz {
        0..=4800 => Some(GfskBw::Bw4800),
        4801..=5800 => Some(GfskBw::Bw5800),
        5801..=7300 => Some(GfskBw::Bw7300),
        7301..=9700 => Some(GfskBw::Bw9700),
        9701..=11700 => Some(GfskBw::Bw11700),
        11701..=14600 => Some(GfskBw::Bw14600),
        14601..=19500 => Some(GfskBw::Bw19500),
        19501..=23400 => Some(GfskBw::Bw23400),
        23401..=29300 => Some(GfskBw::Bw29300),
        29301..=39000 => Some(GfskBw::Bw39000),
        39001..=46900 => Some(GfskBw::Bw46900),
        46901..=58600 => Some(GfskBw::Bw58600),
        58601..=78200 => Some(GfskBw::Bw78200),
        78201..=93800 => Some(GfskBw::Bw93800),
        93801..=117300 => Some(GfskBw::Bw117300),
        117301..=156200 => Some(GfskBw::Bw156200),
        156201..=187200 => Some(GfskBw::Bw187200),
        187201..=234300 => Some(GfskBw::Bw234300),
        234301..=312000 => Some(GfskBw::Bw312000),
        312001..=373600 => Some(GfskBw::Bw373600),
        373601.. => Some(GfskBw::Bw467000),
    }
}

/// Get LoRa time-on-air numerator
///
/// To get actual time-on-air, divide this by the bandwidth in Hz
pub fn get_lora_time_on_air_numerator(
    pkt_params: &LoraPktParams,
    mod_params: &LoraModParams,
) -> u32 {
    let preamble_len = pkt_params.preamble_len_in_symb as i32;
    let sf = mod_params.sf as u8 as i32;
    let cr_denom = (mod_params.cr as u8 as i32) + 4;
    let pld_len = pkt_params.pld_len_in_bytes as i32;
    let crc = if pkt_params.crc_is_on { 16 } else { 0 };
    let header = if pkt_params.header_type == LoraPktLenMode::Explicit {
        20
    } else {
        0
    };
    let ldro = mod_params.ldro as i32;

    // Calculate the numerator
    let payload_symb_nb = 8 * pld_len + crc - 4 * sf + header + 8;
    let mut n_payload = if payload_symb_nb > 0 {
        ((payload_symb_nb * cr_denom + 4 * sf - 8 * ldro - 16) / (4 * (sf - 2 * ldro))) * cr_denom
    } else {
        0
    };

    if n_payload < 0 {
        n_payload = 0;
    }

    let preamble_duration = (preamble_len + 4) * (1 << sf);
    let payload_duration = n_payload * (1 << sf);

    ((preamble_duration + payload_duration + 1) * 1000) as u32
}

/// Get LoRa time-on-air in milliseconds
pub fn get_lora_time_on_air_in_ms(pkt_params: &LoraPktParams, mod_params: &LoraModParams) -> u32 {
    let numerator = get_lora_time_on_air_numerator(pkt_params, mod_params);
    let bw_hz = mod_params.bw.to_hz();
    (numerator + (bw_hz / 2)) / bw_hz
}

/// Get GFSK time-on-air numerator
///
/// To get actual time-on-air, divide this by the bitrate in bps
pub fn get_gfsk_time_on_air_numerator(pkt_params: &GfskPktParams) -> u32 {
    let preamble = pkt_params.preamble_len_in_bits as u32;
    let sync_word = pkt_params.sync_word_len_in_bits as u32;
    let payload = 8 * pkt_params.pld_len_in_bytes as u32;

    let header = match pkt_params.header_type {
        GfskPktLenMode::Fixed => 0,
        GfskPktLenMode::Variable => 8,
    };

    let crc = match pkt_params.crc_type {
        GfskCrcType::Off => 0,
        GfskCrcType::Byte1 | GfskCrcType::Byte1Inv => 8,
        GfskCrcType::Bytes2 | GfskCrcType::Bytes2Inv => 16,
    };

    preamble + sync_word + header + payload + crc
}

/// Get GFSK time-on-air in milliseconds
pub fn get_gfsk_time_on_air_in_ms(pkt_params: &GfskPktParams, mod_params: &GfskModParams) -> u32 {
    let numerator = get_gfsk_time_on_air_numerator(pkt_params);
    let br_bps = mod_params.br_in_bps;
    ((numerator * 1000 + br_bps / 2) / br_bps) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gfsk_bw_param() {
        assert_eq!(get_gfsk_bw_param(4000), Some(GfskBw::Bw4800));
        assert_eq!(get_gfsk_bw_param(10000), Some(GfskBw::Bw11700));
        assert_eq!(get_gfsk_bw_param(500000), Some(GfskBw::Bw467000));
    }

    #[test]
    fn test_lora_bw_to_hz() {
        assert_eq!(LoraBw::Bw125.to_hz(), 125_000);
        assert_eq!(LoraBw::Bw250.to_hz(), 250_000);
        assert_eq!(LoraBw::Bw500.to_hz(), 500_000);
    }
}
