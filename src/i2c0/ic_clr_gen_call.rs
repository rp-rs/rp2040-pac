#[doc = "Register `IC_CLR_GEN_CALL` reader"]
pub struct R(crate::R<IC_CLR_GEN_CALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_GEN_CALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_GEN_CALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_GEN_CALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_GEN_CALL` reader - Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub struct CLR_GEN_CALL_R(crate::FieldReader<bool, bool>);
impl CLR_GEN_CALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_GEN_CALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_GEN_CALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_gen_call(&self) -> CLR_GEN_CALL_R {
        CLR_GEN_CALL_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clear GEN_CALL Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_gen_call](index.html) module"]
pub struct IC_CLR_GEN_CALL_SPEC;
impl crate::RegisterSpec for IC_CLR_GEN_CALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_gen_call::R](R) reader structure"]
impl crate::Readable for IC_CLR_GEN_CALL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_GEN_CALL to value 0"]
impl crate::Resettable for IC_CLR_GEN_CALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
