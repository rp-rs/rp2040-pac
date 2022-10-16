# rp2040-pac - PAC for Raspberry Pi RP2040 microcontrollers

This is a [Peripheral Access Crate] for the Raspberry Pi [RP2040] dual-core
Cortex-M0+ microcontroller.

[Peripheral Access Crate]: https://rust-embedded.github.io/book/start/registers.html
[RP2040]: https://datasheets.raspberrypi.org/rp2040/rp2040_datasheet.pdf

This crate has been built using [svd2rust] version 0.21 and [svdtools], using
the SVD file in the [pico-sdk v1.4.0]. Some manual fixes have been made to the
documentation formatting, and the removal of some unrecognized lints.

[svd2rust]: https://github.com/rust-embedded/svd2rust
[svdtools]: https://github.com/stm32-rs/svdtools
[pico-sdk v1.4.0]: https://github.com/raspberrypi/pico-sdk/blob/1.4.0/src/rp2040/hardware_regs/rp2040.svd

## Licence

The contents of this crate are auto-generated and licensed under the same terms
as the underlying SVD file, which is licensed by Raspberry Pi (Trading)) Ltd
under a BSD-3-Clause licence.

## Changelog

See the [CHANGELOG.md file]

[CHANGELOG.md file]: ./CHANGELOG.md
