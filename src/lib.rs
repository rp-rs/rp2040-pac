#![doc = "Peripheral access API for RP2040 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn TIMER_IRQ_0();
    fn TIMER_IRQ_1();
    fn TIMER_IRQ_2();
    fn TIMER_IRQ_3();
    fn PWM_IRQ_WRAP();
    fn USBCTRL_IRQ();
    fn XIP_IRQ();
    fn PIO0_IRQ_0();
    fn PIO0_IRQ_1();
    fn PIO1_IRQ_0();
    fn PIO1_IRQ_1();
    fn DMA_IRQ_0();
    fn DMA_IRQ_1();
    fn IO_IRQ_BANK0();
    fn IO_IRQ_QSPI();
    fn SIO_IRQ_PROC0();
    fn SIO_IRQ_PROC1();
    fn CLOCKS_IRQ();
    fn SPI0_IRQ();
    fn SPI1_IRQ();
    fn UART0_IRQ();
    fn UART1_IRQ();
    fn ADC_IRQ_FIFO();
    fn I2C0_IRQ();
    fn I2C1_IRQ();
    fn RTC_IRQ();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 26] = [
    Vector {
        _handler: TIMER_IRQ_0,
    },
    Vector {
        _handler: TIMER_IRQ_1,
    },
    Vector {
        _handler: TIMER_IRQ_2,
    },
    Vector {
        _handler: TIMER_IRQ_3,
    },
    Vector {
        _handler: PWM_IRQ_WRAP,
    },
    Vector {
        _handler: USBCTRL_IRQ,
    },
    Vector { _handler: XIP_IRQ },
    Vector {
        _handler: PIO0_IRQ_0,
    },
    Vector {
        _handler: PIO0_IRQ_1,
    },
    Vector {
        _handler: PIO1_IRQ_0,
    },
    Vector {
        _handler: PIO1_IRQ_1,
    },
    Vector {
        _handler: DMA_IRQ_0,
    },
    Vector {
        _handler: DMA_IRQ_1,
    },
    Vector {
        _handler: IO_IRQ_BANK0,
    },
    Vector {
        _handler: IO_IRQ_QSPI,
    },
    Vector {
        _handler: SIO_IRQ_PROC0,
    },
    Vector {
        _handler: SIO_IRQ_PROC1,
    },
    Vector {
        _handler: CLOCKS_IRQ,
    },
    Vector { _handler: SPI0_IRQ },
    Vector { _handler: SPI1_IRQ },
    Vector {
        _handler: UART0_IRQ,
    },
    Vector {
        _handler: UART1_IRQ,
    },
    Vector {
        _handler: ADC_IRQ_FIFO,
    },
    Vector { _handler: I2C0_IRQ },
    Vector { _handler: I2C1_IRQ },
    Vector { _handler: RTC_IRQ },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - TIMER_IRQ_0"]
    TIMER_IRQ_0 = 0,
    #[doc = "1 - TIMER_IRQ_1"]
    TIMER_IRQ_1 = 1,
    #[doc = "2 - TIMER_IRQ_2"]
    TIMER_IRQ_2 = 2,
    #[doc = "3 - TIMER_IRQ_3"]
    TIMER_IRQ_3 = 3,
    #[doc = "4 - PWM_IRQ_WRAP"]
    PWM_IRQ_WRAP = 4,
    #[doc = "5 - USBCTRL_IRQ"]
    USBCTRL_IRQ = 5,
    #[doc = "6 - XIP_IRQ"]
    XIP_IRQ = 6,
    #[doc = "7 - PIO0_IRQ_0"]
    PIO0_IRQ_0 = 7,
    #[doc = "8 - PIO0_IRQ_1"]
    PIO0_IRQ_1 = 8,
    #[doc = "9 - PIO1_IRQ_0"]
    PIO1_IRQ_0 = 9,
    #[doc = "10 - PIO1_IRQ_1"]
    PIO1_IRQ_1 = 10,
    #[doc = "11 - DMA_IRQ_0"]
    DMA_IRQ_0 = 11,
    #[doc = "12 - DMA_IRQ_1"]
    DMA_IRQ_1 = 12,
    #[doc = "13 - IO_IRQ_BANK0"]
    IO_IRQ_BANK0 = 13,
    #[doc = "14 - IO_IRQ_QSPI"]
    IO_IRQ_QSPI = 14,
    #[doc = "15 - SIO_IRQ_PROC0"]
    SIO_IRQ_PROC0 = 15,
    #[doc = "16 - SIO_IRQ_PROC1"]
    SIO_IRQ_PROC1 = 16,
    #[doc = "17 - CLOCKS_IRQ"]
    CLOCKS_IRQ = 17,
    #[doc = "18 - SPI0_IRQ"]
    SPI0_IRQ = 18,
    #[doc = "19 - SPI1_IRQ"]
    SPI1_IRQ = 19,
    #[doc = "20 - UART0_IRQ"]
    UART0_IRQ = 20,
    #[doc = "21 - UART1_IRQ"]
    UART1_IRQ = 21,
    #[doc = "22 - ADC_IRQ_FIFO"]
    ADC_IRQ_FIFO = 22,
    #[doc = "23 - I2C0_IRQ"]
    I2C0_IRQ = 23,
    #[doc = "24 - I2C1_IRQ"]
    I2C1_IRQ = 24,
    #[doc = "25 - RTC_IRQ"]
    RTC_IRQ = 25,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "QSPI flash execute-in-place block"]
