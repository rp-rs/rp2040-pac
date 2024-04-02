#[doc = "Register `IC_CLR_RD_REQ` reader"]
pub type R = crate::R<IC_CLR_RD_REQ_SPEC>;
#[doc = "Field `CLR_RD_REQ` reader - Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub type CLR_RD_REQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rd_req(&self) -> CLR_RD_REQ_R {
        CLR_RD_REQ_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RD_REQ Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rd_req::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_RD_REQ_SPEC;
impl crate::RegisterSpec for IC_CLR_RD_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_rd_req::R`](R) reader structure"]
impl crate::Readable for IC_CLR_RD_REQ_SPEC {}
#[doc = "`reset()` method sets IC_CLR_RD_REQ to value 0"]
impl crate::Resettable for IC_CLR_RD_REQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
