#[doc = "Register `RXOICR` reader"]
pub type R = crate::R<RXOICR_SPEC>;
#[doc = "Field `RXOICR` reader - Clear-on-read receive FIFO overflow interrupt"]
pub type RXOICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read receive FIFO overflow interrupt"]
    #[inline(always)]
    pub fn rxoicr(&self) -> RXOICR_R {
        RXOICR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "RX FIFO overflow interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`rxoicr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXOICR_SPEC;
impl crate::RegisterSpec for RXOICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoicr::R`](R) reader structure"]
impl crate::Readable for RXOICR_SPEC {}
#[doc = "`reset()` method sets RXOICR to value 0"]
impl crate::Resettable for RXOICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
