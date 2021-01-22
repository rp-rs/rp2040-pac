#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0700_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0700_0000
    }
}
#[doc = "Reader of field `TRIGGER`"]
pub type TRIGGER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIGGER`"]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PAUSE_DBG1`"]
pub type PAUSE_DBG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAUSE_DBG1`"]
pub struct PAUSE_DBG1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_DBG1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PAUSE_DBG0`"]
pub type PAUSE_DBG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAUSE_DBG0`"]
pub struct PAUSE_DBG0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_DBG0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PAUSE_JTAG`"]
pub type PAUSE_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAUSE_JTAG`"]
pub struct PAUSE_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_JTAG_W<'a> {
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
#[doc = "Reader of field `TIME`"]
pub type TIME_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 31 - Trigger a watchdog reset"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - When not enabled the watchdog timer is paused"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn pause_dbg1(&self) -> PAUSE_DBG1_R {
        PAUSE_DBG1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn pause_dbg0(&self) -> PAUSE_DBG0_R {
        PAUSE_DBG0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    pub fn pause_jtag(&self) -> PAUSE_JTAG_R {
        PAUSE_JTAG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - Trigger a watchdog reset"]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    #[doc = "Bit 30 - When not enabled the watchdog timer is paused"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 26 - Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn pause_dbg1(&mut self) -> PAUSE_DBG1_W {
        PAUSE_DBG1_W { w: self }
    }
    #[doc = "Bit 25 - Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn pause_dbg0(&mut self) -> PAUSE_DBG0_W {
        PAUSE_DBG0_W { w: self }
    }
    #[doc = "Bit 24 - Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    pub fn pause_jtag(&mut self) -> PAUSE_JTAG_W {
        PAUSE_JTAG_W { w: self }
    }
}
