# SX126x Rust Driver

A `no_std` Rust driver for Semtech SX126x series LoRa transceivers (SX1261, SX1262, SX1268), based on the official C driver from Semtech.

## Features

- **Platform-agnostic**: Uses `embedded-hal` traits for maximum portability
- **no_std compatible**: Works in bare-metal embedded environments
- **Type-safe API**: Leverages Rust's type system for safer radio configuration
- **Feature flags**: Optional BPSK and LR-FHSS support
- **Well-documented**: Comprehensive documentation with examples

## Hardware Support

Tested with:
- Seeed Studio XIAO nRF54L15 SoC
- Seeed Studio SX1262 LoRa module for XIAO

Also compatible with:
- SX1261
- SX1262  
- SX1268

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sx126x = "0.1"
```

### Optional Features

```toml
[dependencies]
sx126x = { version = "0.1", features = ["bpsk", "lr-fhss"] }
```

- `bpsk`: Enable BPSK modulation support
- `lr-fhss`: Enable LR-FHSS (Long Range Frequency Hopping Spread Spectrum) support

## Usage

### Implementing the HAL

The driver requires you to implement the `Hal` trait for your platform:

```rust
use sx126x::hal::Hal;

struct MyHal {
    // Your platform-specific SPI and GPIO handles
    spi: Spi,
    reset_pin: OutputPin,
    busy_pin: InputPin,
}

impl Hal for MyHal {
    type Error = MyError;

    fn reset(&mut self) -> Result<(), Self::Error> {
        // Toggle NRST pin low then high
        self.reset_pin.set_low()?;
        delay_ms(10);
        self.reset_pin.set_high()?;
        delay_ms(10);
        Ok(())
    }

    fn wait_on_busy(&mut self) -> Result<(), Self::Error> {
        // Wait for BUSY pin to go low
        while self.busy_pin.is_high()? {
            // Could add timeout here
        }
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        // Write to SPI
        self.spi.write(data)?;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Read from SPI
        self.spi.transfer(buffer, buffer)?;
        Ok(())
    }

    fn write_read(&mut self, write_data: &[u8], read_data: &mut [u8]) -> Result<(), Self::Error> {
        // Write then read from SPI
        self.spi.write(write_data)?;
        self.spi.transfer(read_data, read_data)?;
        Ok(())
    }
}
```

### Basic LoRa Example

```rust
use sx126x::{SX126x, types::*};

// Create driver instance
let hal = MyHal::new(/* ... */);
let mut radio = SX126x::new(hal);

// Reset and initialize
radio.reset()?;
radio.set_standby(StandbyConfig::Rc)?;

// Configure regulator mode
radio.set_reg_mode(RegMode::Dcdc)?;

// Calibrate
radio.calibrate(CalMask::ALL)?;
radio.calibrate_image_in_mhz(902, 928)?;

// Set packet type to LoRa
radio.set_pkt_type(PacketType::Lora)?;

// Configure RF frequency (915 MHz)
radio.set_rf_freq(915_000_000)?;

// Configure PA
let pa_cfg = PaCfgParams {
    pa_duty_cycle: 0x04,
    hp_max: 0x07,
    device_sel: 0x00,
    pa_lut: 0x01,
};
radio.set_pa_cfg(&pa_cfg)?;

// Set TX power
radio.set_tx_params(14, RampTime::Ramp40Us)?;

// Configure LoRa modulation
let mod_params = LoraModParams {
    sf: LoraSf::Sf7,
    bw: LoraBw::Bw125,
    cr: LoraCr::Cr45,
    ldro: 0,
};
radio.set_lora_mod_params(&mod_params)?;

// Configure packet parameters
let pkt_params = LoraPktParams {
    preamble_len_in_symb: 8,
    header_type: LoraPktLenMode::Explicit,
    pld_len_in_bytes: 64,
    crc_is_on: true,
    invert_iq_is_on: false,
};
radio.set_lora_pkt_params(&pkt_params)?;

// Set DIO IRQ parameters
radio.set_dio_irq_params(
    IrqMask::ALL,
    IrqMask::TX_DONE | IrqMask::RX_DONE | IrqMask::TIMEOUT,
    0,
    0,
)?;

// Transmit
let payload = b"Hello, LoRa!";
radio.write_buffer(0, payload)?;
radio.set_tx(1000)?; // 1 second timeout

// Or receive
radio.set_rx(5000)?; // 5 second timeout
```

### GFSK Example

```rust
use sx126x::types::*;

// Set packet type to GFSK
radio.set_pkt_type(PacketType::Gfsk)?;

// Configure GFSK modulation
let mod_params = GfskModParams {
    br_in_bps: 50_000,
    fdev_in_hz: 25_000,
    pulse_shape: GfskPulseShape::Bt05,
    bw_dsb_param: GfskBw::Bw117300,
};
radio.set_gfsk_mod_params(&mod_params)?;

// Configure packet parameters
let pkt_params = GfskPktParams {
    preamble_len_in_bits: 40,
    preamble_detector: GfskPreambleDetector::Min16Bits,
    sync_word_len_in_bits: 24,
    address_filtering: GfskAddressFiltering::Disable,
    header_type: GfskPktLenMode::Variable,
    pld_len_in_bytes: 64,
    crc_type: GfskCrcType::Bytes2,
    dc_free: GfskDcFree::Whitening,
};
radio.set_gfsk_pkt_params(&pkt_params)?;
```

## Differences from C Driver

### Type Safety

The Rust driver uses enums and type-safe wrappers instead of raw integers:

```rust
// C: sx126x_set_standby(context, SX126X_STANDBY_CFG_RC);
// Rust:
radio.set_standby(StandbyConfig::Rc)?;
```

### Error Handling

Returns `Result<T, Error>` instead of status codes:

```rust
match radio.set_tx(1000) {
    Ok(()) => println!("TX started"),
    Err(e) => println!("Error: {:?}", e),
}
```

### HAL Abstraction

Platform-specific code is abstracted behind the `Hal` trait, making the driver
portable across different microcontrollers and operating systems.

### Memory Safety

No raw pointers or unsafe code in the driver itself (unless required by your HAL implementation).

## Examples

See the `examples/` directory for complete examples for various platforms:

- `examples/nrf54l15.rs` - Nordic nRF54L15
- `examples/stm32.rs` - STM32 microcontrollers

## Documentation

Full API documentation is available at [docs.rs/sx126x](https://docs.rs/sx126x)

Or generate locally:

```bash
cargo doc --open --no-deps
```

## License

This driver maintains the same license as the original Semtech C driver:

**The Clear BSD License**

Copyright Semtech Corporation 2021-2025. All rights reserved.

See [LICENSE.txt](LICENSE.txt) for details.

## Contributing

Contributions are welcome! Please ensure:

1. Code compiles without warnings: `cargo build`
2. Passes clippy: `cargo clippy`
3. Is formatted: `cargo fmt`
4. Documentation is updated

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

## References

- [SX1261/62 Datasheet](https://semtech.my.salesforce.com/sfc/p/E0000000JelG/a/44000000MDnH/V3QJv_qF4Dv7JQ8K4pXKiJjPqV3QH5wEMg5ljXjBOLA)
- [Original C Driver](https://github.com/Lora-net/sx126x_driver)
- [LoRaWAN Specification](https://lora-alliance.org/resource_hub/lorawan-specification-v1-0-4/)

## Related Projects

- [lora-rs](https://github.com/lora-rs/lora-rs) - Complete LoRaWAN stack in Rust
- [embedded-hal](https://github.com/rust-embedded/embedded-hal) - Hardware abstraction layer traits
