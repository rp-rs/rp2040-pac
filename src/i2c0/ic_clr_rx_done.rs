#[doc = "Register `IC_CLR_RX_DONE` reader"]
pub struct R(crate::R<IC_CLR_RX_DONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_RX_DONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_RX_DONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_RX_DONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_RX_DONE` reader - Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub struct CLR_RX_DONE_R(crate::FieldReader<bool, bool>);
impl CLR_RX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_RX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_RX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rx_done(&self) -> CLR_RX_DONE_R {
        CLR_RX_DONE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clear RX_DONE Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_rx_done](index.html) module"]
pub struct IC_CLR_RX_DONE_SPEC;
impl crate::RegisterSpec for IC_CLR_RX_DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_rx_done::R](R) reader structure"]
impl crate::Readable for IC_CLR_RX_DONE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_RX_DONE to value 0"]
impl crate::Resettable for IC_CLR_RX_DONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
