#[doc = "Register `CH6_DBG_TCR` reader"]
pub type R = crate::R<CH6_DBG_TCR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CH6_DBG_TCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch6_dbg_tcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6_DBG_TCR_SPEC;
impl crate::RegisterSpec for CH6_DBG_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_dbg_tcr::R`](R) reader structure"]
impl crate::Readable for CH6_DBG_TCR_SPEC {}
#[doc = "`reset()` method sets CH6_DBG_TCR to value 0"]
impl crate::Resettable for CH6_DBG_TCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
