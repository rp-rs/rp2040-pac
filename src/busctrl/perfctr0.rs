#[doc = "Register `PERFCTR0` reader"]
pub type R = crate::R<PERFCTR0_SPEC>;
#[doc = "Register `PERFCTR0` writer"]
pub type W = crate::W<PERFCTR0_SPEC>;
#[doc = "Field `PERFCTR0` reader - Busfabric saturating performance counter 0 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL0"]
pub type PERFCTR0_R = crate::FieldReader<u32>;
#[doc = "Field `PERFCTR0` writer - Busfabric saturating performance counter 0 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL0"]
pub type PERFCTR0_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 0 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    pub fn perfctr0(&self) -> PERFCTR0_R {
        PERFCTR0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 0 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn perfctr0(&mut self) -> PERFCTR0_W<PERFCTR0_SPEC> {
        PERFCTR0_W::new(self, 0)
    }
}
#[doc = "Bus fabric performance counter 0  

You can [`read`](crate::generic::Reg::read) this register and get [`perfctr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfctr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERFCTR0_SPEC;
impl crate::RegisterSpec for PERFCTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfctr0::R`](R) reader structure"]
impl crate::Readable for PERFCTR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perfctr0::W`](W) writer structure"]
impl crate::Writable for PERFCTR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00ff_ffff;
}
#[doc = "`reset()` method sets PERFCTR0 to value 0"]
impl crate::Resettable for PERFCTR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
