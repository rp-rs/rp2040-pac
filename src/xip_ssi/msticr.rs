#[doc = "Register `MSTICR` reader"]
pub struct R(crate::R<MSTICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSTICR` reader - Clear-on-read multi-master contention interrupt"]
pub struct MSTICR_R(crate::FieldReader<bool, bool>);
impl MSTICR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTICR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTICR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Clear-on-read multi-master contention interrupt"]
    #[inline(always)]
    pub fn msticr(&self) -> MSTICR_R {
        MSTICR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Multi-master interrupt clear  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [msticr](index.html) module"]
pub struct MSTICR_SPEC;
impl crate::RegisterSpec for MSTICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msticr::R](R) reader structure"]
impl crate::Readable for MSTICR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSTICR to value 0"]
impl crate::Resettable for MSTICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
