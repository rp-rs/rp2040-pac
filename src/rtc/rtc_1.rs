#[doc = "Register `RTC_1` reader"]
pub struct R(crate::R<RTC_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `YEAR` reader - Year"]
pub struct YEAR_R(crate::FieldReader<u16, u16>);
impl YEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        YEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH` reader - Month (1..12)"]
pub struct MONTH_R(crate::FieldReader<u8, u8>);
impl MONTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY` reader - Day of the month (1..31)"]
pub struct DAY_R(crate::FieldReader<u8, u8>);
impl DAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "RTC register 1.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [rtc_1](index.html) module"]
pub struct RTC_1_SPEC;
impl crate::RegisterSpec for RTC_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_1::R](R) reader structure"]
impl crate::Readable for RTC_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_1 to value 0"]
impl crate::Resettable for RTC_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
