#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    pub voltage_select: crate::Reg<voltage_select::VOLTAGE_SELECT_SPEC>,
    #[doc = "0x04 - Pad control register"]
    pub gpio_qspi_sclk: crate::Reg<gpio_qspi_sclk::GPIO_QSPI_SCLK_SPEC>,
    #[doc = "0x08 - Pad control register"]
    pub gpio_qspi_sd0: crate::Reg<gpio_qspi_sd0::GPIO_QSPI_SD0_SPEC>,
    #[doc = "0x0c - Pad control register"]
    pub gpio_qspi_sd1: crate::Reg<gpio_qspi_sd1::GPIO_QSPI_SD1_SPEC>,
    #[doc = "0x10 - Pad control register"]
    pub gpio_qspi_sd2: crate::Reg<gpio_qspi_sd2::GPIO_QSPI_SD2_SPEC>,
    #[doc = "0x14 - Pad control register"]
    pub gpio_qspi_sd3: crate::Reg<gpio_qspi_sd3::GPIO_QSPI_SD3_SPEC>,
    #[doc = "0x18 - Pad control register"]
    pub gpio_qspi_ss: crate::Reg<gpio_qspi_ss::GPIO_QSPI_SS_SPEC>,
}
#[doc = "VOLTAGE_SELECT register accessor: an alias for `Reg<VOLTAGE_SELECT_SPEC>`"]
pub type VOLTAGE_SELECT = crate::Reg<voltage_select::VOLTAGE_SELECT_SPEC>;
#[doc = "Voltage select. Per bank control"]
pub mod voltage_select;
#[doc = "GPIO_QSPI_SCLK register accessor: an alias for `Reg<GPIO_QSPI_SCLK_SPEC>`"]
pub type GPIO_QSPI_SCLK = crate::Reg<gpio_qspi_sclk::GPIO_QSPI_SCLK_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sclk;
#[doc = "GPIO_QSPI_SD0 register accessor: an alias for `Reg<GPIO_QSPI_SD0_SPEC>`"]
pub type GPIO_QSPI_SD0 = crate::Reg<gpio_qspi_sd0::GPIO_QSPI_SD0_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sd0;
#[doc = "GPIO_QSPI_SD1 register accessor: an alias for `Reg<GPIO_QSPI_SD1_SPEC>`"]
pub type GPIO_QSPI_SD1 = crate::Reg<gpio_qspi_sd1::GPIO_QSPI_SD1_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sd1;
#[doc = "GPIO_QSPI_SD2 register accessor: an alias for `Reg<GPIO_QSPI_SD2_SPEC>`"]
pub type GPIO_QSPI_SD2 = crate::Reg<gpio_qspi_sd2::GPIO_QSPI_SD2_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sd2;
#[doc = "GPIO_QSPI_SD3 register accessor: an alias for `Reg<GPIO_QSPI_SD3_SPEC>`"]
pub type GPIO_QSPI_SD3 = crate::Reg<gpio_qspi_sd3::GPIO_QSPI_SD3_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_sd3;
#[doc = "GPIO_QSPI_SS register accessor: an alias for `Reg<GPIO_QSPI_SS_SPEC>`"]
pub type GPIO_QSPI_SS = crate::Reg<gpio_qspi_ss::GPIO_QSPI_SS_SPEC>;
#[doc = "Pad control register"]
pub mod gpio_qspi_ss;
