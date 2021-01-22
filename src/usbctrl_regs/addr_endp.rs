#[doc = "Reader of register ADDR_ENDP"]
pub type R = crate::R<u32, super::ADDR_ENDP>;
#[doc = "Writer for register ADDR_ENDP"]
pub type W = crate::W<u32, super::ADDR_ENDP>;
#[doc = "Register ADDR_ENDP `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR_ENDP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENDPOINT`"]
pub type ENDPOINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENDPOINT`"]
pub struct ENDPOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPOINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADDRESS`"]
pub type ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRESS`"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Device endpoint to send data to. Only valid for HOST mode."]
    #[inline(always)]
    pub fn endpoint(&self) -> ENDPOINT_R {
        ENDPOINT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Device endpoint to send data to. Only valid for HOST mode."]
    #[inline(always)]
    pub fn endpoint(&mut self) -> ENDPOINT_W {
        ENDPOINT_W { w: self }
    }
    #[doc = "Bits 0:6 - In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
}
