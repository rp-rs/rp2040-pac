#[doc = "Register `RXOICR` reader"]
pub type R = crate::R<RXOICR_SPEC>;
#[doc = "Register `RXOICR` writer"]
pub type W = crate::W<RXOICR_SPEC>;
#[doc = "Field `RXOICR` reader - Clear-on-read receive FIFO overflow interrupt"]
pub type RXOICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read receive FIFO overflow interrupt"]
    #[inline(always)]
    pub fn rxoicr(&self) -> RXOICR_R {
        RXOICR_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "RX FIFO overflow interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`rxoicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxoicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXOICR_SPEC;
impl crate::RegisterSpec for RXOICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoicr::R`](R) reader structure"]
impl crate::Readable for RXOICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxoicr::W`](W) writer structure"]
impl crate::Writable for RXOICR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXOICR to value 0"]
impl crate::Resettable for RXOICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
