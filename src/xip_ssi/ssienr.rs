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
pub struct SSI_EN_R(crate::FieldReader<bool, bool>);
impl SSI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSI_EN` writer - SSI enable"]
pub struct SSI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SSI enable"]
    #[inline(always)]
    pub fn ssi_en(&self) -> SSI_EN_R {
        SSI_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI enable"]
    #[inline(always)]
    pub fn ssi_en(&mut self) -> SSI_EN_W {
        SSI_EN_W { w: self }
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
}
#[doc = "`reset()` method sets SSIENR to value 0"]
impl crate::Resettable for SSIENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
