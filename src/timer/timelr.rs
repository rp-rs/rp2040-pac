#[doc = "Register `TIMELR` reader"]
pub type R = crate::R<TIMELR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read from bits 31:0 of time  

You can [`read`](crate::Reg::read) this register and get [`timelr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMELR_SPEC;
impl crate::RegisterSpec for TIMELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timelr::R`](R) reader structure"]
impl crate::Readable for TIMELR_SPEC {}
#[doc = "`reset()` method sets TIMELR to value 0"]
impl crate::Resettable for TIMELR_SPEC {
    const RESET_VALUE: u32 = 0;
}
