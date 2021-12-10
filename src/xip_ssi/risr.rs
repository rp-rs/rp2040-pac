#[doc = "Register `RISR` reader"]
pub struct R(crate::R<RISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSTIR` reader - Multi-master contention raw interrupt status"]
pub struct MSTIR_R(crate::FieldReader<bool, bool>);
impl MSTIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIR` reader - Receive FIFO full raw interrupt status"]
pub struct RXFIR_R(crate::FieldReader<bool, bool>);
impl RXFIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOIR` reader - Receive FIFO overflow raw interrupt status"]
pub struct RXOIR_R(crate::FieldReader<bool, bool>);
impl RXOIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUIR` reader - Receive FIFO underflow raw interrupt status"]
pub struct RXUIR_R(crate::FieldReader<bool, bool>);
impl RXUIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOIR` reader - Transmit FIFO overflow raw interrupt status"]
pub struct TXOIR_R(crate::FieldReader<bool, bool>);
impl TXOIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEIR` reader - Transmit FIFO empty raw interrupt status"]
pub struct TXEIR_R(crate::FieldReader<bool, bool>);
impl TXEIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - Multi-master contention raw interrupt status"]
    #[inline(always)]
    pub fn mstir(&self) -> MSTIR_R {
        MSTIR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub fn rxfir(&self) -> RXFIR_R {
        RXFIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub fn rxoir(&self) -> RXOIR_R {
        RXOIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow raw interrupt status"]
    #[inline(always)]
    pub fn rxuir(&self) -> RXUIR_R {
        RXUIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub fn txoir(&self) -> TXOIR_R {
        TXOIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub fn txeir(&self) -> TXEIR_R {
        TXEIR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Raw interrupt status  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [risr](index.html) module"]
pub struct RISR_SPEC;
impl crate::RegisterSpec for RISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [risr::R](R) reader structure"]
impl crate::Readable for RISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RISR to value 0"]
impl crate::Resettable for RISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
