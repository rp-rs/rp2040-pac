#[doc = "Register `UARTRIS` reader"]
pub struct R(crate::R<UARTRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OERIS` reader - Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
pub struct OERIS_R(crate::FieldReader<bool, bool>);
impl OERIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERIS` reader - Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
pub struct BERIS_R(crate::FieldReader<bool, bool>);
impl BERIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIS` reader - Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
pub struct PERIS_R(crate::FieldReader<bool, bool>);
impl PERIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERIS` reader - Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
pub struct FERIS_R(crate::FieldReader<bool, bool>);
impl FERIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTRIS` reader - Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a"]
pub struct RTRIS_R(crate::FieldReader<bool, bool>);
impl RTRIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRIS` reader - Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
pub struct TXRIS_R(crate::FieldReader<bool, bool>);
impl TXRIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRIS` reader - Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
pub struct RXRIS_R(crate::FieldReader<bool, bool>);
impl RXRIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRRMIS` reader - nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
pub struct DSRRMIS_R(crate::FieldReader<bool, bool>);
impl DSRRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDRMIS` reader - nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
pub struct DCDRMIS_R(crate::FieldReader<bool, bool>);
impl DCDRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSRMIS` reader - nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
pub struct CTSRMIS_R(crate::FieldReader<bool, bool>);
impl CTSRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIRMIS` reader - nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt."]
pub struct RIRMIS_R(crate::FieldReader<bool, bool>);
impl RIRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Raw Interrupt Status Register, UARTRIS  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartris](index.html) module"]
pub struct UARTRIS_SPEC;
impl crate::RegisterSpec for UARTRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartris::R](R) reader structure"]
impl crate::Readable for UARTRIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTRIS to value 0"]
impl crate::Resettable for UARTRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
