#[doc = "Register `RXUICR` reader"]
pub struct R(crate::R<RXUICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXUICR` reader - Clear-on-read receive FIFO underflow interrupt"]
pub struct RXUICR_R(crate::FieldReader<bool, bool>);
impl RXUICR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUICR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUICR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Clear-on-read receive FIFO underflow interrupt"]
    #[inline(always)]
    pub fn rxuicr(&self) -> RXUICR_R {
        RXUICR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RX FIFO underflow interrupt clear  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [rxuicr](index.html) module"]
pub struct RXUICR_SPEC;
impl crate::RegisterSpec for RXUICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxuicr::R](R) reader structure"]
impl crate::Readable for RXUICR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXUICR to value 0"]
impl crate::Resettable for RXUICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
