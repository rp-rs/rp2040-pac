#[doc = "Register `TXOICR` reader"]
pub type R = crate::R<TXOICR_SPEC>;
#[doc = "Register `TXOICR` writer"]
pub type W = crate::W<TXOICR_SPEC>;
#[doc = "Field `TXOICR` reader - Clear-on-read transmit FIFO overflow interrupt"]
pub type TXOICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read transmit FIFO overflow interrupt"]
    #[inline(always)]
    pub fn txoicr(&self) -> TXOICR_R {
        TXOICR_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "TX FIFO overflow interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`txoicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txoicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXOICR_SPEC;
impl crate::RegisterSpec for TXOICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoicr::R`](R) reader structure"]
impl crate::Readable for TXOICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txoicr::W`](W) writer structure"]
impl crate::Writable for TXOICR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXOICR to value 0"]
impl crate::Resettable for TXOICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
