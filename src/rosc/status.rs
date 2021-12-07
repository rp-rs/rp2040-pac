#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STABLE` reader - Oscillator is running and stable"]
pub struct STABLE_R(crate::FieldReader<bool, bool>);
impl STABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_RUNNING` reader - post-divider is running this resets to 0 but transitions to 1 during chip startup"]
pub struct DIV_RUNNING_R(crate::FieldReader<bool, bool>);
impl DIV_RUNNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIV_RUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_RUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` reader - Oscillator is enabled but not necessarily running and stable this resets to 0 but transitions to 1 during chip startup"]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Oscillator is running and stable"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 16 - post-divider is running this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn div_running(&self) -> DIV_RUNNING_R {
        DIV_RUNNING_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Oscillator is enabled but not necessarily running and stable this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
#[doc = "Ring Oscillator Status  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
