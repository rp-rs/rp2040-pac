#[doc = "Register `MEMPOWERDOWN` reader"]
pub struct R(crate::R<MEMPOWERDOWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMPOWERDOWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMPOWERDOWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMPOWERDOWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMPOWERDOWN` writer"]
pub struct W(crate::W<MEMPOWERDOWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMPOWERDOWN_SPEC>;
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
impl From<crate::W<MEMPOWERDOWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMPOWERDOWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM` reader - "]
pub struct ROM_R(crate::FieldReader<bool, bool>);
impl ROM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM` writer - "]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
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
#[doc = "Field `USB` reader - "]
pub struct USB_R(crate::FieldReader<bool, bool>);
impl USB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB` writer - "]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
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
#[doc = "Field `SRAM5` reader - "]
pub struct SRAM5_R(crate::FieldReader<bool, bool>);
impl SRAM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM5` writer - "]
pub struct SRAM5_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SRAM4` reader - "]
pub struct SRAM4_R(crate::FieldReader<bool, bool>);
impl SRAM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM4` writer - "]
pub struct SRAM4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SRAM3` reader - "]
pub struct SRAM3_R(crate::FieldReader<bool, bool>);
impl SRAM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM3` writer - "]
pub struct SRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3_W<'a> {
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
#[doc = "Field `SRAM2` reader - "]
pub struct SRAM2_R(crate::FieldReader<bool, bool>);
impl SRAM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2` writer - "]
pub struct SRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2_W<'a> {
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
#[doc = "Field `SRAM1` reader - "]
pub struct SRAM1_R(crate::FieldReader<bool, bool>);
impl SRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1` writer - "]
pub struct SRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_W<'a> {
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
#[doc = "Field `SRAM0` reader - "]
pub struct SRAM0_R(crate::FieldReader<bool, bool>);
impl SRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM0` writer - "]
pub struct SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sram5(&mut self) -> SRAM5_W {
        SRAM5_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sram4(&mut self) -> SRAM4_W {
        SRAM4_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sram3(&mut self) -> SRAM3_W {
        SRAM3_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sram2(&mut self) -> SRAM2_W {
        SRAM2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sram1(&mut self) -> SRAM1_W {
        SRAM1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sram0(&mut self) -> SRAM0_W {
        SRAM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control power downs to memories. Set high to power down memories.  
 Use with extreme caution  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [mempowerdown](index.html) module"]
pub struct MEMPOWERDOWN_SPEC;
impl crate::RegisterSpec for MEMPOWERDOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mempowerdown::R](R) reader structure"]
impl crate::Readable for MEMPOWERDOWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mempowerdown::W](W) writer structure"]
impl crate::Writable for MEMPOWERDOWN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMPOWERDOWN to value 0"]
impl crate::Resettable for MEMPOWERDOWN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
