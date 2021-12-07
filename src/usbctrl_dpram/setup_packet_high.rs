#[doc = "Register `SETUP_PACKET_HIGH` reader"]
pub struct R(crate::R<SETUP_PACKET_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_PACKET_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_PACKET_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_PACKET_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP_PACKET_HIGH` writer"]
pub struct W(crate::W<SETUP_PACKET_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_PACKET_HIGH_SPEC>;
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
impl From<crate::W<SETUP_PACKET_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_PACKET_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLENGTH` reader - "]
pub struct WLENGTH_R(crate::FieldReader<u16, u16>);
impl WLENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WLENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLENGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLENGTH` writer - "]
pub struct WLENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WLENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `WINDEX` reader - "]
pub struct WINDEX_R(crate::FieldReader<u16, u16>);
impl WINDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WINDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINDEX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINDEX` writer - "]
pub struct WINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wlength(&self) -> WLENGTH_R {
        WLENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn windex(&self) -> WINDEX_R {
        WINDEX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wlength(&mut self) -> WLENGTH_W {
        WLENGTH_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn windex(&mut self) -> WINDEX_W {
        WINDEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bytes 4-7 of the setup packet from the host.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [setup_packet_high](index.html) module"]
pub struct SETUP_PACKET_HIGH_SPEC;
impl crate::RegisterSpec for SETUP_PACKET_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup_packet_high::R](R) reader structure"]
impl crate::Readable for SETUP_PACKET_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup_packet_high::W](W) writer structure"]
impl crate::Writable for SETUP_PACKET_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETUP_PACKET_HIGH to value 0"]
impl crate::Resettable for SETUP_PACKET_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
