#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Reader of field `INTR`"]
pub type INTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1.\\n\\n Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1.\\n\\n This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores.\\n\\n It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new((self.bits & 0xffff) as u16)
    }
}
