#[doc = "Register `SETUP_1` reader"]
pub struct R(crate::R<SETUP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP_1` writer"]
pub struct W(crate::W<SETUP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_1_SPEC>;
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
impl From<crate::W<SETUP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOTW` reader - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
pub struct DOTW_R(crate::FieldReader<u8, u8>);
impl DOTW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOTW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOTW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOTW` writer - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
pub struct DOTW_W<'a> {
    w: &'a mut W,
}
impl<'a> DOTW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `HOUR` reader - Hours"]
pub struct HOUR_R(crate::FieldReader<u8, u8>);
impl HOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR` writer - Hours"]
pub struct HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `MIN` reader - Minutes"]
pub struct MIN_R(crate::FieldReader<u8, u8>);
impl MIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN` writer - Minutes"]
pub struct MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `SEC` reader - Seconds"]
pub struct SEC_R(crate::FieldReader<u8, u8>);
impl SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC` writer - Seconds"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    #[inline(always)]
    pub fn dotw(&self) -> DOTW_R {
        DOTW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    #[inline(always)]
    pub fn dotw(&mut self) -> DOTW_W {
        DOTW_W { w: self }
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC setup register 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [setup_1](index.html) module"]
pub struct SETUP_1_SPEC;
impl crate::RegisterSpec for SETUP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup_1::R](R) reader structure"]
impl crate::Readable for SETUP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup_1::W](W) writer structure"]
impl crate::Writable for SETUP_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETUP_1 to value 0"]
impl crate::Resettable for SETUP_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
