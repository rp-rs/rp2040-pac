#[doc = "Reader of register FREQB"]
pub type R = crate::R<u32, super::FREQB>;
#[doc = "Writer for register FREQB"]
pub type W = crate::W<u32, super::FREQB>;
#[doc = "Register FREQB `reset()`'s with value 0"]
impl crate::ResetValue for super::FREQB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set to 0x9696 to apply the settings\\n Any other value in this field will set all drive strengths to 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum PASSWD_A {
    #[doc = "38550: `1001011010010110`"]
    PASS = 38550,
}
impl From<PASSWD_A> for u16 {
    #[inline(always)]
    fn from(variant: PASSWD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PASSWD`"]
pub type PASSWD_R = crate::R<u16, PASSWD_A>;
impl PASSWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, PASSWD_A> {
        use crate::Variant::*;
        match self.bits {
            38550 => Val(PASSWD_A::PASS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == PASSWD_A::PASS
    }
}
#[doc = "Write proxy for field `PASSWD`"]
pub struct PASSWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PASSWD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1001011010010110`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(PASSWD_A::PASS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DS7`"]
pub type DS7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS7`"]
pub struct DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> DS7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `DS6`"]
pub type DS6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS6`"]
pub struct DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> DS6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `DS5`"]
pub type DS5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS5`"]
pub struct DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> DS5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DS4`"]
pub type DS4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS4`"]
pub struct DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> DS4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings\\n Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&self) -> PASSWD_R {
        PASSWD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:14 - Stage 7 drive strength"]
    #[inline(always)]
    pub fn ds7(&self) -> DS7_R {
        DS7_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Stage 6 drive strength"]
    #[inline(always)]
    pub fn ds6(&self) -> DS6_R {
        DS6_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Stage 5 drive strength"]
    #[inline(always)]
    pub fn ds5(&self) -> DS5_R {
        DS5_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Stage 4 drive strength"]
    #[inline(always)]
    pub fn ds4(&self) -> DS4_R {
        DS4_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings\\n Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&mut self) -> PASSWD_W {
        PASSWD_W { w: self }
    }
    #[doc = "Bits 12:14 - Stage 7 drive strength"]
    #[inline(always)]
    pub fn ds7(&mut self) -> DS7_W {
        DS7_W { w: self }
    }
    #[doc = "Bits 8:10 - Stage 6 drive strength"]
    #[inline(always)]
    pub fn ds6(&mut self) -> DS6_W {
        DS6_W { w: self }
    }
    #[doc = "Bits 4:6 - Stage 5 drive strength"]
    #[inline(always)]
    pub fn ds5(&mut self) -> DS5_W {
        DS5_W { w: self }
    }
    #[doc = "Bits 0:2 - Stage 4 drive strength"]
    #[inline(always)]
    pub fn ds4(&mut self) -> DS4_W {
        DS4_W { w: self }
    }
}
