#[doc = "Register `SSPPERIPHID2` reader"]
pub struct R(crate::R<SSPPERIPHID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPPERIPHID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPPERIPHID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPPERIPHID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REVISION` reader - These bits return the peripheral revision"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESIGNER1` reader - These bits read back as 0x4"]
pub struct DESIGNER1_R(crate::FieldReader<u8, u8>);
impl DESIGNER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DESIGNER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESIGNER1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:7 - These bits return the peripheral revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - These bits read back as 0x4"]
    #[inline(always)]
    pub fn designer1(&self) -> DESIGNER1_R {
        DESIGNER1_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspperiphid2](index.html) module"]
pub struct SSPPERIPHID2_SPEC;
impl crate::RegisterSpec for SSPPERIPHID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspperiphid2::R](R) reader structure"]
impl crate::Readable for SSPPERIPHID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPPERIPHID2 to value 0x34"]
impl crate::Resettable for SSPPERIPHID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x34
    }
}
