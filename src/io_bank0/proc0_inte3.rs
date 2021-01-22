#[doc = "Reader of register PROC0_INTE3"]
pub type R = crate::R<u32, super::PROC0_INTE3>;
#[doc = "Writer for register PROC0_INTE3"]
pub type W = crate::W<u32, super::PROC0_INTE3>;
#[doc = "Register PROC0_INTE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PROC0_INTE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO29_EDGE_HIGH`"]
pub type GPIO29_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO29_EDGE_HIGH`"]
pub struct GPIO29_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO29_EDGE_LOW`"]
pub type GPIO29_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO29_EDGE_LOW`"]
pub struct GPIO29_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO29_LEVEL_HIGH`"]
pub type GPIO29_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO29_LEVEL_HIGH`"]
pub struct GPIO29_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29_LEVEL_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `GPIO29_LEVEL_LOW`"]
pub type GPIO29_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO29_LEVEL_LOW`"]
pub struct GPIO29_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29_LEVEL_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `GPIO28_EDGE_HIGH`"]
pub type GPIO28_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO28_EDGE_HIGH`"]
pub struct GPIO28_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO28_EDGE_LOW`"]
pub type GPIO28_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO28_EDGE_LOW`"]
pub struct GPIO28_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO28_LEVEL_HIGH`"]
pub type GPIO28_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO28_LEVEL_HIGH`"]
pub struct GPIO28_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28_LEVEL_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `GPIO28_LEVEL_LOW`"]
pub type GPIO28_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO28_LEVEL_LOW`"]
pub struct GPIO28_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28_LEVEL_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `GPIO27_EDGE_HIGH`"]
pub type GPIO27_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO27_EDGE_HIGH`"]
pub struct GPIO27_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO27_EDGE_LOW`"]
pub type GPIO27_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO27_EDGE_LOW`"]
pub struct GPIO27_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO27_LEVEL_HIGH`"]
pub type GPIO27_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO27_LEVEL_HIGH`"]
pub struct GPIO27_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27_LEVEL_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `GPIO27_LEVEL_LOW`"]
pub type GPIO27_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO27_LEVEL_LOW`"]
pub struct GPIO27_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27_LEVEL_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `GPIO26_EDGE_HIGH`"]
pub type GPIO26_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO26_EDGE_HIGH`"]
pub struct GPIO26_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO26_EDGE_LOW`"]
pub type GPIO26_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO26_EDGE_LOW`"]
pub struct GPIO26_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO26_LEVEL_HIGH`"]
pub type GPIO26_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO26_LEVEL_HIGH`"]
pub struct GPIO26_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26_LEVEL_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `GPIO26_LEVEL_LOW`"]
pub type GPIO26_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO26_LEVEL_LOW`"]
pub struct GPIO26_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26_LEVEL_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `GPIO25_EDGE_HIGH`"]
pub type GPIO25_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO25_EDGE_HIGH`"]
pub struct GPIO25_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO25_EDGE_LOW`"]
pub type GPIO25_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO25_EDGE_LOW`"]
pub struct GPIO25_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO25_LEVEL_HIGH`"]
pub type GPIO25_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO25_LEVEL_HIGH`"]
pub struct GPIO25_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO25_LEVEL_LOW`"]
pub type GPIO25_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO25_LEVEL_LOW`"]
pub struct GPIO25_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO24_EDGE_HIGH`"]
pub type GPIO24_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO24_EDGE_HIGH`"]
pub struct GPIO24_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO24_EDGE_LOW`"]
pub type GPIO24_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO24_EDGE_LOW`"]
pub struct GPIO24_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO24_LEVEL_HIGH`"]
pub type GPIO24_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO24_LEVEL_HIGH`"]
pub struct GPIO24_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO24_LEVEL_LOW`"]
pub type GPIO24_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO24_LEVEL_LOW`"]
pub struct GPIO24_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24_LEVEL_LOW_W<'a> {
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
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio29_edge_high(&self) -> GPIO29_EDGE_HIGH_R {
        GPIO29_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio29_edge_low(&self) -> GPIO29_EDGE_LOW_R {
        GPIO29_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio29_level_high(&self) -> GPIO29_LEVEL_HIGH_R {
        GPIO29_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio29_level_low(&self) -> GPIO29_LEVEL_LOW_R {
        GPIO29_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio28_edge_high(&self) -> GPIO28_EDGE_HIGH_R {
        GPIO28_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio28_edge_low(&self) -> GPIO28_EDGE_LOW_R {
        GPIO28_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio28_level_high(&self) -> GPIO28_LEVEL_HIGH_R {
        GPIO28_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio28_level_low(&self) -> GPIO28_LEVEL_LOW_R {
        GPIO28_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio27_edge_high(&self) -> GPIO27_EDGE_HIGH_R {
        GPIO27_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio27_edge_low(&self) -> GPIO27_EDGE_LOW_R {
        GPIO27_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio27_level_high(&self) -> GPIO27_LEVEL_HIGH_R {
        GPIO27_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio27_level_low(&self) -> GPIO27_LEVEL_LOW_R {
        GPIO27_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio26_edge_high(&self) -> GPIO26_EDGE_HIGH_R {
        GPIO26_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio26_edge_low(&self) -> GPIO26_EDGE_LOW_R {
        GPIO26_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio26_level_high(&self) -> GPIO26_LEVEL_HIGH_R {
        GPIO26_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio26_level_low(&self) -> GPIO26_LEVEL_LOW_R {
        GPIO26_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio25_edge_high(&self) -> GPIO25_EDGE_HIGH_R {
        GPIO25_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio25_edge_low(&self) -> GPIO25_EDGE_LOW_R {
        GPIO25_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio25_level_high(&self) -> GPIO25_LEVEL_HIGH_R {
        GPIO25_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio25_level_low(&self) -> GPIO25_LEVEL_LOW_R {
        GPIO25_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio24_edge_high(&self) -> GPIO24_EDGE_HIGH_R {
        GPIO24_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio24_edge_low(&self) -> GPIO24_EDGE_LOW_R {
        GPIO24_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio24_level_high(&self) -> GPIO24_LEVEL_HIGH_R {
        GPIO24_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio24_level_low(&self) -> GPIO24_LEVEL_LOW_R {
        GPIO24_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio29_edge_high(&mut self) -> GPIO29_EDGE_HIGH_W {
        GPIO29_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio29_edge_low(&mut self) -> GPIO29_EDGE_LOW_W {
        GPIO29_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio29_level_high(&mut self) -> GPIO29_LEVEL_HIGH_W {
        GPIO29_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio29_level_low(&mut self) -> GPIO29_LEVEL_LOW_W {
        GPIO29_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio28_edge_high(&mut self) -> GPIO28_EDGE_HIGH_W {
        GPIO28_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio28_edge_low(&mut self) -> GPIO28_EDGE_LOW_W {
        GPIO28_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio28_level_high(&mut self) -> GPIO28_LEVEL_HIGH_W {
        GPIO28_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio28_level_low(&mut self) -> GPIO28_LEVEL_LOW_W {
        GPIO28_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio27_edge_high(&mut self) -> GPIO27_EDGE_HIGH_W {
        GPIO27_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio27_edge_low(&mut self) -> GPIO27_EDGE_LOW_W {
        GPIO27_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio27_level_high(&mut self) -> GPIO27_LEVEL_HIGH_W {
        GPIO27_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio27_level_low(&mut self) -> GPIO27_LEVEL_LOW_W {
        GPIO27_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio26_edge_high(&mut self) -> GPIO26_EDGE_HIGH_W {
        GPIO26_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio26_edge_low(&mut self) -> GPIO26_EDGE_LOW_W {
        GPIO26_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio26_level_high(&mut self) -> GPIO26_LEVEL_HIGH_W {
        GPIO26_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio26_level_low(&mut self) -> GPIO26_LEVEL_LOW_W {
        GPIO26_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio25_edge_high(&mut self) -> GPIO25_EDGE_HIGH_W {
        GPIO25_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio25_edge_low(&mut self) -> GPIO25_EDGE_LOW_W {
        GPIO25_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio25_level_high(&mut self) -> GPIO25_LEVEL_HIGH_W {
        GPIO25_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio25_level_low(&mut self) -> GPIO25_LEVEL_LOW_W {
        GPIO25_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio24_edge_high(&mut self) -> GPIO24_EDGE_HIGH_W {
        GPIO24_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio24_edge_low(&mut self) -> GPIO24_EDGE_LOW_W {
        GPIO24_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio24_level_high(&mut self) -> GPIO24_LEVEL_HIGH_W {
        GPIO24_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio24_level_low(&mut self) -> GPIO24_LEVEL_LOW_W {
        GPIO24_LEVEL_LOW_W { w: self }
    }
}
