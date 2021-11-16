#[doc = "Register `DBG_PADOUT` reader"]
pub struct R(crate::R<DBG_PADOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_PADOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_PADOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_PADOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dbg_padout](index.html) module"]
pub struct DBG_PADOUT_SPEC;
impl crate::RegisterSpec for DBG_PADOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_padout::R](R) reader structure"]
impl crate::Readable for DBG_PADOUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBG_PADOUT to value 0"]
impl crate::Resettable for DBG_PADOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
