#[doc = "Register `SCRATCH4` reader"]
pub type R = crate::R<SCRATCH4_SPEC>;
#[doc = "Register `SCRATCH4` writer"]
pub type W = crate::W<SCRATCH4_SPEC>;
#[doc = "Field `SCRATCH4` reader - "]
pub type SCRATCH4_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH4` writer - "]
pub type SCRATCH4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch4(&self) -> SCRATCH4_R {
        SCRATCH4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn scratch4(&mut self) -> SCRATCH4_W<SCRATCH4_SPEC> {
        SCRATCH4_W::new(self, 0)
    }
}
#[doc = "Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH4_SPEC;
impl crate::RegisterSpec for SCRATCH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch4::R`](R) reader structure"]
impl crate::Readable for SCRATCH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch4::W`](W) writer structure"]
impl crate::Writable for SCRATCH4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH4 to value 0"]
impl crate::Resettable for SCRATCH4_SPEC {
    const RESET_VALUE: u32 = 0;
}
