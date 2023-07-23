#[doc = "Register `FCS` reader"]
pub struct R(crate::R<FCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCS` writer"]
pub struct W(crate::W<FCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCS_SPEC>;
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
impl From<crate::W<FCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - If 1: write result to the FIFO after each conversion."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - If 1: write result to the FIFO after each conversion."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCS_SPEC, bool, O>;
#[doc = "Field `SHIFT` reader - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
pub type SHIFT_R = crate::BitReader<bool>;
#[doc = "Field `SHIFT` writer - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
pub type SHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCS_SPEC, bool, O>;
#[doc = "Field `ERR` reader - If 1: conversion error bit appears in the FIFO alongside the result"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - If 1: conversion error bit appears in the FIFO alongside the result"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCS_SPEC, bool, O>;
#[doc = "Field `DREQ_EN` reader - If 1: assert DMA requests when FIFO contains data"]
pub type DREQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `DREQ_EN` writer - If 1: assert DMA requests when FIFO contains data"]
pub type DREQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCS_SPEC, bool, O>;
#[doc = "Field `EMPTY` reader - "]
pub type EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FULL` reader - "]
pub type FULL_R = crate::BitReader<bool>;
#[doc = "Field `UNDER` reader - 1 if the FIFO has been underflowed. Write 1 to clear."]
pub type UNDER_R = crate::BitReader<bool>;
#[doc = "Field `UNDER` writer - 1 if the FIFO has been underflowed. Write 1 to clear."]
pub type UNDER_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FCS_SPEC, bool, O>;
#[doc = "Field `OVER` reader - 1 if the FIFO has been overflowed. Write 1 to clear."]
pub type OVER_R = crate::BitReader<bool>;
#[doc = "Field `OVER` writer - 1 if the FIFO has been overflowed. Write 1 to clear."]
pub type OVER_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FCS_SPEC, bool, O>;
#[doc = "Field `LEVEL` reader - The number of conversion results currently waiting in the FIFO"]
pub type LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESH` reader - DREQ/IRQ asserted when level >= threshold"]
pub type THRESH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESH` writer - DREQ/IRQ asserted when level >= threshold"]
pub type THRESH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - If 1: write result to the FIFO after each conversion."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1: conversion error bit appears in the FIFO alongside the result"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1: assert DMA requests when FIFO contains data"]
    #[inline(always)]
    pub fn dreq_en(&self) -> DREQ_EN_R {
        DREQ_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1 if the FIFO has been underflowed. Write 1 to clear."]
    #[inline(always)]
    pub fn under(&self) -> UNDER_R {
        UNDER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1 if the FIFO has been overflowed. Write 1 to clear."]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - The number of conversion results currently waiting in the FIFO"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DREQ/IRQ asserted when level >= threshold"]
    #[inline(always)]
    pub fn thresh(&self) -> THRESH_R {
        THRESH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If 1: write result to the FIFO after each conversion."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    #[inline(always)]
    #[must_use]
    pub fn shift(&mut self) -> SHIFT_W<1> {
        SHIFT_W::new(self)
    }
    #[doc = "Bit 2 - If 1: conversion error bit appears in the FIFO alongside the result"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<2> {
        ERR_W::new(self)
    }
    #[doc = "Bit 3 - If 1: assert DMA requests when FIFO contains data"]
    #[inline(always)]
    #[must_use]
    pub fn dreq_en(&mut self) -> DREQ_EN_W<3> {
        DREQ_EN_W::new(self)
    }
    #[doc = "Bit 10 - 1 if the FIFO has been underflowed. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn under(&mut self) -> UNDER_W<10> {
        UNDER_W::new(self)
    }
    #[doc = "Bit 11 - 1 if the FIFO has been overflowed. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn over(&mut self) -> OVER_W<11> {
        OVER_W::new(self)
    }
    #[doc = "Bits 24:27 - DREQ/IRQ asserted when level >= threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thresh(&mut self) -> THRESH_W<24> {
        THRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO control and status  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fcs](index.html) module"]
pub struct FCS_SPEC;
impl crate::RegisterSpec for FCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcs::R](R) reader structure"]
impl crate::Readable for FCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcs::W](W) writer structure"]
impl crate::Writable for FCS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0c00;
}
#[doc = "`reset()` method sets FCS to value 0"]
impl crate::Resettable for FCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
