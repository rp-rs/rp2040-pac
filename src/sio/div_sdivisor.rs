#[doc = "Register `DIV_SDIVISOR` reader"]
pub type R = crate::R<DIV_SDIVISOR_SPEC>;
#[doc = "Register `DIV_SDIVISOR` writer"]
pub type W = crate::W<DIV_SDIVISOR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Divider signed divisor  
 The same as UDIVISOR, but starts a signed calculation, rather than unsigned.  

You can [`read`](crate::Reg::read) this register and get [`div_sdivisor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_sdivisor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SDIVISOR_SPEC;
impl crate::RegisterSpec for DIV_SDIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_sdivisor::R`](R) reader structure"]
impl crate::Readable for DIV_SDIVISOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_sdivisor::W`](W) writer structure"]
impl crate::Writable for DIV_SDIVISOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_SDIVISOR to value 0"]
impl crate::Resettable for DIV_SDIVISOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
