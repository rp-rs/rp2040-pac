#[doc = "Register `GITREF_RP2040` reader"]
pub type R = crate::R<GITREF_RP2040_SPEC>;
#[doc = "Register `GITREF_RP2040` writer"]
pub type W = crate::W<GITREF_RP2040_SPEC>;
#[doc = "Field `GITREF_RP2040` reader - "]
pub type GITREF_RP2040_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gitref_rp2040(&self) -> GITREF_RP2040_R {
        GITREF_RP2040_R::new(self.bits)
    }
}
impl W {}
#[doc = "Git hash of the chip source. Used to identify chip version.  

You can [`read`](crate::generic::Reg::read) this register and get [`gitref_rp2040::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gitref_rp2040::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GITREF_RP2040_SPEC;
impl crate::RegisterSpec for GITREF_RP2040_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gitref_rp2040::R`](R) reader structure"]
impl crate::Readable for GITREF_RP2040_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gitref_rp2040::W`](W) writer structure"]
impl crate::Writable for GITREF_RP2040_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GITREF_RP2040 to value 0"]
impl crate::Resettable for GITREF_RP2040_SPEC {
    const RESET_VALUE: u32 = 0;
}
