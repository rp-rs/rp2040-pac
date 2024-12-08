#[doc = "Register `CH2_DBG_TCR` reader"]
pub type R = crate::R<CH2_DBG_TCR_SPEC>;
#[doc = "Register `CH2_DBG_TCR` writer"]
pub type W = crate::W<CH2_DBG_TCR_SPEC>;
#[doc = "Field `CH2_DBG_TCR` reader - "]
pub type CH2_DBG_TCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch2_dbg_tcr(&self) -> CH2_DBG_TCR_R {
        CH2_DBG_TCR_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch2_dbg_tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dbg_tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_DBG_TCR_SPEC;
impl crate::RegisterSpec for CH2_DBG_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_dbg_tcr::R`](R) reader structure"]
impl crate::Readable for CH2_DBG_TCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_dbg_tcr::W`](W) writer structure"]
impl crate::Writable for CH2_DBG_TCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_DBG_TCR to value 0"]
impl crate::Resettable for CH2_DBG_TCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
