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
#[doc = "Field `DELAY_LS` reader - NAK polling interval for a low speed device"]
pub type DELAY_LS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY_LS` writer - NAK polling interval for a low speed device"]
pub type DELAY_LS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAK_POLL_SPEC, u16, u16, 10, O>;
#[doc = "Field `DELAY_FS` reader - NAK polling interval for a full speed device"]
pub type DELAY_FS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY_FS` writer - NAK polling interval for a full speed device"]
pub type DELAY_FS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAK_POLL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - NAK polling interval for a low speed device"]
    #[inline(always)]
    pub fn delay_ls(&self) -> DELAY_LS_R {
        DELAY_LS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    pub fn delay_fs(&self) -> DELAY_FS_R {
        DELAY_FS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - NAK polling interval for a low speed device"]
    #[inline(always)]
    #[must_use]
    pub fn delay_ls(&mut self) -> DELAY_LS_W<0> {
        DELAY_LS_W::new(self)
    }
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    #[must_use]
    pub fn delay_fs(&mut self) -> DELAY_FS_W<16> {
        DELAY_FS_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NAK_POLL to value 0x0010_0010"]
impl crate::Resettable for NAK_POLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0010;
}
