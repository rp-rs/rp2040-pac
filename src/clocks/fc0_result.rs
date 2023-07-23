#[doc = "Register `FC0_RESULT` reader"]
pub struct R(crate::R<FC0_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC0_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC0_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC0_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAC` reader - "]
pub type FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KHZ` reader - "]
pub type KHZ_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:29"]
    #[inline(always)]
    pub fn khz(&self) -> KHZ_R {
        KHZ_R::new(((self.bits >> 5) & 0x01ff_ffff) as u32)
    }
}
#[doc = "Result of frequency measurement, only valid when status_done=1  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fc0_result](index.html) module"]
pub struct FC0_RESULT_SPEC;
impl crate::RegisterSpec for FC0_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc0_result::R](R) reader structure"]
impl crate::Readable for FC0_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FC0_RESULT to value 0"]
impl crate::Resettable for FC0_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
