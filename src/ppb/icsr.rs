#[doc = "Register `ICSR` reader"]
pub type R = crate::R<ICSR_SPEC>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<ICSR_SPEC>;
#[doc = "Field `VECTACTIVE` reader - Active exception number field. Reset clears the VECTACTIVE field."]
pub type VECTACTIVE_R = crate::FieldReader<u16>;
#[doc = "Field `VECTPENDING` reader - Indicates the exception number for the highest priority pending exception: 0 = no pending exceptions. Non zero = The pending state includes the effect of memory-mapped enable and mask registers. It does not include the PRIMASK special-purpose register qualifier."]
pub type VECTPENDING_R = crate::FieldReader<u16>;
#[doc = "Field `ISRPENDING` reader - External interrupt pending flag"]
pub type ISRPENDING_R = crate::BitReader;
#[doc = "Field `ISRPREEMPT` reader - The system can only access this bit when the core is halted. It indicates that a pending interrupt is to be taken in the next running cycle. If C_MASKINTS is clear in the Debug Halting Control and Status Register, the interrupt is serviced."]
pub type ISRPREEMPT_R = crate::BitReader;
#[doc = "Field `PENDSTCLR` reader - SysTick exception clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the SysTick exception. This bit is WO. On a register read its value is Unknown."]
pub type PENDSTCLR_R = crate::BitReader;
#[doc = "Field `PENDSTCLR` writer - SysTick exception clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the SysTick exception. This bit is WO. On a register read its value is Unknown."]
pub type PENDSTCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSTSET` reader - SysTick exception set-pending bit. Write: 0 = No effect. 1 = Changes SysTick exception state to pending. Read: 0 = SysTick exception is not pending. 1 = SysTick exception is pending."]
pub type PENDSTSET_R = crate::BitReader;
#[doc = "Field `PENDSTSET` writer - SysTick exception set-pending bit. Write: 0 = No effect. 1 = Changes SysTick exception state to pending. Read: 0 = SysTick exception is not pending. 1 = SysTick exception is pending."]
pub type PENDSTSET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSVCLR` reader - PendSV clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the PendSV exception."]
pub type PENDSVCLR_R = crate::BitReader;
#[doc = "Field `PENDSVCLR` writer - PendSV clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the PendSV exception."]
pub type PENDSVCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit. Write: 0 = No effect. 1 = Changes PendSV exception state to pending. Read: 0 = PendSV exception is not pending. 1 = PendSV exception is pending. Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
pub type PENDSVSET_R = crate::BitReader;
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit. Write: 0 = No effect. 1 = Changes PendSV exception state to pending. Read: 0 = PendSV exception is not pending. 1 = PendSV exception is pending. Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
pub type PENDSVSET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIPENDSET` reader - Setting this bit will activate an NMI. Since NMI is the highest priority exception, it will activate as soon as it is registered. NMI set-pending bit. Write: 0 = No effect. 1 = Changes NMI exception state to pending. Read: 0 = NMI exception is not pending. 1 = NMI exception is pending. Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
pub type NMIPENDSET_R = crate::BitReader;
#[doc = "Field `NMIPENDSET` writer - Setting this bit will activate an NMI. Since NMI is the highest priority exception, it will activate as soon as it is registered. NMI set-pending bit. Write: 0 = No effect. 1 = Changes NMI exception state to pending. Read: 0 = NMI exception is not pending. 1 = NMI exception is pending. Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
pub type NMIPENDSET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Active exception number field. Reset clears the VECTACTIVE field."]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - Indicates the exception number for the highest priority pending exception: 0 = no pending exceptions. Non zero = The pending state includes the effect of memory-mapped enable and mask registers. It does not include the PRIMASK special-purpose register qualifier."]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 22 - External interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The system can only access this bit when the core is halted. It indicates that a pending interrupt is to be taken in the next running cycle. If C_MASKINTS is clear in the Debug Halting Control and Status Register, the interrupt is serviced."]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - SysTick exception clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the SysTick exception. This bit is WO. On a register read its value is Unknown."]
    #[inline(always)]
    pub fn pendstclr(&self) -> PENDSTCLR_R {
        PENDSTCLR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit. Write: 0 = No effect. 1 = Changes SysTick exception state to pending. Read: 0 = SysTick exception is not pending. 1 = SysTick exception is pending."]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the PendSV exception."]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PENDSVCLR_R {
        PENDSVCLR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit. Write: 0 = No effect. 1 = Changes PendSV exception state to pending. Read: 0 = PendSV exception is not pending. 1 = PendSV exception is pending. Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Setting this bit will activate an NMI. Since NMI is the highest priority exception, it will activate as soon as it is registered. NMI set-pending bit. Write: 0 = No effect. 1 = Changes NMI exception state to pending. Read: 0 = NMI exception is not pending. 1 = NMI exception is pending. Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - SysTick exception clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the SysTick exception. This bit is WO. On a register read its value is Unknown."]
    #[inline(always)]
    #[must_use]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W<ICSR_SPEC> {
        PENDSTCLR_W::new(self, 25)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit. Write: 0 = No effect. 1 = Changes SysTick exception state to pending. Read: 0 = SysTick exception is not pending. 1 = SysTick exception is pending."]
    #[inline(always)]
    #[must_use]
    pub fn pendstset(&mut self) -> PENDSTSET_W<ICSR_SPEC> {
        PENDSTSET_W::new(self, 26)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the PendSV exception."]
    #[inline(always)]
    #[must_use]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W<ICSR_SPEC> {
        PENDSVCLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - PendSV set-pending bit. Write: 0 = No effect. 1 = Changes PendSV exception state to pending. Read: 0 = PendSV exception is not pending. 1 = PendSV exception is pending. Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
    #[inline(always)]
    #[must_use]
    pub fn pendsvset(&mut self) -> PENDSVSET_W<ICSR_SPEC> {
        PENDSVSET_W::new(self, 28)
    }
    #[doc = "Bit 31 - Setting this bit will activate an NMI. Since NMI is the highest priority exception, it will activate as soon as it is registered. NMI set-pending bit. Write: 0 = No effect. 1 = Changes NMI exception state to pending. Read: 0 = NMI exception is not pending. 1 = NMI exception is pending. Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
    #[inline(always)]
    #[must_use]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W<ICSR_SPEC> {
        NMIPENDSET_W::new(self, 31)
    }
}
#[doc = "Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception.  

You can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for ICSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
