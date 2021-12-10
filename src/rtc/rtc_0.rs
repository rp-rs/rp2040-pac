#[doc = "Register `RTC_0` reader"]
pub struct R(crate::R<RTC_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOTW` reader - Day of the week"]
pub struct DOTW_R(crate::FieldReader<u8, u8>);
impl DOTW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOTW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOTW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR` reader - Hours"]
pub struct HOUR_R(crate::FieldReader<u8, u8>);
impl HOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN` reader - Minutes"]
pub struct MIN_R(crate::FieldReader<u8, u8>);
impl MIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC` reader - Seconds"]
pub struct SEC_R(crate::FieldReader<u8, u8>);
impl SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:26 - Day of the week"]
    #[inline(always)]
    pub fn dotw(&self) -> DOTW_R {
        DOTW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "RTC register 0  
 Read this before RTC 1!  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [rtc_0](index.html) module"]
pub struct RTC_0_SPEC;
impl crate::RegisterSpec for RTC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_0::R](R) reader structure"]
impl crate::Readable for RTC_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_0 to value 0"]
impl crate::Resettable for RTC_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
