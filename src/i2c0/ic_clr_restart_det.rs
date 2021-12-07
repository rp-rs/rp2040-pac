#[doc = "Register `IC_CLR_RESTART_DET` reader"]
pub struct R(crate::R<IC_CLR_RESTART_DET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_RESTART_DET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_RESTART_DET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_RESTART_DET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_RESTART_DET` reader - Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub struct CLR_RESTART_DET_R(crate::FieldReader<bool, bool>);
impl CLR_RESTART_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_RESTART_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_RESTART_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_restart_det(&self) -> CLR_RESTART_DET_R {
        CLR_RESTART_DET_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clear RESTART_DET Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_restart_det](index.html) module"]
pub struct IC_CLR_RESTART_DET_SPEC;
impl crate::RegisterSpec for IC_CLR_RESTART_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_restart_det::R](R) reader structure"]
impl crate::Readable for IC_CLR_RESTART_DET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_RESTART_DET to value 0"]
impl crate::Resettable for IC_CLR_RESTART_DET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
