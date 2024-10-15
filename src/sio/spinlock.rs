#[doc = "Register `SPINLOCK%s` reader"]
pub type R = crate::R<SPINLOCK_SPEC>;
#[doc = "Register `SPINLOCK%s` writer"]
pub type W = crate::W<SPINLOCK_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 &lt;&lt; lock number.  

You can [`read`](crate::Reg::read) this register and get [`spinlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spinlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPINLOCK_SPEC;
impl crate::RegisterSpec for SPINLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spinlock::R`](R) reader structure"]
impl crate::Readable for SPINLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spinlock::W`](W) writer structure"]
impl crate::Writable for SPINLOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPINLOCK%s to value 0"]
impl crate::Resettable for SPINLOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
