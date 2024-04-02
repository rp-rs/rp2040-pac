#[doc = "Register `GITREF_RP2040` reader"]
pub type R = crate::R<GITREF_RP2040_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Git hash of the chip source. Used to identify chip version.  

You can [`read`](crate::Reg::read) this register and get [`gitref_rp2040::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GITREF_RP2040_SPEC;
impl crate::RegisterSpec for GITREF_RP2040_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gitref_rp2040::R`](R) reader structure"]
impl crate::Readable for GITREF_RP2040_SPEC {}
#[doc = "`reset()` method sets GITREF_RP2040 to value 0"]
impl crate::Resettable for GITREF_RP2040_SPEC {
    const RESET_VALUE: u32 = 0;
}
