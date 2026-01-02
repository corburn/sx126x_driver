# Changelog - Rust Version

All notable changes to the Rust version of the SX126x driver will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-02

### Added - Initial Rust Migration

#### Core Driver Features
- Complete Rust implementation of SX126x radio driver
- `no_std` compatible for embedded systems
- Platform-agnostic HAL trait for hardware abstraction
- Type-safe API using Rust enums and structs
- Comprehensive error handling with `Result` types

#### Operational Modes
- Sleep mode with warm/cold start options
- Standby mode (RC and XOSC)
- Frequency synthesis mode
- TX and RX modes with configurable timeouts
- RX duty cycle mode
- CAD (Channel Activity Detection) mode
- Continuous wave and infinite preamble test modes

#### LoRa Support (Complete)
- Full LoRa modulation parameter configuration
- Spreading factors: SF5 to SF12
- Bandwidths: 7.8 kHz to 500 kHz
- Coding rates: 4/5, 4/6, 4/7, 4/8
- Explicit and implicit header modes
- CRC and IQ inversion support
- Time-on-air calculations
- Packet status (RSSI, SNR)
- Statistics tracking

#### GFSK Support (Complete)
- Full GFSK modulation parameter configuration
- Bitrates up to several hundred kbps
- Multiple bandwidth options
- Pulse shaping (Gaussian BT filters)
- Sync word configuration
- Address filtering
- Whitening and CRC
- Time-on-air calculations
- Packet status and statistics

#### Register and Buffer Operations
- Direct register read/write
- Buffer read/write operations
- Configurable base addresses

#### Interrupt Management
- DIO interrupt configuration
- IRQ status reading and clearing
- Support for all interrupt sources
- Get-and-clear atomic operation

#### RF Configuration
- Frequency setting in Hz or PLL steps
- PA configuration
- TX power and ramp time settings
- TX/RX fallback modes
- DIO2 as RF switch control
- DIO3 as TCXO control

#### Calibration
- Manual calibration of all blocks
- Image calibration by frequency range
- Automated calibration helpers

#### Power Management
- Regulator mode selection (LDO/DCDC)
- Over-current protection configuration
- RX boost mode
- Trimming capacitor configuration

#### Workarounds
- GFSK 600 bps workaround
- GFSK 1200 bps workaround
- TX modulation workaround for LoRa 500 kHz
- Inverted IQ optimization
- Antenna mismatch resistance (SX1262)
- RTC stop workaround
- TX clamp configuration

#### Utilities
- Frequency conversion (Hz to PLL steps)
- Timeout conversion (ms to RTC steps)
- LoRa time-on-air calculation
- GFSK time-on-air calculation
- GFSK bandwidth parameter lookup
- LoRa bandwidth to Hz conversion

#### Documentation
- Comprehensive README with usage examples
- Inline rustdoc documentation
- HAL trait implementation guide
- Migration guide from C driver
- Complete API reference
- nRF54L15 example application
- Action plan for remaining work

#### Build System
- Cargo.toml with proper metadata
- Feature flags: `bpsk`, `lr-fhss`
- Size optimization profile
- embedded-hal 1.0 dependency
- no_std compatibility verified

### Planned (Not Yet Implemented)

#### BPSK Support
- Basic module structure created
- Full implementation pending
- Requires BPSK-capable hardware for testing

#### LR-FHSS Support  
- Basic module structure created
- Full implementation pending
- MAC layer migration needed
- Requires LR-FHSS network for testing

### Notes on C to Rust Migration

#### Type Safety Improvements
- Enums instead of defines for mode selection
- Structs for complex parameters
- No raw pointer manipulation in public API
- Compile-time checks for many errors

#### Error Handling
- `Result<T, Error>` instead of status codes
- Explicit error propagation with `?` operator
- No silent failures

#### Memory Safety
- Bounds-checked array access
- No buffer overflows
- Ownership system prevents use-after-free
- No unsafe code in driver (only in user HAL if needed)

#### Platform Abstraction
- HAL trait instead of function pointers
- Generic over HAL implementation
- Easy to mock for testing
- Platform-specific code isolated in HAL

### Breaking Changes from C API

- HAL functions are now trait methods
- Functions return `Result` instead of status codes
- Parameters use type-safe enums instead of raw values
- Buffer lengths use `usize` instead of `uint8_t`
- Some function names adjusted for Rust conventions

### Compatibility

- Maintains protocol compatibility with C driver
- Radio registers and commands identical
- Can communicate with devices using C driver
- Configuration values portable between versions

### Known Limitations

- BPSK module is stub only (feature incomplete)
- LR-FHSS module is stub only (feature incomplete)
- Some platform-specific optimizations not yet implemented
- Limited to embedded-hal 1.0 compatible platforms

### Testing Status

- ✅ Compiles successfully
- ✅ Passes clippy lints (with warnings noted)
- ✅ Formatted with rustfmt
- ⏳ Hardware validation pending
- ⏳ Interoperability testing pending
- ⏳ Long-term reliability testing pending

### Dependencies

- `embedded-hal = "1.0"` - Hardware abstraction layer traits

### Supported Hardware

Designed for use with:
- Seeed Studio XIAO nRF54L15 SoC
- Seeed Studio SX1262 LoRa module for XIAO
- Generic SX1261/SX1262/SX1268 modules
- Any platform with SPI and GPIO support

### License

The Clear BSD License - Same as original C driver
Copyright Semtech Corporation 2021-2025

---

[0.1.0]: https://github.com/corburn/sx126x_driver/releases/tag/v0.1.0
