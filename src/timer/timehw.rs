#[doc = "Register `TIMEHW` writer"]
pub type W = crate::W<TIMEHW_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TIMEHW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Write to bits 63:32 of time  
 always write timelw before timehw  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timehw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEHW_SPEC;
impl crate::RegisterSpec for TIMEHW_SPEC {
    type Ux = u32;
}
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
