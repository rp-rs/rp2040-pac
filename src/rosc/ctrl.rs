#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0aa0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0aa0
    }
}
#[doc = "On power-up this field is initialised to ENABLE\\n The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up\\n The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ENABLE_A {
    #[doc = "3358: `110100011110`"]
    DISABLE = 3358,
    #[doc = "4011: `111110101011`"]
    ENABLE = 4011,
}
impl From<ENABLE_A> for u16 {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<u16, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ENABLE_A> {
        use crate::Variant::*;
        match self.bits {
            3358 => Val(ENABLE_A::DISABLE),
            4011 => Val(ENABLE_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`110100011110`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "`111110101011`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | (((value as u32) & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Controls the number of delay stages in the ROSC ring\\n LOW uses stages 0 to 7\\n MEDIUM uses stages 0 to 5\\n HIGH uses stages 0 to 3\\n TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications\\n The clock output will not glitch when changing the range up one step at a time\\n The clock output will glitch when changing the range down\\n Note: the values here are gray coded which is why HIGH comes before TOOHIGH\n\nValue on reset: 2720"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FREQ_RANGE_A {
    #[doc = "4004: `111110100100`"]
    LOW = 4004,
    #[doc = "4005: `111110100101`"]
    MEDIUM = 4005,
    #[doc = "4007: `111110100111`"]
    HIGH = 4007,
    #[doc = "4006: `111110100110`"]
    TOOHIGH = 4006,
}
impl From<FREQ_RANGE_A> for u16 {
    #[inline(always)]
    fn from(variant: FREQ_RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FREQ_RANGE`"]
pub type FREQ_RANGE_R = crate::R<u16, FREQ_RANGE_A>;
impl FREQ_RANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FREQ_RANGE_A> {
        use crate::Variant::*;
        match self.bits {
            4004 => Val(FREQ_RANGE_A::LOW),
            4005 => Val(FREQ_RANGE_A::MEDIUM),
            4007 => Val(FREQ_RANGE_A::HIGH),
            4006 => Val(FREQ_RANGE_A::TOOHIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FREQ_RANGE_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == FREQ_RANGE_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FREQ_RANGE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOOHIGH`"]
    #[inline(always)]
    pub fn is_toohigh(&self) -> bool {
        *self == FREQ_RANGE_A::TOOHIGH
    }
}
#[doc = "Write proxy for field `FREQ_RANGE`"]
pub struct FREQ_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQ_RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`111110100100`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::LOW)
    }
    #[doc = "`111110100101`"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::MEDIUM)
    }
    #[doc = "`111110100111`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::HIGH)
    }
    #[doc = "`111110100110`"]
    #[inline(always)]
    pub fn toohigh(self) -> &'a mut W {
        self.variant(FREQ_RANGE_A::TOOHIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:23 - On power-up this field is initialised to ENABLE\\n The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up\\n The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Controls the number of delay stages in the ROSC ring\\n LOW uses stages 0 to 7\\n MEDIUM uses stages 0 to 5\\n HIGH uses stages 0 to 3\\n TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications\\n The clock output will not glitch when changing the range up one step at a time\\n The clock output will glitch when changing the range down\\n Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    pub fn freq_range(&self) -> FREQ_RANGE_R {
        FREQ_RANGE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:23 - On power-up this field is initialised to ENABLE\\n The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up\\n The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:11 - Controls the number of delay stages in the ROSC ring\\n LOW uses stages 0 to 7\\n MEDIUM uses stages 0 to 5\\n HIGH uses stages 0 to 3\\n TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications\\n The clock output will not glitch when changing the range up one step at a time\\n The clock output will glitch when changing the range down\\n Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    pub fn freq_range(&mut self) -> FREQ_RANGE_W {
        FREQ_RANGE_W { w: self }
    }
}
