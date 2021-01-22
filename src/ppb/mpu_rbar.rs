#[doc = "Reader of register MPU_RBAR"]
pub type R = crate::R<u32, super::MPU_RBAR>;
#[doc = "Writer for register MPU_RBAR"]
pub type W = crate::W<u32, super::MPU_RBAR>;
#[doc = "Register MPU_RBAR `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_RBAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
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
#[doc = "Reader of field `REGION`"]
pub type REGION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGION`"]
pub struct REGION_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Base address of the region."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 4 - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region.\\n Write:\\n 0 = MPU_RNR not changed, and the processor:\\n Updates the base address for the region specified in the MPU_RNR.\\n Ignores the value of the REGION field.\\n 1 = The processor:\\n Updates the value of the MPU_RNR to the value of the REGION field.\\n Updates the base address for the region specified in the REGION field.\\n Always reads as zero."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - Base address of the region."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 4 - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region.\\n Write:\\n 0 = MPU_RNR not changed, and the processor:\\n Updates the base address for the region specified in the MPU_RNR.\\n Ignores the value of the REGION field.\\n 1 = The processor:\\n Updates the value of the MPU_RNR to the value of the REGION field.\\n Updates the base address for the region specified in the REGION field.\\n Always reads as zero."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
    #[doc = "Bits 0:3 - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
    #[inline(always)]
    pub fn region(&mut self) -> REGION_W {
        REGION_W { w: self }
    }
}
