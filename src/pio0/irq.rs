#[doc = "Register `IRQ` reader"]
pub struct R(crate::R<IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ` writer"]
pub struct W(crate::W<IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_SPEC>;
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
impl From<crate::W<IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ` reader - "]
pub type IRQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQ` writer - "]
pub type IRQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IRQ_W<0> {
        IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag.  

 Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [irq](index.html) module"]
pub struct IRQ_SPEC;
impl crate::RegisterSpec for IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq::R](R) reader structure"]
impl crate::Readable for IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq::W](W) writer structure"]
impl crate::Writable for IRQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
