#[doc = "Register `RXFLR` reader"]
pub type R = crate::R<RXFLR_SPEC>;
#[doc = "Register `RXFLR` writer"]
pub type W = crate::W<RXFLR_SPEC>;
#[doc = "Field `RXTFL` reader - Receive FIFO level"]
pub type RXTFL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO level"]
    #[inline(always)]
    pub fn rxtfl(&self) -> RXTFL_R {
        RXTFL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "RX FIFO level  

You can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFLR_SPEC;
impl crate::RegisterSpec for RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RXFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxflr::W`](W) writer structure"]
impl crate::Writable for RXFLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RXFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
