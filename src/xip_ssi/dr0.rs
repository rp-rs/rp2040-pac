#[doc = "Register `DR0` reader"]
pub struct R(crate::R<DR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR0` writer"]
pub struct W(crate::W<DR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR0_SPEC>;
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
impl From<crate::W<DR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR` reader - First data register of 36"]
pub type DR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DR` writer - First data register of 36"]
pub type DR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - First data register of 36"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - First data register of 36"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<0> {
        DR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Register 0 (of 36)  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dr0](index.html) module"]
pub struct DR0_SPEC;
impl crate::RegisterSpec for DR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr0::R](R) reader structure"]
impl crate::Readable for DR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr0::W](W) writer structure"]
impl crate::Writable for DR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for DR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
