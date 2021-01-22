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
#[doc = "Reader of field `DIV_RUNNING`"]
pub type DIV_RUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - Oscillator is running and stable"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FRFEQA or FREQB or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&self) -> BADWRITE_R {
        BADWRITE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - post-divider is running\\n this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn div_running(&self) -> DIV_RUNNING_R {
        DIV_RUNNING_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Oscillator is enabled but not necessarily running and stable\\n this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FRFEQA or FREQB or DORMANT"]
    #[inline(always)]
    pub fn badwrite(&mut self) -> BADWRITE_W {
        BADWRITE_W { w: self }
    }
}
