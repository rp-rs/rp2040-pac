#[doc = "Register `IC_CLR_RX_DONE` reader"]
pub type R = crate::R<IC_CLR_RX_DONE_SPEC>;
#[doc = "Field `CLR_RX_DONE` reader - Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub type CLR_RX_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rx_done(&self) -> CLR_RX_DONE_R {
        CLR_RX_DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_DONE Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_RX_DONE_SPEC;
impl crate::RegisterSpec for IC_CLR_RX_DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_rx_done::R`](R) reader structure"]
impl crate::Readable for IC_CLR_RX_DONE_SPEC {}
#[doc = "`reset()` method sets IC_CLR_RX_DONE to value 0"]
impl crate::Resettable for IC_CLR_RX_DONE_SPEC {
    const RESET_VALUE: u32 = 0;
}
