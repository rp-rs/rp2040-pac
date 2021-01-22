#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    pub voltage_select: VOLTAGE_SELECT,
    #[doc = "0x04 - Pad control register"]
    pub gpio_qspi_sclk: GPIO_QSPI_SCLK,
    #[doc = "0x08 - Pad control register"]
    pub gpio_qspi_sd0: GPIO_QSPI_SD0,
    #[doc = "0x0c - Pad control register"]
    pub gpio_qspi_sd1: GPIO_QSPI_SD1,
    #[doc = "0x10 - Pad control register"]
    pub gpio_qspi_sd2: GPIO_QSPI_SD2,
    #[doc = "0x14 - Pad control register"]
    pub gpio_qspi_sd3: GPIO_QSPI_SD3,
    #[doc = "0x18 - Pad control register"]
    pub gpio_qspi_ss: GPIO_QSPI_SS,
}
#[doc = "Voltage select. Per bank control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voltage_select](voltage_select) module"]
pub type VOLTAGE_SELECT = crate::Reg<u32, _VOLTAGE_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VOLTAGE_SELECT;
#[doc = "`read()` method returns [voltage_select::R](voltage_select::R) reader structure"]
impl crate::Readable for VOLTAGE_SELECT {}
#[doc = "`write(|w| ..)` method takes [voltage_select::W](voltage_select::W) writer structure"]
impl crate::Writable for VOLTAGE_SELECT {}
#[doc = "Voltage select. Per bank control"]
pub mod voltage_select;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sclk](gpio_qspi_sclk) module"]
pub type GPIO_QSPI_SCLK = crate::Reg<u32, _GPIO_QSPI_SCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SCLK;
#[doc = "`read()` method returns [gpio_qspi_sclk::R](gpio_qspi_sclk::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SCLK {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sclk::W](gpio_qspi_sclk::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SCLK {}
#[doc = "Pad control register"]
pub mod gpio_qspi_sclk;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd0](gpio_qspi_sd0) module"]
pub type GPIO_QSPI_SD0 = crate::Reg<u32, _GPIO_QSPI_SD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD0;
#[doc = "`read()` method returns [gpio_qspi_sd0::R](gpio_qspi_sd0::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD0 {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd0::W](gpio_qspi_sd0::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD0 {}
#[doc = "Pad control register"]
pub mod gpio_qspi_sd0;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd1](gpio_qspi_sd1) module"]
pub type GPIO_QSPI_SD1 = crate::Reg<u32, _GPIO_QSPI_SD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD1;
#[doc = "`read()` method returns [gpio_qspi_sd1::R](gpio_qspi_sd1::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD1 {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd1::W](gpio_qspi_sd1::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD1 {}
#[doc = "Pad control register"]
pub mod gpio_qspi_sd1;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd2](gpio_qspi_sd2) module"]
pub type GPIO_QSPI_SD2 = crate::Reg<u32, _GPIO_QSPI_SD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD2;
#[doc = "`read()` method returns [gpio_qspi_sd2::R](gpio_qspi_sd2::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD2 {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd2::W](gpio_qspi_sd2::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD2 {}
#[doc = "Pad control register"]
pub mod gpio_qspi_sd2;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd3](gpio_qspi_sd3) module"]
pub type GPIO_QSPI_SD3 = crate::Reg<u32, _GPIO_QSPI_SD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD3;
#[doc = "`read()` method returns [gpio_qspi_sd3::R](gpio_qspi_sd3::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD3 {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd3::W](gpio_qspi_sd3::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD3 {}
#[doc = "Pad control register"]
pub mod gpio_qspi_sd3;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_ss](gpio_qspi_ss) module"]
pub type GPIO_QSPI_SS = crate::Reg<u32, _GPIO_QSPI_SS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SS;
#[doc = "`read()` method returns [gpio_qspi_ss::R](gpio_qspi_ss::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SS {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_ss::W](gpio_qspi_ss::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SS {}
#[doc = "Pad control register"]
pub mod gpio_qspi_ss;
