#[doc = "Register `CTR_HIT` reader"]
pub type R = crate::R<CTR_HIT_SPEC>;
#[doc = "Register `CTR_HIT` writer"]
pub type W = crate::W<CTR_HIT_SPEC>;
#[doc = "Field `CTR_HIT` reader - A 32 bit saturating counter that increments upon each cache hit, i.e. when an XIP access is serviced directly from cached data. Write any value to clear."]
pub type CTR_HIT_R = crate::FieldReader<u32>;
#[doc = "Field `CTR_HIT` writer - A 32 bit saturating counter that increments upon each cache hit, i.e. when an XIP access is serviced directly from cached data. Write any value to clear."]
pub type CTR_HIT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - A 32 bit saturating counter that increments upon each cache hit, i.e. when an XIP access is serviced directly from cached data. Write any value to clear."]
    #[inline(always)]
    pub fn ctr_hit(&self) -> CTR_HIT_R {
        CTR_HIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A 32 bit saturating counter that increments upon each cache hit, i.e. when an XIP access is serviced directly from cached data. Write any value to clear."]
    #[inline(always)]
    #[must_use]
    pub fn ctr_hit(&mut self) -> CTR_HIT_W<CTR_HIT_SPEC> {
        CTR_HIT_W::new(self, 0)
    }
}
#[doc = "Cache Hit counter  

You can [`read`](crate::generic::Reg::read) this register and get [`ctr_hit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr_hit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets CTR_HIT to value 0"]
impl crate::Resettable for CTR_HIT_SPEC {
    const RESET_VALUE: u32 = 0;
}
