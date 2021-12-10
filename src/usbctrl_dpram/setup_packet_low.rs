#[doc = "Register `SETUP_PACKET_LOW` reader"]
pub struct R(crate::R<SETUP_PACKET_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_PACKET_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_PACKET_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_PACKET_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP_PACKET_LOW` writer"]
pub struct W(crate::W<SETUP_PACKET_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_PACKET_LOW_SPEC>;
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
impl From<crate::W<SETUP_PACKET_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_PACKET_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVALUE` reader - "]
pub struct WVALUE_R(crate::FieldReader<u16, u16>);
impl WVALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WVALUE` writer - "]
pub struct WVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> WVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `BREQUEST` reader - "]
pub struct BREQUEST_R(crate::FieldReader<u8, u8>);
impl BREQUEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BREQUEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREQUEST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREQUEST` writer - "]
pub struct BREQUEST_W<'a> {
    w: &'a mut W,
}
impl<'a> BREQUEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BMREQUESTTYPE` reader - "]
pub struct BMREQUESTTYPE_R(crate::FieldReader<u8, u8>);
impl BMREQUESTTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BMREQUESTTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMREQUESTTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMREQUESTTYPE` writer - "]
pub struct BMREQUESTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMREQUESTTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wvalue(&self) -> WVALUE_R {
        WVALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BMREQUESTTYPE_R {
        BMREQUESTTYPE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wvalue(&mut self) -> WVALUE_W {
        WVALUE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn brequest(&mut self) -> BREQUEST_W {
        BREQUEST_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bmrequesttype(&mut self) -> BMREQUESTTYPE_W {
        BMREQUESTTYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bytes 0-3 of the SETUP packet from the host.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [setup_packet_low](index.html) module"]
pub struct SETUP_PACKET_LOW_SPEC;
impl crate::RegisterSpec for SETUP_PACKET_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup_packet_low::R](R) reader structure"]
impl crate::Readable for SETUP_PACKET_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup_packet_low::W](W) writer structure"]
impl crate::Writable for SETUP_PACKET_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETUP_PACKET_LOW to value 0"]
impl crate::Resettable for SETUP_PACKET_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
