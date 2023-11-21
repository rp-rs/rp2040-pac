#[doc = "Register `RXUICR` reader"]
pub type R = crate::R<RXUICR_SPEC>;
#[doc = "Field `RXUICR` reader - Clear-on-read receive FIFO underflow interrupt"]
pub type RXUICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read receive FIFO underflow interrupt"]
    #[inline(always)]
    pub fn rxuicr(&self) -> RXUICR_R {
        RXUICR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "RX FIFO underflow interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`rxuicr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUICR_SPEC;
impl crate::RegisterSpec for RXUICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxuicr::R`](R) reader structure"]
impl crate::Readable for RXUICR_SPEC {}
#[doc = "`reset()` method sets RXUICR to value 0"]
impl crate::Resettable for RXUICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
