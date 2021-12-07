#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR` reader - Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1.  

 Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1.  

 This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores.  

 It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
pub struct INTR_R(crate::FieldReader<u16, u16>);
impl INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1.  

 Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1.  

 This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores.  

 It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Interrupt Status (raw)  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
