#[doc = "Register `INTE0` reader"]
pub type R = crate::R<INTE0_SPEC>;
#[doc = "Register `INTE0` writer"]
pub type W = crate::W<INTE0_SPEC>;
#[doc = "Field `INTE0` reader - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
pub type INTE0_R = crate::FieldReader<u16>;
#[doc = "Field `INTE0` writer - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
pub type INTE0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
    #[inline(always)]
    pub fn inte0(&self) -> INTE0_R {
        INTE0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
    #[inline(always)]
    #[must_use]
    pub fn inte0(&mut self) -> INTE0_W<INTE0_SPEC> {
        INTE0_W::new(self, 0)
    }
}
#[doc = "Interrupt Enables for IRQ 0  

You can [`read`](crate::Reg::read) this register and get [`inte0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE0_SPEC;
impl crate::RegisterSpec for INTE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte0::R`](R) reader structure"]
impl crate::Readable for INTE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte0::W`](W) writer structure"]
impl crate::Writable for INTE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE0 to value 0"]
impl crate::Resettable for INTE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
