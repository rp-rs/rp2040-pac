#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    pub voltage_select: VOLTAGE_SELECT,
    #[doc = "0x04..0x7c - Pad control register"]
    pub gpio: [GPIO; 30],
    #[doc = "0x7c - Pad control register"]
    pub swclk: SWCLK,
    #[doc = "0x80 - Pad control register"]
    pub swd: SWD,
}
#[doc = "VOLTAGE_SELECT (rw) register accessor: an alias for `Reg<VOLTAGE_SELECT_SPEC>`"]
pub type VOLTAGE_SELECT = crate::Reg<voltage_select::VOLTAGE_SELECT_SPEC>;
#[doc = "Voltage select. Per bank control"]
pub mod voltage_select;
#[doc = "GPIO (rw) register accessor: an alias for `Reg<GPIO_SPEC>`"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "Pad control register"]
pub mod gpio;
#[doc = "SWCLK (rw) register accessor: an alias for `Reg<SWCLK_SPEC>`"]
pub type SWCLK = crate::Reg<swclk::SWCLK_SPEC>;
#[doc = "Pad control register"]
pub mod swclk;
#[doc = "SWD (rw) register accessor: an alias for `Reg<SWD_SPEC>`"]
pub type SWD = crate::Reg<swd::SWD_SPEC>;
#[doc = "Pad control register"]
pub mod swd;
