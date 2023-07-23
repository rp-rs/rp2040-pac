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
#[doc = "Field `TIME` reader - Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
pub type TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PAUSE_JTAG` reader - Pause the watchdog timer when JTAG is accessing the bus fabric"]
pub type PAUSE_JTAG_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_JTAG` writer - Pause the watchdog timer when JTAG is accessing the bus fabric"]
pub type PAUSE_JTAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PAUSE_DBG0` reader - Pause the watchdog timer when processor 0 is in debug mode"]
pub type PAUSE_DBG0_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_DBG0` writer - Pause the watchdog timer when processor 0 is in debug mode"]
pub type PAUSE_DBG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PAUSE_DBG1` reader - Pause the watchdog timer when processor 1 is in debug mode"]
pub type PAUSE_DBG1_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_DBG1` writer - Pause the watchdog timer when processor 1 is in debug mode"]
pub type PAUSE_DBG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - When not enabled the watchdog timer is paused"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - When not enabled the watchdog timer is paused"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TRIGGER` reader - Trigger a watchdog reset"]
pub type TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TRIGGER` writer - Trigger a watchdog reset"]
pub type TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    pub fn pause_jtag(&self) -> PAUSE_JTAG_R {
        PAUSE_JTAG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn pause_dbg0(&self) -> PAUSE_DBG0_R {
        PAUSE_DBG0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn pause_dbg1(&self) -> PAUSE_DBG1_R {
        PAUSE_DBG1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - When not enabled the watchdog timer is paused"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Trigger a watchdog reset"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    #[must_use]
    pub fn pause_jtag(&mut self) -> PAUSE_JTAG_W<24> {
        PAUSE_JTAG_W::new(self)
    }
    #[doc = "Bit 25 - Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn pause_dbg0(&mut self) -> PAUSE_DBG0_W<25> {
        PAUSE_DBG0_W::new(self)
    }
    #[doc = "Bit 26 - Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn pause_dbg1(&mut self) -> PAUSE_DBG1_W<26> {
        PAUSE_DBG1_W::new(self)
    }
    #[doc = "Bit 30 - When not enabled the watchdog timer is paused"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<30> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - Trigger a watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<31> {
        TRIGGER_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0700_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700_0000;
}
