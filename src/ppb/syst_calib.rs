#[doc = "Reader of register SYST_CALIB"]
pub type R = crate::R<u32, super::SYST_CALIB>;
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TENMS`"]
pub type TENMS_R = crate::R<u32, u32>;
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
