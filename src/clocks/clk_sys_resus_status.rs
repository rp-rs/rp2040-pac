#[doc = "Register `CLK_SYS_RESUS_STATUS` reader"]
pub type R = crate::R<CLK_SYS_RESUS_STATUS_SPEC>;
#[doc = "Register `CLK_SYS_RESUS_STATUS` writer"]
pub type W = crate::W<CLK_SYS_RESUS_STATUS_SPEC>;
#[doc = "Field `RESUSSED` reader - Clock has been resuscitated, correct the error then send ctrl_clear=1"]
pub type RESUSSED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clock has been resuscitated, correct the error then send ctrl_clear=1"]
    #[inline(always)]
    pub fn resussed(&self) -> RESUSSED_R {
        RESUSSED_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_sys_resus_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sys_resus_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SYS_RESUS_STATUS_SPEC;
impl crate::RegisterSpec for CLK_SYS_RESUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_sys_resus_status::R`](R) reader structure"]
impl crate::Readable for CLK_SYS_RESUS_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_sys_resus_status::W`](W) writer structure"]
impl crate::Writable for CLK_SYS_RESUS_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_SYS_RESUS_STATUS to value 0"]
impl crate::Resettable for CLK_SYS_RESUS_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
