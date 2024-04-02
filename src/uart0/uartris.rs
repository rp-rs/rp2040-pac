#[doc = "Register `UARTRIS` reader"]
pub type R = crate::R<UARTRIS_SPEC>;
#[doc = "Field `RIRMIS` reader - nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt."]
pub type RIRMIS_R = crate::BitReader;
#[doc = "Field `CTSRMIS` reader - nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
pub type CTSRMIS_R = crate::BitReader;
#[doc = "Field `DCDRMIS` reader - nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
pub type DCDRMIS_R = crate::BitReader;
#[doc = "Field `DSRRMIS` reader - nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
pub type DSRRMIS_R = crate::BitReader;
#[doc = "Field `RXRIS` reader - Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
pub type RXRIS_R = crate::BitReader;
#[doc = "Field `TXRIS` reader - Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
pub type TXRIS_R = crate::BitReader;
#[doc = "Field `RTRIS` reader - Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a"]
pub type RTRIS_R = crate::BitReader;
#[doc = "Field `FERIS` reader - Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
pub type FERIS_R = crate::BitReader;
#[doc = "Field `PERIS` reader - Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
pub type PERIS_R = crate::BitReader;
#[doc = "Field `BERIS` reader - Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
pub type BERIS_R = crate::BitReader;
#[doc = "Field `OERIS` reader - Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
pub type OERIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn rirmis(&self) -> RIRMIS_R {
        RIRMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsrmis(&self) -> CTSRMIS_R {
        CTSRMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdrmis(&self) -> DCDRMIS_R {
        DCDRMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrrmis(&self) -> DSRRMIS_R {
        DSRRMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status Register, UARTRIS  

You can [`read`](crate::Reg::read) this register and get [`uartris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTRIS_SPEC;
impl crate::RegisterSpec for UARTRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartris::R`](R) reader structure"]
impl crate::Readable for UARTRIS_SPEC {}
#[doc = "`reset()` method sets UARTRIS to value 0"]
impl crate::Resettable for UARTRIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
