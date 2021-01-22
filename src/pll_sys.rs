#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and Status\\n GENERAL CONSTRAINTS:\\n Reference clock frequency min=5MHz, max=800MHz\\n Feedback divider min=16, max=320\\n VCO frequency min=400MHz, max=1600MHz"]
    pub cs: CS,
    #[doc = "0x04 - Controls the PLL power modes."]
    pub pwr: PWR,
    #[doc = "0x08 - Feedback divisor\\n (note: this PLL does not support fractional division)"]
    pub fbdiv_int: FBDIV_INT,
    #[doc = "0x0c - Controls the PLL post dividers for the primary output\\n (note: this PLL does not have a secondary output)\\n the primary output is driven from VCO divided by postdiv1*postdiv2"]
    pub prim: PRIM,
}
#[doc = "Control and Status\\n GENERAL CONSTRAINTS:\\n Reference clock frequency min=5MHz, max=800MHz\\n Feedback divider min=16, max=320\\n VCO frequency min=400MHz, max=1600MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](cs) module"]
pub type CS = crate::Reg<u32, _CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS;
#[doc = "`read()` method returns [cs::R](cs::R) reader structure"]
impl crate::Readable for CS {}
#[doc = "`write(|w| ..)` method takes [cs::W](cs::W) writer structure"]
impl crate::Writable for CS {}
#[doc = "Control and Status\\n GENERAL CONSTRAINTS:\\n Reference clock frequency min=5MHz, max=800MHz\\n Feedback divider min=16, max=320\\n VCO frequency min=400MHz, max=1600MHz"]
pub mod cs;
#[doc = "Controls the PLL power modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr](pwr) module"]
pub type PWR = crate::Reg<u32, _PWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR;
#[doc = "`read()` method returns [pwr::R](pwr::R) reader structure"]
impl crate::Readable for PWR {}
#[doc = "`write(|w| ..)` method takes [pwr::W](pwr::W) writer structure"]
impl crate::Writable for PWR {}
#[doc = "Controls the PLL power modes."]
pub mod pwr;
#[doc = "Feedback divisor\\n (note: this PLL does not support fractional division)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbdiv_int](fbdiv_int) module"]
pub type FBDIV_INT = crate::Reg<u32, _FBDIV_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBDIV_INT;
#[doc = "`read()` method returns [fbdiv_int::R](fbdiv_int::R) reader structure"]
impl crate::Readable for FBDIV_INT {}
#[doc = "`write(|w| ..)` method takes [fbdiv_int::W](fbdiv_int::W) writer structure"]
impl crate::Writable for FBDIV_INT {}
#[doc = "Feedback divisor\\n (note: this PLL does not support fractional division)"]
pub mod fbdiv_int;
#[doc = "Controls the PLL post dividers for the primary output\\n (note: this PLL does not have a secondary output)\\n the primary output is driven from VCO divided by postdiv1*postdiv2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prim](prim) module"]
pub type PRIM = crate::Reg<u32, _PRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIM;
#[doc = "`read()` method returns [prim::R](prim::R) reader structure"]
impl crate::Readable for PRIM {}
#[doc = "`write(|w| ..)` method takes [prim::W](prim::W) writer structure"]
impl crate::Writable for PRIM {}
#[doc = "Controls the PLL post dividers for the primary output\\n (note: this PLL does not have a secondary output)\\n the primary output is driven from VCO divided by postdiv1*postdiv2"]
pub mod prim;
