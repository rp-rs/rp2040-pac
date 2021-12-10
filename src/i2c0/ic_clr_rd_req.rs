#[doc = "Register `IC_CLR_RD_REQ` reader"]
pub struct R(crate::R<IC_CLR_RD_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_RD_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_RD_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_RD_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_RD_REQ` reader - Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub struct CLR_RD_REQ_R(crate::FieldReader<bool, bool>);
impl CLR_RD_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_RD_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_RD_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rd_req(&self) -> CLR_RD_REQ_R {
        CLR_RD_REQ_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clear RD_REQ Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_rd_req](index.html) module"]
pub struct IC_CLR_RD_REQ_SPEC;
impl crate::RegisterSpec for IC_CLR_RD_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_rd_req::R](R) reader structure"]
impl crate::Readable for IC_CLR_RD_REQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_RD_REQ to value 0"]
impl crate::Resettable for IC_CLR_RD_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
