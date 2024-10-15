#[doc = "Register `SCRATCH2` reader"]
pub type R = crate::R<SCRATCH2_SPEC>;
#[doc = "Register `SCRATCH2` writer"]
pub type W = crate::W<SCRATCH2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::Reg::read) this register and get [`scratch2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH2_SPEC;
impl crate::RegisterSpec for SCRATCH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch2::R`](R) reader structure"]
impl crate::Readable for SCRATCH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch2::W`](W) writer structure"]
impl crate::Writable for SCRATCH2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH2 to value 0"]
impl crate::Resettable for SCRATCH2_SPEC {
    const RESET_VALUE: u32 = 0;
}
