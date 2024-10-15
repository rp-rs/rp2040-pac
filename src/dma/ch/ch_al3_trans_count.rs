#[doc = "Register `CH_AL3_TRANS_COUNT` reader"]
pub type R = crate::R<CH_AL3_TRANS_COUNT_SPEC>;
#[doc = "Register `CH_AL3_TRANS_COUNT` writer"]
pub type W = crate::W<CH_AL3_TRANS_COUNT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Alias for channel 0 TRANS_COUNT register  

You can [`read`](crate::Reg::read) this register and get [`ch_al3_trans_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_al3_trans_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_AL3_TRANS_COUNT_SPEC;
impl crate::RegisterSpec for CH_AL3_TRANS_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_al3_trans_count::R`](R) reader structure"]
impl crate::Readable for CH_AL3_TRANS_COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_al3_trans_count::W`](W) writer structure"]
impl crate::Writable for CH_AL3_TRANS_COUNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_AL3_TRANS_COUNT to value 0"]
impl crate::Resettable for CH_AL3_TRANS_COUNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
