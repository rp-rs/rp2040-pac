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
#[doc = "Field `BMREQUESTTYPE` reader - "]
pub type BMREQUESTTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BMREQUESTTYPE` writer - "]
pub type BMREQUESTTYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SETUP_PACKET_LOW_SPEC, u8, u8, 8, O>;
#[doc = "Field `BREQUEST` reader - "]
pub type BREQUEST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREQUEST` writer - "]
pub type BREQUEST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SETUP_PACKET_LOW_SPEC, u8, u8, 8, O>;
#[doc = "Field `WVALUE` reader - "]
pub type WVALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVALUE` writer - "]
pub type WVALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SETUP_PACKET_LOW_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BMREQUESTTYPE_R {
        BMREQUESTTYPE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wvalue(&self) -> WVALUE_R {
        WVALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn bmrequesttype(&mut self) -> BMREQUESTTYPE_W<0> {
        BMREQUESTTYPE_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn brequest(&mut self) -> BREQUEST_W<8> {
        BREQUEST_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn wvalue(&mut self) -> WVALUE_W<16> {
        WVALUE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP_PACKET_LOW to value 0"]
impl crate::Resettable for SETUP_PACKET_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
