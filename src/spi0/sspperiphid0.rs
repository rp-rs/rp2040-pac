#[doc = "Register `SSPPERIPHID0` reader"]
pub struct R(crate::R<SSPPERIPHID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPPERIPHID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPPERIPHID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPPERIPHID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNUMBER0` reader - These bits read back as 0x22"]
pub type PARTNUMBER0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x22"]
    #[inline(always)]
    pub fn partnumber0(&self) -> PARTNUMBER0_R {
        PARTNUMBER0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspperiphid0](index.html) module"]
pub struct SSPPERIPHID0_SPEC;
impl crate::RegisterSpec for SSPPERIPHID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspperiphid0::R](R) reader structure"]
impl crate::Readable for SSPPERIPHID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPPERIPHID0 to value 0x22"]
impl crate::Resettable for SSPPERIPHID0_SPEC {
    const RESET_VALUE: Self::Ux = 0x22;
}
