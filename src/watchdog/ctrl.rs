#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `TIME` reader - Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
pub type TIME_R = crate::FieldReader<u32>;
#[doc = "Field `PAUSE_JTAG` reader - Pause the watchdog timer when JTAG is accessing the bus fabric"]
pub type PAUSE_JTAG_R = crate::BitReader;
#[doc = "Field `PAUSE_JTAG` writer - Pause the watchdog timer when JTAG is accessing the bus fabric"]
pub type PAUSE_JTAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSE_DBG0` reader - Pause the watchdog timer when processor 0 is in debug mode"]
pub type PAUSE_DBG0_R = crate::BitReader;
#[doc = "Field `PAUSE_DBG0` writer - Pause the watchdog timer when processor 0 is in debug mode"]
pub type PAUSE_DBG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSE_DBG1` reader - Pause the watchdog timer when processor 1 is in debug mode"]
pub type PAUSE_DBG1_R = crate::BitReader;
#[doc = "Field `PAUSE_DBG1` writer - Pause the watchdog timer when processor 1 is in debug mode"]
pub type PAUSE_DBG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - When not enabled the watchdog timer is paused"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - When not enabled the watchdog timer is paused"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGGER` writer - Trigger a watchdog reset"]
pub type TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl W {
    #[doc = "Bit 24 - Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    #[must_use]
    pub fn pause_jtag(&mut self) -> PAUSE_JTAG_W<CTRL_SPEC> {
        PAUSE_JTAG_W::new(self, 24)
    }
    #[doc = "Bit 25 - Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn pause_dbg0(&mut self) -> PAUSE_DBG0_W<CTRL_SPEC> {
        PAUSE_DBG0_W::new(self, 25)
    }
    #[doc = "Bit 26 - Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn pause_dbg1(&mut self) -> PAUSE_DBG1_W<CTRL_SPEC> {
        PAUSE_DBG1_W::new(self, 26)
    }
    #[doc = "Bit 30 - When not enabled the watchdog timer is paused"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRL_SPEC> {
        ENABLE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Trigger a watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<CTRL_SPEC> {
        TRIGGER_W::new(self, 31)
    }
}
#[doc = "Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software.  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0700_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0700_0000;
}
