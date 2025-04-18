#[doc = "Register `AIRCR` reader"]
pub type R = crate::R<AIRCR_SPEC>;
#[doc = "Register `AIRCR` writer"]
pub type W = crate::W<AIRCR_SPEC>;
#[doc = "Field `VECTCLRACTIVE` reader - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
pub type VECTCLRACTIVE_R = crate::BitReader;
#[doc = "Field `VECTCLRACTIVE` writer - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
pub type VECTCLRACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESETREQ` reader - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
pub type SYSRESETREQ_R = crate::BitReader;
#[doc = "Field `SYSRESETREQ` writer - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
pub type SYSRESETREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIANESS` reader - Data endianness implemented: 0 = Little-endian."]
pub type ENDIANESS_R = crate::BitReader;
#[doc = "Field `VECTKEY` reader - Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
pub type VECTKEY_R = crate::FieldReader<u16>;
#[doc = "Field `VECTKEY` writer - Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
pub type VECTKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
    #[inline(always)]
    pub fn vectclractive(&self) -> VECTCLRACTIVE_R {
        VECTCLRACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SYSRESETREQ_R {
        SYSRESETREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 15 - Data endianness implemented: 0 = Little-endian."]
    #[inline(always)]
    pub fn endianess(&self) -> ENDIANESS_R {
        ENDIANESS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W<AIRCR_SPEC> {
        VECTCLRACTIVE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<AIRCR_SPEC> {
        SYSRESETREQ_W::new(self, 2)
    }
    #[doc = "Bits 16:31 - Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VECTKEY_W<AIRCR_SPEC> {
        VECTKEY_W::new(self, 16)
    }
}
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset.  

You can [`read`](crate::generic::Reg::read) this register and get [`aircr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aircr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aircr::R`](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aircr::W`](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0"]
impl crate::Resettable for AIRCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
