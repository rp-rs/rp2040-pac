#[doc = "Register `SCRATCH7` reader"]
pub type R = crate::R<SCRATCH7_SPEC>;
#[doc = "Register `SCRATCH7` writer"]
pub type W = crate::W<SCRATCH7_SPEC>;
#[doc = "Field `SCRATCH7` reader - "]
pub type SCRATCH7_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH7` writer - "]
pub type SCRATCH7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch7(&self) -> SCRATCH7_R {
        SCRATCH7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn scratch7(&mut self) -> SCRATCH7_W<SCRATCH7_SPEC> {
        SCRATCH7_W::new(self, 0)
    }
}
#[doc = "Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH7_SPEC;
impl crate::RegisterSpec for SCRATCH7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch7::R`](R) reader structure"]
impl crate::Readable for SCRATCH7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch7::W`](W) writer structure"]
impl crate::Writable for SCRATCH7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH7 to value 0"]
impl crate::Resettable for SCRATCH7_SPEC {
    const RESET_VALUE: u32 = 0;
}
