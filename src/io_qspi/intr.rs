#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD3_EDGE_HIGH`"]
pub type GPIO_QSPI_SD3_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SD3_EDGE_HIGH`"]
pub struct GPIO_QSPI_SD3_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD3_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD3_EDGE_LOW`"]
pub type GPIO_QSPI_SD3_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SD3_EDGE_LOW`"]
pub struct GPIO_QSPI_SD3_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD3_EDGE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD3_LEVEL_HIGH`"]
pub type GPIO_QSPI_SD3_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD3_LEVEL_LOW`"]
pub type GPIO_QSPI_SD3_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD2_EDGE_HIGH`"]
pub type GPIO_QSPI_SD2_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SD2_EDGE_HIGH`"]
pub struct GPIO_QSPI_SD2_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD2_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD2_EDGE_LOW`"]
pub type GPIO_QSPI_SD2_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SD2_EDGE_LOW`"]
pub struct GPIO_QSPI_SD2_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD2_EDGE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD2_LEVEL_HIGH`"]
pub type GPIO_QSPI_SD2_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD2_LEVEL_LOW`"]
pub type GPIO_QSPI_SD2_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD1_EDGE_HIGH`"]
pub type GPIO_QSPI_SD1_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SD1_EDGE_HIGH`"]
pub struct GPIO_QSPI_SD1_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD1_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD1_EDGE_LOW`"]
pub type GPIO_QSPI_SD1_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SD1_EDGE_LOW`"]
pub struct GPIO_QSPI_SD1_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD1_EDGE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD1_LEVEL_HIGH`"]
pub type GPIO_QSPI_SD1_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD1_LEVEL_LOW`"]
pub type GPIO_QSPI_SD1_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD0_EDGE_HIGH`"]
pub type GPIO_QSPI_SD0_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SD0_EDGE_HIGH`"]
pub struct GPIO_QSPI_SD0_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD0_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD0_EDGE_LOW`"]
pub type GPIO_QSPI_SD0_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SD0_EDGE_LOW`"]
pub struct GPIO_QSPI_SD0_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD0_EDGE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SD0_LEVEL_HIGH`"]
pub type GPIO_QSPI_SD0_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD0_LEVEL_LOW`"]
pub type GPIO_QSPI_SD0_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SS_EDGE_HIGH`"]
pub type GPIO_QSPI_SS_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SS_EDGE_HIGH`"]
pub struct GPIO_QSPI_SS_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SS_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SS_EDGE_LOW`"]
pub type GPIO_QSPI_SS_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SS_EDGE_LOW`"]
pub struct GPIO_QSPI_SS_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SS_EDGE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GPIO_QSPI_SS_LEVEL_HIGH`"]
pub type GPIO_QSPI_SS_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SS_LEVEL_LOW`"]
pub type GPIO_QSPI_SS_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SCLK_EDGE_HIGH`"]
pub type GPIO_QSPI_SCLK_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SCLK_EDGE_HIGH`"]
pub struct GPIO_QSPI_SCLK_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SCLK_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO_QSPI_SCLK_EDGE_LOW`"]
pub type GPIO_QSPI_SCLK_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_QSPI_SCLK_EDGE_LOW`"]
pub struct GPIO_QSPI_SCLK_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SCLK_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO_QSPI_SCLK_LEVEL_HIGH`"]
pub type GPIO_QSPI_SCLK_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SCLK_LEVEL_LOW`"]
pub type GPIO_QSPI_SCLK_LEVEL_LOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> GPIO_QSPI_SD3_EDGE_HIGH_R {
        GPIO_QSPI_SD3_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> GPIO_QSPI_SD3_EDGE_LOW_R {
        GPIO_QSPI_SD3_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> GPIO_QSPI_SD3_LEVEL_HIGH_R {
        GPIO_QSPI_SD3_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> GPIO_QSPI_SD3_LEVEL_LOW_R {
        GPIO_QSPI_SD3_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> GPIO_QSPI_SD2_EDGE_HIGH_R {
        GPIO_QSPI_SD2_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> GPIO_QSPI_SD2_EDGE_LOW_R {
        GPIO_QSPI_SD2_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> GPIO_QSPI_SD2_LEVEL_HIGH_R {
        GPIO_QSPI_SD2_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> GPIO_QSPI_SD2_LEVEL_LOW_R {
        GPIO_QSPI_SD2_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> GPIO_QSPI_SD1_EDGE_HIGH_R {
        GPIO_QSPI_SD1_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> GPIO_QSPI_SD1_EDGE_LOW_R {
        GPIO_QSPI_SD1_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> GPIO_QSPI_SD1_LEVEL_HIGH_R {
        GPIO_QSPI_SD1_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> GPIO_QSPI_SD1_LEVEL_LOW_R {
        GPIO_QSPI_SD1_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> GPIO_QSPI_SD0_EDGE_HIGH_R {
        GPIO_QSPI_SD0_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> GPIO_QSPI_SD0_EDGE_LOW_R {
        GPIO_QSPI_SD0_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> GPIO_QSPI_SD0_LEVEL_HIGH_R {
        GPIO_QSPI_SD0_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> GPIO_QSPI_SD0_LEVEL_LOW_R {
        GPIO_QSPI_SD0_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> GPIO_QSPI_SS_EDGE_HIGH_R {
        GPIO_QSPI_SS_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> GPIO_QSPI_SS_EDGE_LOW_R {
        GPIO_QSPI_SS_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> GPIO_QSPI_SS_LEVEL_HIGH_R {
        GPIO_QSPI_SS_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> GPIO_QSPI_SS_LEVEL_LOW_R {
        GPIO_QSPI_SS_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> GPIO_QSPI_SCLK_EDGE_HIGH_R {
        GPIO_QSPI_SCLK_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> GPIO_QSPI_SCLK_EDGE_LOW_R {
        GPIO_QSPI_SCLK_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> GPIO_QSPI_SCLK_LEVEL_HIGH_R {
        GPIO_QSPI_SCLK_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> GPIO_QSPI_SCLK_LEVEL_LOW_R {
        GPIO_QSPI_SCLK_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&mut self) -> GPIO_QSPI_SD3_EDGE_HIGH_W {
        GPIO_QSPI_SD3_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&mut self) -> GPIO_QSPI_SD3_EDGE_LOW_W {
        GPIO_QSPI_SD3_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&mut self) -> GPIO_QSPI_SD2_EDGE_HIGH_W {
        GPIO_QSPI_SD2_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&mut self) -> GPIO_QSPI_SD2_EDGE_LOW_W {
        GPIO_QSPI_SD2_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&mut self) -> GPIO_QSPI_SD1_EDGE_HIGH_W {
        GPIO_QSPI_SD1_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&mut self) -> GPIO_QSPI_SD1_EDGE_LOW_W {
        GPIO_QSPI_SD1_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&mut self) -> GPIO_QSPI_SD0_EDGE_HIGH_W {
        GPIO_QSPI_SD0_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&mut self) -> GPIO_QSPI_SD0_EDGE_LOW_W {
        GPIO_QSPI_SD0_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&mut self) -> GPIO_QSPI_SS_EDGE_HIGH_W {
        GPIO_QSPI_SS_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&mut self) -> GPIO_QSPI_SS_EDGE_LOW_W {
        GPIO_QSPI_SS_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&mut self) -> GPIO_QSPI_SCLK_EDGE_HIGH_W {
        GPIO_QSPI_SCLK_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&mut self) -> GPIO_QSPI_SCLK_EDGE_LOW_W {
        GPIO_QSPI_SCLK_EDGE_LOW_W { w: self }
    }
}
