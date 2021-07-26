#[doc = "Register `INTERP1_POP_LANE0` reader"]
pub struct R(crate::R<INTERP1_POP_LANE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP1_POP_LANE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERP1_POP_LANE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERP1_POP_LANE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP).  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [interp1_pop_lane0](index.html) module"]
pub struct INTERP1_POP_LANE0_SPEC;
impl crate::RegisterSpec for INTERP1_POP_LANE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp1_pop_lane0::R](R) reader structure"]
impl crate::Readable for INTERP1_POP_LANE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERP1_POP_LANE0 to value 0"]
impl crate::Resettable for INTERP1_POP_LANE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
