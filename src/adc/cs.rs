#[doc = "Reader of register CS"]
pub type R = crate::R<u32, super::CS>;
#[doc = "Writer for register CS"]
pub type W = crate::W<u32, super::CS>;
#[doc = "Register CS `reset()`'s with value 0"]
impl crate::ResetValue for super::CS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RROBIN`"]
pub type RROBIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RROBIN`"]
pub struct RROBIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RROBIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `AINSEL`"]
pub type AINSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AINSEL`"]
pub struct AINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `ERR_STICKY`"]
pub type ERR_STICKY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR_STICKY`"]
pub struct ERR_STICKY_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_STICKY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `START_MANY`"]
pub type START_MANY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_MANY`"]
pub struct START_MANY_W<'a> {
    w: &'a mut W,
}
impl<'a> START_MANY_W<'a> {
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
#[doc = "Reader of field `START_ONCE`"]
pub type START_ONCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_ONCE`"]
pub struct START_ONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_ONCE_W<'a> {
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
#[doc = "Reader of field `TS_EN`"]
pub type TS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_EN`"]
pub struct TS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_EN_W<'a> {
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
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bits 16:20 - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.\\n Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.\\n The first channel to be sampled will be the one currently indicated by AINSEL.\\n AINSEL will be updated after each conversion with the newly-selected channel."]
    #[inline(always)]
    pub fn rrobin(&self) -> RROBIN_R {
        RROBIN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Select analog mux input. Updated automatically in round-robin mode."]
    #[inline(always)]
    pub fn ainsel(&self) -> AINSEL_R {
        AINSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Some past ADC conversion encountered an error. Write 1 to clear."]
    #[inline(always)]
    pub fn err_sticky(&self) -> ERR_STICKY_R {
        ERR_STICKY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The most recent ADC conversion encountered an error; result is undefined or noisy."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed.\\n 0 whilst conversion in progress."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    #[inline(always)]
    pub fn start_many(&self) -> START_MANY_R {
        START_MANY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    #[inline(always)]
    pub fn start_once(&self) -> START_ONCE_R {
        START_ONCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power on temperature sensor. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn ts_en(&self) -> TS_EN_R {
        TS_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Power on ADC and enable its clock.\\n 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable.\\n Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion.\\n The first channel to be sampled will be the one currently indicated by AINSEL.\\n AINSEL will be updated after each conversion with the newly-selected channel."]
    #[inline(always)]
    pub fn rrobin(&mut self) -> RROBIN_W {
        RROBIN_W { w: self }
    }
    #[doc = "Bits 12:14 - Select analog mux input. Updated automatically in round-robin mode."]
    #[inline(always)]
    pub fn ainsel(&mut self) -> AINSEL_W {
        AINSEL_W { w: self }
    }
    #[doc = "Bit 10 - Some past ADC conversion encountered an error. Write 1 to clear."]
    #[inline(always)]
    pub fn err_sticky(&mut self) -> ERR_STICKY_W {
        ERR_STICKY_W { w: self }
    }
    #[doc = "Bit 3 - Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    #[inline(always)]
    pub fn start_many(&mut self) -> START_MANY_W {
        START_MANY_W { w: self }
    }
    #[doc = "Bit 2 - Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    #[inline(always)]
    pub fn start_once(&mut self) -> START_ONCE_W {
        START_ONCE_W { w: self }
    }
    #[doc = "Bit 1 - Power on temperature sensor. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn ts_en(&mut self) -> TS_EN_W {
        TS_EN_W { w: self }
    }
    #[doc = "Bit 0 - Power on ADC and enable its clock.\\n 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
