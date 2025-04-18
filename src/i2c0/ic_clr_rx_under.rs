#[doc = "Register `IC_CLR_RX_UNDER` reader"]
pub type R = crate::R<IC_CLR_RX_UNDER_SPEC>;
#[doc = "Register `IC_CLR_RX_UNDER` writer"]
pub type W = crate::W<IC_CLR_RX_UNDER_SPEC>;
#[doc = "Field `CLR_RX_UNDER` reader - Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
pub type CLR_RX_UNDER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rx_under(&self) -> CLR_RX_UNDER_R {
        CLR_RX_UNDER_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Clear RX_UNDER Interrupt Register  

You can [`read`](crate::generic::Reg::read) this register and get [`ic_clr_rx_under::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_clr_rx_under::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_RX_UNDER_SPEC;
impl crate::RegisterSpec for IC_CLR_RX_UNDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_rx_under::R`](R) reader structure"]
impl crate::Readable for IC_CLR_RX_UNDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_clr_rx_under::W`](W) writer structure"]
impl crate::Writable for IC_CLR_RX_UNDER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_CLR_RX_UNDER to value 0"]
impl crate::Resettable for IC_CLR_RX_UNDER_SPEC {
    const RESET_VALUE: u32 = 0;
}
