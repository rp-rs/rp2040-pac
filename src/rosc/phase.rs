#[doc = "Reader of register PHASE"]
pub type R = crate::R<u32, super::PHASE>;
#[doc = "Writer for register PHASE"]
pub type W = crate::W<u32, super::PHASE>;
#[doc = "Register PHASE `reset()`'s with value 0x08"]
impl crate::ResetValue for super::PHASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `PASSWD`"]
pub type PASSWD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PASSWD`"]
pub struct PASSWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FLIP`"]
pub type FLIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLIP`"]
pub struct FLIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIP_W<'a> {
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
pub type SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHIFT`"]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:11 - set to 0xaa0\\n any other value enables the output with shift=0"]
    #[inline(always)]
    pub fn passwd(&self) -> PASSWD_R {
        PASSWD_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 3 - enable the phase-shifted output\\n this can be changed on-the-fly"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - invert the phase-shifted output\\n this is ignored when div=1"]
    #[inline(always)]
    pub fn flip(&self) -> FLIP_R {
        FLIP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - phase shift the phase-shifted output by SHIFT input clocks\\n this can be changed on-the-fly\\n must be set to 0 before setting div=1"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:11 - set to 0xaa0\\n any other value enables the output with shift=0"]
    #[inline(always)]
    pub fn passwd(&mut self) -> PASSWD_W {
        PASSWD_W { w: self }
    }
    #[doc = "Bit 3 - enable the phase-shifted output\\n this can be changed on-the-fly"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - invert the phase-shifted output\\n this is ignored when div=1"]
    #[inline(always)]
    pub fn flip(&mut self) -> FLIP_W {
        FLIP_W { w: self }
    }
    #[doc = "Bits 0:1 - phase shift the phase-shifted output by SHIFT input clocks\\n this can be changed on-the-fly\\n must be set to 0 before setting div=1"]
    #[inline(always)]
    pub fn shift(&mut self) -> SHIFT_W {
        SHIFT_W { w: self }
    }
}
