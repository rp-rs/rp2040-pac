#[doc = "Register `IC_CLR_START_DET` reader"]
pub struct R(crate::R<IC_CLR_START_DET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_START_DET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_START_DET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_START_DET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_START_DET` reader - Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub struct CLR_START_DET_R(crate::FieldReader<bool, bool>);
impl CLR_START_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_START_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_START_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_start_det(&self) -> CLR_START_DET_R {
        CLR_START_DET_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clear START_DET Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_start_det](index.html) module"]
pub struct IC_CLR_START_DET_SPEC;
impl crate::RegisterSpec for IC_CLR_START_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_start_det::R](R) reader structure"]
impl crate::Readable for IC_CLR_START_DET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_START_DET to value 0"]
impl crate::Resettable for IC_CLR_START_DET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
