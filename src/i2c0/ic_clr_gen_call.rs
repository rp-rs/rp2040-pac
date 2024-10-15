#[doc = "Register `IC_CLR_GEN_CALL` reader"]
pub type R = crate::R<IC_CLR_GEN_CALL_SPEC>;
#[doc = "Field `CLR_GEN_CALL` reader - Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub type CLR_GEN_CALL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_gen_call(&self) -> CLR_GEN_CALL_R {
        CLR_GEN_CALL_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear GEN_CALL Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_gen_call::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_GEN_CALL_SPEC;
impl crate::RegisterSpec for IC_CLR_GEN_CALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_gen_call::R`](R) reader structure"]
impl crate::Readable for IC_CLR_GEN_CALL_SPEC {}
#[doc = "`reset()` method sets IC_CLR_GEN_CALL to value 0"]
impl crate::Resettable for IC_CLR_GEN_CALL_SPEC {
    const RESET_VALUE: u32 = 0;
}
