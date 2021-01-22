#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
    pub clkdiv_m1: CLKDIV_M1,
    #[doc = "0x04 - RTC setup register 0"]
    pub setup_0: SETUP_0,
    #[doc = "0x08 - RTC setup register 1"]
    pub setup_1: SETUP_1,
    #[doc = "0x0c - RTC Control and status"]
    pub ctrl: CTRL,
    #[doc = "0x10 - Interrupt setup register 0"]
    pub irq_setup_0: IRQ_SETUP_0,
    #[doc = "0x14 - Interrupt setup register 1"]
    pub irq_setup_1: IRQ_SETUP_1,
    #[doc = "0x18 - RTC register 1."]
    pub rtc_1: RTC_1,
    #[doc = "0x1c - RTC register 0\\n Read this before RTC 1!"]
    pub rtc_0: RTC_0,
    #[doc = "0x20 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0x24 - Interrupt Enable"]
    pub inte: INTE,
    #[doc = "0x28 - Interrupt Force"]
    pub intf: INTF,
    #[doc = "0x2c - Interrupt status after masking & forcing"]
    pub ints: INTS,
}
#[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv_m1](clkdiv_m1) module"]
pub type CLKDIV_M1 = crate::Reg<u32, _CLKDIV_M1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV_M1;
#[doc = "`read()` method returns [clkdiv_m1::R](clkdiv_m1::R) reader structure"]
impl crate::Readable for CLKDIV_M1 {}
#[doc = "`write(|w| ..)` method takes [clkdiv_m1::W](clkdiv_m1::W) writer structure"]
impl crate::Writable for CLKDIV_M1 {}
#[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
pub mod clkdiv_m1;
#[doc = "RTC setup register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup_0](setup_0) module"]
pub type SETUP_0 = crate::Reg<u32, _SETUP_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_0;
#[doc = "`read()` method returns [setup_0::R](setup_0::R) reader structure"]
impl crate::Readable for SETUP_0 {}
#[doc = "`write(|w| ..)` method takes [setup_0::W](setup_0::W) writer structure"]
impl crate::Writable for SETUP_0 {}
#[doc = "RTC setup register 0"]
pub mod setup_0;
#[doc = "RTC setup register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup_1](setup_1) module"]
pub type SETUP_1 = crate::Reg<u32, _SETUP_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_1;
#[doc = "`read()` method returns [setup_1::R](setup_1::R) reader structure"]
impl crate::Readable for SETUP_1 {}
#[doc = "`write(|w| ..)` method takes [setup_1::W](setup_1::W) writer structure"]
impl crate::Writable for SETUP_1 {}
#[doc = "RTC setup register 1"]
pub mod setup_1;
#[doc = "RTC Control and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "RTC Control and status"]
pub mod ctrl;
#[doc = "Interrupt setup register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_setup_0](irq_setup_0) module"]
pub type IRQ_SETUP_0 = crate::Reg<u32, _IRQ_SETUP_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_SETUP_0;
#[doc = "`read()` method returns [irq_setup_0::R](irq_setup_0::R) reader structure"]
impl crate::Readable for IRQ_SETUP_0 {}
#[doc = "`write(|w| ..)` method takes [irq_setup_0::W](irq_setup_0::W) writer structure"]
impl crate::Writable for IRQ_SETUP_0 {}
#[doc = "Interrupt setup register 0"]
pub mod irq_setup_0;
#[doc = "Interrupt setup register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_setup_1](irq_setup_1) module"]
pub type IRQ_SETUP_1 = crate::Reg<u32, _IRQ_SETUP_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_SETUP_1;
#[doc = "`read()` method returns [irq_setup_1::R](irq_setup_1::R) reader structure"]
impl crate::Readable for IRQ_SETUP_1 {}
#[doc = "`write(|w| ..)` method takes [irq_setup_1::W](irq_setup_1::W) writer structure"]
impl crate::Writable for IRQ_SETUP_1 {}
#[doc = "Interrupt setup register 1"]
pub mod irq_setup_1;
#[doc = "RTC register 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_1](rtc_1) module"]
pub type RTC_1 = crate::Reg<u32, _RTC_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_1;
#[doc = "`read()` method returns [rtc_1::R](rtc_1::R) reader structure"]
impl crate::Readable for RTC_1 {}
#[doc = "RTC register 1."]
pub mod rtc_1;
#[doc = "RTC register 0\\n Read this before RTC 1!\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_0](rtc_0) module"]
pub type RTC_0 = crate::Reg<u32, _RTC_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_0;
#[doc = "`read()` method returns [rtc_0::R](rtc_0::R) reader structure"]
impl crate::Readable for RTC_0 {}
#[doc = "RTC register 0\\n Read this before RTC 1!"]
pub mod rtc_0;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inte](inte) module"]
pub type INTE = crate::Reg<u32, _INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTE;
#[doc = "`read()` method returns [inte::R](inte::R) reader structure"]
impl crate::Readable for INTE {}
#[doc = "`write(|w| ..)` method takes [inte::W](inte::W) writer structure"]
impl crate::Writable for INTE {}
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "Interrupt Force\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u32, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "`write(|w| ..)` method takes [intf::W](intf::W) writer structure"]
impl crate::Writable for INTF {}
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "Interrupt status after masking & forcing\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ints](ints) module"]
pub type INTS = crate::Reg<u32, _INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTS;
#[doc = "`read()` method returns [ints::R](ints::R) reader structure"]
impl crate::Readable for INTS {}
#[doc = "Interrupt status after masking & forcing"]
pub mod ints;
