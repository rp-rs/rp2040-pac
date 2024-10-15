#[doc = "Register `CTR_HIT` reader"]
pub type R = crate::R<CTR_HIT_SPEC>;
#[doc = "Register `CTR_HIT` writer"]
pub type W = crate::W<CTR_HIT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Cache Hit counter  
 A 32 bit saturating counter that increments upon each cache hit,  
 i.e. when an XIP access is serviced directly from cached data.  
 Write any value to clear.  

You can [`read`](crate::Reg::read) this register and get [`ctr_hit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr_hit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_HIT_SPEC;
impl crate::RegisterSpec for CTR_HIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr_hit::R`](R) reader structure"]
impl crate::Readable for CTR_HIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr_hit::W`](W) writer structure"]
impl crate::Writable for CTR_HIT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTR_HIT to value 0"]
impl crate::Resettable for CTR_HIT_SPEC {
    const RESET_VALUE: u32 = 0;
}
