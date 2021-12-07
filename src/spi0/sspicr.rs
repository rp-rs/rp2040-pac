#[doc = "Register `SSPICR` reader"]
pub struct R(crate::R<SSPICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPICR` writer"]
pub struct W(crate::W<SSPICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPICR_SPEC>;
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
impl From<crate::W<SSPICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTIC` reader - Clears the SSPRTINTR interrupt"]
pub struct RTIC_R(crate::FieldReader<bool, bool>);
impl RTIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIC` writer - Clears the SSPRTINTR interrupt"]
pub struct RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RORIC` reader - Clears the SSPRORINTR interrupt"]
pub struct RORIC_R(crate::FieldReader<bool, bool>);
impl RORIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RORIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RORIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RORIC` writer - Clears the SSPRORINTR interrupt"]
pub struct RORIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RORIC_W<'a> {
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
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn roric(&self) -> RORIC_R {
        RORIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W { w: self }
    }
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn roric(&mut self) -> RORIC_W {
        RORIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register, SSPICR on page 3-11  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspicr](index.html) module"]
pub struct SSPICR_SPEC;
impl crate::RegisterSpec for SSPICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspicr::R](R) reader structure"]
impl crate::Readable for SSPICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspicr::W](W) writer structure"]
impl crate::Writable for SSPICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSPICR to value 0"]
impl crate::Resettable for SSPICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
