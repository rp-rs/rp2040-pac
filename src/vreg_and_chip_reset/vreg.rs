#[doc = "Reader of register VREG"]
pub type R = crate::R<u32, super::VREG>;
#[doc = "Writer for register VREG"]
pub type W = crate::W<u32, super::VREG>;
#[doc = "Register VREG `reset()`'s with value 0xb1"]
impl crate::ResetValue for super::VREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb1
    }
}
#[doc = "Reader of field `ROK`"]
pub type ROK_R = crate::R<bool, bool>;
#[doc = "Reader of field `VSEL`"]
pub type VSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSEL`"]
pub struct VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `HIZ`"]
pub type HIZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIZ`"]
pub struct HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIZ_W<'a> {
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
    #[doc = "Bit 12 - regulation status  
 0=not in regulation, 1=in regulation"]
    #[inline(always)]
    pub fn rok(&self) -> ROK_R {
        ROK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
    #[inline(always)]
    pub fn vsel(&mut self) -> VSEL_W {
        VSEL_W { w: self }
    }
    #[doc = "Bit 1 - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W { w: self }
    }
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
