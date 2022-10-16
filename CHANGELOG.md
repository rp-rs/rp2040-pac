# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] [Crates.io](https://crates.io/crates/rp2040-pac/0.4.0) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.4.0)

- Added all possible variants for DREQ enum in CH*_CTRL*
- Update rustdoc to clarify DMA CHAIN_TO behaviour
- Add 6 unused interrupts as sw[0..5]_irq under new SW_IRQ peripheral
- Pin to specific svdtools to avoid spontaneous breakage
- Disable clippy warning for derive_partial_eq_without_eq
- Update to SVD from pico-sdk 1.4.0

## [0.3.0] [Crates.io](https://crates.io/crates/rp2040-pac/0.3.0) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.3.0)

- Change `FUNCSEL` on `GPIOx_CTRL` to be generic
    - it was previously duplicating GPIO0 functions across all GPIO pins
- Arrayify `spinlockX` registers in `SIO`
- Rebuild with svd2rust 0.21.0

## [0.2.1] [Crates.io](https://crates.io/crates/rp2040-pac/0.2.1) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.2.1)

- Rebuild with svd2rust 0.2.0
- Improvements to comment formatting from newer svd2rust

## [0.2.0] [Crates.io](https://crates.io/crates/rp2040-pac/0.2.0) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.2.0)

- Update source SVD to pico-sdk 1.3.0
- Remove patches that were no longer required thanks to new SVD file.
- Arrayify `procX_intX` registers in `IO_BANK1`. This is a breaking change.

## [0.1.5] [Crates.io](https://crates.io/crates/rp2040-pac/0.1.5) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.5)

- Update cortex-m-rt to 0.7.0
- Fix cargo license declaration
- arrayify USB endpoint registers
- add USB line state variants as enum

## [0.1.4] [Crates.io](https://crates.io/crates/rp2040-pac/0.1.4) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.4)

- Removed broken register (as per errata)
- Fixed lineendings in documentation
- Update svd2rust to 0.19.0
- Made SIE_STATUS CONNECTED read-write
- Fixed update script for OSX
- Changed SIO SPINLOCKs to read-write

## [0.1.3] [Crates.io](https://crates.io/crates/rp2040-pac/0.1.3) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.3)

- Update source SVD to pico-sdk 1.2.0
- Cluster PWM channels
- Bump cortex-m dep to 0.7.3

## [0.1.2] [Crates.io](https://crates.io/crates/rp2040-pac/0.1.2) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.2)

- Switched GPIO for IO_BANK and QSPI_BANK to be arrays instead.
- Change BUFF_STATUS access to read-write
- Re-clustered IO_QSPI. Fixed naming to remove double underscore
- Renamed GPIO_QSPI_[STAT,CTRL] -> GPIO_[STAT,CTRL]
- Convert DMA chunnels to a list of register clusters

## [0.1.1] [Crates.io](https://crates.io/crates/rp2040-pac/0.1.1) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.1)

- Created using svd2rust 0.17.0 from https://github.com/raspberrypi/pico-sdk/blob/1.0.0/src/rp2040/hardware_regs/rp2040.svd
- Alphabetized Peripherals

## [0.1.0] [Crates.io](https://crates.io/crates/rp2040-pac/0.1.0) [Github](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.0)

- Initialized crate

[Unreleased]: https://github.com/rp-rs/rp2040-pac/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/rp-rs/rp2040-pac/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/rp-rs/rp2040-pac/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/rp-rs/rp2040-pac/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/rp-rs/rp2040-pac/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/rp-rs/rp2040-pac/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/rp-rs/rp2040-pac/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/rp-rs/rp2040-pac/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.0
