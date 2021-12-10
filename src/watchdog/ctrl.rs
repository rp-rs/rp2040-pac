#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` reader - Trigger a watchdog reset"]
pub struct TRIGGER_R(crate::FieldReader<bool, bool>);
impl TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER` writer - Trigger a watchdog reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - When not enabled the watchdog timer is paused"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - When not enabled the watchdog timer is paused"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PAUSE_DBG1` reader - Pause the watchdog timer when processor 1 is in debug mode"]
pub struct PAUSE_DBG1_R(crate::FieldReader<bool, bool>);
impl PAUSE_DBG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAUSE_DBG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSE_DBG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUSE_DBG1` writer - Pause the watchdog timer when processor 1 is in debug mode"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PAUSE_DBG0` reader - Pause the watchdog timer when processor 0 is in debug mode"]
pub struct PAUSE_DBG0_R(crate::FieldReader<bool, bool>);
impl PAUSE_DBG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAUSE_DBG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSE_DBG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUSE_DBG0` writer - Pause the watchdog timer when processor 0 is in debug mode"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PAUSE_JTAG` reader - Pause the watchdog timer when JTAG is accessing the bus fabric"]
pub struct PAUSE_JTAG_R(crate::FieldReader<bool, bool>);
impl PAUSE_JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAUSE_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSE_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUSE_JTAG` writer - Pause the watchdog timer when JTAG is accessing the bus fabric"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `TIME` reader - Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
pub struct TIME_R(crate::FieldReader<u32, u32>);
impl TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog control  
 The rst_wdsel register determines which subsystems are reset when the watchdog is triggered.  
 The watchdog can be triggered in software.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0700_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700_0000
    }
}
