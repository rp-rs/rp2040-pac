#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STABLE`"]
pub type STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BADWRITE`"]
pub type BADWRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BADWRITE`"]
pub struct BADWRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> BADWRITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "The current frequency range setting, always reads 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FREQ_RANGE_A {
    #[doc = "0: `0`"]
    _1_15MHZ = 0,
    #[doc = "1: `1`"]
    RESERVED_1 = 1,
    #[doc = "2: `10`"]
    RESERVED_2 = 2,
    #[doc = "3: `11`"]
    RESERVED_3 = 3,
}
impl From<FREQ_RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQ_RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FREQ_RANGE`"]
pub type FREQ_RANGE_R = crate::R<u8, FREQ_RANGE_A>;
impl FREQ_RANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQ_RANGE_A {
        match self.bits {
            0 => FREQ_RANGE_A::_1_15MHZ,
            1 => FREQ_RANGE_A::RESERVED_1,
            2 => FREQ_RANGE_A::RESERVED_2,
            3 => FREQ_RANGE_A::RESERVED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_15MHZ`"]
    #[inline(always)]
    pub fn is_1_15mhz(&self) -> bool {
        *self == FREQ_RANGE_A::_1_15MHZ
    }
    #[doc = "Checks if the value of the field is `RESERVED_1`"]
    #[inline(always)]
    pub fn is_reserved_1(&self) -> bool {
        *self == FREQ_RANGE_A::RESERVED_1
    }
    #[doc = "Checks if the value of the field is `RESERVED_2`"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        *self == FREQ_RANGE_A::RESERVED_2
    }
    #[doc = "Checks if the value of the field is `RESERVED_3`"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        *self == FREQ_RANGE_A::RESERVED_3
    }
}
impl R {
    #[doc = "Bit 31 - Oscillator is running and stable"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&self) -> BADWRITE_R {
        BADWRITE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Oscillator is enabled but not necessarily running and stable, resets to 0"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - The current frequency range setting, always reads 0"]
    #[inline(always)]
    pub fn freq_range(&self) -> FREQ_RANGE_R {
        FREQ_RANGE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&mut self) -> BADWRITE_W {
        BADWRITE_W { w: self }
    }
}
