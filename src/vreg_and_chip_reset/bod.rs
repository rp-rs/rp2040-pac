#[doc = "Reader of register BOD"]
pub type R = crate::R<u32, super::BOD>;
#[doc = "Writer for register BOD"]
pub type W = crate::W<u32, super::BOD>;
#[doc = "Register BOD `reset()`'s with value 0x91"]
impl crate::ResetValue for super::BOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x91
    }
}
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
    #[doc = "Bits 4:7 - threshold select\\n 0000 - 0.473V\\n 0001 - 0.516V\\n 0010 - 0.559V\\n 0011 - 0.602V\\n 0100 - 0.645V\\n 0101 - 0.688V\\n 0110 - 0.731V\\n 0111 - 0.774V\\n 1000 - 0.817V\\n 1001 - 0.860V (default)\\n 1010 - 0.903V\\n 1011 - 0.946V\\n 1100 - 0.989V\\n 1101 - 1.032V\\n 1110 - 1.075V\\n 1111 - 1.118V"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - enable\\n 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - threshold select\\n 0000 - 0.473V\\n 0001 - 0.516V\\n 0010 - 0.559V\\n 0011 - 0.602V\\n 0100 - 0.645V\\n 0101 - 0.688V\\n 0110 - 0.731V\\n 0111 - 0.774V\\n 1000 - 0.817V\\n 1001 - 0.860V (default)\\n 1010 - 0.903V\\n 1011 - 0.946V\\n 1100 - 0.989V\\n 1101 - 1.032V\\n 1110 - 1.075V\\n 1111 - 1.118V"]
    #[inline(always)]
    pub fn vsel(&mut self) -> VSEL_W {
        VSEL_W { w: self }
    }
    #[doc = "Bit 0 - enable\\n 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
