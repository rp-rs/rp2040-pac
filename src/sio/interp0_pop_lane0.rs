#[doc = "Register `INTERP0_POP_LANE0` reader"]
pub type R = crate::R<INTERP0_POP_LANE0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTERP0_POP_LANE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_pop_lane0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_POP_LANE0_SPEC;
impl crate::RegisterSpec for INTERP0_POP_LANE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_pop_lane0::R`](R) reader structure"]
impl crate::Readable for INTERP0_POP_LANE0_SPEC {}
#[doc = "`reset()` method sets INTERP0_POP_LANE0 to value 0"]
impl crate::Resettable for INTERP0_POP_LANE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
