#[doc = "Register `RTC_1` reader"]
pub type R = crate::R<RTC_1_SPEC>;
#[doc = "Register `RTC_1` writer"]
pub type W = crate::W<RTC_1_SPEC>;
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
impl W {}
#[doc = "RTC register 1.  

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_1_SPEC;
impl crate::RegisterSpec for RTC_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_1::R`](R) reader structure"]
impl crate::Readable for RTC_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_1::W`](W) writer structure"]
impl crate::Writable for RTC_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_1 to value 0"]
impl crate::Resettable for RTC_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
