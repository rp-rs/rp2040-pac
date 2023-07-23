#[doc = "Register `CHAN_ABORT` reader"]
pub struct R(crate::R<CHAN_ABORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_ABORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_ABORT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_ABORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_ABORT` writer"]
pub struct W(crate::W<CHAN_ABORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_ABORT_SPEC>;
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
impl From<crate::W<CHAN_ABORT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_ABORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHAN_ABORT` reader - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.  

 After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
pub type CHAN_ABORT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHAN_ABORT` writer - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.  

 After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
pub type CHAN_ABORT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHAN_ABORT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.  

 After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    #[inline(always)]
    pub fn chan_abort(&self) -> CHAN_ABORT_R {
        CHAN_ABORT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.  

 After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    #[inline(always)]
    #[must_use]
    pub fn chan_abort(&mut self) -> CHAN_ABORT_W<0> {
        CHAN_ABORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Abort an in-progress transfer sequence on one or more channels  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [chan_abort](index.html) module"]
pub struct CHAN_ABORT_SPEC;
impl crate::RegisterSpec for CHAN_ABORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_abort::R](R) reader structure"]
impl crate::Readable for CHAN_ABORT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_abort::W](W) writer structure"]
impl crate::Writable for CHAN_ABORT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHAN_ABORT to value 0"]
impl crate::Resettable for CHAN_ABORT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
