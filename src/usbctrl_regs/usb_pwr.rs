#[doc = "Reader of register USB_PWR"]
pub type R = crate::R<u32, super::USB_PWR>;
#[doc = "Writer for register USB_PWR"]
pub type W = crate::W<u32, super::USB_PWR>;
#[doc = "Register USB_PWR `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_PWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERCURR_DETECT_EN`"]
pub type OVERCURR_DETECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERCURR_DETECT_EN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `OVERCURR_DETECT`"]
pub type OVERCURR_DETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERCURR_DETECT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `VBUS_DETECT_OVERRIDE_EN`"]
pub type VBUS_DETECT_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUS_DETECT_OVERRIDE_EN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `VBUS_DETECT`"]
pub type VBUS_DETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUS_DETECT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `VBUS_EN_OVERRIDE_EN`"]
pub type VBUS_EN_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUS_EN_OVERRIDE_EN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `VBUS_EN`"]
pub type VBUS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUS_EN`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
