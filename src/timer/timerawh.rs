#[doc = "Register `TIMERAWH` reader"]
pub struct R(crate::R<TIMERAWH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERAWH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERAWH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERAWH_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIMERAWH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Raw read from bits 63:32 of time (no side effects)  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [timerawh](index.html) module"]
pub struct TIMERAWH_SPEC;
impl crate::RegisterSpec for TIMERAWH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timerawh::R](R) reader structure"]
impl crate::Readable for TIMERAWH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMERAWH to value 0"]
impl crate::Resettable for TIMERAWH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
