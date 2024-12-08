#[doc = "Register `RXUICR` reader"]
pub type R = crate::R<RXUICR_SPEC>;
#[doc = "Register `RXUICR` writer"]
pub type W = crate::W<RXUICR_SPEC>;
#[doc = "Field `RXUICR` reader - Clear-on-read receive FIFO underflow interrupt"]
pub type RXUICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read receive FIFO underflow interrupt"]
    #[inline(always)]
    pub fn rxuicr(&self) -> RXUICR_R {
        RXUICR_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "RX FIFO underflow interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`rxuicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxuicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUICR_SPEC;
impl crate::RegisterSpec for RXUICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxuicr::R`](R) reader structure"]
impl crate::Readable for RXUICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxuicr::W`](W) writer structure"]
impl crate::Writable for RXUICR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXUICR to value 0"]
impl crate::Resettable for RXUICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
