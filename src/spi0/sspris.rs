#[doc = "Register `SSPRIS` reader"]
pub struct R(crate::R<SSPRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXRIS` reader - Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
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
#[doc = "Field `RXRIS` reader - Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
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
#[doc = "Field `RTRIS` reader - Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
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
#[doc = "Field `RORRIS` reader - Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
pub struct RORRIS_R(crate::FieldReader<bool, bool>);
impl RORRIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RORRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RORRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn rorris(&self) -> RORRIS_R {
        RORRIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Raw interrupt status register, SSPRIS on page 3-10  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspris](index.html) module"]
pub struct SSPRIS_SPEC;
impl crate::RegisterSpec for SSPRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspris::R](R) reader structure"]
impl crate::Readable for SSPRIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPRIS to value 0x08"]
impl crate::Resettable for SSPRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
