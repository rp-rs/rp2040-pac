#[doc = "Reader of register ICSR"]
pub type R = crate::R<u32, super::ICSR>;
#[doc = "Writer for register ICSR"]
pub type W = crate::W<u32, super::ICSR>;
#[doc = "Register ICSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NMIPENDSET`"]
pub type NMIPENDSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIPENDSET`"]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `PENDSVSET`"]
pub type PENDSVSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSVSET`"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PENDSVCLR`"]
pub type PENDSVCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSVCLR`"]
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PENDSTSET`"]
pub type PENDSTSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSTSET`"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PENDSTCLR`"]
pub type PENDSTCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSTCLR`"]
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `ISRPREEMPT`"]
pub type ISRPREEMPT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISRPENDING`"]
pub type ISRPENDING_R = crate::R<bool, bool>;
#[doc = "Reader of field `VECTPENDING`"]
pub type VECTPENDING_R = crate::R<u16, u16>;
#[doc = "Reader of field `VECTACTIVE`"]
pub type VECTACTIVE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 31 - Setting this bit will activate an NMI. Since NMI is the highest priority exception, it will activate as soon as it is registered.\\n NMI set-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Changes NMI exception state to pending.\\n Read:\\n 0 = NMI exception is not pending.\\n 1 = NMI exception is pending.\\n Because NMI is the highest-priority exception, normally the processor enters the NMI\\n exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears\\n this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the\\n NMI signal is reasserted while the processor is executing that handler."]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Changes PendSV exception state to pending.\\n Read:\\n 0 = PendSV exception is not pending.\\n 1 = PendSV exception is pending.\\n Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Removes the pending state from the PendSV exception."]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PENDSVCLR_R {
        PENDSVCLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Changes SysTick exception state to pending.\\n Read:\\n 0 = SysTick exception is not pending.\\n 1 = SysTick exception is pending."]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SysTick exception clear-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Removes the pending state from the SysTick exception.\\n This bit is WO. On a register read its value is Unknown."]
    #[inline(always)]
    pub fn pendstclr(&self) -> PENDSTCLR_R {
        PENDSTCLR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The system can only access this bit when the core is halted. It indicates that a pending interrupt is to be taken in the next running cycle. If C_MASKINTS is clear in the Debug Halting Control and Status Register, the interrupt is serviced."]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - External interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 12:20 - Indicates the exception number for the highest priority pending exception: 0 = no pending exceptions. Non zero = The pending state includes the effect of memory-mapped enable and mask registers. It does not include the PRIMASK special-purpose register qualifier."]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8 - Active exception number field. Reset clears the VECTACTIVE field."]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Setting this bit will activate an NMI. Since NMI is the highest priority exception, it will activate as soon as it is registered.\\n NMI set-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Changes NMI exception state to pending.\\n Read:\\n 0 = NMI exception is not pending.\\n 1 = NMI exception is pending.\\n Because NMI is the highest-priority exception, normally the processor enters the NMI\\n exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears\\n this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the\\n NMI signal is reasserted while the processor is executing that handler."]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
    }
    #[doc = "Bit 28 - PendSV set-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Changes PendSV exception state to pending.\\n Read:\\n 0 = PendSV exception is not pending.\\n 1 = PendSV exception is pending.\\n Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 27 - PendSV clear-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Removes the pending state from the PendSV exception."]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Changes SysTick exception state to pending.\\n Read:\\n 0 = SysTick exception is not pending.\\n 1 = SysTick exception is pending."]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 25 - SysTick exception clear-pending bit.\\n Write:\\n 0 = No effect.\\n 1 = Removes the pending state from the SysTick exception.\\n This bit is WO. On a register read its value is Unknown."]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
}
