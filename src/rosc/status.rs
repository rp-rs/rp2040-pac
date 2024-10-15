#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
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
#[doc = "Ring Oscillator Status  

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
