#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage regulator control and status"]
    pub vreg: VREG,
    #[doc = "0x04 - brown-out detection control"]
    pub bod: BOD,
    #[doc = "0x08 - Chip reset control and status"]
    pub chip_reset: CHIP_RESET,
}
#[doc = "Voltage regulator control and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreg](vreg) module"]
pub type VREG = crate::Reg<u32, _VREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREG;
#[doc = "`read()` method returns [vreg::R](vreg::R) reader structure"]
impl crate::Readable for VREG {}
#[doc = "`write(|w| ..)` method takes [vreg::W](vreg::W) writer structure"]
impl crate::Writable for VREG {}
#[doc = "Voltage regulator control and status"]
pub mod vreg;
#[doc = "brown-out detection control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod](bod) module"]
pub type BOD = crate::Reg<u32, _BOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD;
#[doc = "`read()` method returns [bod::R](bod::R) reader structure"]
impl crate::Readable for BOD {}
#[doc = "`write(|w| ..)` method takes [bod::W](bod::W) writer structure"]
impl crate::Writable for BOD {}
#[doc = "brown-out detection control"]
pub mod bod;
#[doc = "Chip reset control and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_reset](chip_reset) module"]
pub type CHIP_RESET = crate::Reg<u32, _CHIP_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIP_RESET;
#[doc = "`read()` method returns [chip_reset::R](chip_reset::R) reader structure"]
impl crate::Readable for CHIP_RESET {}
#[doc = "`write(|w| ..)` method takes [chip_reset::W](chip_reset::W) writer structure"]
impl crate::Writable for CHIP_RESET {}
#[doc = "Chip reset control and status"]
pub mod chip_reset;
