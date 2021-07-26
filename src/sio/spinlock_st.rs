#[doc = "Register `SPINLOCK_ST` reader"]
pub struct R(crate::R<SPINLOCK_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Spinlock state  
 A bitmap containing the state of all 32 spinlocks (1=locked).  
 Mainly intended for debugging.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [spinlock_st](index.html) module"]
pub struct SPINLOCK_ST_SPEC;
impl crate::RegisterSpec for SPINLOCK_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock_st::R](R) reader structure"]
impl crate::Readable for SPINLOCK_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPINLOCK_ST to value 0"]
impl crate::Resettable for SPINLOCK_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
