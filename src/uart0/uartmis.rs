#[doc = "Register `UARTMIS` reader"]
pub struct R(crate::R<UARTMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OEMIS` reader - Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt."]
pub struct OEMIS_R(crate::FieldReader<bool, bool>);
impl OEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEMIS` reader - Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt."]
pub struct BEMIS_R(crate::FieldReader<bool, bool>);
impl BEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEMIS` reader - Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt."]
pub struct PEMIS_R(crate::FieldReader<bool, bool>);
impl PEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEMIS` reader - Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt."]
pub struct FEMIS_R(crate::FieldReader<bool, bool>);
impl FEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTMIS` reader - Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt."]
pub struct RTMIS_R(crate::FieldReader<bool, bool>);
impl RTMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIS` reader - Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt."]
pub struct TXMIS_R(crate::FieldReader<bool, bool>);
impl TXMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMIS` reader - Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt."]
pub struct RXMIS_R(crate::FieldReader<bool, bool>);
impl RXMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMMIS` reader - nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
pub struct DSRMMIS_R(crate::FieldReader<bool, bool>);
impl DSRMMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMMIS` reader - nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
pub struct DCDMMIS_R(crate::FieldReader<bool, bool>);
impl DCDMMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMMIS` reader - nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
pub struct CTSMMIS_R(crate::FieldReader<bool, bool>);
impl CTSMMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIMMIS` reader - nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt."]
pub struct RIMMIS_R(crate::FieldReader<bool, bool>);
impl RIMMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Masked Interrupt Status Register, UARTMIS  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartmis](index.html) module"]
pub struct UARTMIS_SPEC;
impl crate::RegisterSpec for UARTMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartmis::R](R) reader structure"]
impl crate::Readable for UARTMIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTMIS to value 0"]
impl crate::Resettable for UARTMIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
