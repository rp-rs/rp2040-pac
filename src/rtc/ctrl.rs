#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORCE_NOTLEAPYEAR`"]
pub type FORCE_NOTLEAPYEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_NOTLEAPYEAR`"]
pub struct FORCE_NOTLEAPYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_NOTLEAPYEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LOAD`"]
pub type LOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOAD`"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
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
#[doc = "Reader of field `RTC_ACTIVE`"]
pub type RTC_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_ENABLE`"]
pub type RTC_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_ENABLE`"]
pub struct RTC_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ENABLE_W<'a> {
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
    #[doc = "Bit 8 - If set, leapyear is forced off.\\n Useful for years divisible by 100 but not by 400"]
    #[inline(always)]
    pub fn force_notleapyear(&self) -> FORCE_NOTLEAPYEAR_R {
        FORCE_NOTLEAPYEAR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Load RTC"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC enabled (running)"]
    #[inline(always)]
    pub fn rtc_active(&self) -> RTC_ACTIVE_R {
        RTC_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable RTC"]
    #[inline(always)]
    pub fn rtc_enable(&self) -> RTC_ENABLE_R {
        RTC_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - If set, leapyear is forced off.\\n Useful for years divisible by 100 but not by 400"]
    #[inline(always)]
    pub fn force_notleapyear(&mut self) -> FORCE_NOTLEAPYEAR_W {
        FORCE_NOTLEAPYEAR_W { w: self }
    }
    #[doc = "Bit 4 - Load RTC"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
    #[doc = "Bit 0 - Enable RTC"]
    #[inline(always)]
    pub fn rtc_enable(&mut self) -> RTC_ENABLE_W {
        RTC_ENABLE_W { w: self }
    }
}
