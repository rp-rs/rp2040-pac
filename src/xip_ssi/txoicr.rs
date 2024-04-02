#[doc = "Register `TXOICR` reader"]
pub type R = crate::R<TXOICR_SPEC>;
#[doc = "Field `TXOICR` reader - Clear-on-read transmit FIFO overflow interrupt"]
pub type TXOICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read transmit FIFO overflow interrupt"]
    #[inline(always)]
    pub fn txoicr(&self) -> TXOICR_R {
        TXOICR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "TX FIFO overflow interrupt clear  

You can [`read`](crate::Reg::read) this register and get [`txoicr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXOICR_SPEC;
impl crate::RegisterSpec for TXOICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoicr::R`](R) reader structure"]
impl crate::Readable for TXOICR_SPEC {}
#[doc = "`reset()` method sets TXOICR to value 0"]
impl crate::Resettable for TXOICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
