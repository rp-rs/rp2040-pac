#[doc = "Register `INTR1` reader"]
pub type R = crate::R<INTR1_SPEC>;
#[doc = "Register `INTR1` writer"]
pub type W = crate::W<INTR1_SPEC>;
#[doc = "Field `INTR1` reader - Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1. Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1. This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores. It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
pub type INTR1_R = crate::FieldReader<u16>;
#[doc = "Field `INTR1` writer - Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1. Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1. This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores. It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
pub type INTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1. Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1. This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores. It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
    #[inline(always)]
    pub fn intr1(&self) -> INTR1_R {
        INTR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1. Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1. This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores. It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
    #[inline(always)]
    #[must_use]
    pub fn intr1(&mut self) -> INTR1_W<INTR1_SPEC> {
        INTR1_W::new(self, 0)
    }
}
#[doc = "Interrupt Status (raw)  

You can [`read`](crate::generic::Reg::read) this register and get [`intr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR1_SPEC;
impl crate::RegisterSpec for INTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1::R`](R) reader structure"]
impl crate::Readable for INTR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr1::W`](W) writer structure"]
impl crate::Writable for INTR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets INTR1 to value 0"]
impl crate::Resettable for INTR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
