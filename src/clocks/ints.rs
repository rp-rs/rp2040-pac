#[doc = "Register `INTS` reader"]
pub type R = crate::R<INTS_SPEC>;
#[doc = "Register `INTS` writer"]
pub type W = crate::W<INTS_SPEC>;
#[doc = "Field `CLK_SYS_RESUS` reader - "]
pub type CLK_SYS_RESUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_resus(&self) -> CLK_SYS_RESUS_R {
        CLK_SYS_RESUS_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt status after masking &amp; forcing  

You can [`read`](crate::generic::Reg::read) this register and get [`ints::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for INTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ints::W`](W) writer structure"]
impl crate::Writable for INTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
