#[doc = "Register `ADDR_ENDP` reader"]
pub struct R(crate::R<ADDR_ENDP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_ENDP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_ENDP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_ENDP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR_ENDP` writer"]
pub struct W(crate::W<ADDR_ENDP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_ENDP_SPEC>;
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
impl From<crate::W<ADDR_ENDP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_ENDP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
pub type ADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRESS` writer - In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_ENDP_SPEC, u8, u8, 7, O>;
#[doc = "Field `ENDPOINT` reader - Device endpoint to send data to. Only valid for HOST mode."]
pub type ENDPOINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENDPOINT` writer - Device endpoint to send data to. Only valid for HOST mode."]
pub type ENDPOINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_ENDP_SPEC, u8, u8, 4, O>;
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
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bits 16:19 - Device endpoint to send data to. Only valid for HOST mode."]
    #[inline(always)]
    #[must_use]
    pub fn endpoint(&mut self) -> ENDPOINT_W<16> {
        ENDPOINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device address and endpoint control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [addr_endp](index.html) module"]
pub struct ADDR_ENDP_SPEC;
impl crate::RegisterSpec for ADDR_ENDP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr_endp::R](R) reader structure"]
impl crate::Readable for ADDR_ENDP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr_endp::W](W) writer structure"]
impl crate::Writable for ADDR_ENDP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR_ENDP to value 0"]
impl crate::Resettable for ADDR_ENDP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
