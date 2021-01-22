#[doc = "Reader of register BUS_PRIORITY"]
pub type R = crate::R<u32, super::BUS_PRIORITY>;
#[doc = "Writer for register BUS_PRIORITY"]
pub type W = crate::W<u32, super::BUS_PRIORITY>;
#[doc = "Register BUS_PRIORITY `reset()`'s with value 0"]
impl crate::ResetValue for super::BUS_PRIORITY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_W`"]
pub type DMA_W_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_W`"]
pub struct DMA_W_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DMA_R`"]
pub type DMA_R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_R`"]
pub struct DMA_R_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PROC1`"]
pub type PROC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROC1`"]
pub struct PROC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PROC0`"]
pub type PROC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROC0`"]
pub struct PROC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_W<'a> {
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
    #[doc = "Bit 12 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_w(&self) -> DMA_W_R {
        DMA_W_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_r(&self) -> DMA_R_R {
        DMA_R_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_w(&mut self) -> DMA_W_W {
        DMA_W_W { w: self }
    }
    #[doc = "Bit 8 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_r(&mut self) -> DMA_R_W {
        DMA_R_W { w: self }
    }
    #[doc = "Bit 4 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc1(&mut self) -> PROC1_W {
        PROC1_W { w: self }
    }
    #[doc = "Bit 0 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc0(&mut self) -> PROC0_W {
        PROC0_W { w: self }
    }
}
