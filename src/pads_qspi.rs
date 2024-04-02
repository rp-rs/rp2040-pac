#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    voltage_select: VOLTAGE_SELECT,
    gpio_qspi_sclk: GPIO_QSPI_SCLK,
    gpio_qspi_sd0: GPIO_QSPI_SD0,
    gpio_qspi_sd1: GPIO_QSPI_SD1,
    gpio_qspi_sd2: GPIO_QSPI_SD2,
    gpio_qspi_sd3: GPIO_QSPI_SD3,
    gpio_qspi_ss: GPIO_QSPI_SS,
}
impl RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    #[inline(always)]
    pub const fn voltage_select(&self) -> &VOLTAGE_SELECT {
        &self.voltage_select
    }
    #[doc = "0x04 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sclk(&self) -> &GPIO_QSPI_SCLK {
        &self.gpio_qspi_sclk
    }
    #[doc = "0x08 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sd0(&self) -> &GPIO_QSPI_SD0 {
        &self.gpio_qspi_sd0
    }
    #[doc = "0x0c - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sd1(&self) -> &GPIO_QSPI_SD1 {
        &self.gpio_qspi_sd1
    }
    #[doc = "0x10 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sd2(&self) -> &GPIO_QSPI_SD2 {
        &self.gpio_qspi_sd2
    }
    #[doc = "0x14 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sd3(&self) -> &GPIO_QSPI_SD3 {
        &self.gpio_qspi_sd3
    }
    #[doc = "0x18 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_ss(&self) -> &GPIO_QSPI_SS {
        &self.gpio_qspi_ss
    }
}
#[doc = "VOLTAGE_SELECT (rw) register accessor: Voltage select. Per bank control  

You can [`read`](crate::Reg::read) this register and get [`voltage_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`voltage_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@voltage_select`]
module"]
pub type VOLTAGE_SELECT = crate::Reg<voltage_select::VOLTAGE_SELECT_SPEC>;
#[doc = "Voltage select. Per bank control"]
pub mod voltage_select;
#[doc = "GPIO_QSPI_SCLK (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`gpio_qspi_sclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_qspi_sclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_qspi_sclk`]
module"]
pub type GPIO_QSPI_SCLK = crate::Reg<gpio_qspi_sclk::GPIO_QSPI_SCLK_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sclk;
#[doc = "GPIO_QSPI_SD0 (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`gpio_qspi_sd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_qspi_sd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_qspi_sd0`]
module"]
pub type GPIO_QSPI_SD0 = crate::Reg<gpio_qspi_sd0::GPIO_QSPI_SD0_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sd0;
#[doc = "GPIO_QSPI_SD1 (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`gpio_qspi_sd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_qspi_sd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_qspi_sd1`]
module"]
pub type GPIO_QSPI_SD1 = crate::Reg<gpio_qspi_sd1::GPIO_QSPI_SD1_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sd1;
#[doc = "GPIO_QSPI_SD2 (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`gpio_qspi_sd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_qspi_sd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_qspi_sd2`]
module"]
pub type GPIO_QSPI_SD2 = crate::Reg<gpio_qspi_sd2::GPIO_QSPI_SD2_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sd2;
#[doc = "GPIO_QSPI_SD3 (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`gpio_qspi_sd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_qspi_sd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_qspi_sd3`]
module"]
pub type GPIO_QSPI_SD3 = crate::Reg<gpio_qspi_sd3::GPIO_QSPI_SD3_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sd3;
#[doc = "GPIO_QSPI_SS (rw) register accessor: Pad control register  

You can [`read`](crate::Reg::read) this register and get [`gpio_qspi_ss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_qspi_ss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_qspi_ss`]
module"]
pub type GPIO_QSPI_SS = crate::Reg<gpio_qspi_ss::GPIO_QSPI_SS_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_ss;
