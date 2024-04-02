#[doc = "Register `SYST_RVR` reader"]
pub type R = crate::R<SYST_RVR_SPEC>;
#[doc = "Register `SYST_RVR` writer"]
pub type W = crate::W<SYST_RVR_SPEC>;
#[doc = "Field `RELOAD` reader - Value to load into the SysTick Current Value Register when the counter reaches 0."]
pub type RELOAD_R = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Value to load into the SysTick Current Value Register when the counter reaches 0."]
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<SYST_RVR_SPEC> {
        RELOAD_W::new(self, 0)
    }
}
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN.  
 To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99.  

You can [`read`](crate::Reg::read) this register and get [`syst_rvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_rvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYST_RVR_SPEC;
impl crate::RegisterSpec for SYST_RVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_rvr::R`](R) reader structure"]
impl crate::Readable for SYST_RVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syst_rvr::W`](W) writer structure"]
impl crate::Writable for SYST_RVR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST_RVR to value 0"]
impl crate::Resettable for SYST_RVR_SPEC {
    const RESET_VALUE: u32 = 0;
}
