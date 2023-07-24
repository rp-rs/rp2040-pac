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
#[doc = "Field `ALARM_0` reader - "]
pub type ALARM_0_R = crate::BitReader<bool>;
#[doc = "Field `ALARM_1` reader - "]
pub type ALARM_1_R = crate::BitReader<bool>;
#[doc = "Field `ALARM_2` reader - "]
pub type ALARM_2_R = crate::BitReader<bool>;
#[doc = "Field `ALARM_3` reader - "]
pub type ALARM_3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alarm_0(&self) -> ALARM_0_R {
        ALARM_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alarm_1(&self) -> ALARM_1_R {
        ALARM_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alarm_2(&self) -> ALARM_2_R {
        ALARM_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alarm_3(&self) -> ALARM_3_R {
        ALARM_3_R::new(((self.bits >> 3) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
