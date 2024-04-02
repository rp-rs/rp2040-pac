#[doc = "Register `UARTDMACR` reader"]
pub type R = crate::R<UARTDMACR_SPEC>;
#[doc = "Register `UARTDMACR` writer"]
pub type W = crate::W<UARTDMACR_SPEC>;
#[doc = "Field `RXDMAE` reader - Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_R = crate::BitReader;
#[doc = "Field `RXDMAE` writer - Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_R = crate::BitReader;
#[doc = "Field `TXDMAE` writer - Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAONERR` reader - DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
pub type DMAONERR_R = crate::BitReader;
#[doc = "Field `DMAONERR` writer - DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
pub type DMAONERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DMAONERR_R {
        DMAONERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<UARTDMACR_SPEC> {
        RXDMAE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<UARTDMACR_SPEC> {
        TXDMAE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn dmaonerr(&mut self) -> DMAONERR_W<UARTDMACR_SPEC> {
        DMAONERR_W::new(self, 2)
    }
}
#[doc = "DMA Control Register, UARTDMACR  

You can [`read`](crate::Reg::read) this register and get [`uartdmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTDMACR_SPEC;
impl crate::RegisterSpec for UARTDMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdmacr::R`](R) reader structure"]
impl crate::Readable for UARTDMACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartdmacr::W`](W) writer structure"]
impl crate::Writable for UARTDMACR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTDMACR to value 0"]
impl crate::Resettable for UARTDMACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
