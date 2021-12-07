#[doc = "Register `FC0_STATUS` reader"]
pub struct R(crate::R<FC0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIED` reader - Test clock stopped during test"]
pub struct DIED_R(crate::FieldReader<bool, bool>);
impl DIED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAST` reader - Test clock faster than expected, only valid when status_done=1"]
pub struct FAST_R(crate::FieldReader<bool, bool>);
impl FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW` reader - Test clock slower than expected, only valid when status_done=1"]
pub struct SLOW_R(crate::FieldReader<bool, bool>);
impl SLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAIL` reader - Test failed"]
pub struct FAIL_R(crate::FieldReader<bool, bool>);
impl FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITING` reader - Waiting for test clock to start"]
pub struct WAITING_R(crate::FieldReader<bool, bool>);
impl WAITING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNNING` reader - Test running"]
pub struct RUNNING_R(crate::FieldReader<bool, bool>);
impl RUNNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` reader - Test complete"]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASS` reader - Test passed"]
pub struct PASS_R(crate::FieldReader<bool, bool>);
impl PASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 28 - Test clock stopped during test"]
    #[inline(always)]
    pub fn died(&self) -> DIED_R {
        DIED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Test clock faster than expected, only valid when status_done=1"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Test clock slower than expected, only valid when status_done=1"]
    #[inline(always)]
    pub fn slow(&self) -> SLOW_R {
        SLOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Test failed"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Waiting for test clock to start"]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Test running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Test complete"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Test passed"]
    #[inline(always)]
    pub fn pass(&self) -> PASS_R {
        PASS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Frequency counter status  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fc0_status](index.html) module"]
pub struct FC0_STATUS_SPEC;
impl crate::RegisterSpec for FC0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc0_status::R](R) reader structure"]
impl crate::Readable for FC0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FC0_STATUS to value 0"]
impl crate::Resettable for FC0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
