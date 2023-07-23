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
#[doc = "Field `DS4` reader - Stage 4 drive strength"]
pub type DS4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS4` writer - Stage 4 drive strength"]
pub type DS4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREQB_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS5` reader - Stage 5 drive strength"]
pub type DS5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS5` writer - Stage 5 drive strength"]
pub type DS5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREQB_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS6` reader - Stage 6 drive strength"]
pub type DS6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS6` writer - Stage 6 drive strength"]
pub type DS6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREQB_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS7` reader - Stage 7 drive strength"]
pub type DS7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS7` writer - Stage 7 drive strength"]
pub type DS7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREQB_SPEC, u8, u8, 3, O>;
#[doc = "Field `PASSWD` reader - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
pub type PASSWD_R = crate::FieldReader<u16, PASSWD_A>;
#[doc = "Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PASSWD_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PASSWD_A::PASS
    }
}
#[doc = "Field `PASSWD` writer - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
pub type PASSWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREQB_SPEC, u16, PASSWD_A, 16, O>;
impl<'a, const O: u8> PASSWD_W<'a, O> {
    #[doc = "`1001011010010110`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(PASSWD_A::PASS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Stage 4 drive strength"]
    #[inline(always)]
    pub fn ds4(&self) -> DS4_R {
        DS4_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Stage 5 drive strength"]
    #[inline(always)]
    pub fn ds5(&self) -> DS5_R {
        DS5_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Stage 6 drive strength"]
    #[inline(always)]
    pub fn ds6(&self) -> DS6_R {
        DS6_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Stage 7 drive strength"]
    #[inline(always)]
    pub fn ds7(&self) -> DS7_R {
        DS7_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&self) -> PASSWD_R {
        PASSWD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stage 4 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds4(&mut self) -> DS4_W<0> {
        DS4_W::new(self)
    }
    #[doc = "Bits 4:6 - Stage 5 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds5(&mut self) -> DS5_W<4> {
        DS5_W::new(self)
    }
    #[doc = "Bits 8:10 - Stage 6 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds6(&mut self) -> DS6_W<8> {
        DS6_W::new(self)
    }
    #[doc = "Bits 12:14 - Stage 7 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds7(&mut self) -> DS7_W<12> {
        DS7_W::new(self)
    }
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<16> {
        PASSWD_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQB to value 0"]
impl crate::Resettable for FREQB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
