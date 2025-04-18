#[doc = "Register `RANDOMBIT` reader"]
pub type R = crate::R<RANDOMBIT_SPEC>;
#[doc = "Register `RANDOMBIT` writer"]
pub type W = crate::W<RANDOMBIT_SPEC>;
#[doc = "Field `RANDOMBIT` reader - "]
pub type RANDOMBIT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn randombit(&self) -> RANDOMBIT_R {
        RANDOMBIT_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency  

You can [`read`](crate::generic::Reg::read) this register and get [`randombit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`randombit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDOMBIT_SPEC;
impl crate::RegisterSpec for RANDOMBIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randombit::R`](R) reader structure"]
impl crate::Readable for RANDOMBIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randombit::W`](W) writer structure"]
impl crate::Writable for RANDOMBIT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANDOMBIT to value 0x01"]
impl crate::Resettable for RANDOMBIT_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
