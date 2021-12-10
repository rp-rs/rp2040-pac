#[doc = "Register `PHASE` reader"]
pub struct R(crate::R<PHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHASE` writer"]
pub struct W(crate::W<PHASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHASE_SPEC>;
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
impl From<crate::W<PHASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PASSWD` reader - set to 0xaa  
 any other value enables the output with shift=0"]
pub struct PASSWD_R(crate::FieldReader<u8, u8>);
impl PASSWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PASSWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PASSWD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASSWD` writer - set to 0xaa  
 any other value enables the output with shift=0"]
pub struct PASSWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | ((value as u32 & 0xff) << 4);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - enable the phase-shifted output  
 this can be changed on-the-fly"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - enable the phase-shifted output  
 this can be changed on-the-fly"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FLIP` reader - invert the phase-shifted output  
 this is ignored when div=1"]
pub struct FLIP_R(crate::FieldReader<bool, bool>);
impl FLIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIP` writer - invert the phase-shifted output  
 this is ignored when div=1"]
pub struct FLIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIP_W<'a> {
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
#[doc = "Field `SHIFT` reader - phase shift the phase-shifted output by SHIFT input clocks  
 this can be changed on-the-fly  
 must be set to 0 before setting div=1"]
pub struct SHIFT_R(crate::FieldReader<u8, u8>);
impl SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT` writer - phase shift the phase-shifted output by SHIFT input clocks  
 this can be changed on-the-fly  
 must be set to 0 before setting div=1"]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:11 - set to 0xaa  
 any other value enables the output with shift=0"]
    #[inline(always)]
    pub fn passwd(&self) -> PASSWD_R {
        PASSWD_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 3 - enable the phase-shifted output  
 this can be changed on-the-fly"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - invert the phase-shifted output  
 this is ignored when div=1"]
    #[inline(always)]
    pub fn flip(&self) -> FLIP_R {
        FLIP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - phase shift the phase-shifted output by SHIFT input clocks  
 this can be changed on-the-fly  
 must be set to 0 before setting div=1"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:11 - set to 0xaa  
 any other value enables the output with shift=0"]
    #[inline(always)]
    pub fn passwd(&mut self) -> PASSWD_W {
        PASSWD_W { w: self }
    }
    #[doc = "Bit 3 - enable the phase-shifted output  
 this can be changed on-the-fly"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - invert the phase-shifted output  
 this is ignored when div=1"]
    #[inline(always)]
    pub fn flip(&mut self) -> FLIP_W {
        FLIP_W { w: self }
    }
    #[doc = "Bits 0:1 - phase shift the phase-shifted output by SHIFT input clocks  
 this can be changed on-the-fly  
 must be set to 0 before setting div=1"]
    #[inline(always)]
    pub fn shift(&mut self) -> SHIFT_W {
        SHIFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the phase shifted output  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [phase](index.html) module"]
pub struct PHASE_SPEC;
impl crate::RegisterSpec for PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phase::R](R) reader structure"]
impl crate::Readable for PHASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phase::W](W) writer structure"]
impl crate::Writable for PHASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHASE to value 0x08"]
impl crate::Resettable for PHASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
