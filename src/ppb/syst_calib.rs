#[doc = "Register `SYST_CALIB` reader"]
pub struct R(crate::R<SYST_CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NOREF` reader - If reads as 1, the Reference clock is not provided - the CLKSOURCE bit of the SysTick Control and Status register will be forced to 1 and cannot be cleared to 0."]
pub struct NOREF_R(crate::FieldReader<bool, bool>);
impl NOREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKEW` reader - If reads as 1, the calibration value for 10ms is inexact (due to clock frequency)."]
pub struct SKEW_R(crate::FieldReader<bool, bool>);
impl SKEW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SKEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKEW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENMS` reader - An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as 0, the calibration value is not known."]
pub struct TENMS_R(crate::FieldReader<u32, u32>);
impl TENMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TENMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENMS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - If reads as 1, the Reference clock is not provided - the CLKSOURCE bit of the SysTick Control and Status register will be forced to 1 and cannot be cleared to 0."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - If reads as 1, the calibration value for 10ms is inexact (due to clock frequency)."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as 0, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [syst_calib](index.html) module"]
pub struct SYST_CALIB_SPEC;
impl crate::RegisterSpec for SYST_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_calib::R](R) reader structure"]
impl crate::Readable for SYST_CALIB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYST_CALIB to value 0"]
impl crate::Resettable for SYST_CALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
