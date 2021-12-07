#[doc = "Register `IC_CLR_TX_ABRT` reader"]
pub struct R(crate::R<IC_CLR_TX_ABRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_TX_ABRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_TX_ABRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_TX_ABRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_TX_ABRT` reader - Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.  

 Reset value: 0x0"]
pub struct CLR_TX_ABRT_R(crate::FieldReader<bool, bool>);
impl CLR_TX_ABRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_TX_ABRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_TX_ABRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_tx_abrt(&self) -> CLR_TX_ABRT_R {
        CLR_TX_ABRT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clear TX_ABRT Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_tx_abrt](index.html) module"]
pub struct IC_CLR_TX_ABRT_SPEC;
impl crate::RegisterSpec for IC_CLR_TX_ABRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_tx_abrt::R](R) reader structure"]
impl crate::Readable for IC_CLR_TX_ABRT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_TX_ABRT to value 0"]
impl crate::Resettable for IC_CLR_TX_ABRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
