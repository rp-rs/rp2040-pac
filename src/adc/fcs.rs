#[doc = "Register `FCS` reader"]
pub type R = crate::R<FCS_SPEC>;
#[doc = "Register `FCS` writer"]
pub type W = crate::W<FCS_SPEC>;
#[doc = "Field `EN` reader - If 1: write result to the FIFO after each conversion."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - If 1: write result to the FIFO after each conversion."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHIFT` reader - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
pub type SHIFT_R = crate::BitReader;
#[doc = "Field `SHIFT` writer - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
pub type SHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - If 1: conversion error bit appears in the FIFO alongside the result"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - If 1: conversion error bit appears in the FIFO alongside the result"]
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREQ_EN` reader - If 1: assert DMA requests when FIFO contains data"]
pub type DREQ_EN_R = crate::BitReader;
#[doc = "Field `DREQ_EN` writer - If 1: assert DMA requests when FIFO contains data"]
pub type DREQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - "]
pub type EMPTY_R = crate::BitReader;
#[doc = "Field `FULL` reader - "]
pub type FULL_R = crate::BitReader;
#[doc = "Field `UNDER` reader - 1 if the FIFO has been underflowed. Write 1 to clear."]
pub type UNDER_R = crate::BitReader;
#[doc = "Field `UNDER` writer - 1 if the FIFO has been underflowed. Write 1 to clear."]
pub type UNDER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVER` reader - 1 if the FIFO has been overflowed. Write 1 to clear."]
pub type OVER_R = crate::BitReader;
#[doc = "Field `OVER` writer - 1 if the FIFO has been overflowed. Write 1 to clear."]
pub type OVER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LEVEL` reader - The number of conversion results currently waiting in the FIFO"]
pub type LEVEL_R = crate::FieldReader;
#[doc = "Field `THRESH` reader - DREQ/IRQ asserted when level >= threshold"]
pub type THRESH_R = crate::FieldReader;
#[doc = "Field `THRESH` writer - DREQ/IRQ asserted when level >= threshold"]
pub type THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    pub fn en(&mut self) -> EN_W<FCS_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    #[inline(always)]
    #[must_use]
    pub fn shift(&mut self) -> SHIFT_W<FCS_SPEC> {
        SHIFT_W::new(self, 1)
    }
    #[doc = "Bit 2 - If 1: conversion error bit appears in the FIFO alongside the result"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<FCS_SPEC> {
        ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - If 1: assert DMA requests when FIFO contains data"]
    #[inline(always)]
    #[must_use]
    pub fn dreq_en(&mut self) -> DREQ_EN_W<FCS_SPEC> {
        DREQ_EN_W::new(self, 3)
    }
    #[doc = "Bit 10 - 1 if the FIFO has been underflowed. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn under(&mut self) -> UNDER_W<FCS_SPEC> {
        UNDER_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1 if the FIFO has been overflowed. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn over(&mut self) -> OVER_W<FCS_SPEC> {
        OVER_W::new(self, 11)
    }
    #[doc = "Bits 24:27 - DREQ/IRQ asserted when level >= threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thresh(&mut self) -> THRESH_W<FCS_SPEC> {
        THRESH_W::new(self, 24)
    }
}
#[doc = "FIFO control and status  

You can [`read`](crate::Reg::read) this register and get [`fcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCS_SPEC;
impl crate::RegisterSpec for FCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcs::R`](R) reader structure"]
impl crate::Readable for FCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcs::W`](W) writer structure"]
impl crate::Writable for FCS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c00;
}
#[doc = "`reset()` method sets FCS to value 0"]
impl crate::Resettable for FCS_SPEC {
    const RESET_VALUE: u32 = 0;
}
