#[doc = "Register `INTERP1_BASE1` reader"]
pub type R = crate::R<INTERP1_BASE1_SPEC>;
#[doc = "Register `INTERP1_BASE1` writer"]
pub type W = crate::W<INTERP1_BASE1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTERP1_BASE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Read/write access to BASE1 register.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_base1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_base1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_BASE1_SPEC;
impl crate::RegisterSpec for INTERP1_BASE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp1_base1::R`](R) reader structure"]
impl crate::Readable for INTERP1_BASE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp1_base1::W`](W) writer structure"]
impl crate::Writable for INTERP1_BASE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP1_BASE1 to value 0"]
impl crate::Resettable for INTERP1_BASE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
