#[doc = "Register `ADDR_ENDP1` reader"]
pub struct R(crate::R<ADDR_ENDP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_ENDP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_ENDP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_ENDP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR_ENDP1` writer"]
pub struct W(crate::W<ADDR_ENDP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_ENDP1_SPEC>;
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
impl From<crate::W<ADDR_ENDP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_ENDP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEP_PREAMBLE` reader - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
pub struct INTEP_PREAMBLE_R(crate::FieldReader<bool, bool>);
impl INTEP_PREAMBLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTEP_PREAMBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEP_PREAMBLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEP_PREAMBLE` writer - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `INTEP_DIR` reader - Direction of the interrupt endpoint. In=0, Out=1"]
pub struct INTEP_DIR_R(crate::FieldReader<bool, bool>);
impl INTEP_DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTEP_DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEP_DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEP_DIR` writer - Direction of the interrupt endpoint. In=0, Out=1"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `ENDPOINT` reader - Endpoint number of the interrupt endpoint"]
pub struct ENDPOINT_R(crate::FieldReader<u8, u8>);
impl ENDPOINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ENDPOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDPOINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDPOINT` writer - Endpoint number of the interrupt endpoint"]
pub struct ENDPOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPOINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `ADDRESS` reader - Device address"]
pub struct ADDRESS_R(crate::FieldReader<u8, u8>);
impl ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS` writer - Device address"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt endpoint 1. Only valid for HOST mode.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [addr_endp1](index.html) module"]
pub struct ADDR_ENDP1_SPEC;
impl crate::RegisterSpec for ADDR_ENDP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr_endp1::R](R) reader structure"]
impl crate::Readable for ADDR_ENDP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr_endp1::W](W) writer structure"]
impl crate::Writable for ADDR_ENDP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR_ENDP1 to value 0"]
impl crate::Resettable for ADDR_ENDP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
