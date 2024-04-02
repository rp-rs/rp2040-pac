#[doc = "Register `SYST_CALIB` reader"]
pub type R = crate::R<SYST_CALIB_SPEC>;
#[doc = "Field `TENMS` reader - An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as 0, the calibration value is not known."]
pub type TENMS_R = crate::FieldReader<u32>;
#[doc = "Field `SKEW` reader - If reads as 1, the calibration value for 10ms is inexact (due to clock frequency)."]
pub type SKEW_R = crate::BitReader;
#[doc = "Field `NOREF` reader - If reads as 1, the Reference clock is not provided - the CLKSOURCE bit of the SysTick Control and Status register will be forced to 1 and cannot be cleared to 0."]
pub type NOREF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as 0, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - If reads as 1, the calibration value for 10ms is inexact (due to clock frequency)."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If reads as 1, the Reference clock is not provided - the CLKSOURCE bit of the SysTick Control and Status register will be forced to 1 and cannot be cleared to 0."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply.  

You can [`read`](crate::Reg::read) this register and get [`syst_calib::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYST_CALIB_SPEC;
impl crate::RegisterSpec for SYST_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_calib::R`](R) reader structure"]
impl crate::Readable for SYST_CALIB_SPEC {}
#[doc = "`reset()` method sets SYST_CALIB to value 0"]
impl crate::Resettable for SYST_CALIB_SPEC {
    const RESET_VALUE: u32 = 0;
}
