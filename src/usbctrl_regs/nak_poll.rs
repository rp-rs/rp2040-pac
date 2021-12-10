#[doc = "Register `NAK_POLL` reader"]
pub struct R(crate::R<NAK_POLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NAK_POLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NAK_POLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NAK_POLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NAK_POLL` writer"]
pub struct W(crate::W<NAK_POLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NAK_POLL_SPEC>;
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
impl From<crate::W<NAK_POLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NAK_POLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY_FS` reader - NAK polling interval for a full speed device"]
pub struct DELAY_FS_R(crate::FieldReader<u16, u16>);
impl DELAY_FS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DELAY_FS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_FS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELAY_FS` writer - NAK polling interval for a full speed device"]
pub struct DELAY_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_FS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `DELAY_LS` reader - NAK polling interval for a low speed device"]
pub struct DELAY_LS_R(crate::FieldReader<u16, u16>);
impl DELAY_LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DELAY_LS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_LS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELAY_LS` writer - NAK polling interval for a low speed device"]
pub struct DELAY_LS_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_LS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    pub fn delay_fs(&self) -> DELAY_FS_R {
        DELAY_FS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - NAK polling interval for a low speed device"]
    #[inline(always)]
    pub fn delay_ls(&self) -> DELAY_LS_R {
        DELAY_LS_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    pub fn delay_fs(&mut self) -> DELAY_FS_W {
        DELAY_FS_W { w: self }
    }
    #[doc = "Bits 0:9 - NAK polling interval for a low speed device"]
    #[inline(always)]
    pub fn delay_ls(&mut self) -> DELAY_LS_W {
        DELAY_LS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [nak_poll](index.html) module"]
pub struct NAK_POLL_SPEC;
impl crate::RegisterSpec for NAK_POLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nak_poll::R](R) reader structure"]
impl crate::Readable for NAK_POLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nak_poll::W](W) writer structure"]
impl crate::Writable for NAK_POLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NAK_POLL to value 0x0010_0010"]
impl crate::Resettable for NAK_POLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0010
    }
}
