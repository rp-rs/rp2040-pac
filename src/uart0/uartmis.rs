#[doc = "Register `UARTMIS` reader"]
pub type R = crate::R<UARTMIS_SPEC>;
#[doc = "Field `RIMMIS` reader - nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt."]
pub type RIMMIS_R = crate::BitReader;
#[doc = "Field `CTSMMIS` reader - nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
pub type CTSMMIS_R = crate::BitReader;
#[doc = "Field `DCDMMIS` reader - nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
pub type DCDMMIS_R = crate::BitReader;
#[doc = "Field `DSRMMIS` reader - nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
pub type DSRMMIS_R = crate::BitReader;
#[doc = "Field `RXMIS` reader - Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt."]
pub type RXMIS_R = crate::BitReader;
#[doc = "Field `TXMIS` reader - Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt."]
pub type TXMIS_R = crate::BitReader;
#[doc = "Field `RTMIS` reader - Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt."]
pub type RTMIS_R = crate::BitReader;
#[doc = "Field `FEMIS` reader - Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt."]
pub type FEMIS_R = crate::BitReader;
#[doc = "Field `PEMIS` reader - Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt."]
pub type PEMIS_R = crate::BitReader;
#[doc = "Field `BEMIS` reader - Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt."]
pub type BEMIS_R = crate::BitReader;
#[doc = "Field `OEMIS` reader - Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt."]
pub type OEMIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn rimmis(&self) -> RIMMIS_R {
        RIMMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CTSMMIS_R {
        CTSMMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DCDMMIS_R {
        DCDMMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DSRMMIS_R {
        DSRMMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status Register, UARTMIS  

You can [`read`](crate::Reg::read) this register and get [`uartmis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTMIS_SPEC;
impl crate::RegisterSpec for UARTMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartmis::R`](R) reader structure"]
impl crate::Readable for UARTMIS_SPEC {}
#[doc = "`reset()` method sets UARTMIS to value 0"]
impl crate::Resettable for UARTMIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
