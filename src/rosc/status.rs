#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `ENABLED` reader - Oscillator is enabled but not necessarily running and stable this resets to 0 but transitions to 1 during chip startup"]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `DIV_RUNNING` reader - post-divider is running this resets to 0 but transitions to 1 during chip startup"]
pub type DIV_RUNNING_R = crate::BitReader;
#[doc = "Field `STABLE` reader - Oscillator is running and stable"]
pub type STABLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 12 - Oscillator is enabled but not necessarily running and stable this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - post-divider is running this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn div_running(&self) -> DIV_RUNNING_R {
        DIV_RUNNING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Oscillator is running and stable"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Ring Oscillator Status  

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
