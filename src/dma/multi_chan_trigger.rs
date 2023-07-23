#[doc = "Register `MULTI_CHAN_TRIGGER` reader"]
pub struct R(crate::R<MULTI_CHAN_TRIGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULTI_CHAN_TRIGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULTI_CHAN_TRIGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULTI_CHAN_TRIGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MULTI_CHAN_TRIGGER` writer"]
pub struct W(crate::W<MULTI_CHAN_TRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MULTI_CHAN_TRIGGER_SPEC>;
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
impl From<crate::W<MULTI_CHAN_TRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MULTI_CHAN_TRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MULTI_CHAN_TRIGGER` reader - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
pub type MULTI_CHAN_TRIGGER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MULTI_CHAN_TRIGGER` writer - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
pub type MULTI_CHAN_TRIGGER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MULTI_CHAN_TRIGGER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
    #[inline(always)]
    pub fn multi_chan_trigger(&self) -> MULTI_CHAN_TRIGGER_R {
        MULTI_CHAN_TRIGGER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
    #[inline(always)]
    #[must_use]
    pub fn multi_chan_trigger(&mut self) -> MULTI_CHAN_TRIGGER_W<0> {
        MULTI_CHAN_TRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger one or more channels simultaneously  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [multi_chan_trigger](index.html) module"]
pub struct MULTI_CHAN_TRIGGER_SPEC;
impl crate::RegisterSpec for MULTI_CHAN_TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [multi_chan_trigger::R](R) reader structure"]
impl crate::Readable for MULTI_CHAN_TRIGGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [multi_chan_trigger::W](W) writer structure"]
impl crate::Writable for MULTI_CHAN_TRIGGER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MULTI_CHAN_TRIGGER to value 0"]
impl crate::Resettable for MULTI_CHAN_TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
