#[doc = "Register `PERFCTR2` reader"]
pub type R = crate::R<PERFCTR2_SPEC>;
#[doc = "Register `PERFCTR2` writer"]
pub type W = crate::W<PERFCTR2_SPEC>;
#[doc = "Field `PERFCTR2` reader - Busfabric saturating performance counter 2 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL2"]
pub type PERFCTR2_R = crate::FieldReader<u32>;
#[doc = "Field `PERFCTR2` writer - Busfabric saturating performance counter 2 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL2"]
pub type PERFCTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 2 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL2"]
    #[inline(always)]
    pub fn perfctr2(&self) -> PERFCTR2_R {
        PERFCTR2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 2 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL2"]
    #[inline(always)]
    #[must_use]
    pub fn perfctr2(&mut self) -> PERFCTR2_W<PERFCTR2_SPEC> {
        PERFCTR2_W::new(self, 0)
    }
}
#[doc = "Bus fabric performance counter 2  

You can [`read`](crate::generic::Reg::read) this register and get [`perfctr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfctr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERFCTR2_SPEC;
impl crate::RegisterSpec for PERFCTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfctr2::R`](R) reader structure"]
impl crate::Readable for PERFCTR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perfctr2::W`](W) writer structure"]
impl crate::Writable for PERFCTR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00ff_ffff;
}
#[doc = "`reset()` method sets PERFCTR2 to value 0"]
impl crate::Resettable for PERFCTR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
