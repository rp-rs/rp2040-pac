#[doc = "Reader of register GPIO23"]
pub type R = crate::R<u32, super::GPIO23>;
#[doc = "Writer for register GPIO23"]
pub type W = crate::W<u32, super::GPIO23>;
#[doc = "Register GPIO23 `reset()`'s with value 0x56"]
impl crate::ResetValue for super::GPIO23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x56
    }
}
#[doc = "Reader of field `OD`"]
pub type OD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OD`"]
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Drive strength.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_A {
    #[doc = "0: `0`"]
    _2MA = 0,
    #[doc = "1: `1`"]
    _4MA = 1,
    #[doc = "2: `10`"]
    _8MA = 2,
    #[doc = "3: `11`"]
    _12MA = 3,
}
impl From<DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DRIVE`"]
pub type DRIVE_R = crate::R<u8, DRIVE_A>;
impl DRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_A {
        match self.bits {
            0 => DRIVE_A::_2MA,
            1 => DRIVE_A::_4MA,
            2 => DRIVE_A::_8MA,
            3 => DRIVE_A::_12MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2MA`"]
    #[inline(always)]
    pub fn is_2m_a(&self) -> bool {
        *self == DRIVE_A::_2MA
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4m_a(&self) -> bool {
        *self == DRIVE_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8m_a(&self) -> bool {
        *self == DRIVE_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12m_a(&self) -> bool {
        *self == DRIVE_A::_12MA
    }
}
#[doc = "Write proxy for field `DRIVE`"]
pub struct DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _2m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_2MA)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _4m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_4MA)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _8m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_8MA)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _12m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_12MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PUE`"]
pub type PUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUE`"]
pub struct PUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUE_W<'a> {
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
#[doc = "Reader of field `PDE`"]
pub type PDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDE`"]
pub struct PDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDE_W<'a> {
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
#[doc = "Reader of field `SCHMITT`"]
pub type SCHMITT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT`"]
pub struct SCHMITT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT_W<'a> {
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
#[doc = "Reader of field `SLEWFAST`"]
pub type SLEWFAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEWFAST`"]
pub struct SLEWFAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEWFAST_W<'a> {
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
    #[doc = "Bit 7 - Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Input enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Drive strength."]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Pull up enable"]
    #[inline(always)]
    pub fn pue(&self) -> PUE_R {
        PUE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull down enable"]
    #[inline(always)]
    pub fn pde(&self) -> PDE_R {
        PDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    pub fn schmitt(&self) -> SCHMITT_R {
        SCHMITT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Slew rate control. 1 = Fast, 0 = Slow"]
    #[inline(always)]
    pub fn slewfast(&self) -> SLEWFAST_R {
        SLEWFAST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W {
        OD_W { w: self }
    }
    #[doc = "Bit 6 - Input enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bits 4:5 - Drive strength."]
    #[inline(always)]
    pub fn drive(&mut self) -> DRIVE_W {
        DRIVE_W { w: self }
    }
    #[doc = "Bit 3 - Pull up enable"]
    #[inline(always)]
    pub fn pue(&mut self) -> PUE_W {
        PUE_W { w: self }
    }
    #[doc = "Bit 2 - Pull down enable"]
    #[inline(always)]
    pub fn pde(&mut self) -> PDE_W {
        PDE_W { w: self }
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    pub fn schmitt(&mut self) -> SCHMITT_W {
        SCHMITT_W { w: self }
    }
    #[doc = "Bit 0 - Slew rate control. 1 = Fast, 0 = Slow"]
    #[inline(always)]
    pub fn slewfast(&mut self) -> SLEWFAST_W {
        SLEWFAST_W { w: self }
    }
}
