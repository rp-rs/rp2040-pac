#[doc = "Register `RTC_1` reader"]
pub type R = crate::R<RTC_1_SPEC>;
#[doc = "Field `DAY` reader - Day of the month (1..31)"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `MONTH` reader - Month (1..12)"]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `YEAR` reader - Year"]
pub type YEAR_R = crate::FieldReader<u16>;
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

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_1_SPEC;
impl crate::RegisterSpec for RTC_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_1::R`](R) reader structure"]
impl crate::Readable for RTC_1_SPEC {}
#[doc = "`reset()` method sets RTC_1 to value 0"]
impl crate::Resettable for RTC_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
