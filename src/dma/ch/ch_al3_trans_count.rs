#[doc = "Register `CH_AL3_TRANS_COUNT` reader"]
pub struct R(crate::R<CH_AL3_TRANS_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_AL3_TRANS_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_AL3_TRANS_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_AL3_TRANS_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Alias for channel 0 TRANS_COUNT register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch_al3_trans_count](index.html) module"]
pub struct CH_AL3_TRANS_COUNT_SPEC;
impl crate::RegisterSpec for CH_AL3_TRANS_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_al3_trans_count::R](R) reader structure"]
impl crate::Readable for CH_AL3_TRANS_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH_AL3_TRANS_COUNT to value 0"]
impl crate::Resettable for CH_AL3_TRANS_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
