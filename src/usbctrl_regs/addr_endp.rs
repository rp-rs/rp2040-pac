#[doc = "Register `ADDR_ENDP` reader"]
pub type R = crate::R<ADDR_ENDP_SPEC>;
#[doc = "Register `ADDR_ENDP` writer"]
pub type W = crate::W<ADDR_ENDP_SPEC>;
#[doc = "Field `ADDRESS` reader - In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `ENDPOINT` reader - Device endpoint to send data to. Only valid for HOST mode."]
pub type ENDPOINT_R = crate::FieldReader;
#[doc = "Field `ENDPOINT` writer - Device endpoint to send data to. Only valid for HOST mode."]
pub type ENDPOINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:6 - In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - Device endpoint to send data to. Only valid for HOST mode."]
    #[inline(always)]
    pub fn endpoint(&self) -> ENDPOINT_R {
        ENDPOINT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<ADDR_ENDP_SPEC, 0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bits 16:19 - Device endpoint to send data to. Only valid for HOST mode."]
    #[inline(always)]
    #[must_use]
    pub fn endpoint(&mut self) -> ENDPOINT_W<ADDR_ENDP_SPEC, 16> {
        ENDPOINT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device address and endpoint control  

You can [`read`](crate::generic::Reg::read) this register and get [`addr_endp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_endp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_ENDP_SPEC;
impl crate::RegisterSpec for ADDR_ENDP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_endp::R`](R) reader structure"]
impl crate::Readable for ADDR_ENDP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_endp::W`](W) writer structure"]
impl crate::Writable for ADDR_ENDP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR_ENDP to value 0"]
impl crate::Resettable for ADDR_ENDP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
