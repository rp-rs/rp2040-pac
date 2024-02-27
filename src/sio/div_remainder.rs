#[doc = "Register `DIV_REMAINDER` reader"]
pub type R = crate::R<DIV_REMAINDER_SPEC>;
#[doc = "Register `DIV_REMAINDER` writer"]
pub type W = crate::W<DIV_REMAINDER_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIV_REMAINDER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Divider result remainder  
 The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low.  
 For signed calculations, REMAINDER is negative only when DIVIDEND is negative.  
 This register can be written to directly, for context save/restore purposes. This halts any  
 in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_remainder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_remainder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_REMAINDER_SPEC;
impl crate::RegisterSpec for DIV_REMAINDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_remainder::R`](R) reader structure"]
impl crate::Readable for DIV_REMAINDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_remainder::W`](W) writer structure"]
impl crate::Writable for DIV_REMAINDER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_REMAINDER to value 0"]
impl crate::Resettable for DIV_REMAINDER_SPEC {
    const RESET_VALUE: u32 = 0;
}
