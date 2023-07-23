#[doc = "Register `REASON` reader"]
pub struct R(crate::R<REASON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REASON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REASON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REASON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - "]
pub type TIMER_R = crate::BitReader<bool>;
#[doc = "Field `FORCE` reader - "]
pub type FORCE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [reason](index.html) module"]
pub struct REASON_SPEC;
impl crate::RegisterSpec for REASON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reason::R](R) reader structure"]
impl crate::Readable for REASON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REASON to value 0"]
impl crate::Resettable for REASON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
