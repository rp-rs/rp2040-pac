#[doc = "Register `CTRLR1` reader"]
pub struct R(crate::R<CTRLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLR1` writer"]
pub struct W(crate::W<CTRLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLR1_SPEC>;
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
impl From<crate::W<CTRLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDF` reader - Number of data frames"]
pub type NDF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDF` writer - Number of data frames"]
pub type NDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Number of data frames"]
    #[inline(always)]
    pub fn ndf(&self) -> NDF_R {
        NDF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data frames"]
    #[inline(always)]
    #[must_use]
    pub fn ndf(&mut self) -> NDF_W<0> {
        NDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Control register 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrlr1](index.html) module"]
pub struct CTRLR1_SPEC;
impl crate::RegisterSpec for CTRLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlr1::R](R) reader structure"]
impl crate::Readable for CTRLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlr1::W](W) writer structure"]
impl crate::Writable for CTRLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLR1 to value 0"]
impl crate::Resettable for CTRLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
