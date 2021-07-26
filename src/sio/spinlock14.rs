#[doc = "Register `SPINLOCK14` reader"]
pub struct R(crate::R<SPINLOCK14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [spinlock14](index.html) module"]
pub struct SPINLOCK14_SPEC;
impl crate::RegisterSpec for SPINLOCK14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock14::R](R) reader structure"]
impl crate::Readable for SPINLOCK14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPINLOCK14 to value 0"]
impl crate::Resettable for SPINLOCK14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
