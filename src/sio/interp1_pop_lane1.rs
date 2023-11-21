#[doc = "Register `INTERP1_POP_LANE1` reader"]
pub type R = crate::R<INTERP1_POP_LANE1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTERP1_POP_LANE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_pop_lane1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_POP_LANE1_SPEC;
impl crate::RegisterSpec for INTERP1_POP_LANE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp1_pop_lane1::R`](R) reader structure"]
impl crate::Readable for INTERP1_POP_LANE1_SPEC {}
#[doc = "`reset()` method sets INTERP1_POP_LANE1 to value 0"]
impl crate::Resettable for INTERP1_POP_LANE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
