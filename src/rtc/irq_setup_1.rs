#[doc = "Register `IRQ_SETUP_1` reader"]
pub struct R(crate::R<IRQ_SETUP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_SETUP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_SETUP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_SETUP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_SETUP_1` writer"]
pub struct W(crate::W<IRQ_SETUP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_SETUP_1_SPEC>;
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
impl From<crate::W<IRQ_SETUP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_SETUP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOTW_ENA` reader - Enable day of the week matching"]
pub struct DOTW_ENA_R(crate::FieldReader<bool, bool>);
impl DOTW_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOTW_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOTW_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOTW_ENA` writer - Enable day of the week matching"]
pub struct DOTW_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DOTW_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `HOUR_ENA` reader - Enable hour matching"]
pub struct HOUR_ENA_R(crate::FieldReader<bool, bool>);
impl HOUR_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUR_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR_ENA` writer - Enable hour matching"]
pub struct HOUR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `MIN_ENA` reader - Enable minute matching"]
pub struct MIN_ENA_R(crate::FieldReader<bool, bool>);
impl MIN_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIN_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN_ENA` writer - Enable minute matching"]
pub struct MIN_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `SEC_ENA` reader - Enable second matching"]
pub struct SEC_ENA_R(crate::FieldReader<bool, bool>);
impl SEC_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEC_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_ENA` writer - Enable second matching"]
pub struct SEC_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DOTW` reader - Day of the week"]
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
#[doc = "Field `DOTW` writer - Day of the week"]
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
    #[doc = "Bit 31 - Enable day of the week matching"]
    #[inline(always)]
    pub fn dotw_ena(&self) -> DOTW_ENA_R {
        DOTW_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable hour matching"]
    #[inline(always)]
    pub fn hour_ena(&self) -> HOUR_ENA_R {
        HOUR_ENA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable minute matching"]
    #[inline(always)]
    pub fn min_ena(&self) -> MIN_ENA_R {
        MIN_ENA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable second matching"]
    #[inline(always)]
    pub fn sec_ena(&self) -> SEC_ENA_R {
        SEC_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Day of the week"]
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
    #[doc = "Bit 31 - Enable day of the week matching"]
    #[inline(always)]
    pub fn dotw_ena(&mut self) -> DOTW_ENA_W {
        DOTW_ENA_W { w: self }
    }
    #[doc = "Bit 30 - Enable hour matching"]
    #[inline(always)]
    pub fn hour_ena(&mut self) -> HOUR_ENA_W {
        HOUR_ENA_W { w: self }
    }
    #[doc = "Bit 29 - Enable minute matching"]
    #[inline(always)]
    pub fn min_ena(&mut self) -> MIN_ENA_W {
        MIN_ENA_W { w: self }
    }
    #[doc = "Bit 28 - Enable second matching"]
    #[inline(always)]
    pub fn sec_ena(&mut self) -> SEC_ENA_W {
        SEC_ENA_W { w: self }
    }
    #[doc = "Bits 24:26 - Day of the week"]
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
#[doc = "Interrupt setup register 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [irq_setup_1](index.html) module"]
pub struct IRQ_SETUP_1_SPEC;
impl crate::RegisterSpec for IRQ_SETUP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_setup_1::R](R) reader structure"]
impl crate::Readable for IRQ_SETUP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_setup_1::W](W) writer structure"]
impl crate::Writable for IRQ_SETUP_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_SETUP_1 to value 0"]
impl crate::Resettable for IRQ_SETUP_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
