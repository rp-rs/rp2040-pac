#[doc = "Register `TXFTLR` reader"]
pub struct R(crate::R<TXFTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFTLR` writer"]
pub struct W(crate::W<TXFTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TXFTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFT` reader - Transmit FIFO threshold"]
pub struct TFT_R(crate::FieldReader<u8, u8>);
impl TFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFT` writer - Transmit FIFO threshold"]
pub struct TFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO threshold"]
    #[inline(always)]
    pub fn tft(&self) -> TFT_R {
        TFT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit FIFO threshold"]
    #[inline(always)]
    pub fn tft(&mut self) -> TFT_W {
        TFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO threshold level  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [txftlr](index.html) module"]
pub struct TXFTLR_SPEC;
impl crate::RegisterSpec for TXFTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txftlr::R](R) reader structure"]
impl crate::Readable for TXFTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txftlr::W](W) writer structure"]
impl crate::Writable for TXFTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFTLR to value 0"]
impl crate::Resettable for TXFTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
