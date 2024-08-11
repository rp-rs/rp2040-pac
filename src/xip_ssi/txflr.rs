#[doc = "Register `TXFLR` reader"]
pub type R = crate::R<TXFLR_SPEC>;
#[doc = "Register `TXFLR` writer"]
pub type W = crate::W<TXFLR_SPEC>;
#[doc = "Field `TFTFL` reader - Transmit FIFO level"]
pub type TFTFL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO level"]
    #[inline(always)]
    pub fn tftfl(&self) -> TFTFL_R {
        TFTFL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "TX FIFO level  

You can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFLR_SPEC;
impl crate::RegisterSpec for TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TXFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txflr::W`](W) writer structure"]
impl crate::Writable for TXFLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TXFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
