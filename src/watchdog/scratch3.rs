#[doc = "Register `SCRATCH3` reader"]
pub type R = crate::R<SCRATCH3_SPEC>;
#[doc = "Register `SCRATCH3` writer"]
pub type W = crate::W<SCRATCH3_SPEC>;
#[doc = "Field `SCRATCH3` reader - "]
pub type SCRATCH3_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH3` writer - "]
pub type SCRATCH3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch3(&self) -> SCRATCH3_R {
        SCRATCH3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn scratch3(&mut self) -> SCRATCH3_W<SCRATCH3_SPEC> {
        SCRATCH3_W::new(self, 0)
    }
}
#[doc = "Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH3_SPEC;
impl crate::RegisterSpec for SCRATCH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch3::R`](R) reader structure"]
impl crate::Readable for SCRATCH3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch3::W`](W) writer structure"]
impl crate::Writable for SCRATCH3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH3 to value 0"]
impl crate::Resettable for SCRATCH3_SPEC {
    const RESET_VALUE: u32 = 0;
}
