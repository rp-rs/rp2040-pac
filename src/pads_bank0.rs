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
#[doc = "VOLTAGE_SELECT (rw) register accessor: Voltage select. Per bank control  

You can [`read`](crate::generic::Reg::read) this register and get [`voltage_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`voltage_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@voltage_select`]
module"]
pub type VOLTAGE_SELECT = crate::Reg<voltage_select::VOLTAGE_SELECT_SPEC>;
#[doc = "Voltage select. Per bank control"]
pub mod voltage_select;
#[doc = "GPIO (rw) register accessor: Pad control register  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio`]
module"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "Pad control register"]
pub mod gpio;
#[doc = "SWCLK (rw) register accessor: Pad control register  

You can [`read`](crate::generic::Reg::read) this register and get [`swclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@swclk`]
module"]
pub type SWCLK = crate::Reg<swclk::SWCLK_SPEC>;
#[doc = "Pad control register"]
pub mod swclk;
#[doc = "SWD (rw) register accessor: Pad control register  

You can [`read`](crate::generic::Reg::read) this register and get [`swd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@swd`]
module"]
pub type SWD = crate::Reg<swd::SWD_SPEC>;
#[doc = "Pad control register"]
pub mod swd;
