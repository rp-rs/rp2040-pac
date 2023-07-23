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
#[doc = "Field `WINDEX` reader - "]
pub type WINDEX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINDEX` writer - "]
pub type WINDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SETUP_PACKET_HIGH_SPEC, u16, u16, 16, O>;
#[doc = "Field `WLENGTH` reader - "]
pub type WLENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WLENGTH` writer - "]
pub type WLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SETUP_PACKET_HIGH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn windex(&self) -> WINDEX_R {
        WINDEX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wlength(&self) -> WLENGTH_R {
        WLENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn windex(&mut self) -> WINDEX_W<0> {
        WINDEX_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn wlength(&mut self) -> WLENGTH_W<16> {
        WLENGTH_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP_PACKET_HIGH to value 0"]
impl crate::Resettable for SETUP_PACKET_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
