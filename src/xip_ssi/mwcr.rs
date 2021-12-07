#[doc = "Register `MWCR` reader"]
pub struct R(crate::R<MWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MWCR` writer"]
pub struct W(crate::W<MWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWCR_SPEC>;
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
impl From<crate::W<MWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MHS` reader - Microwire handshaking"]
pub struct MHS_R(crate::FieldReader<bool, bool>);
impl MHS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MHS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MHS` writer - Microwire handshaking"]
pub struct MHS_W<'a> {
    w: &'a mut W,
}
impl<'a> MHS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MDD` reader - Microwire control"]
pub struct MDD_R(crate::FieldReader<bool, bool>);
impl MDD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDD` writer - Microwire control"]
pub struct MDD_W<'a> {
    w: &'a mut W,
}
impl<'a> MDD_W<'a> {
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
#[doc = "Field `MWMOD` reader - Microwire transfer mode"]
pub struct MWMOD_R(crate::FieldReader<bool, bool>);
impl MWMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MWMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MWMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MWMOD` writer - Microwire transfer mode"]
pub struct MWMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MWMOD_W<'a> {
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
    #[doc = "Bit 2 - Microwire handshaking"]
    #[inline(always)]
    pub fn mhs(&self) -> MHS_R {
        MHS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Microwire control"]
    #[inline(always)]
    pub fn mdd(&self) -> MDD_R {
        MDD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Microwire transfer mode"]
    #[inline(always)]
    pub fn mwmod(&self) -> MWMOD_R {
        MWMOD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Microwire handshaking"]
    #[inline(always)]
    pub fn mhs(&mut self) -> MHS_W {
        MHS_W { w: self }
    }
    #[doc = "Bit 1 - Microwire control"]
    #[inline(always)]
    pub fn mdd(&mut self) -> MDD_W {
        MDD_W { w: self }
    }
    #[doc = "Bit 0 - Microwire transfer mode"]
    #[inline(always)]
    pub fn mwmod(&mut self) -> MWMOD_W {
        MWMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Microwire Control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [mwcr](index.html) module"]
pub struct MWCR_SPEC;
impl crate::RegisterSpec for MWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mwcr::R](R) reader structure"]
impl crate::Readable for MWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mwcr::W](W) writer structure"]
impl crate::Writable for MWCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MWCR to value 0"]
impl crate::Resettable for MWCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
