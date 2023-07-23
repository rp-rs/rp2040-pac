#[doc = "Register `SSIENR` reader"]
pub struct R(crate::R<SSIENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSIENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSIENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSIENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSIENR` writer"]
pub struct W(crate::W<SSIENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSIENR_SPEC>;
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
impl From<crate::W<SSIENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSIENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_EN` reader - SSI enable"]
pub type SSI_EN_R = crate::BitReader<bool>;
#[doc = "Field `SSI_EN` writer - SSI enable"]
pub type SSI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSIENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SSI enable"]
    #[inline(always)]
    pub fn ssi_en(&self) -> SSI_EN_R {
        SSI_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_en(&mut self) -> SSI_EN_W<0> {
        SSI_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Enable  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ssienr](index.html) module"]
pub struct SSIENR_SPEC;
impl crate::RegisterSpec for SSIENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssienr::R](R) reader structure"]
impl crate::Readable for SSIENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssienr::W](W) writer structure"]
impl crate::Writable for SSIENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSIENR to value 0"]
impl crate::Resettable for SSIENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
