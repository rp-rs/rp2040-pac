#[doc = "Register `INTS` reader"]
pub struct R(crate::R<INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALARM_3` reader - "]
pub struct ALARM_3_R(crate::FieldReader<bool, bool>);
impl ALARM_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM_2` reader - "]
pub struct ALARM_2_R(crate::FieldReader<bool, bool>);
impl ALARM_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM_1` reader - "]
pub struct ALARM_1_R(crate::FieldReader<bool, bool>);
impl ALARM_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM_0` reader - "]
pub struct ALARM_0_R(crate::FieldReader<bool, bool>);
impl ALARM_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alarm_3(&self) -> ALARM_3_R {
        ALARM_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alarm_2(&self) -> ALARM_2_R {
        ALARM_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alarm_1(&self) -> ALARM_1_R {
        ALARM_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alarm_0(&self) -> ALARM_0_R {
        ALARM_0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status after masking & forcing  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ints](index.html) module"]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ints::R](R) reader structure"]
impl crate::Readable for INTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
