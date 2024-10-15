#[doc = "Register `UARTIMSC` reader"]
pub type R = crate::R<UARTIMSC_SPEC>;
#[doc = "Register `UARTIMSC` writer"]
pub type W = crate::W<UARTIMSC_SPEC>;
#[doc = "Field `RIMIM` reader - nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
pub type RIMIM_R = crate::BitReader;
#[doc = "Field `RIMIM` writer - nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
pub type RIMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIM` reader - nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
pub type CTSMIM_R = crate::BitReader;
#[doc = "Field `CTSMIM` writer - nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
pub type CTSMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMIM` reader - nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
pub type DCDMIM_R = crate::BitReader;
#[doc = "Field `DCDMIM` writer - nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
pub type DCDMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMIM` reader - nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
pub type DSRMIM_R = crate::BitReader;
#[doc = "Field `DSRMIM` writer - nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
pub type DSRMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
pub type RXIM_R = crate::BitReader;
#[doc = "Field `RXIM` writer - Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
pub type RXIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
pub type TXIM_R = crate::BitReader;
#[doc = "Field `TXIM` writer - Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
pub type TXIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
pub type RTIM_R = crate::BitReader;
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
pub type RTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIM` reader - Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
pub type FEIM_R = crate::BitReader;
#[doc = "Field `FEIM` writer - Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
pub type FEIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIM` reader - Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
pub type PEIM_R = crate::BitReader;
#[doc = "Field `PEIM` writer - Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
pub type PEIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIM` reader - Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
pub type BEIM_R = crate::BitReader;
#[doc = "Field `BEIM` writer - Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
pub type BEIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIM` reader - Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
pub type OEIM_R = crate::BitReader;
#[doc = "Field `OEIM` writer - Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
pub type OEIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rimim(&self) -> RIMIM_R {
        RIMIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn rimim(&mut self) -> RIMIM_W<UARTIMSC_SPEC> {
        RIMIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmim(&mut self) -> CTSMIM_W<UARTIMSC_SPEC> {
        CTSMIM_W::new(self, 1)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn dcdmim(&mut self) -> DCDMIM_W<UARTIMSC_SPEC> {
        DCDMIM_W::new(self, 2)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn dsrmim(&mut self) -> DSRMIM_W<UARTIMSC_SPEC> {
        DSRMIM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<UARTIMSC_SPEC> {
        RXIM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<UARTIMSC_SPEC> {
        TXIM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<UARTIMSC_SPEC> {
        RTIM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn feim(&mut self) -> FEIM_W<UARTIMSC_SPEC> {
        FEIM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn peim(&mut self) -> PEIM_W<UARTIMSC_SPEC> {
        PEIM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn beim(&mut self) -> BEIM_W<UARTIMSC_SPEC> {
        BEIM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    #[must_use]
    pub fn oeim(&mut self) -> OEIM_W<UARTIMSC_SPEC> {
        OEIM_W::new(self, 10)
    }
}
#[doc = "Interrupt Mask Set/Clear Register, UARTIMSC  

You can [`read`](crate::Reg::read) this register and get [`uartimsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartimsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTIMSC_SPEC;
impl crate::RegisterSpec for UARTIMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartimsc::R`](R) reader structure"]
impl crate::Readable for UARTIMSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartimsc::W`](W) writer structure"]
impl crate::Writable for UARTIMSC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTIMSC to value 0"]
impl crate::Resettable for UARTIMSC_SPEC {
    const RESET_VALUE: u32 = 0;
}
