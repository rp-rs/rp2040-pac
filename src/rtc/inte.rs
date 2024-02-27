#[doc = "Register `INTE` reader"]
pub type R = crate::R<INTE_SPEC>;
#[doc = "Register `INTE` writer"]
pub type W = crate::W<INTE_SPEC>;
#[doc = "Field `RTC` reader - "]
pub type RTC_R = crate::BitReader;
#[doc = "Field `RTC` writer - "]
pub type RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<INTE_SPEC> {
        RTC_W::new(self, 0)
    }
}
#[doc = "Interrupt Enable  

You can [`read`](crate::generic::Reg::read) this register and get [`inte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE_SPEC;
impl crate::RegisterSpec for INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte::R`](R) reader structure"]
impl crate::Readable for INTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte::W`](W) writer structure"]
impl crate::Writable for INTE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE to value 0"]
impl crate::Resettable for INTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
