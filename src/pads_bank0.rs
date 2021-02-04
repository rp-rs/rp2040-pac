#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    pub voltage_select: VOLTAGE_SELECT,
    #[doc = "0x04 - Pad control register"]
    pub gpio: [GPIO; 30],
    #[doc = "0x7c - Pad control register"]
    pub swclk: SWCLK,
    #[doc = "0x80 - Pad control register"]
    pub swd: SWD,
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
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio](gpio) module"]
pub type GPIO = crate::Reg<u32, _GPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO;
#[doc = "`read()` method returns [gpio::R](gpio::R) reader structure"]
impl crate::Readable for GPIO {}
#[doc = "`write(|w| ..)` method takes [gpio::W](gpio::W) writer structure"]
impl crate::Writable for GPIO {}
#[doc = "Pad control register"]
pub mod gpio;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swclk](swclk) module"]
pub type SWCLK = crate::Reg<u32, _SWCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWCLK;
#[doc = "`read()` method returns [swclk::R](swclk::R) reader structure"]
impl crate::Readable for SWCLK {}
#[doc = "`write(|w| ..)` method takes [swclk::W](swclk::W) writer structure"]
impl crate::Writable for SWCLK {}
#[doc = "Pad control register"]
pub mod swclk;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd](swd) module"]
pub type SWD = crate::Reg<u32, _SWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWD;
#[doc = "`read()` method returns [swd::R](swd::R) reader structure"]
impl crate::Readable for SWD {}
#[doc = "`write(|w| ..)` method takes [swd::W](swd::W) writer structure"]
impl crate::Writable for SWD {}
#[doc = "Pad control register"]
pub mod swd;
