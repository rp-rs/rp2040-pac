#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSTIS` reader - Multi-master contention interrupt status"]
pub struct MSTIS_R(crate::FieldReader<bool, bool>);
impl MSTIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIS` reader - Receive FIFO full interrupt status"]
pub struct RXFIS_R(crate::FieldReader<bool, bool>);
impl RXFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOIS` reader - Receive FIFO overflow interrupt status"]
pub struct RXOIS_R(crate::FieldReader<bool, bool>);
impl RXOIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUIS` reader - Receive FIFO underflow interrupt status"]
pub struct RXUIS_R(crate::FieldReader<bool, bool>);
impl RXUIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOIS` reader - Transmit FIFO overflow interrupt status"]
pub struct TXOIS_R(crate::FieldReader<bool, bool>);
impl TXOIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEIS` reader - Transmit FIFO empty interrupt status"]
pub struct TXEIS_R(crate::FieldReader<bool, bool>);
impl TXEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - Multi-master contention interrupt status"]
    #[inline(always)]
    pub fn mstis(&self) -> MSTIS_R {
        MSTIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt status"]
    #[inline(always)]
    pub fn rxfis(&self) -> RXFIS_R {
        RXFIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt status"]
    #[inline(always)]
    pub fn rxois(&self) -> RXOIS_R {
        RXOIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt status"]
    #[inline(always)]
    pub fn rxuis(&self) -> RXUIS_R {
        RXUIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt status"]
    #[inline(always)]
    pub fn txois(&self) -> TXOIS_R {
        TXOIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit FIFO empty interrupt status"]
    #[inline(always)]
    pub fn txeis(&self) -> TXEIS_R {
        TXEIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
