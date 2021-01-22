#[doc = "Reader of register FREQA"]
pub type R = crate::R<u32, super::FREQA>;
#[doc = "Writer for register FREQA"]
pub type W = crate::W<u32, super::FREQA>;
#[doc = "Register FREQA `reset()`'s with value 0"]
impl crate::ResetValue for super::FREQA {
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
#[doc = "Reader of field `DS3`"]
pub type DS3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS3`"]
pub struct DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> DS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `DS2`"]
pub type DS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS2`"]
pub struct DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> DS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `DS1`"]
pub type DS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS1`"]
pub struct DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> DS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DS0`"]
pub type DS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS0`"]
pub struct DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> DS0_W<'a> {
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
    #[doc = "Bits 12:14 - Stage 3 drive strength"]
    #[inline(always)]
    pub fn ds3(&self) -> DS3_R {
        DS3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Stage 2 drive strength"]
    #[inline(always)]
    pub fn ds2(&self) -> DS2_R {
        DS2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Stage 1 drive strength"]
    #[inline(always)]
    pub fn ds1(&self) -> DS1_R {
        DS1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Stage 0 drive strength"]
    #[inline(always)]
    pub fn ds0(&self) -> DS0_R {
        DS0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings\\n Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&mut self) -> PASSWD_W {
        PASSWD_W { w: self }
    }
    #[doc = "Bits 12:14 - Stage 3 drive strength"]
    #[inline(always)]
    pub fn ds3(&mut self) -> DS3_W {
        DS3_W { w: self }
    }
    #[doc = "Bits 8:10 - Stage 2 drive strength"]
    #[inline(always)]
    pub fn ds2(&mut self) -> DS2_W {
        DS2_W { w: self }
    }
    #[doc = "Bits 4:6 - Stage 1 drive strength"]
    #[inline(always)]
    pub fn ds1(&mut self) -> DS1_W {
        DS1_W { w: self }
    }
    #[doc = "Bits 0:2 - Stage 0 drive strength"]
    #[inline(always)]
    pub fn ds0(&mut self) -> DS0_W {
        DS0_W { w: self }
    }
}
