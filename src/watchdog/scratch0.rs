#[doc = "Register `SCRATCH0` reader"]
pub type R = crate::R<SCRATCH0_SPEC>;
#[doc = "Register `SCRATCH0` writer"]
pub type W = crate::W<SCRATCH0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCRATCH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH0_SPEC;
impl crate::RegisterSpec for SCRATCH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch0::R`](R) reader structure"]
impl crate::Readable for SCRATCH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch0::W`](W) writer structure"]
impl crate::Writable for SCRATCH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH0 to value 0"]
impl crate::Resettable for SCRATCH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
