#[doc = "Reader of register UARTRIS"]
pub type R = crate::R<u32, super::UARTRIS>;
#[doc = "Reader of field `OERIS`"]
pub type OERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BERIS`"]
pub type BERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERIS`"]
pub type PERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERIS`"]
pub type FERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTRIS`"]
pub type RTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSRRMIS`"]
pub type DSRRMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDRMIS`"]
pub type DCDRMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTSRMIS`"]
pub type CTSRMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RIRMIS`"]
pub type RIRMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 10 - Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrrmis(&self) -> DSRRMIS_R {
        DSRRMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdrmis(&self) -> DCDRMIS_R {
        DCDRMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsrmis(&self) -> CTSRMIS_R {
        CTSRMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn rirmis(&self) -> RIRMIS_R {
        RIRMIS_R::new((self.bits & 0x01) != 0)
    }
}
