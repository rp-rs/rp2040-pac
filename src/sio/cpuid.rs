#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CPUID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Processor core identifier  
 Value is 0 when read from processor core 0, and 1 when read from processor core 1.  

You can [`read`](crate::Reg::read) this register and get [`cpuid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CPUID_SPEC {}
#[doc = "`reset()` method sets CPUID to value 0"]
impl crate::Resettable for CPUID_SPEC {
    const RESET_VALUE: u32 = 0;
}
