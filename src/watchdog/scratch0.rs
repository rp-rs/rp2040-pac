#[doc = "Register `SCRATCH0` reader"]
pub type R = crate::R<SCRATCH0_SPEC>;
#[doc = "Register `SCRATCH0` writer"]
pub type W = crate::W<SCRATCH0_SPEC>;
#[doc = "Field `SCRATCH0` reader - "]
pub type SCRATCH0_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH0` writer - "]
pub type SCRATCH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch0(&self) -> SCRATCH0_R {
        SCRATCH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn scratch0(&mut self) -> SCRATCH0_W<SCRATCH0_SPEC> {
        SCRATCH0_W::new(self, 0)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH0 to value 0"]
impl crate::Resettable for SCRATCH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
