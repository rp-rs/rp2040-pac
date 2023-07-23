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
pub type DMARDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMARDL` writer - Receive data watermark level (DMARDLR+1)"]
pub type DMARDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMARDLR_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn dmardl(&mut self) -> DMARDL_W<0> {
        DMARDL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMARDLR to value 0"]
impl crate::Resettable for DMARDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
