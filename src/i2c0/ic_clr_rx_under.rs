#[doc = "Register `IC_CLR_RX_UNDER` reader"]
pub type R = crate::R<IC_CLR_RX_UNDER_SPEC>;
#[doc = "Field `CLR_RX_UNDER` reader - Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub type CLR_RX_UNDER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rx_under(&self) -> CLR_RX_UNDER_R {
        CLR_RX_UNDER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_UNDER Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_under::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_RX_UNDER_SPEC;
impl crate::RegisterSpec for IC_CLR_RX_UNDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_rx_under::R`](R) reader structure"]
impl crate::Readable for IC_CLR_RX_UNDER_SPEC {}
#[doc = "`reset()` method sets IC_CLR_RX_UNDER to value 0"]
impl crate::Resettable for IC_CLR_RX_UNDER_SPEC {
    const RESET_VALUE: u32 = 0;
}
