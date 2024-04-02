#[doc = "Register `IRQ_FORCE` writer"]
pub type W = crate::W<IRQ_FORCE_SPEC>;
#[doc = "Field `IRQ_FORCE` writer - "]
pub type IRQ_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn irq_force(&mut self) -> IRQ_FORCE_W<IRQ_FORCE_SPEC> {
        IRQ_FORCE_W::new(self, 0)
    }
}
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines.  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_force::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_FORCE_SPEC;
impl crate::RegisterSpec for IRQ_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`irq_force::W`](W) writer structure"]
impl crate::Writable for IRQ_FORCE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_FORCE to value 0"]
impl crate::Resettable for IRQ_FORCE_SPEC {
    const RESET_VALUE: u32 = 0;
}
