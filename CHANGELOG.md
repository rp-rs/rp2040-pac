# Changelog

## Unreleased Changes

Working on patching the svd for better workflow, using [svdtools](https://pypi.org/project/svdtools/)

## Release 0.1.3
[Release 0.1.3 on Crates.io](https://crates.io/crates/rp2040-pac/0.1.3)

[Release 0.1.3 on GitHub](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.3)

- Update source SVD to pico-sdk 1.2.0
- Cluster PWM channels
- Bump cortex-m dep to 0.7.3

## Release 0.1.2
[Release 0.1.2 on Crates.io](https://crates.io/crates/rp2040-pac/0.1.2)

[Release 0.1.2 on GitHub](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.2)

- Switched GPIO for IO_BANK and QSPI_BANK to be arrays instead.
- Change BUFF_STATUS access to read-write
- Re-clustered IO_QSPI. Fixed naming to remove double underscore
- Renamed GPIO_QSPI_[STAT,CTRL] -> GPIO_[STAT,CTRL]
- Convert DMA chunnels to a list of register clusters

## Release 0.1.1
[Release 0.1.1 on Crates.io](https://crates.io/crates/rp2040-pac/0.1.1)

[Release 0.1.1 on GitHub](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.1)

- Created using svd2rust 0.17.0 from https://github.com/raspberrypi/pico-sdk/blob/1.0.0/src/rp2040/hardware_regs/rp2040.svd
- Alphabetized Peripherals

## Release 0.1.0
[Release 0.1.0 on Crates.io](https://crates.io/crates/rp2040-pac/0.1.0)

[Release 0.1.0 on GitHub](https://github.com/rp-rs/rp2040-pac/releases/tag/v0.1.0)

- Initialized crate
