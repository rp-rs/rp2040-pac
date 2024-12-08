#[doc = "Register `CTR_ACC` reader"]
pub type R = crate::R<CTR_ACC_SPEC>;
#[doc = "Register `CTR_ACC` writer"]
pub type W = crate::W<CTR_ACC_SPEC>;
#[doc = "Field `CTR_ACC` reader - A 32 bit saturating counter that increments upon each XIP access, whether the cache is hit or not. This includes noncacheable accesses. Write any value to clear."]
pub type CTR_ACC_R = crate::FieldReader<u32>;
#[doc = "Field `CTR_ACC` writer - A 32 bit saturating counter that increments upon each XIP access, whether the cache is hit or not. This includes noncacheable accesses. Write any value to clear."]
pub type CTR_ACC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - A 32 bit saturating counter that increments upon each XIP access, whether the cache is hit or not. This includes noncacheable accesses. Write any value to clear."]
    #[inline(always)]
    pub fn ctr_acc(&self) -> CTR_ACC_R {
        CTR_ACC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A 32 bit saturating counter that increments upon each XIP access, whether the cache is hit or not. This includes noncacheable accesses. Write any value to clear."]
    #[inline(always)]
    #[must_use]
    pub fn ctr_acc(&mut self) -> CTR_ACC_W<CTR_ACC_SPEC> {
        CTR_ACC_W::new(self, 0)
    }
}
#[doc = "Cache Access counter  

You can [`read`](crate::generic::Reg::read) this register and get [`ctr_acc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr_acc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_ACC_SPEC;
impl crate::RegisterSpec for CTR_ACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr_acc::R`](R) reader structure"]
impl crate::Readable for CTR_ACC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr_acc::W`](W) writer structure"]
impl crate::Writable for CTR_ACC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets CTR_ACC to value 0"]
impl crate::Resettable for CTR_ACC_SPEC {
    const RESET_VALUE: u32 = 0;
}
