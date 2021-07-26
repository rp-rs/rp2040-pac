#[doc = "Register `IRQ_FORCE` writer"]
pub struct W(crate::W<IRQ_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IRQ_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_FORCE` writer - "]
pub struct IRQ_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn irq_force(&mut self) -> IRQ_FORCE_W {
        IRQ_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines.  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [irq_force](index.html) module"]
pub struct IRQ_FORCE_SPEC;
impl crate::RegisterSpec for IRQ_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [irq_force::W](W) writer structure"]
impl crate::Writable for IRQ_FORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_FORCE to value 0"]
impl crate::Resettable for IRQ_FORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
