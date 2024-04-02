#[doc = "Register `IC_CLR_INTR` reader"]
pub type R = crate::R<IC_CLR_INTR_SPEC>;
#[doc = "Field `CLR_INTR` reader - Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.  

 Reset value: 0x0"]
pub type CLR_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_intr(&self) -> CLR_INTR_R {
        CLR_INTR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear Combined and Individual Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_intr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_INTR_SPEC;
impl crate::RegisterSpec for IC_CLR_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_intr::R`](R) reader structure"]
impl crate::Readable for IC_CLR_INTR_SPEC {}
#[doc = "`reset()` method sets IC_CLR_INTR to value 0"]
impl crate::Resettable for IC_CLR_INTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
