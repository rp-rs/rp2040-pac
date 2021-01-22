#[doc = "Reader of register FCS"]
pub type R = crate::R<u32, super::FCS>;
#[doc = "Writer for register FCS"]
pub type W = crate::W<u32, super::FCS>;
#[doc = "Register FCS `reset()`'s with value 0"]
impl crate::ResetValue for super::FCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THRESH`"]
pub type THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THRESH`"]
pub struct THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `LEVEL`"]
pub type LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVER`"]
pub type OVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVER`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `UNDER`"]
pub type UNDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDER`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DREQ_EN`"]
pub type DREQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DREQ_EN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SHIFT`"]
pub type SHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHIFT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
