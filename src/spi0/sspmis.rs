#[doc = "Register `SSPMIS` reader"]
pub struct R(crate::R<SSPMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXMIS` reader - Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
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
#[doc = "Field `RXMIS` reader - Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
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
#[doc = "Field `RTMIS` reader - Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
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
#[doc = "Field `RORMIS` reader - Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
pub struct RORMIS_R(crate::FieldReader<bool, bool>);
impl RORMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RORMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RORMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Masked interrupt status register, SSPMIS on page 3-11  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspmis](index.html) module"]
pub struct SSPMIS_SPEC;
impl crate::RegisterSpec for SSPMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspmis::R](R) reader structure"]
impl crate::Readable for SSPMIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPMIS to value 0"]
impl crate::Resettable for SSPMIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
