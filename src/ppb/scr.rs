#[doc = "Reader of register SCR"]
pub type R = crate::R<u32, super::SCR>;
#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEVONPEND`"]
pub type SEVONPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEVONPEND`"]
pub struct SEVONPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVONPEND_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SLEEPDEEP`"]
pub type SLEEPDEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEPDEEP`"]
pub struct SLEEPDEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPDEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SLEEPONEXIT`"]
pub type SLEEPONEXIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEPONEXIT`"]
pub struct SLEEPONEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPONEXIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Send Event on Pending bit:\\n 0 = Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded.\\n 1 = Enabled events and all interrupts, including disabled interrupts, can wakeup the processor.\\n When an event or interrupt becomes pending, the event signal wakes up the processor from WFE. If the\\n processor is not waiting for an event, the event is registered and affects the next WFE.\\n The processor also wakes up on execution of an SEV instruction or an external event."]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls whether the processor uses sleep or deep sleep as its low power mode:\\n 0 = Sleep.\\n 1 = Deep sleep."]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates sleep-on-exit when returning from Handler mode to Thread mode:\\n 0 = Do not sleep when returning to Thread mode.\\n 1 = Enter sleep, or deep sleep, on return from an ISR to Thread mode.\\n Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Send Event on Pending bit:\\n 0 = Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded.\\n 1 = Enabled events and all interrupts, including disabled interrupts, can wakeup the processor.\\n When an event or interrupt becomes pending, the event signal wakes up the processor from WFE. If the\\n processor is not waiting for an event, the event is registered and affects the next WFE.\\n The processor also wakes up on execution of an SEV instruction or an external event."]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SEVONPEND_W {
        SEVONPEND_W { w: self }
    }
    #[doc = "Bit 2 - Controls whether the processor uses sleep or deep sleep as its low power mode:\\n 0 = Sleep.\\n 1 = Deep sleep."]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W {
        SLEEPDEEP_W { w: self }
    }
    #[doc = "Bit 1 - Indicates sleep-on-exit when returning from Handler mode to Thread mode:\\n 0 = Do not sleep when returning to Thread mode.\\n 1 = Enter sleep, or deep sleep, on return from an ISR to Thread mode.\\n Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W {
        SLEEPONEXIT_W { w: self }
    }
}
