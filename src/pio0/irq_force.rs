#[doc = "Register `IRQ_FORCE` writer"]
pub type W = crate::W<IRQ_FORCE_SPEC>;
#[doc = "Field `IRQ_FORCE` writer - "]
pub type IRQ_FORCE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn irq_force(&mut self) -> IRQ_FORCE_W<IRQ_FORCE_SPEC, 0> {
        IRQ_FORCE_W::new(self)
    }
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
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines.  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_force::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_FORCE_SPEC;
impl crate::RegisterSpec for IRQ_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`irq_force::W`](W) writer structure"]
impl crate::Writable for IRQ_FORCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_FORCE to value 0"]
impl crate::Resettable for IRQ_FORCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
