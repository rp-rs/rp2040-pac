#[doc = "Reader of register UARTMIS"]
pub type R = crate::R<u32, super::UARTMIS>;
#[doc = "Reader of field `OEMIS`"]
pub type OEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BEMIS`"]
pub type BEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEMIS`"]
pub type PEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEMIS`"]
pub type FEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTMIS`"]
pub type RTMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSRMMIS`"]
pub type DSRMMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDMMIS`"]
pub type DCDMMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTSMMIS`"]
pub type CTSMMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RIMMIS`"]
pub type RIMMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 10 - Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DSRMMIS_R {
        DSRMMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DCDMMIS_R {
        DCDMMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CTSMMIS_R {
        CTSMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn rimmis(&self) -> RIMMIS_R {
        RIMMIS_R::new((self.bits & 0x01) != 0)
    }
}
