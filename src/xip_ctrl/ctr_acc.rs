#[doc = "Register `CTR_ACC` reader"]
pub type R = crate::R<CTR_ACC_SPEC>;
#[doc = "Register `CTR_ACC` writer"]
pub type W = crate::W<CTR_ACC_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTR_ACC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache Access counter  
 A 32 bit saturating counter that increments upon each XIP access,  
 whether the cache is hit or not. This includes noncacheable accesses.  
 Write any value to clear.  

You can [`read`](crate::generic::Reg::read) this register and get [`ctr_acc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr_acc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_ACC_SPEC;
impl crate::RegisterSpec for CTR_ACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr_acc::R`](R) reader structure"]
impl crate::Readable for CTR_ACC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr_acc::W`](W) writer structure"]
impl crate::Writable for CTR_ACC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR_ACC to value 0"]
impl crate::Resettable for CTR_ACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
