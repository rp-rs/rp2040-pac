#[doc = "Reader of register MPU_RASR"]
pub type R = crate::R<u32, super::MPU_RASR>;
#[doc = "Writer for register MPU_RASR"]
pub type W = crate::W<u32, super::MPU_RASR>;
#[doc = "Register MPU_RASR `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_RASR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATTRS`"]
pub type ATTRS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ATTRS`"]
pub struct ATTRS_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SRD`"]
pub type SRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRD`"]
pub struct SRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 16:31 - The MPU Region Attribute field. Use to define the region attribute control.\\n 28 = XN: Instruction access disable bit:\\n 0 = Instruction fetches enabled.\\n 1 = Instruction fetches disabled.\\n 26:24 = AP: Access permission field\\n 18 = S: Shareable bit\\n 17 = C: Cacheable bit\\n 16 = B: Bufferable bit"]
    #[inline(always)]
    pub fn attrs(&self) -> ATTRS_R {
        ATTRS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    #[inline(always)]
    pub fn srd(&self) -> SRD_R {
        SRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 1:5 - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Enables the region."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - The MPU Region Attribute field. Use to define the region attribute control.\\n 28 = XN: Instruction access disable bit:\\n 0 = Instruction fetches enabled.\\n 1 = Instruction fetches disabled.\\n 26:24 = AP: Access permission field\\n 18 = S: Shareable bit\\n 17 = C: Cacheable bit\\n 16 = B: Bufferable bit"]
    #[inline(always)]
    pub fn attrs(&mut self) -> ATTRS_W {
        ATTRS_W { w: self }
    }
    #[doc = "Bits 8:15 - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    #[inline(always)]
    pub fn srd(&mut self) -> SRD_W {
        SRD_W { w: self }
    }
    #[doc = "Bits 1:5 - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 0 - Enables the region."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
