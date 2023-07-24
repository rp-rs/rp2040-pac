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
pub type TFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFT` writer - Transmit FIFO threshold"]
pub type TFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXFTLR_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn tft(&mut self) -> TFT_W<0> {
        TFT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFTLR to value 0"]
impl crate::Resettable for TXFTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
