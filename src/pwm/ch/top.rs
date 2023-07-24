#[doc = "Register `TOP` reader"]
pub struct R(crate::R<TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOP` writer"]
pub struct W(crate::W<TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOP_SPEC>;
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
impl From<crate::W<TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOP` reader - "]
pub type TOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOP` writer - "]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<0> {
        TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter wrap value  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [top](index.html) module"]
pub struct TOP_SPEC;
impl crate::RegisterSpec for TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [top::R](R) reader structure"]
impl crate::Readable for TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [top::W](W) writer structure"]
impl crate::Writable for TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOP to value 0xffff"]
impl crate::Resettable for TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