pub struct XIP_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XIP_CTRL {}
impl XIP_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xip_ctrl::RegisterBlock {
        0x1400_0000 as *const _
    }
}
impl Deref for XIP_CTRL {
    type Target = xip_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*XIP_CTRL::ptr() }
    }
}
#[doc = "QSPI flash execute-in-place block"]
pub mod xip_ctrl;
#[doc = "DW_apb_ssi has the following features:\\n * APB interface – Allows for easy integration into a DesignWare Synthesizable Components for AMBA 2 implementation.\\n * APB3 and APB4 protocol support.\\n * Scalable APB data bus width – Supports APB data bus widths of 8, 16, and 32 bits.\\n * Serial-master or serial-slave operation – Enables serial communication with serial-master or serial-slave peripheral devices.\\n * Programmable Dual/Quad/Octal SPI support in Master Mode.\\n * Dual Data Rate (DDR) and Read Data Strobe (RDS) Support - Enables the DW_apb_ssi master to perform operations with the device in DDR and RDS modes when working in Dual/Quad/Octal mode of operation.\\n * Data Mask Support - Enables the DW_apb_ssi to selectively update the bytes in the device. This feature is applicable only in enhanced SPI modes.\\n * eXecute-In-Place (XIP) support - Enables the DW_apb_ssi master to behave as a memory mapped I/O and fetches the data from the device based on the APB read request. This feature is applicable only in enhanced SPI modes.\\n * DMA Controller Interface – Enables the DW_apb_ssi to interface to a DMA controller over the bus using a handshaking interface for transfer requests.\\n * Independent masking of interrupts – Master collision, transmit FIFO overflow, transmit FIFO empty, receive FIFO full, receive FIFO underflow, and receive FIFO overflow interrupts can all be masked independently.\\n * Multi-master contention detection – Informs the processor of multiple serial-master accesses on the serial bus.\\n * Bypass of meta-stability flip-flops for synchronous clocks – When the APB clock (pclk) and the DW_apb_ssi serial clock (ssi_clk) are synchronous, meta-stable flip-flops are not used when transferring control signals across these clock domains.\\n * Programmable delay on the sample time of the received serial data bit (rxd); enables programmable control of routing delays resulting in higher serial data-bit rates.\\n * Programmable features:\\n - Serial interface operation – Choice of Motorola SPI, Texas Instruments Synchronous Serial Protocol or National Semiconductor Microwire.\\n - Clock bit-rate – Dynamic control of the serial bit rate of the data transfer; used in only serial-master mode of operation.\\n - Data Item size (4 to 32 bits) – Item size of each data transfer under the control of the programmer.\\n * Configured features:\\n - FIFO depth – 16 words deep. The FIFO width is fixed at 32 bits.\\n - 1 slave select output.\\n - Hardware slave-select – Dedicated hardware slave-select line.\\n - Combined interrupt line - one combined interrupt line from the DW_apb_ssi to the interrupt controller.\\n - Interrupt polarity – active high interrupt lines.\\n - Serial clock polarity – low serial-clock polarity directly after reset.\\n - Serial clock phase – capture on first edge of serial-clock directly after reset."]
pub struct XIP_SSI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XIP_SSI {}
impl XIP_SSI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xip_ssi::RegisterBlock {
        0x1800_0000 as *const _
    }
}
impl Deref for XIP_SSI {
    type Target = xip_ssi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*XIP_SSI::ptr() }
    }
}
#[doc = "DW_apb_ssi has the following features:\\n * APB interface – Allows for easy integration into a DesignWare Synthesizable Components for AMBA 2 implementation.\\n * APB3 and APB4 protocol support.\\n * Scalable APB data bus width – Supports APB data bus widths of 8, 16, and 32 bits.\\n * Serial-master or serial-slave operation – Enables serial communication with serial-master or serial-slave peripheral devices.\\n * Programmable Dual/Quad/Octal SPI support in Master Mode.\\n * Dual Data Rate (DDR) and Read Data Strobe (RDS) Support - Enables the DW_apb_ssi master to perform operations with the device in DDR and RDS modes when working in Dual/Quad/Octal mode of operation.\\n * Data Mask Support - Enables the DW_apb_ssi to selectively update the bytes in the device. This feature is applicable only in enhanced SPI modes.\\n * eXecute-In-Place (XIP) support - Enables the DW_apb_ssi master to behave as a memory mapped I/O and fetches the data from the device based on the APB read request. This feature is applicable only in enhanced SPI modes.\\n * DMA Controller Interface – Enables the DW_apb_ssi to interface to a DMA controller over the bus using a handshaking interface for transfer requests.\\n * Independent masking of interrupts – Master collision, transmit FIFO overflow, transmit FIFO empty, receive FIFO full, receive FIFO underflow, and receive FIFO overflow interrupts can all be masked independently.\\n * Multi-master contention detection – Informs the processor of multiple serial-master accesses on the serial bus.\\n * Bypass of meta-stability flip-flops for synchronous clocks – When the APB clock (pclk) and the DW_apb_ssi serial clock (ssi_clk) are synchronous, meta-stable flip-flops are not used when transferring control signals across these clock domains.\\n * Programmable delay on the sample time of the received serial data bit (rxd); enables programmable control of routing delays resulting in higher serial data-bit rates.\\n * Programmable features:\\n - Serial interface operation – Choice of Motorola SPI, Texas Instruments Synchronous Serial Protocol or National Semiconductor Microwire.\\n - Clock bit-rate – Dynamic control of the serial bit rate of the data transfer; used in only serial-master mode of operation.\\n - Data Item size (4 to 32 bits) – Item size of each data transfer under the control of the programmer.\\n * Configured features:\\n - FIFO depth – 16 words deep. The FIFO width is fixed at 32 bits.\\n - 1 slave select output.\\n - Hardware slave-select – Dedicated hardware slave-select line.\\n - Combined interrupt line - one combined interrupt line from the DW_apb_ssi to the interrupt controller.\\n - Interrupt polarity – active high interrupt lines.\\n - Serial clock polarity – low serial-clock polarity directly after reset.\\n - Serial clock phase – capture on first edge of serial-clock directly after reset."]
pub mod xip_ssi;
#[doc = "SYSINFO"]
pub struct SYSINFO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSINFO {}
impl SYSINFO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysinfo::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for SYSINFO {
    type Target = sysinfo::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSINFO::ptr() }
    }
}
#[doc = "SYSINFO"]
pub mod sysinfo;
#[doc = "Register block for various chip control signals"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "Register block for various chip control signals"]
pub mod syscfg;
#[doc = "CLOCKS"]
pub struct CLOCKS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCKS {}
impl CLOCKS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clocks::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for CLOCKS {
    type Target = clocks::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLOCKS::ptr() }
    }
}
#[doc = "CLOCKS"]
pub mod clocks;
#[doc = "RESETS"]
pub struct RESETS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RESETS {}
impl RESETS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const resets::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for RESETS {
    type Target = resets::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RESETS::ptr() }
    }
}
#[doc = "RESETS"]
pub mod resets;
#[doc = "PSM"]
pub struct PSM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PSM {}
impl PSM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const psm::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for PSM {
    type Target = psm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PSM::ptr() }
    }
}
#[doc = "PSM"]
pub mod psm;
#[doc = "IO_BANK0"]
pub struct IO_BANK0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IO_BANK0 {}
impl IO_BANK0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const io_bank0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for IO_BANK0 {
    type Target = io_bank0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IO_BANK0::ptr() }
    }
}
#[doc = "IO_BANK0"]
pub mod io_bank0;
#[doc = "IO_QSPI"]
pub struct IO_QSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IO_QSPI {}
impl IO_QSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const io_qspi::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for IO_QSPI {
    type Target = io_qspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IO_QSPI::ptr() }
    }
}
#[doc = "IO_QSPI"]
pub mod io_qspi;
#[doc = "PADS_BANK0"]
pub struct PADS_BANK0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PADS_BANK0 {}
impl PADS_BANK0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pads_bank0::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for PADS_BANK0 {
    type Target = pads_bank0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PADS_BANK0::ptr() }
    }
}
#[doc = "PADS_BANK0"]
pub mod pads_bank0;
#[doc = "PADS_QSPI"]
pub struct PADS_QSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PADS_QSPI {}
impl PADS_QSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pads_qspi::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for PADS_QSPI {
    type Target = pads_qspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PADS_QSPI::ptr() }
    }
}
#[doc = "PADS_QSPI"]
pub mod pads_qspi;
#[doc = "Controls the crystal oscillator"]
pub struct XOSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XOSC {}
impl XOSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xosc::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for XOSC {
    type Target = xosc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*XOSC::ptr() }
    }
}
#[doc = "Controls the crystal oscillator"]
pub mod xosc;
#[doc = "PLL_SYS"]
pub struct PLL_SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PLL_SYS {}
impl PLL_SYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pll_sys::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for PLL_SYS {
    type Target = pll_sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PLL_SYS::ptr() }
    }
}
#[doc = "PLL_SYS"]
pub mod pll_sys;
#[doc = "PLL_USB"]
pub struct PLL_USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PLL_USB {}
impl PLL_USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pll_sys::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for PLL_USB {
    type Target = pll_sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PLL_USB::ptr() }
    }
}
#[doc = "Register block for busfabric control signals and performance counters"]
pub struct BUSCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BUSCTRL {}
impl BUSCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const busctrl::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for BUSCTRL {
    type Target = busctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BUSCTRL::ptr() }
    }
}
#[doc = "Register block for busfabric control signals and performance counters"]
pub mod busctrl;
#[doc = "UART0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART0"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "SPI0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "SPI0"]
pub mod spi0;
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "DW_apb_i2c address block"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "DW_apb_i2c address block"]
pub mod i2c0;
#[doc = "DW_apb_i2c address block"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Control and data interface to SAR ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4004_c000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Control and data interface to SAR ADC"]
pub mod adc;
#[doc = "Simple PWM"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        0x4005_0000 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "Simple PWM"]
pub mod pwm;
#[doc = "Controls time and alarms\\n time is a 64 bit value indicating the time in usec since power-on\\n timeh is the top 32 bits of time & timel is the bottom 32 bits\\n to change time write to timelw before timehw\\n to read time read from timelr before timehr\\n An alarm is set by setting alarm_enable and writing to the corresponding alarm register\\n When an alarm is pending, the corresponding alarm_running signal will be high\\n An alarm can be cancelled before it has finished by clearing the alarm_enable\\n When an alarm fires, the corresponding alarm_irq is set and alarm_running is cleared\\n To clear the interrupt write a 1 to the corresponding alarm_irq"]
pub struct TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER {}
impl TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer::RegisterBlock {
        0x4005_4000 as *const _
    }
}
impl Deref for TIMER {
    type Target = timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER::ptr() }
    }
}
#[doc = "Controls time and alarms\\n time is a 64 bit value indicating the time in usec since power-on\\n timeh is the top 32 bits of time & timel is the bottom 32 bits\\n to change time write to timelw before timehw\\n to read time read from timelr before timehr\\n An alarm is set by setting alarm_enable and writing to the corresponding alarm register\\n When an alarm is pending, the corresponding alarm_running signal will be high\\n An alarm can be cancelled before it has finished by clearing the alarm_enable\\n When an alarm fires, the corresponding alarm_irq is set and alarm_running is cleared\\n To clear the interrupt write a 1 to the corresponding alarm_irq"]
pub mod timer;
#[doc = "WATCHDOG"]
pub struct WATCHDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG {}
impl WATCHDOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog::RegisterBlock {
        0x4005_8000 as *const _
    }
}
impl Deref for WATCHDOG {
    type Target = watchdog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG::ptr() }
    }
}
#[doc = "WATCHDOG"]
pub mod watchdog;
#[doc = "Register block to control RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Register block to control RTC"]
pub mod rtc;
#[doc = "ROSC"]
pub struct ROSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROSC {}
impl ROSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rosc::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for ROSC {
    type Target = rosc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ROSC::ptr() }
    }
}
#[doc = "ROSC"]
pub mod rosc;
#[doc = "control and status for on-chip voltage regulator and chip level reset subsystem"]
pub struct VREG_AND_CHIP_RESET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREG_AND_CHIP_RESET {}
impl VREG_AND_CHIP_RESET {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vreg_and_chip_reset::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for VREG_AND_CHIP_RESET {
    type Target = vreg_and_chip_reset::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREG_AND_CHIP_RESET::ptr() }
    }
}
#[doc = "control and status for on-chip voltage regulator and chip level reset subsystem"]
pub mod vreg_and_chip_reset;
#[doc = "Testbench manager. Allows the programmer to know what platform their software is running on."]
pub struct TBMAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TBMAN {}
impl TBMAN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tbman::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for TBMAN {
    type Target = tbman::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TBMAN::ptr() }
    }
}
#[doc = "Testbench manager. Allows the programmer to know what platform their software is running on."]
pub mod tbman;
#[doc = "DMA with separate read and write masters"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA with separate read and write masters"]
pub mod dma;
#[doc = "USB FS/LS controller device registers"]
pub struct USBCTRL_REGS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBCTRL_REGS {}
impl USBCTRL_REGS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbctrl_regs::RegisterBlock {
        0x5011_0000 as *const _
    }
}
impl Deref for USBCTRL_REGS {
    type Target = usbctrl_regs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBCTRL_REGS::ptr() }
    }
}
#[doc = "USB FS/LS controller device registers"]
pub mod usbctrl_regs;
#[doc = "Programmable IO block"]
pub struct PIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIO0 {}
impl PIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pio0::RegisterBlock {
        0x5020_0000 as *const _
    }
}
impl Deref for PIO0 {
    type Target = pio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIO0::ptr() }
    }
}
#[doc = "Programmable IO block"]
pub mod pio0;
#[doc = "Programmable IO block"]
pub struct PIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIO1 {}
impl PIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pio0::RegisterBlock {
        0x5030_0000 as *const _
    }
}
impl Deref for PIO1 {
    type Target = pio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIO1::ptr() }
    }
}
#[doc = "Single-cycle IO block\\n Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
pub struct SIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIO {}
impl SIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sio::RegisterBlock {
        0xd000_0000 as *const _
    }
}
impl Deref for SIO {
    type Target = sio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SIO::ptr() }
    }
}
#[doc = "Single-cycle IO block\\n Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
pub mod sio;
#[doc = "PPB"]
pub struct PPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPB {}
impl PPB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ppb::RegisterBlock {
        0xe000_0000 as *const _
    }
}
impl Deref for PPB {
    type Target = ppb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PPB::ptr() }
    }
}
#[doc = "PPB"]
pub mod ppb;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "XIP_CTRL"]
    pub XIP_CTRL: XIP_CTRL,
    #[doc = "XIP_SSI"]
    pub XIP_SSI: XIP_SSI,
    #[doc = "SYSINFO"]
    pub SYSINFO: SYSINFO,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "CLOCKS"]
    pub CLOCKS: CLOCKS,
    #[doc = "RESETS"]
    pub RESETS: RESETS,
    #[doc = "PSM"]
    pub PSM: PSM,
    #[doc = "IO_BANK0"]
    pub IO_BANK0: IO_BANK0,
    #[doc = "IO_QSPI"]
    pub IO_QSPI: IO_QSPI,
    #[doc = "PADS_BANK0"]
    pub PADS_BANK0: PADS_BANK0,
    #[doc = "PADS_QSPI"]
    pub PADS_QSPI: PADS_QSPI,
    #[doc = "XOSC"]
    pub XOSC: XOSC,
    #[doc = "PLL_SYS"]
    pub PLL_SYS: PLL_SYS,
    #[doc = "PLL_USB"]
    pub PLL_USB: PLL_USB,
    #[doc = "BUSCTRL"]
    pub BUSCTRL: BUSCTRL,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "TIMER"]
    pub TIMER: TIMER,
    #[doc = "WATCHDOG"]
    pub WATCHDOG: WATCHDOG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "ROSC"]
    pub ROSC: ROSC,
    #[doc = "VREG_AND_CHIP_RESET"]
    pub VREG_AND_CHIP_RESET: VREG_AND_CHIP_RESET,
    #[doc = "TBMAN"]
    pub TBMAN: TBMAN,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "USBCTRL_REGS"]
    pub USBCTRL_REGS: USBCTRL_REGS,
    #[doc = "PIO0"]
    pub PIO0: PIO0,
    #[doc = "PIO1"]
    pub PIO1: PIO1,
    #[doc = "SIO"]
    pub SIO: SIO,
    #[doc = "PPB"]
    pub PPB: PPB,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            XIP_CTRL: XIP_CTRL {
                _marker: PhantomData,
            },
            XIP_SSI: XIP_SSI {
                _marker: PhantomData,
            },
            SYSINFO: SYSINFO {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            CLOCKS: CLOCKS {
                _marker: PhantomData,
            },
            RESETS: RESETS {
                _marker: PhantomData,
            },
            PSM: PSM {
                _marker: PhantomData,
            },
            IO_BANK0: IO_BANK0 {
                _marker: PhantomData,
            },
            IO_QSPI: IO_QSPI {
                _marker: PhantomData,
            },
            PADS_BANK0: PADS_BANK0 {
                _marker: PhantomData,
            },
            PADS_QSPI: PADS_QSPI {
                _marker: PhantomData,
            },
            XOSC: XOSC {
                _marker: PhantomData,
            },
            PLL_SYS: PLL_SYS {
                _marker: PhantomData,
            },
            PLL_USB: PLL_USB {
                _marker: PhantomData,
            },
            BUSCTRL: BUSCTRL {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            TIMER: TIMER {
                _marker: PhantomData,
            },
            WATCHDOG: WATCHDOG {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            ROSC: ROSC {
                _marker: PhantomData,
            },
            VREG_AND_CHIP_RESET: VREG_AND_CHIP_RESET {
                _marker: PhantomData,
            },
            TBMAN: TBMAN {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            USBCTRL_REGS: USBCTRL_REGS {
                _marker: PhantomData,
            },
            PIO0: PIO0 {
                _marker: PhantomData,
            },
            PIO1: PIO1 {
                _marker: PhantomData,
            },
            SIO: SIO {
                _marker: PhantomData,
            },
            PPB: PPB {
                _marker: PhantomData,
            },
        }
    }
}
