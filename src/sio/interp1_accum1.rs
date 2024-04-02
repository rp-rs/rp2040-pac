#[doc = "Register `INTERP1_ACCUM1` reader"]
pub type R = crate::R<INTERP1_ACCUM1_SPEC>;
#[doc = "Register `INTERP1_ACCUM1` writer"]
pub type W = crate::W<INTERP1_ACCUM1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Read/write access to accumulator 1  

You can [`read`](crate::Reg::read) this register and get [`interp1_accum1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_accum1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_ACCUM1_SPEC;
impl crate::RegisterSpec for INTERP1_ACCUM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp1_accum1::R`](R) reader structure"]
impl crate::Readable for INTERP1_ACCUM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp1_accum1::W`](W) writer structure"]
impl crate::Writable for INTERP1_ACCUM1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP1_ACCUM1 to value 0"]
impl crate::Resettable for INTERP1_ACCUM1_SPEC {
    const RESET_VALUE: u32 = 0;
}
