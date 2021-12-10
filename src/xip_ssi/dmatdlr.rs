#[doc = "Register `DMATDLR` reader"]
pub struct R(crate::R<DMATDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATDLR` writer"]
pub struct W(crate::W<DMATDLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATDLR_SPEC>;
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
impl From<crate::W<DMATDLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATDLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMATDL` reader - Transmit data watermark level"]
pub struct DMATDL_R(crate::FieldReader<u8, u8>);
impl DMATDL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMATDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMATDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMATDL` writer - Transmit data watermark level"]
pub struct DMATDL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit data watermark level"]
    #[inline(always)]
    pub fn dmatdl(&self) -> DMATDL_R {
        DMATDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data watermark level"]
    #[inline(always)]
    pub fn dmatdl(&mut self) -> DMATDL_W {
        DMATDL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA TX data level  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dmatdlr](index.html) module"]
pub struct DMATDLR_SPEC;
impl crate::RegisterSpec for DMATDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatdlr::R](R) reader structure"]
impl crate::Readable for DMATDLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatdlr::W](W) writer structure"]
impl crate::Writable for DMATDLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATDLR to value 0"]
impl crate::Resettable for DMATDLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
