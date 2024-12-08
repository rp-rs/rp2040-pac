#[doc = "Register `TIMEHW` reader"]
pub type R = crate::R<TIMEHW_SPEC>;
#[doc = "Register `TIMEHW` writer"]
pub type W = crate::W<TIMEHW_SPEC>;
#[doc = "Field `TIMEHW` writer - "]
pub type TIMEHW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn timehw(&mut self) -> TIMEHW_W<TIMEHW_SPEC> {
        TIMEHW_W::new(self, 0)
    }
}
#[doc = "Write to bits 63:32 of time always write timelw before timehw  

You can [`read`](crate::generic::Reg::read) this register and get [`timehw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timehw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEHW_SPEC;
impl crate::RegisterSpec for TIMEHW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timehw::R`](R) reader structure"]
impl crate::Readable for TIMEHW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timehw::W`](W) writer structure"]
impl crate::Writable for TIMEHW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEHW to value 0"]
impl crate::Resettable for TIMEHW_SPEC {
    const RESET_VALUE: u32 = 0;
}
