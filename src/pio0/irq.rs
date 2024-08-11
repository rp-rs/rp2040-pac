#[doc = "Register `IRQ` reader"]
pub type R = crate::R<IRQ_SPEC>;
#[doc = "Register `IRQ` writer"]
pub type W = crate::W<IRQ_SPEC>;
#[doc = "Field `IRQ` reader - "]
pub type IRQ_R = crate::FieldReader;
#[doc = "Field `IRQ` writer - "]
pub type IRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn irq(&mut self) -> IRQ_W<IRQ_SPEC> {
        IRQ_W::new(self, 0)
    }
}
#[doc = "State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag. Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE.  

You can [`read`](crate::generic::Reg::read) this register and get [`irq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_SPEC;
impl crate::RegisterSpec for IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq::R`](R) reader structure"]
impl crate::Readable for IRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq::W`](W) writer structure"]
impl crate::Writable for IRQ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IRQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
