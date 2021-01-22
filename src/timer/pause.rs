#[doc = "Reader of register PAUSE"]
pub type R = crate::R<u32, super::PAUSE>;
#[doc = "Writer for register PAUSE"]
pub type W = crate::W<u32, super::PAUSE>;
#[doc = "Register PAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::PAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAUSE`"]
pub type PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAUSE`"]
pub struct PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pause(&mut self) -> PAUSE_W {
        PAUSE_W { w: self }
    }
}
