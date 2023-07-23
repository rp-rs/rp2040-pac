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
#[doc = "Field `DAY` reader - Day of the month (1..31)"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONTH` reader - Month (1..12)"]
pub type MONTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` reader - Year"]
pub type YEAR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 12) & 0x0fff) as u16)
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
    const RESET_VALUE: Self::Ux = 0;
}
