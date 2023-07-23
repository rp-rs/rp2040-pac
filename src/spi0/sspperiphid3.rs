#[doc = "Register `SSPPERIPHID3` reader"]
pub struct R(crate::R<SSPPERIPHID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPPERIPHID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPPERIPHID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPPERIPHID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONFIGURATION` reader - These bits read back as 0x00"]
pub type CONFIGURATION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x00"]
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspperiphid3](index.html) module"]
pub struct SSPPERIPHID3_SPEC;
impl crate::RegisterSpec for SSPPERIPHID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspperiphid3::R](R) reader structure"]
impl crate::Readable for SSPPERIPHID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPPERIPHID3 to value 0"]
impl crate::Resettable for SSPPERIPHID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
