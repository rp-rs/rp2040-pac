#[doc = "Register `USB_PWR` reader"]
pub struct R(crate::R<USB_PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_PWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_PWR` writer"]
pub struct W(crate::W<USB_PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_PWR_SPEC>;
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
impl From<crate::W<USB_PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_PWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERCURR_DETECT_EN` reader - "]
pub struct OVERCURR_DETECT_EN_R(crate::FieldReader<bool, bool>);
impl OVERCURR_DETECT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERCURR_DETECT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERCURR_DETECT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERCURR_DETECT_EN` writer - "]
pub struct OVERCURR_DETECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERCURR_DETECT_EN_W<'a> {
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
#[doc = "Field `OVERCURR_DETECT` reader - "]
pub struct OVERCURR_DETECT_R(crate::FieldReader<bool, bool>);
impl OVERCURR_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERCURR_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERCURR_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERCURR_DETECT` writer - "]
pub struct OVERCURR_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERCURR_DETECT_W<'a> {
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
#[doc = "Field `VBUS_DETECT_OVERRIDE_EN` reader - "]
pub struct VBUS_DETECT_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl VBUS_DETECT_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_DETECT_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_DETECT_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS_DETECT_OVERRIDE_EN` writer - "]
pub struct VBUS_DETECT_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_DETECT_OVERRIDE_EN_W<'a> {
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
#[doc = "Field `VBUS_DETECT` reader - "]
pub struct VBUS_DETECT_R(crate::FieldReader<bool, bool>);
impl VBUS_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS_DETECT` writer - "]
pub struct VBUS_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_DETECT_W<'a> {
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
#[doc = "Field `VBUS_EN_OVERRIDE_EN` reader - "]
pub struct VBUS_EN_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl VBUS_EN_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_EN_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_EN_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS_EN_OVERRIDE_EN` writer - "]
pub struct VBUS_EN_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_EN_OVERRIDE_EN_W<'a> {
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
#[doc = "Field `VBUS_EN` reader - "]
pub struct VBUS_EN_R(crate::FieldReader<bool, bool>);
impl VBUS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS_EN` writer - "]
pub struct VBUS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_EN_W<'a> {
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn overcurr_detect_en(&self) -> OVERCURR_DETECT_EN_R {
        OVERCURR_DETECT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn overcurr_detect(&self) -> OVERCURR_DETECT_R {
        OVERCURR_DETECT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbus_detect_override_en(&self) -> VBUS_DETECT_OVERRIDE_EN_R {
        VBUS_DETECT_OVERRIDE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vbus_detect(&self) -> VBUS_DETECT_R {
        VBUS_DETECT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vbus_en_override_en(&self) -> VBUS_EN_OVERRIDE_EN_R {
        VBUS_EN_OVERRIDE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbus_en(&self) -> VBUS_EN_R {
        VBUS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn overcurr_detect_en(&mut self) -> OVERCURR_DETECT_EN_W {
        OVERCURR_DETECT_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn overcurr_detect(&mut self) -> OVERCURR_DETECT_W {
        OVERCURR_DETECT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbus_detect_override_en(&mut self) -> VBUS_DETECT_OVERRIDE_EN_W {
        VBUS_DETECT_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vbus_detect(&mut self) -> VBUS_DETECT_W {
        VBUS_DETECT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vbus_en_override_en(&mut self) -> VBUS_EN_OVERRIDE_EN_W {
        VBUS_EN_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbus_en(&mut self) -> VBUS_EN_W {
        VBUS_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [usb_pwr](index.html) module"]
pub struct USB_PWR_SPEC;
impl crate::RegisterSpec for USB_PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_pwr::R](R) reader structure"]
impl crate::Readable for USB_PWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_pwr::W](W) writer structure"]
impl crate::Writable for USB_PWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_PWR to value 0"]
impl crate::Resettable for USB_PWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
