#[doc = "Register `DIV_QUOTIENT` reader"]
pub type R = crate::R<DIV_QUOTIENT_SPEC>;
#[doc = "Register `DIV_QUOTIENT` writer"]
pub type W = crate::W<DIV_QUOTIENT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIV_QUOTIENT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Divider result quotient  
 The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low.  
 For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ.  
 This register can be written to directly, for context save/restore purposes. This halts any  
 in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.  
 Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order  
 REMAINDER, QUOTIENT if CSR_DIRTY is used.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_quotient::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_quotient::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_QUOTIENT_SPEC;
impl crate::RegisterSpec for DIV_QUOTIENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_quotient::R`](R) reader structure"]
impl crate::Readable for DIV_QUOTIENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_quotient::W`](W) writer structure"]
impl crate::Writable for DIV_QUOTIENT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_QUOTIENT to value 0"]
impl crate::Resettable for DIV_QUOTIENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
