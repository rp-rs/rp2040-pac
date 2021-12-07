#[doc = "Register `USB_MUXING` reader"]
pub struct R(crate::R<USB_MUXING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_MUXING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_MUXING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_MUXING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_MUXING` writer"]
pub struct W(crate::W<USB_MUXING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_MUXING_SPEC>;
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
impl From<crate::W<USB_MUXING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_MUXING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTCON` reader - "]
pub struct SOFTCON_R(crate::FieldReader<bool, bool>);
impl SOFTCON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFTCON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTCON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTCON` writer - "]
pub struct SOFTCON_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTCON_W<'a> {
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
#[doc = "Field `TO_DIGITAL_PAD` reader - "]
pub struct TO_DIGITAL_PAD_R(crate::FieldReader<bool, bool>);
impl TO_DIGITAL_PAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_DIGITAL_PAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_DIGITAL_PAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO_DIGITAL_PAD` writer - "]
pub struct TO_DIGITAL_PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_DIGITAL_PAD_W<'a> {
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
#[doc = "Field `TO_EXTPHY` reader - "]
pub struct TO_EXTPHY_R(crate::FieldReader<bool, bool>);
impl TO_EXTPHY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_EXTPHY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_EXTPHY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO_EXTPHY` writer - "]
pub struct TO_EXTPHY_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_EXTPHY_W<'a> {
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
#[doc = "Field `TO_PHY` reader - "]
pub struct TO_PHY_R(crate::FieldReader<bool, bool>);
impl TO_PHY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_PHY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_PHY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO_PHY` writer - "]
pub struct TO_PHY_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_PHY_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn softcon(&self) -> SOFTCON_R {
        SOFTCON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn to_digital_pad(&self) -> TO_DIGITAL_PAD_R {
        TO_DIGITAL_PAD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn to_extphy(&self) -> TO_EXTPHY_R {
        TO_EXTPHY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn to_phy(&self) -> TO_PHY_R {
        TO_PHY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn softcon(&mut self) -> SOFTCON_W {
        SOFTCON_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn to_digital_pad(&mut self) -> TO_DIGITAL_PAD_W {
        TO_DIGITAL_PAD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn to_extphy(&mut self) -> TO_EXTPHY_W {
        TO_EXTPHY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn to_phy(&mut self) -> TO_PHY_W {
        TO_PHY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Where to connect the USB controller. Should be to_phy by default.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [usb_muxing](index.html) module"]
pub struct USB_MUXING_SPEC;
impl crate::RegisterSpec for USB_MUXING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_muxing::R](R) reader structure"]
impl crate::Readable for USB_MUXING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_muxing::W](W) writer structure"]
impl crate::Writable for USB_MUXING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_MUXING to value 0"]
impl crate::Resettable for USB_MUXING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
