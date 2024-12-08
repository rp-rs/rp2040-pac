#[doc = "Register `INTERP1_POP_LANE1` reader"]
pub type R = crate::R<INTERP1_POP_LANE1_SPEC>;
#[doc = "Register `INTERP1_POP_LANE1` writer"]
pub type W = crate::W<INTERP1_POP_LANE1_SPEC>;
#[doc = "Field `INTERP1_POP_LANE1` reader - "]
pub type INTERP1_POP_LANE1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp1_pop_lane1(&self) -> INTERP1_POP_LANE1_R {
        INTERP1_POP_LANE1_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_pop_lane1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_pop_lane1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_POP_LANE1_SPEC;
impl crate::RegisterSpec for INTERP1_POP_LANE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp1_pop_lane1::R`](R) reader structure"]
impl crate::Readable for INTERP1_POP_LANE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp1_pop_lane1::W`](W) writer structure"]
impl crate::Writable for INTERP1_POP_LANE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP1_POP_LANE1 to value 0"]
impl crate::Resettable for INTERP1_POP_LANE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
