#[doc = "Register `DBG_PADOUT` reader"]
pub type R = crate::R<DBG_PADOUT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`dbg_padout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_PADOUT_SPEC;
impl crate::RegisterSpec for DBG_PADOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_padout::R`](R) reader structure"]
impl crate::Readable for DBG_PADOUT_SPEC {}
#[doc = "`reset()` method sets DBG_PADOUT to value 0"]
impl crate::Resettable for DBG_PADOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
