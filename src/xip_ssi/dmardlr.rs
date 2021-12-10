#[doc = "Register `DMARDLR` reader"]
pub struct R(crate::R<DMARDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMARDLR` writer"]
pub struct W(crate::W<DMARDLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMARDLR_SPEC>;
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
impl From<crate::W<DMARDLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMARDLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARDL` reader - Receive data watermark level (DMARDLR+1)"]
pub struct DMARDL_R(crate::FieldReader<u8, u8>);
impl DMARDL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMARDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMARDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMARDL` writer - Receive data watermark level (DMARDLR+1)"]
pub struct DMARDL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive data watermark level (DMARDLR+1)"]
    #[inline(always)]
    pub fn dmardl(&self) -> DMARDL_R {
        DMARDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive data watermark level (DMARDLR+1)"]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DMARDL_W {
        DMARDL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA RX data level  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dmardlr](index.html) module"]
pub struct DMARDLR_SPEC;
impl crate::RegisterSpec for DMARDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmardlr::R](R) reader structure"]
impl crate::Readable for DMARDLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmardlr::W](W) writer structure"]
impl crate::Writable for DMARDLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMARDLR to value 0"]
impl crate::Resettable for DMARDLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
