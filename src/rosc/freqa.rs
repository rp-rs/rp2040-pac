#[doc = "Register `FREQA` reader"]
pub struct R(crate::R<FREQA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQA` writer"]
pub struct W(crate::W<FREQA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQA_SPEC>;
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
impl From<crate::W<FREQA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQA_SPEC>) -> Self {
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
#[doc = "Field `DS3` reader - Stage 3 drive strength"]
pub struct DS3_R(crate::FieldReader<u8, u8>);
impl DS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS3` writer - Stage 3 drive strength"]
pub struct DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> DS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `DS2` reader - Stage 2 drive strength"]
pub struct DS2_R(crate::FieldReader<u8, u8>);
impl DS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS2` writer - Stage 2 drive strength"]
pub struct DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> DS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DS1` reader - Stage 1 drive strength"]
pub struct DS1_R(crate::FieldReader<u8, u8>);
impl DS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS1` writer - Stage 1 drive strength"]
pub struct DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> DS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DS0` reader - Stage 0 drive strength"]
pub struct DS0_R(crate::FieldReader<u8, u8>);
impl DS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS0` writer - Stage 0 drive strength"]
pub struct DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> DS0_W<'a> {
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
    #[doc = "Bits 12:14 - Stage 3 drive strength"]
    #[inline(always)]
    pub fn ds3(&self) -> DS3_R {
        DS3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Stage 2 drive strength"]
    #[inline(always)]
    pub fn ds2(&self) -> DS2_R {
        DS2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Stage 1 drive strength"]
    #[inline(always)]
    pub fn ds1(&self) -> DS1_R {
        DS1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Stage 0 drive strength"]
    #[inline(always)]
    pub fn ds0(&self) -> DS0_R {
        DS0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&mut self) -> PASSWD_W {
        PASSWD_W { w: self }
    }
    #[doc = "Bits 12:14 - Stage 3 drive strength"]
    #[inline(always)]
    pub fn ds3(&mut self) -> DS3_W {
        DS3_W { w: self }
    }
    #[doc = "Bits 8:10 - Stage 2 drive strength"]
    #[inline(always)]
    pub fn ds2(&mut self) -> DS2_W {
        DS2_W { w: self }
    }
    #[doc = "Bits 4:6 - Stage 1 drive strength"]
    #[inline(always)]
    pub fn ds1(&mut self) -> DS1_W {
        DS1_W { w: self }
    }
    #[doc = "Bits 0:2 - Stage 0 drive strength"]
    #[inline(always)]
    pub fn ds0(&mut self) -> DS0_W {
        DS0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage  
 The drive strength has 4 levels determined by the number of bits set  
 Increasing the number of bits set increases the drive strength and increases the oscillation frequency  
 0 bits set is the default drive strength  
 1 bit set doubles the drive strength  
 2 bits set triples drive strength  
 3 bits set quadruples drive strength  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [freqa](index.html) module"]
pub struct FREQA_SPEC;
impl crate::RegisterSpec for FREQA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqa::R](R) reader structure"]
impl crate::Readable for FREQA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqa::W](W) writer structure"]
impl crate::Writable for FREQA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQA to value 0"]
impl crate::Resettable for FREQA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
