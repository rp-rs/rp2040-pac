#[doc = "Register `INT_EP_CTRL` reader"]
pub struct R(crate::R<INT_EP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EP_CTRL` writer"]
pub struct W(crate::W<INT_EP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EP_CTRL_SPEC>;
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
impl From<crate::W<INT_EP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_EP_ACTIVE` reader - Host: Enable interrupt endpoint 1 -> 15"]
pub struct INT_EP_ACTIVE_R(crate::FieldReader<u16, u16>);
impl INT_EP_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INT_EP_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_EP_ACTIVE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_EP_ACTIVE` writer - Host: Enable interrupt endpoint 1 -> 15"]
pub struct INT_EP_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EP_ACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | ((value as u32 & 0x7fff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:15 - Host: Enable interrupt endpoint 1 -> 15"]
    #[inline(always)]
    pub fn int_ep_active(&self) -> INT_EP_ACTIVE_R {
        INT_EP_ACTIVE_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:15 - Host: Enable interrupt endpoint 1 -> 15"]
    #[inline(always)]
    pub fn int_ep_active(&mut self) -> INT_EP_ACTIVE_W {
        INT_EP_ACTIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt endpoint control register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [int_ep_ctrl](index.html) module"]
pub struct INT_EP_CTRL_SPEC;
impl crate::RegisterSpec for INT_EP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ep_ctrl::R](R) reader structure"]
impl crate::Readable for INT_EP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ep_ctrl::W](W) writer structure"]
impl crate::Writable for INT_EP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_EP_CTRL to value 0"]
impl crate::Resettable for INT_EP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
