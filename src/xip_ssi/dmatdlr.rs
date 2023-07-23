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
pub type DMATDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMATDL` writer - Transmit data watermark level"]
pub type DMATDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMATDLR_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn dmatdl(&mut self) -> DMATDL_W<0> {
        DMATDL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATDLR to value 0"]
impl crate::Resettable for DMATDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
