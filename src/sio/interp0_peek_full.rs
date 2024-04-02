#[doc = "Register `INTERP0_PEEK_FULL` reader"]
pub type R = crate::R<INTERP0_PEEK_FULL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read FULL result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp0_peek_full::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_PEEK_FULL_SPEC;
impl crate::RegisterSpec for INTERP0_PEEK_FULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_peek_full::R`](R) reader structure"]
impl crate::Readable for INTERP0_PEEK_FULL_SPEC {}
#[doc = "`reset()` method sets INTERP0_PEEK_FULL to value 0"]
impl crate::Resettable for INTERP0_PEEK_FULL_SPEC {
    const RESET_VALUE: u32 = 0;
}
