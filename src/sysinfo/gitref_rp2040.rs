#[doc = "Register `GITREF_RP2040` reader"]
pub struct R(crate::R<GITREF_RP2040_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GITREF_RP2040_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GITREF_RP2040_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GITREF_RP2040_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Git hash of the chip source. Used to identify chip version.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gitref_rp2040](index.html) module"]
pub struct GITREF_RP2040_SPEC;
impl crate::RegisterSpec for GITREF_RP2040_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gitref_rp2040::R](R) reader structure"]
impl crate::Readable for GITREF_RP2040_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GITREF_RP2040 to value 0"]
impl crate::Resettable for GITREF_RP2040_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
