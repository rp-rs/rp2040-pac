#[doc = "Register `SCRATCH1` reader"]
pub type R = crate::R<SCRATCH1_SPEC>;
#[doc = "Register `SCRATCH1` writer"]
pub type W = crate::W<SCRATCH1_SPEC>;
#[doc = "Field `SCRATCH1` reader - "]
pub type SCRATCH1_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH1` writer - "]
pub type SCRATCH1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch1(&self) -> SCRATCH1_R {
        SCRATCH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn scratch1(&mut self) -> SCRATCH1_W<SCRATCH1_SPEC> {
        SCRATCH1_W::new(self, 0)
    }
}
#[doc = "Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH1_SPEC;
impl crate::RegisterSpec for SCRATCH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch1::R`](R) reader structure"]
impl crate::Readable for SCRATCH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch1::W`](W) writer structure"]
impl crate::Writable for SCRATCH1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH1 to value 0"]
impl crate::Resettable for SCRATCH1_SPEC {
    const RESET_VALUE: u32 = 0;
}
