#[doc = "Reader of register USB_MUXING"]
pub type R = crate::R<u32, super::USB_MUXING>;
#[doc = "Writer for register USB_MUXING"]
pub type W = crate::W<u32, super::USB_MUXING>;
#[doc = "Register USB_MUXING `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_MUXING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFTCON`"]
pub type SOFTCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTCON`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TO_DIGITAL_PAD`"]
pub type TO_DIGITAL_PAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TO_DIGITAL_PAD`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TO_EXTPHY`"]
pub type TO_EXTPHY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TO_EXTPHY`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TO_PHY`"]
pub type TO_PHY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TO_PHY`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
