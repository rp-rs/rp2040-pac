#[doc = "Register `SSPSR` reader"]
pub struct R(crate::R<SSPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BSY` reader - PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub struct BSY_R(crate::FieldReader<bool, bool>);
impl BSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
pub struct RFF_R(crate::FieldReader<bool, bool>);
impl RFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNE` reader - Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
pub struct RNE_R(crate::FieldReader<bool, bool>);
impl RNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TNF` reader - Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
pub struct TNF_R(crate::FieldReader<bool, bool>);
impl TNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
pub struct TFE_R(crate::FieldReader<bool, bool>);
impl TFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status register, SSPSR on page 3-7  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspsr](index.html) module"]
pub struct SSPSR_SPEC;
impl crate::RegisterSpec for SSPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspsr::R](R) reader structure"]
impl crate::Readable for SSPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPSR to value 0x03"]
impl crate::Resettable for SSPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
