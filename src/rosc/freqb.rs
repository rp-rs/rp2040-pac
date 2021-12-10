#[doc = "Register `FREQB` reader"]
pub struct R(crate::R<FREQB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQB` writer"]
pub struct W(crate::W<FREQB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQB_SPEC>;
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
impl From<crate::W<FREQB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum PASSWD_A {
    #[doc = "38550: `1001011010010110`"]
    PASS = 38550,
}
impl From<PASSWD_A> for u16 {
    #[inline(always)]
    fn from(variant: PASSWD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PASSWD` reader - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
pub struct PASSWD_R(crate::FieldReader<u16, PASSWD_A>);
impl PASSWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PASSWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PASSWD_A> {
        match self.bits {
            38550 => Some(PASSWD_A::PASS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == PASSWD_A::PASS
    }
}
impl core::ops::Deref for PASSWD_R {
    type Target = crate::FieldReader<u16, PASSWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASSWD` writer - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
pub struct PASSWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PASSWD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1001011010010110`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(PASSWD_A::PASS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `DS7` reader - Stage 7 drive strength"]
pub struct DS7_R(crate::FieldReader<u8, u8>);
impl DS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS7` writer - Stage 7 drive strength"]
pub struct DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> DS7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `DS6` reader - Stage 6 drive strength"]
pub struct DS6_R(crate::FieldReader<u8, u8>);
impl DS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS6` writer - Stage 6 drive strength"]
pub struct DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> DS6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DS5` reader - Stage 5 drive strength"]
pub struct DS5_R(crate::FieldReader<u8, u8>);
impl DS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS5` writer - Stage 5 drive strength"]
pub struct DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> DS5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DS4` reader - Stage 4 drive strength"]
pub struct DS4_R(crate::FieldReader<u8, u8>);
impl DS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS4` writer - Stage 4 drive strength"]
pub struct DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> DS4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&self) -> PASSWD_R {
        PASSWD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:14 - Stage 7 drive strength"]
    #[inline(always)]
    pub fn ds7(&self) -> DS7_R {
        DS7_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Stage 6 drive strength"]
    #[inline(always)]
    pub fn ds6(&self) -> DS6_R {
        DS6_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Stage 5 drive strength"]
    #[inline(always)]
    pub fn ds5(&self) -> DS5_R {
        DS5_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Stage 4 drive strength"]
    #[inline(always)]
    pub fn ds4(&self) -> DS4_R {
        DS4_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&mut self) -> PASSWD_W {
        PASSWD_W { w: self }
    }
    #[doc = "Bits 12:14 - Stage 7 drive strength"]
    #[inline(always)]
    pub fn ds7(&mut self) -> DS7_W {
        DS7_W { w: self }
    }
    #[doc = "Bits 8:10 - Stage 6 drive strength"]
    #[inline(always)]
    pub fn ds6(&mut self) -> DS6_W {
        DS6_W { w: self }
    }
    #[doc = "Bits 4:6 - Stage 5 drive strength"]
    #[inline(always)]
    pub fn ds5(&mut self) -> DS5_W {
        DS5_W { w: self }
    }
    #[doc = "Bits 0:2 - Stage 4 drive strength"]
    #[inline(always)]
    pub fn ds4(&mut self) -> DS4_W {
        DS4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For a detailed description see freqa register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [freqb](index.html) module"]
pub struct FREQB_SPEC;
impl crate::RegisterSpec for FREQB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqb::R](R) reader structure"]
impl crate::Readable for FREQB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqb::W](W) writer structure"]
impl crate::Writable for FREQB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQB to value 0"]
impl crate::Resettable for FREQB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
