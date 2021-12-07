#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PH_ADV` reader - Advance the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running  
 at less than full speed (div_int + div_frac / 16 > 1)"]
pub struct PH_ADV_R(crate::FieldReader<bool, bool>);
impl PH_ADV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PH_ADV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH_ADV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH_ADV` writer - Advance the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running  
 at less than full speed (div_int + div_frac / 16 > 1)"]
pub struct PH_ADV_W<'a> {
    w: &'a mut W,
}
impl<'a> PH_ADV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PH_RET` reader - Retard the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running."]
pub struct PH_RET_R(crate::FieldReader<bool, bool>);
impl PH_RET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PH_RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH_RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH_RET` writer - Retard the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running."]
pub struct PH_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> PH_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVMODE_A {
    #[doc = "0: Free-running counting at rate dictated by fractional divider"]
    DIV = 0,
    #[doc = "1: Fractional divider operation is gated by the PWM B pin."]
    LEVEL = 1,
    #[doc = "2: Counter advances with each rising edge of the PWM B pin."]
    RISE = 2,
    #[doc = "3: Counter advances with each falling edge of the PWM B pin."]
    FALL = 3,
}
impl From<DIVMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVMODE` reader - "]
pub struct DIVMODE_R(crate::FieldReader<u8, DIVMODE_A>);
impl DIVMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVMODE_A {
        match self.bits {
            0 => DIVMODE_A::DIV,
            1 => DIVMODE_A::LEVEL,
            2 => DIVMODE_A::RISE,
            3 => DIVMODE_A::FALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        **self == DIVMODE_A::DIV
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        **self == DIVMODE_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == DIVMODE_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == DIVMODE_A::FALL
    }
}
impl core::ops::Deref for DIVMODE_R {
    type Target = crate::FieldReader<u8, DIVMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVMODE` writer - "]
pub struct DIVMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Free-running counting at rate dictated by fractional divider"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(DIVMODE_A::DIV)
    }
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DIVMODE_A::LEVEL)
    }
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(DIVMODE_A::RISE)
    }
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(DIVMODE_A::FALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `B_INV` reader - Invert output B"]
pub struct B_INV_R(crate::FieldReader<bool, bool>);
impl B_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        B_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B_INV` writer - Invert output B"]
pub struct B_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> B_INV_W<'a> {
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
#[doc = "Field `A_INV` reader - Invert output A"]
pub struct A_INV_R(crate::FieldReader<bool, bool>);
impl A_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        A_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A_INV` writer - Invert output A"]
pub struct A_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> A_INV_W<'a> {
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
#[doc = "Field `PH_CORRECT` reader - 1: Enable phase-correct modulation. 0: Trailing-edge"]
pub struct PH_CORRECT_R(crate::FieldReader<bool, bool>);
impl PH_CORRECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PH_CORRECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH_CORRECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH_CORRECT` writer - 1: Enable phase-correct modulation. 0: Trailing-edge"]
pub struct PH_CORRECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PH_CORRECT_W<'a> {
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
#[doc = "Field `EN` reader - Enable the PWM channel."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable the PWM channel."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bit 7 - Advance the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running  
 at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn ph_adv(&self) -> PH_ADV_R {
        PH_ADV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Retard the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn ph_ret(&self) -> PH_RET_R {
        PH_RET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn divmode(&self) -> DIVMODE_R {
        DIVMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Invert output B"]
    #[inline(always)]
    pub fn b_inv(&self) -> B_INV_R {
        B_INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Invert output A"]
    #[inline(always)]
    pub fn a_inv(&self) -> A_INV_R {
        A_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn ph_correct(&self) -> PH_CORRECT_R {
        PH_CORRECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable the PWM channel."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Advance the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running  
 at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn ph_adv(&mut self) -> PH_ADV_W {
        PH_ADV_W { w: self }
    }
    #[doc = "Bit 6 - Retard the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn ph_ret(&mut self) -> PH_RET_W {
        PH_RET_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn divmode(&mut self) -> DIVMODE_W {
        DIVMODE_W { w: self }
    }
    #[doc = "Bit 3 - Invert output B"]
    #[inline(always)]
    pub fn b_inv(&mut self) -> B_INV_W {
        B_INV_W { w: self }
    }
    #[doc = "Bit 2 - Invert output A"]
    #[inline(always)]
    pub fn a_inv(&mut self) -> A_INV_W {
        A_INV_W { w: self }
    }
    #[doc = "Bit 1 - 1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn ph_correct(&mut self) -> PH_CORRECT_W {
        PH_CORRECT_W { w: self }
    }
    #[doc = "Bit 0 - Enable the PWM channel."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control and status register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
