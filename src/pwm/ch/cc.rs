#[doc = "Register `CC` reader"]
pub struct R(crate::R<CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC` writer"]
pub struct W(crate::W<CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_SPEC>;
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
impl From<crate::W<CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A` reader - "]
pub type A_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A` writer - "]
pub type A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC_SPEC, u16, u16, 16, O>;
#[doc = "Field `B` reader - "]
pub type B_R = crate::FieldReader<u16, u16>;
#[doc = "Field `B` writer - "]
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<0> {
        A_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<16> {
        B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter compare values  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [cc](index.html) module"]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc::R](R) reader structure"]
impl crate::Readable for CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc::W](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
