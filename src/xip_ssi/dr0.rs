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
pub struct DR_R(crate::FieldReader<u32, u32>);
impl DR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR` writer - First data register of 36"]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
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
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
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
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for DR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
