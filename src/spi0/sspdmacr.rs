#[doc = "Register `SSPDMACR` reader"]
pub type R = crate::R<SSPDMACR_SPEC>;
#[doc = "Register `SSPDMACR` writer"]
pub type W = crate::W<SSPDMACR_SPEC>;
#[doc = "Field `RXDMAE` reader - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_R = crate::BitReader;
#[doc = "Field `RXDMAE` writer - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_R = crate::BitReader;
#[doc = "Field `TXDMAE` writer - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<SSPDMACR_SPEC> {
        RXDMAE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<SSPDMACR_SPEC> {
        TXDMAE_W::new(self, 1)
    }
}
#[doc = "DMA control register, SSPDMACR on page 3-12  

You can [`read`](crate::Reg::read) this register and get [`sspdmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPDMACR_SPEC;
impl crate::RegisterSpec for SSPDMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspdmacr::R`](R) reader structure"]
impl crate::Readable for SSPDMACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspdmacr::W`](W) writer structure"]
impl crate::Writable for SSPDMACR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPDMACR to value 0"]
impl crate::Resettable for SSPDMACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
