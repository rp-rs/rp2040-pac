#[doc = "Register `RTC_0` reader"]
pub type R = crate::R<RTC_0_SPEC>;
#[doc = "Field `SEC` reader - Seconds"]
pub type SEC_R = crate::FieldReader;
#[doc = "Field `MIN` reader - Minutes"]
pub type MIN_R = crate::FieldReader;
#[doc = "Field `HOUR` reader - Hours"]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `DOTW` reader - Day of the week"]
pub type DOTW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Day of the week"]
    #[inline(always)]
    pub fn dotw(&self) -> DOTW_R {
        DOTW_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "RTC register 0  
 Read this before RTC 1!  

You can [`read`](crate::Reg::read) this register and get [`rtc_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_0_SPEC;
impl crate::RegisterSpec for RTC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_0::R`](R) reader structure"]
impl crate::Readable for RTC_0_SPEC {}
#[doc = "`reset()` method sets RTC_0 to value 0"]
impl crate::Resettable for RTC_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
