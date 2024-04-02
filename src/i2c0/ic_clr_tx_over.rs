#[doc = "Register `IC_CLR_TX_OVER` reader"]
pub type R = crate::R<IC_CLR_TX_OVER_SPEC>;
#[doc = "Field `CLR_TX_OVER` reader - Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub type CLR_TX_OVER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_tx_over(&self) -> CLR_TX_OVER_R {
        CLR_TX_OVER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_OVER Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_TX_OVER_SPEC;
impl crate::RegisterSpec for IC_CLR_TX_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_tx_over::R`](R) reader structure"]
impl crate::Readable for IC_CLR_TX_OVER_SPEC {}
#[doc = "`reset()` method sets IC_CLR_TX_OVER to value 0"]
impl crate::Resettable for IC_CLR_TX_OVER_SPEC {
    const RESET_VALUE: u32 = 0;
}
