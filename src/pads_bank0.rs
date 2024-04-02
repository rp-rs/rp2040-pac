#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    voltage_select: VOLTAGE_SELECT,
    gpio: [GPIO; 30],
    swclk: SWCLK,
    swd: SWD,
}
impl RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    #[inline(always)]
    pub const fn voltage_select(&self) -> &VOLTAGE_SELECT {
        &self.voltage_select
    }
    #[doc = "0x04..0x7c - Pad control register"]
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x7c - Pad control register"]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    #[doc = "0x7c - Pad control register"]
    #[inline(always)]
    pub const fn swclk(&self) -> &SWCLK {
        &self.swclk
    }
    #[doc = "0x80 - Pad control register"]
    #[inline(always)]
    pub const fn swd(&self) -> &SWD {
        &self.swd
    }
}
#[doc = "VOLTAGE_SELECT (rw) register accessor: Voltage select. Per bank control  

You can [`read`](crate::Reg::read) this register and get [`voltage_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`voltage_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@voltage_select`]
module"]
pub type VOLTAGE_SELECT = crate::Reg<voltage_select::VOLTAGE_SELECT_SPEC>;
#[doc = "Voltage select. Per bank control"]
pub mod voltage_select;
#[doc = "GPIO (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`gpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio`]
module"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "Pad control register"]
pub mod gpio;
#[doc = "SWCLK (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`swclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@swclk`]
module"]
pub type SWCLK = crate::Reg<swclk::SWCLK_SPEC>;
#[doc = "Pad control register"]
pub mod swclk;
#[doc = "SWD (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`swd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@swd`]
module"]
pub type SWD = crate::Reg<swd::SWD_SPEC>;
#[doc = "Pad control register"]
pub mod swd;
