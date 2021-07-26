#[doc = "Register `CH_AL1_TRANS_COUNT_TRIG` reader"]
pub struct R(crate::R<CH_AL1_TRANS_COUNT_TRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_AL1_TRANS_COUNT_TRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_AL1_TRANS_COUNT_TRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_AL1_TRANS_COUNT_TRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Alias for channel 0 TRANS_COUNT register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch_al1_trans_count_trig](index.html) module"]
pub struct CH_AL1_TRANS_COUNT_TRIG_SPEC;
impl crate::RegisterSpec for CH_AL1_TRANS_COUNT_TRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_al1_trans_count_trig::R](R) reader structure"]
impl crate::Readable for CH_AL1_TRANS_COUNT_TRIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH_AL1_TRANS_COUNT_TRIG to value 0"]
impl crate::Resettable for CH_AL1_TRANS_COUNT_TRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
