#[doc = "Reader of register DORMANT_WAKE_INTF1"]
pub type R = crate::R<u32, super::DORMANT_WAKE_INTF1>;
#[doc = "Writer for register DORMANT_WAKE_INTF1"]
pub type W = crate::W<u32, super::DORMANT_WAKE_INTF1>;
#[doc = "Register DORMANT_WAKE_INTF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DORMANT_WAKE_INTF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO15_EDGE_HIGH`"]
pub type GPIO15_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO15_EDGE_HIGH`"]
pub struct GPIO15_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `GPIO15_EDGE_LOW`"]
pub type GPIO15_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO15_EDGE_LOW`"]
pub struct GPIO15_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15_EDGE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `GPIO15_LEVEL_HIGH`"]
pub type GPIO15_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO15_LEVEL_HIGH`"]
pub struct GPIO15_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15_LEVEL_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `GPIO15_LEVEL_LOW`"]
pub type GPIO15_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO15_LEVEL_LOW`"]
pub struct GPIO15_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15_LEVEL_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `GPIO14_EDGE_HIGH`"]
pub type GPIO14_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO14_EDGE_HIGH`"]
pub struct GPIO14_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `GPIO14_EDGE_LOW`"]
pub type GPIO14_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO14_EDGE_LOW`"]
pub struct GPIO14_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14_EDGE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `GPIO14_LEVEL_HIGH`"]
pub type GPIO14_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO14_LEVEL_HIGH`"]
pub struct GPIO14_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14_LEVEL_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `GPIO14_LEVEL_LOW`"]
pub type GPIO14_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO14_LEVEL_LOW`"]
pub struct GPIO14_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14_LEVEL_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `GPIO13_EDGE_HIGH`"]
pub type GPIO13_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO13_EDGE_HIGH`"]
pub struct GPIO13_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO13_EDGE_LOW`"]
pub type GPIO13_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO13_EDGE_LOW`"]
pub struct GPIO13_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO13_LEVEL_HIGH`"]
pub type GPIO13_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO13_LEVEL_HIGH`"]
pub struct GPIO13_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO13_LEVEL_LOW`"]
pub type GPIO13_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO13_LEVEL_LOW`"]
pub struct GPIO13_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO12_EDGE_HIGH`"]
pub type GPIO12_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO12_EDGE_HIGH`"]
pub struct GPIO12_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO12_EDGE_LOW`"]
pub type GPIO12_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO12_EDGE_LOW`"]
pub struct GPIO12_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO12_LEVEL_HIGH`"]
pub type GPIO12_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO12_LEVEL_HIGH`"]
pub struct GPIO12_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO12_LEVEL_LOW`"]
pub type GPIO12_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO12_LEVEL_LOW`"]
pub struct GPIO12_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO11_EDGE_HIGH`"]
pub type GPIO11_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO11_EDGE_HIGH`"]
pub struct GPIO11_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO11_EDGE_LOW`"]
pub type GPIO11_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO11_EDGE_LOW`"]
pub struct GPIO11_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO11_LEVEL_HIGH`"]
pub type GPIO11_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO11_LEVEL_HIGH`"]
pub struct GPIO11_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO11_LEVEL_LOW`"]
pub type GPIO11_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO11_LEVEL_LOW`"]
pub struct GPIO11_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO10_EDGE_HIGH`"]
pub type GPIO10_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO10_EDGE_HIGH`"]
pub struct GPIO10_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO10_EDGE_LOW`"]
pub type GPIO10_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO10_EDGE_LOW`"]
pub struct GPIO10_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO10_LEVEL_HIGH`"]
pub type GPIO10_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO10_LEVEL_HIGH`"]
pub struct GPIO10_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO10_LEVEL_LOW`"]
pub type GPIO10_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO10_LEVEL_LOW`"]
pub struct GPIO10_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO9_EDGE_HIGH`"]
pub type GPIO9_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO9_EDGE_HIGH`"]
pub struct GPIO9_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO9_EDGE_LOW`"]
pub type GPIO9_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO9_EDGE_LOW`"]
pub struct GPIO9_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO9_LEVEL_HIGH`"]
pub type GPIO9_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO9_LEVEL_HIGH`"]
pub struct GPIO9_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO9_LEVEL_LOW`"]
pub type GPIO9_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO9_LEVEL_LOW`"]
pub struct GPIO9_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO8_EDGE_HIGH`"]
pub type GPIO8_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO8_EDGE_HIGH`"]
pub struct GPIO8_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO8_EDGE_LOW`"]
pub type GPIO8_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO8_EDGE_LOW`"]
pub struct GPIO8_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO8_LEVEL_HIGH`"]
pub type GPIO8_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO8_LEVEL_HIGH`"]
pub struct GPIO8_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO8_LEVEL_LOW`"]
pub type GPIO8_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO8_LEVEL_LOW`"]
pub struct GPIO8_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8_LEVEL_LOW_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio15_edge_high(&self) -> GPIO15_EDGE_HIGH_R {
        GPIO15_EDGE_HIGH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio15_edge_low(&self) -> GPIO15_EDGE_LOW_R {
        GPIO15_EDGE_LOW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio15_level_high(&self) -> GPIO15_LEVEL_HIGH_R {
        GPIO15_LEVEL_HIGH_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio15_level_low(&self) -> GPIO15_LEVEL_LOW_R {
        GPIO15_LEVEL_LOW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio14_edge_high(&self) -> GPIO14_EDGE_HIGH_R {
        GPIO14_EDGE_HIGH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio14_edge_low(&self) -> GPIO14_EDGE_LOW_R {
        GPIO14_EDGE_LOW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpio14_level_high(&self) -> GPIO14_LEVEL_HIGH_R {
        GPIO14_LEVEL_HIGH_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpio14_level_low(&self) -> GPIO14_LEVEL_LOW_R {
        GPIO14_LEVEL_LOW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio13_edge_high(&self) -> GPIO13_EDGE_HIGH_R {
        GPIO13_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio13_edge_low(&self) -> GPIO13_EDGE_LOW_R {
        GPIO13_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio13_level_high(&self) -> GPIO13_LEVEL_HIGH_R {
        GPIO13_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio13_level_low(&self) -> GPIO13_LEVEL_LOW_R {
        GPIO13_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio12_edge_high(&self) -> GPIO12_EDGE_HIGH_R {
        GPIO12_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio12_edge_low(&self) -> GPIO12_EDGE_LOW_R {
        GPIO12_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio12_level_high(&self) -> GPIO12_LEVEL_HIGH_R {
        GPIO12_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio12_level_low(&self) -> GPIO12_LEVEL_LOW_R {
        GPIO12_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio11_edge_high(&self) -> GPIO11_EDGE_HIGH_R {
        GPIO11_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio11_edge_low(&self) -> GPIO11_EDGE_LOW_R {
        GPIO11_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio11_level_high(&self) -> GPIO11_LEVEL_HIGH_R {
        GPIO11_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio11_level_low(&self) -> GPIO11_LEVEL_LOW_R {
        GPIO11_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio10_edge_high(&self) -> GPIO10_EDGE_HIGH_R {
        GPIO10_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio10_edge_low(&self) -> GPIO10_EDGE_LOW_R {
        GPIO10_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio10_level_high(&self) -> GPIO10_LEVEL_HIGH_R {
        GPIO10_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio10_level_low(&self) -> GPIO10_LEVEL_LOW_R {
        GPIO10_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio9_edge_high(&self) -> GPIO9_EDGE_HIGH_R {
        GPIO9_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio9_edge_low(&self) -> GPIO9_EDGE_LOW_R {
        GPIO9_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio9_level_high(&self) -> GPIO9_LEVEL_HIGH_R {
        GPIO9_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio9_level_low(&self) -> GPIO9_LEVEL_LOW_R {
        GPIO9_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio8_edge_high(&self) -> GPIO8_EDGE_HIGH_R {
        GPIO8_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio8_edge_low(&self) -> GPIO8_EDGE_LOW_R {
        GPIO8_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio8_level_high(&self) -> GPIO8_LEVEL_HIGH_R {
        GPIO8_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio8_level_low(&self) -> GPIO8_LEVEL_LOW_R {
        GPIO8_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio15_edge_high(&mut self) -> GPIO15_EDGE_HIGH_W {
        GPIO15_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio15_edge_low(&mut self) -> GPIO15_EDGE_LOW_W {
        GPIO15_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio15_level_high(&mut self) -> GPIO15_LEVEL_HIGH_W {
        GPIO15_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio15_level_low(&mut self) -> GPIO15_LEVEL_LOW_W {
        GPIO15_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio14_edge_high(&mut self) -> GPIO14_EDGE_HIGH_W {
        GPIO14_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio14_edge_low(&mut self) -> GPIO14_EDGE_LOW_W {
        GPIO14_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpio14_level_high(&mut self) -> GPIO14_LEVEL_HIGH_W {
        GPIO14_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpio14_level_low(&mut self) -> GPIO14_LEVEL_LOW_W {
        GPIO14_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio13_edge_high(&mut self) -> GPIO13_EDGE_HIGH_W {
        GPIO13_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio13_edge_low(&mut self) -> GPIO13_EDGE_LOW_W {
        GPIO13_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio13_level_high(&mut self) -> GPIO13_LEVEL_HIGH_W {
        GPIO13_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio13_level_low(&mut self) -> GPIO13_LEVEL_LOW_W {
        GPIO13_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio12_edge_high(&mut self) -> GPIO12_EDGE_HIGH_W {
        GPIO12_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio12_edge_low(&mut self) -> GPIO12_EDGE_LOW_W {
        GPIO12_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio12_level_high(&mut self) -> GPIO12_LEVEL_HIGH_W {
        GPIO12_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio12_level_low(&mut self) -> GPIO12_LEVEL_LOW_W {
        GPIO12_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio11_edge_high(&mut self) -> GPIO11_EDGE_HIGH_W {
        GPIO11_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio11_edge_low(&mut self) -> GPIO11_EDGE_LOW_W {
        GPIO11_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio11_level_high(&mut self) -> GPIO11_LEVEL_HIGH_W {
        GPIO11_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio11_level_low(&mut self) -> GPIO11_LEVEL_LOW_W {
        GPIO11_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio10_edge_high(&mut self) -> GPIO10_EDGE_HIGH_W {
        GPIO10_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio10_edge_low(&mut self) -> GPIO10_EDGE_LOW_W {
        GPIO10_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio10_level_high(&mut self) -> GPIO10_LEVEL_HIGH_W {
        GPIO10_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio10_level_low(&mut self) -> GPIO10_LEVEL_LOW_W {
        GPIO10_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio9_edge_high(&mut self) -> GPIO9_EDGE_HIGH_W {
        GPIO9_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio9_edge_low(&mut self) -> GPIO9_EDGE_LOW_W {
        GPIO9_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio9_level_high(&mut self) -> GPIO9_LEVEL_HIGH_W {
        GPIO9_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio9_level_low(&mut self) -> GPIO9_LEVEL_LOW_W {
        GPIO9_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio8_edge_high(&mut self) -> GPIO8_EDGE_HIGH_W {
        GPIO8_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio8_edge_low(&mut self) -> GPIO8_EDGE_LOW_W {
        GPIO8_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio8_level_high(&mut self) -> GPIO8_LEVEL_HIGH_W {
        GPIO8_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio8_level_low(&mut self) -> GPIO8_LEVEL_LOW_W {
        GPIO8_LEVEL_LOW_W { w: self }
    }
}
