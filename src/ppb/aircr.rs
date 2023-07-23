#[doc = "Register `AIRCR` reader"]
pub struct R(crate::R<AIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIRCR` writer"]
pub struct W(crate::W<AIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIRCR_SPEC>;
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
impl From<crate::W<AIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTCLRACTIVE` reader - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
pub type VECTCLRACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `VECTCLRACTIVE` writer - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
pub type VECTCLRACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `SYSRESETREQ` reader - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
pub type SYSRESETREQ_R = crate::BitReader<bool>;
#[doc = "Field `SYSRESETREQ` writer - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
pub type SYSRESETREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `ENDIANESS` reader - Data endianness implemented:  
 0 = Little-endian."]
pub type ENDIANESS_R = crate::BitReader<bool>;
#[doc = "Field `VECTKEY` reader - Register key:  
 Reads as Unknown  
 On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
pub type VECTKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VECTKEY` writer - Register key:  
 Reads as Unknown  
 On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
pub type VECTKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u16, u16, 16, O>;
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
    #[doc = "Bit 15 - Data endianness implemented:  
 0 = Little-endian."]
    #[inline(always)]
    pub fn endianess(&self) -> ENDIANESS_R {
        ENDIANESS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key:  
 Reads as Unknown  
 On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W<1> {
        VECTCLRACTIVE_W::new(self)
    }
    #[doc = "Bit 2 - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<2> {
        SYSRESETREQ_W::new(self)
    }
    #[doc = "Bits 16:31 - Register key:  
 Reads as Unknown  
 On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VECTKEY_W<16> {
        VECTKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [aircr](index.html) module"]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aircr::R](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aircr::W](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0"]
impl crate::Resettable for AIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
