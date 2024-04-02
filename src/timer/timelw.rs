#[doc = "Register `TIMELW` writer"]
pub type W = crate::W<TIMELW_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TIMELW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Write to bits 31:0 of time  
 writes do not get copied to time until timehw is written  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timelw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMELW_SPEC;
impl crate::RegisterSpec for TIMELW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timelw::W`](W) writer structure"]
impl crate::Writable for TIMELW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMELW to value 0"]
impl crate::Resettable for TIMELW_SPEC {
    const RESET_VALUE: u32 = 0;
}
