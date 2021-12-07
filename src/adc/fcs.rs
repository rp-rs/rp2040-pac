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
#[doc = "Field `THRESH` reader - DREQ/IRQ asserted when level >= threshold"]
pub struct THRESH_R(crate::FieldReader<u8, u8>);
impl THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRESH` writer - DREQ/IRQ asserted when level >= threshold"]
pub struct THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `LEVEL` reader - The number of conversion results currently waiting in the FIFO"]
pub struct LEVEL_R(crate::FieldReader<u8, u8>);
impl LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER` reader - 1 if the FIFO has been overflowed. Write 1 to clear."]
pub struct OVER_R(crate::FieldReader<bool, bool>);
impl OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER` writer - 1 if the FIFO has been overflowed. Write 1 to clear."]
pub struct OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `UNDER` reader - 1 if the FIFO has been underflowed. Write 1 to clear."]
pub struct UNDER_R(crate::FieldReader<bool, bool>);
impl UNDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDER` writer - 1 if the FIFO has been underflowed. Write 1 to clear."]
pub struct UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FULL` reader - "]
pub struct FULL_R(crate::FieldReader<bool, bool>);
impl FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPTY` reader - "]
pub struct EMPTY_R(crate::FieldReader<bool, bool>);
impl EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREQ_EN` reader - If 1: assert DMA requests when FIFO contains data"]
pub struct DREQ_EN_R(crate::FieldReader<bool, bool>);
impl DREQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DREQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREQ_EN` writer - If 1: assert DMA requests when FIFO contains data"]
pub struct DREQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DREQ_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ERR` reader - If 1: conversion error bit appears in the FIFO alongside the result"]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` writer - If 1: conversion error bit appears in the FIFO alongside the result"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SHIFT` reader - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
pub struct SHIFT_R(crate::FieldReader<bool, bool>);
impl SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT` writer - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - If 1: write result to the FIFO after each conversion."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - If 1: write result to the FIFO after each conversion."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - DREQ/IRQ asserted when level >= threshold"]
    #[inline(always)]
    pub fn thresh(&self) -> THRESH_R {
        THRESH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The number of conversion results currently waiting in the FIFO"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 1 if the FIFO has been overflowed. Write 1 to clear."]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 1 if the FIFO has been underflowed. Write 1 to clear."]
    #[inline(always)]
    pub fn under(&self) -> UNDER_R {
        UNDER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If 1: assert DMA requests when FIFO contains data"]
    #[inline(always)]
    pub fn dreq_en(&self) -> DREQ_EN_R {
        DREQ_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If 1: conversion error bit appears in the FIFO alongside the result"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - If 1: write result to the FIFO after each conversion."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - DREQ/IRQ asserted when level >= threshold"]
    #[inline(always)]
    pub fn thresh(&mut self) -> THRESH_W {
        THRESH_W { w: self }
    }
    #[doc = "Bit 11 - 1 if the FIFO has been overflowed. Write 1 to clear."]
    #[inline(always)]
    pub fn over(&mut self) -> OVER_W {
        OVER_W { w: self }
    }
    #[doc = "Bit 10 - 1 if the FIFO has been underflowed. Write 1 to clear."]
    #[inline(always)]
    pub fn under(&mut self) -> UNDER_W {
        UNDER_W { w: self }
    }
    #[doc = "Bit 3 - If 1: assert DMA requests when FIFO contains data"]
    #[inline(always)]
    pub fn dreq_en(&mut self) -> DREQ_EN_W {
        DREQ_EN_W { w: self }
    }
    #[doc = "Bit 2 - If 1: conversion error bit appears in the FIFO alongside the result"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 1 - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    #[inline(always)]
    pub fn shift(&mut self) -> SHIFT_W {
        SHIFT_W { w: self }
    }
    #[doc = "Bit 0 - If 1: write result to the FIFO after each conversion."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
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
}
#[doc = "`reset()` method sets FCS to value 0"]
impl crate::Resettable for FCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
