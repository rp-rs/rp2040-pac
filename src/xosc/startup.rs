#[doc = "Reader of register STARTUP"]
pub type R = crate::R<u32, super::STARTUP>;
#[doc = "Writer for register STARTUP"]
pub type W = crate::W<u32, super::STARTUP>;
#[doc = "Register STARTUP `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X4`"]
pub type X4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X4`"]
pub struct X4_W<'a> {
    w: &'a mut W,
}
impl<'a> X4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DELAY`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly"]
    #[inline(always)]
    pub fn x4(&self) -> X4_R {
        X4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:13 - in multiples of 256*xtal_period"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 20 - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly"]
    #[inline(always)]
    pub fn x4(&mut self) -> X4_W {
        X4_W { w: self }
    }
    #[doc = "Bits 0:13 - in multiples of 256*xtal_period"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
}
