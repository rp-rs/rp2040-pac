#[doc = "Register `DIV_UDIVISOR` reader"]
pub type R = crate::R<DIV_UDIVISOR_SPEC>;
#[doc = "Register `DIV_UDIVISOR` writer"]
pub type W = crate::W<DIV_UDIVISOR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIV_UDIVISOR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Divider unsigned divisor  
 Write to the DIVISOR operand of the divider, i.e. the q in `p / q`.  
 Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.  
 UDIVISOR/SDIVISOR are aliases of the same internal register. The U alias starts an  
 unsigned calculation, and the S alias starts a signed calculation.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_udivisor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_udivisor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_UDIVISOR_SPEC;
impl crate::RegisterSpec for DIV_UDIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_udivisor::R`](R) reader structure"]
impl crate::Readable for DIV_UDIVISOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_udivisor::W`](W) writer structure"]
impl crate::Writable for DIV_UDIVISOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_UDIVISOR to value 0"]
impl crate::Resettable for DIV_UDIVISOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
