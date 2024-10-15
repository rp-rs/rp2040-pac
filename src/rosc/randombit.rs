#[doc = "Register `RANDOMBIT` reader"]
pub type R = crate::R<RANDOMBIT_SPEC>;
#[doc = "Field `RANDOMBIT` reader - "]
pub type RANDOMBIT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn randombit(&self) -> RANDOMBIT_R {
        RANDOMBIT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency  

You can [`read`](crate::Reg::read) this register and get [`randombit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDOMBIT_SPEC;
impl crate::RegisterSpec for RANDOMBIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randombit::R`](R) reader structure"]
impl crate::Readable for RANDOMBIT_SPEC {}
#[doc = "`reset()` method sets RANDOMBIT to value 0x01"]
impl crate::Resettable for RANDOMBIT_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
