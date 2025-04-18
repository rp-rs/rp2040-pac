#[doc = "Register `SPINLOCK_ST` reader"]
pub type R = crate::R<SPINLOCK_ST_SPEC>;
#[doc = "Register `SPINLOCK_ST` writer"]
pub type W = crate::W<SPINLOCK_ST_SPEC>;
#[doc = "Field `SPINLOCK_ST` reader - "]
pub type SPINLOCK_ST_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spinlock_st(&self) -> SPINLOCK_ST_R {
        SPINLOCK_ST_R::new(self.bits)
    }
}
impl W {}
#[doc = "Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging.  

You can [`read`](crate::generic::Reg::read) this register and get [`spinlock_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPINLOCK_ST_SPEC;
impl crate::RegisterSpec for SPINLOCK_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spinlock_st::R`](R) reader structure"]
impl crate::Readable for SPINLOCK_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spinlock_st::W`](W) writer structure"]
impl crate::Writable for SPINLOCK_ST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPINLOCK_ST to value 0"]
impl crate::Resettable for SPINLOCK_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
