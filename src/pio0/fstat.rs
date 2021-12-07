#[doc = "Register `FSTAT` reader"]
pub struct R(crate::R<FSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXEMPTY` reader - State machine TX FIFO is empty"]
pub struct TXEMPTY_R(crate::FieldReader<u8, u8>);
impl TXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFULL` reader - State machine TX FIFO is full"]
pub struct TXFULL_R(crate::FieldReader<u8, u8>);
impl TXFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFULL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEMPTY` reader - State machine RX FIFO is empty"]
pub struct RXEMPTY_R(crate::FieldReader<u8, u8>);
impl RXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEMPTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFULL` reader - State machine RX FIFO is full"]
pub struct RXFULL_R(crate::FieldReader<u8, u8>);
impl RXFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFULL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:27 - State machine TX FIFO is empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - State machine TX FIFO is full"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - State machine RX FIFO is empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - State machine RX FIFO is full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "FIFO status register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fstat](index.html) module"]
pub struct FSTAT_SPEC;
impl crate::RegisterSpec for FSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fstat::R](R) reader structure"]
impl crate::Readable for FSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSTAT to value 0x0f00_0f00"]
impl crate::Resettable for FSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00_0f00
    }
}
