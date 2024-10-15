#[doc = "Register `CLK_SYS_RESUS_STATUS` reader"]
pub type R = crate::R<CLK_SYS_RESUS_STATUS_SPEC>;
#[doc = "Field `RESUSSED` reader - Clock has been resuscitated, correct the error then send ctrl_clear=1"]
pub type RESUSSED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clock has been resuscitated, correct the error then send ctrl_clear=1"]
    #[inline(always)]
    pub fn resussed(&self) -> RESUSSED_R {
        RESUSSED_R::new((self.bits & 1) != 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`clk_sys_resus_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SYS_RESUS_STATUS_SPEC;
impl crate::RegisterSpec for CLK_SYS_RESUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_sys_resus_status::R`](R) reader structure"]
impl crate::Readable for CLK_SYS_RESUS_STATUS_SPEC {}
#[doc = "`reset()` method sets CLK_SYS_RESUS_STATUS to value 0"]
impl crate::Resettable for CLK_SYS_RESUS_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
