#[doc = "Register `PERFCTR3` reader"]
pub type R = crate::R<PERFCTR3_SPEC>;
#[doc = "Register `PERFCTR3` writer"]
pub type W = crate::W<PERFCTR3_SPEC>;
#[doc = "Field `PERFCTR3` reader - Busfabric saturating performance counter 3 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL3"]
pub type PERFCTR3_R = crate::FieldReader<u32>;
#[doc = "Field `PERFCTR3` writer - Busfabric saturating performance counter 3 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL3"]
pub type PERFCTR3_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 3 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL3"]
    #[inline(always)]
    pub fn perfctr3(&self) -> PERFCTR3_R {
        PERFCTR3_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 3 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL3"]
    #[inline(always)]
    #[must_use]
    pub fn perfctr3(&mut self) -> PERFCTR3_W<PERFCTR3_SPEC> {
        PERFCTR3_W::new(self, 0)
    }
}
#[doc = "Bus fabric performance counter 3  

You can [`read`](crate::generic::Reg::read) this register and get [`perfctr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfctr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERFCTR3_SPEC;
impl crate::RegisterSpec for PERFCTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfctr3::R`](R) reader structure"]
impl crate::Readable for PERFCTR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perfctr3::W`](W) writer structure"]
impl crate::Writable for PERFCTR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00ff_ffff;
}
#[doc = "`reset()` method sets PERFCTR3 to value 0"]
impl crate::Resettable for PERFCTR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
