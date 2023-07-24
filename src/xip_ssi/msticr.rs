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
pub type MSTICR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Clear-on-read multi-master contention interrupt"]
    #[inline(always)]
    pub fn msticr(&self) -> MSTICR_R {
        MSTICR_R::new((self.bits & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
