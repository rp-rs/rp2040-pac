#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Field `CLK_SYS_RESUS` reader - "]
pub type CLK_SYS_RESUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_resus(&self) -> CLK_SYS_RESUS_R {
        CLK_SYS_RESUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Raw Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
