#[doc = "Register `IC_CLR_RESTART_DET` reader"]
pub type R = crate::R<IC_CLR_RESTART_DET_SPEC>;
#[doc = "Field `CLR_RESTART_DET` reader - Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub type CLR_RESTART_DET_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_restart_det(&self) -> CLR_RESTART_DET_R {
        CLR_RESTART_DET_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RESTART_DET Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_restart_det::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_RESTART_DET_SPEC;
impl crate::RegisterSpec for IC_CLR_RESTART_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_restart_det::R`](R) reader structure"]
impl crate::Readable for IC_CLR_RESTART_DET_SPEC {}
#[doc = "`reset()` method sets IC_CLR_RESTART_DET to value 0"]
impl crate::Resettable for IC_CLR_RESTART_DET_SPEC {
    const RESET_VALUE: u32 = 0;
}
