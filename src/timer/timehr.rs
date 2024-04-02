#[doc = "Register `TIMEHR` reader"]
pub type R = crate::R<TIMEHR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read from bits 63:32 of time  
 always read timelr before timehr  

You can [`read`](crate::Reg::read) this register and get [`timehr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEHR_SPEC;
impl crate::RegisterSpec for TIMEHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timehr::R`](R) reader structure"]
impl crate::Readable for TIMEHR_SPEC {}
#[doc = "`reset()` method sets TIMEHR to value 0"]
impl crate::Resettable for TIMEHR_SPEC {
    const RESET_VALUE: u32 = 0;
}
