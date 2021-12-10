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
#[doc = "Field `KHZ` reader - "]
pub struct KHZ_R(crate::FieldReader<u32, u32>);
impl KHZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        KHZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KHZ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAC` reader - "]
pub struct FRAC_R(crate::FieldReader<u8, u8>);
impl FRAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 5:29"]
    #[inline(always)]
    pub fn khz(&self) -> KHZ_R {
        KHZ_R::new(((self.bits >> 5) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x1f) as u8)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
