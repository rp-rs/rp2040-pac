#[doc = "Register `TIMERAWL` reader"]
pub type R = crate::R<TIMERAWL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Raw read from bits 31:0 of time (no side effects)  

You can [`read`](crate::Reg::read) this register and get [`timerawl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERAWL_SPEC;
impl crate::RegisterSpec for TIMERAWL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerawl::R`](R) reader structure"]
impl crate::Readable for TIMERAWL_SPEC {}
#[doc = "`reset()` method sets TIMERAWL to value 0"]
impl crate::Resettable for TIMERAWL_SPEC {
    const RESET_VALUE: u32 = 0;
}
