#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control and Status"]
    pub cs: CS,
    #[doc = "0x04 - Result of most recent ADC conversion"]
    pub result: RESULT,
    #[doc = "0x08 - FIFO control and status"]
    pub fcs: FCS,
    #[doc = "0x0c - Conversion result FIFO"]
    pub fifo: FIFO,
    #[doc = "0x10 - Clock divider. If non-zero, CS_START_MANY will start conversions\\n at regular intervals rather than back-to-back.\\n The divider is reset when either of these fields are written.\\n Total period is 1 + INT + FRAC / 256"]
    pub div: DIV,
    #[doc = "0x14 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0x18 - Interrupt Enable"]
    pub inte: INTE,
    #[doc = "0x1c - Interrupt Force"]
    pub intf: INTF,
    #[doc = "0x20 - Interrupt status after masking & forcing"]
    pub ints: INTS,
}
#[doc = "ADC Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](cs) module"]
pub type CS = crate::Reg<u32, _CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS;
#[doc = "`read()` method returns [cs::R](cs::R) reader structure"]
impl crate::Readable for CS {}
#[doc = "`write(|w| ..)` method takes [cs::W](cs::W) writer structure"]
impl crate::Writable for CS {}
#[doc = "ADC Control and Status"]
pub mod cs;
#[doc = "Result of most recent ADC conversion\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Result of most recent ADC conversion"]
pub mod result;
#[doc = "FIFO control and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcs](fcs) module"]
pub type FCS = crate::Reg<u32, _FCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCS;
#[doc = "`read()` method returns [fcs::R](fcs::R) reader structure"]
impl crate::Readable for FCS {}
#[doc = "`write(|w| ..)` method takes [fcs::W](fcs::W) writer structure"]
impl crate::Writable for FCS {}
#[doc = "FIFO control and status"]
pub mod fcs;
#[doc = "Conversion result FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "Conversion result FIFO"]
pub mod fifo;
#[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions\\n at regular intervals rather than back-to-back.\\n The divider is reset when either of these fields are written.\\n Total period is 1 + INT + FRAC / 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](div) module"]
pub type DIV = crate::Reg<u32, _DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV;
#[doc = "`read()` method returns [div::R](div::R) reader structure"]
impl crate::Readable for DIV {}
#[doc = "`write(|w| ..)` method takes [div::W](div::W) writer structure"]
impl crate::Writable for DIV {}
#[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions\\n at regular intervals rather than back-to-back.\\n The divider is reset when either of these fields are written.\\n Total period is 1 + INT + FRAC / 256"]
pub mod div;
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
