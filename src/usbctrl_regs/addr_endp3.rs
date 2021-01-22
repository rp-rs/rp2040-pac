#[doc = "Reader of register ADDR_ENDP3"]
pub type R = crate::R<u32, super::ADDR_ENDP3>;
#[doc = "Writer for register ADDR_ENDP3"]
pub type W = crate::W<u32, super::ADDR_ENDP3>;
#[doc = "Register ADDR_ENDP3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR_ENDP3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTEP_PREAMBLE`"]
pub type INTEP_PREAMBLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEP_PREAMBLE`"]
pub struct INTEP_PREAMBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEP_PREAMBLE_W<'a> {
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
#[doc = "Reader of field `INTEP_DIR`"]
pub type INTEP_DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEP_DIR`"]
pub struct INTEP_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEP_DIR_W<'a> {
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
    #[doc = "Bit 26 - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    #[inline(always)]
    pub fn intep_preamble(&self) -> INTEP_PREAMBLE_R {
        INTEP_PREAMBLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Direction of the interrupt endpoint. In=0, Out=1"]
    #[inline(always)]
    pub fn intep_dir(&self) -> INTEP_DIR_R {
        INTEP_DIR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Endpoint number of the interrupt endpoint"]
    #[inline(always)]
    pub fn endpoint(&self) -> ENDPOINT_R {
        ENDPOINT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 26 - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    #[inline(always)]
    pub fn intep_preamble(&mut self) -> INTEP_PREAMBLE_W {
        INTEP_PREAMBLE_W { w: self }
    }
    #[doc = "Bit 25 - Direction of the interrupt endpoint. In=0, Out=1"]
    #[inline(always)]
    pub fn intep_dir(&mut self) -> INTEP_DIR_W {
        INTEP_DIR_W { w: self }
    }
    #[doc = "Bits 16:19 - Endpoint number of the interrupt endpoint"]
    #[inline(always)]
    pub fn endpoint(&mut self) -> ENDPOINT_W {
        ENDPOINT_W { w: self }
    }
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
}
