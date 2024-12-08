#[doc = "Register `INTS0` reader"]
pub type R = crate::R<INTS0_SPEC>;
#[doc = "Register `INTS0` writer"]
pub type W = crate::W<INTS0_SPEC>;
#[doc = "Field `INTS0` reader - Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted. Channel interrupts can be cleared by writing a bit mask here."]
pub type INTS0_R = crate::FieldReader<u16>;
#[doc = "Field `INTS0` writer - Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted. Channel interrupts can be cleared by writing a bit mask here."]
pub type INTS0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted. Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    pub fn ints0(&self) -> INTS0_R {
        INTS0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted. Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    #[must_use]
    pub fn ints0(&mut self) -> INTS0_W<INTS0_SPEC> {
        INTS0_W::new(self, 0)
    }
}
#[doc = "Interrupt Status for IRQ 0  

You can [`read`](crate::generic::Reg::read) this register and get [`ints0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS0_SPEC;
impl crate::RegisterSpec for INTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints0::R`](R) reader structure"]
impl crate::Readable for INTS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ints0::W`](W) writer structure"]
impl crate::Writable for INTS0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets INTS0 to value 0"]
impl crate::Resettable for INTS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
