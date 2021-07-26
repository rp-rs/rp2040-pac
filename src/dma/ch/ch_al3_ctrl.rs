#[doc = "Register `CH_AL3_CTRL` reader"]
pub struct R(crate::R<CH_AL3_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_AL3_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_AL3_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_AL3_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Alias for channel 0 CTRL register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch_al3_ctrl](index.html) module"]
pub struct CH_AL3_CTRL_SPEC;
impl crate::RegisterSpec for CH_AL3_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_al3_ctrl::R](R) reader structure"]
impl crate::Readable for CH_AL3_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH_AL3_CTRL to value 0"]
impl crate::Resettable for CH_AL3_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
