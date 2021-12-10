#[doc = "Register `DMACR` reader"]
pub struct R(crate::R<DMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACR` writer"]
pub struct W(crate::W<DMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACR_SPEC>;
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
impl From<crate::W<DMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDMAE` reader - Transmit DMA enable"]
pub struct TDMAE_R(crate::FieldReader<bool, bool>);
impl TDMAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDMAE` writer - Transmit DMA enable"]
pub struct TDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMAE_W<'a> {
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
#[doc = "Field `RDMAE` reader - Receive DMA enable"]
pub struct RDMAE_R(crate::FieldReader<bool, bool>);
impl RDMAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDMAE` writer - Receive DMA enable"]
pub struct RDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAE_W<'a> {
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
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn tdmae(&mut self) -> TDMAE_W {
        TDMAE_W { w: self }
    }
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rdmae(&mut self) -> RDMAE_W {
        RDMAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dmacr](index.html) module"]
pub struct DMACR_SPEC;
impl crate::RegisterSpec for DMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacr::R](R) reader structure"]
impl crate::Readable for DMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacr::W](W) writer structure"]
impl crate::Writable for DMACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DMACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
