#[doc = "Register `IC_CLR_RX_UNDER` reader"]
pub struct R(crate::R<IC_CLR_RX_UNDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_RX_UNDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_RX_UNDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_RX_UNDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_RX_UNDER` reader - Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub type CLR_RX_UNDER_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rx_under(&self) -> CLR_RX_UNDER_R {
        CLR_RX_UNDER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_UNDER Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_rx_under](index.html) module"]
pub struct IC_CLR_RX_UNDER_SPEC;
impl crate::RegisterSpec for IC_CLR_RX_UNDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_rx_under::R](R) reader structure"]
impl crate::Readable for IC_CLR_RX_UNDER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_RX_UNDER to value 0"]
impl crate::Resettable for IC_CLR_RX_UNDER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
