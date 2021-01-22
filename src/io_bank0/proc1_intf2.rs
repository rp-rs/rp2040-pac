#[doc = "Reader of register PROC1_INTF2"]
pub type R = crate::R<u32, super::PROC1_INTF2>;
#[doc = "Writer for register PROC1_INTF2"]
pub type W = crate::W<u32, super::PROC1_INTF2>;
#[doc = "Register PROC1_INTF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PROC1_INTF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO23_EDGE_HIGH`"]
pub type GPIO23_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO23_EDGE_HIGH`"]
pub struct GPIO23_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO23_EDGE_LOW`"]
pub type GPIO23_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO23_EDGE_LOW`"]
pub struct GPIO23_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO23_LEVEL_HIGH`"]
pub type GPIO23_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO23_LEVEL_HIGH`"]
pub struct GPIO23_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO23_LEVEL_LOW`"]
pub type GPIO23_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO23_LEVEL_LOW`"]
pub struct GPIO23_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO22_EDGE_HIGH`"]
pub type GPIO22_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO22_EDGE_HIGH`"]
pub struct GPIO22_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO22_EDGE_LOW`"]
pub type GPIO22_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO22_EDGE_LOW`"]
pub struct GPIO22_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO22_LEVEL_HIGH`"]
pub type GPIO22_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO22_LEVEL_HIGH`"]
pub struct GPIO22_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO22_LEVEL_LOW`"]
pub type GPIO22_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO22_LEVEL_LOW`"]
pub struct GPIO22_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO21_EDGE_HIGH`"]
pub type GPIO21_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO21_EDGE_HIGH`"]
pub struct GPIO21_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO21_EDGE_LOW`"]
pub type GPIO21_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO21_EDGE_LOW`"]
pub struct GPIO21_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO21_LEVEL_HIGH`"]
pub type GPIO21_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO21_LEVEL_HIGH`"]
pub struct GPIO21_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO21_LEVEL_LOW`"]
pub type GPIO21_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO21_LEVEL_LOW`"]
pub struct GPIO21_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO20_EDGE_HIGH`"]
pub type GPIO20_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO20_EDGE_HIGH`"]
pub struct GPIO20_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO20_EDGE_LOW`"]
pub type GPIO20_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO20_EDGE_LOW`"]
pub struct GPIO20_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO20_LEVEL_HIGH`"]
pub type GPIO20_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO20_LEVEL_HIGH`"]
pub struct GPIO20_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO20_LEVEL_LOW`"]
pub type GPIO20_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO20_LEVEL_LOW`"]
pub struct GPIO20_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO19_EDGE_HIGH`"]
pub type GPIO19_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO19_EDGE_HIGH`"]
pub struct GPIO19_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO19_EDGE_LOW`"]
pub type GPIO19_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO19_EDGE_LOW`"]
pub struct GPIO19_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO19_LEVEL_HIGH`"]
pub type GPIO19_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO19_LEVEL_HIGH`"]
pub struct GPIO19_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO19_LEVEL_LOW`"]
pub type GPIO19_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO19_LEVEL_LOW`"]
pub struct GPIO19_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO18_EDGE_HIGH`"]
pub type GPIO18_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO18_EDGE_HIGH`"]
pub struct GPIO18_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO18_EDGE_LOW`"]
pub type GPIO18_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO18_EDGE_LOW`"]
pub struct GPIO18_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO18_LEVEL_HIGH`"]
pub type GPIO18_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO18_LEVEL_HIGH`"]
pub struct GPIO18_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO18_LEVEL_LOW`"]
pub type GPIO18_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO18_LEVEL_LOW`"]
pub struct GPIO18_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO17_EDGE_HIGH`"]
pub type GPIO17_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO17_EDGE_HIGH`"]
pub struct GPIO17_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO17_EDGE_LOW`"]
pub type GPIO17_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO17_EDGE_LOW`"]
pub struct GPIO17_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO17_LEVEL_HIGH`"]
pub type GPIO17_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO17_LEVEL_HIGH`"]
pub struct GPIO17_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO17_LEVEL_LOW`"]
pub type GPIO17_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO17_LEVEL_LOW`"]
pub struct GPIO17_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17_LEVEL_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO16_EDGE_HIGH`"]
pub type GPIO16_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO16_EDGE_HIGH`"]
pub struct GPIO16_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16_EDGE_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO16_EDGE_LOW`"]
pub type GPIO16_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO16_EDGE_LOW`"]
pub struct GPIO16_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16_EDGE_LOW_W<'a> {
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
#[doc = "Reader of field `GPIO16_LEVEL_HIGH`"]
pub type GPIO16_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO16_LEVEL_HIGH`"]
pub struct GPIO16_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16_LEVEL_HIGH_W<'a> {
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
#[doc = "Reader of field `GPIO16_LEVEL_LOW`"]
pub type GPIO16_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO16_LEVEL_LOW`"]
pub struct GPIO16_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16_LEVEL_LOW_W<'a> {
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
    pub fn gpio23_edge_high(&self) -> GPIO23_EDGE_HIGH_R {
        GPIO23_EDGE_HIGH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio23_edge_low(&self) -> GPIO23_EDGE_LOW_R {
        GPIO23_EDGE_LOW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio23_level_high(&self) -> GPIO23_LEVEL_HIGH_R {
        GPIO23_LEVEL_HIGH_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio23_level_low(&self) -> GPIO23_LEVEL_LOW_R {
        GPIO23_LEVEL_LOW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio22_edge_high(&self) -> GPIO22_EDGE_HIGH_R {
        GPIO22_EDGE_HIGH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio22_edge_low(&self) -> GPIO22_EDGE_LOW_R {
        GPIO22_EDGE_LOW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpio22_level_high(&self) -> GPIO22_LEVEL_HIGH_R {
        GPIO22_LEVEL_HIGH_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpio22_level_low(&self) -> GPIO22_LEVEL_LOW_R {
        GPIO22_LEVEL_LOW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio21_edge_high(&self) -> GPIO21_EDGE_HIGH_R {
        GPIO21_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio21_edge_low(&self) -> GPIO21_EDGE_LOW_R {
        GPIO21_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio21_level_high(&self) -> GPIO21_LEVEL_HIGH_R {
        GPIO21_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio21_level_low(&self) -> GPIO21_LEVEL_LOW_R {
        GPIO21_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio20_edge_high(&self) -> GPIO20_EDGE_HIGH_R {
        GPIO20_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio20_edge_low(&self) -> GPIO20_EDGE_LOW_R {
        GPIO20_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio20_level_high(&self) -> GPIO20_LEVEL_HIGH_R {
        GPIO20_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio20_level_low(&self) -> GPIO20_LEVEL_LOW_R {
        GPIO20_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio19_edge_high(&self) -> GPIO19_EDGE_HIGH_R {
        GPIO19_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio19_edge_low(&self) -> GPIO19_EDGE_LOW_R {
        GPIO19_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio19_level_high(&self) -> GPIO19_LEVEL_HIGH_R {
        GPIO19_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio19_level_low(&self) -> GPIO19_LEVEL_LOW_R {
        GPIO19_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio18_edge_high(&self) -> GPIO18_EDGE_HIGH_R {
        GPIO18_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio18_edge_low(&self) -> GPIO18_EDGE_LOW_R {
        GPIO18_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio18_level_high(&self) -> GPIO18_LEVEL_HIGH_R {
        GPIO18_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio18_level_low(&self) -> GPIO18_LEVEL_LOW_R {
        GPIO18_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio17_edge_high(&self) -> GPIO17_EDGE_HIGH_R {
        GPIO17_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio17_edge_low(&self) -> GPIO17_EDGE_LOW_R {
        GPIO17_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio17_level_high(&self) -> GPIO17_LEVEL_HIGH_R {
        GPIO17_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio17_level_low(&self) -> GPIO17_LEVEL_LOW_R {
        GPIO17_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio16_edge_high(&self) -> GPIO16_EDGE_HIGH_R {
        GPIO16_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio16_edge_low(&self) -> GPIO16_EDGE_LOW_R {
        GPIO16_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio16_level_high(&self) -> GPIO16_LEVEL_HIGH_R {
        GPIO16_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio16_level_low(&self) -> GPIO16_LEVEL_LOW_R {
        GPIO16_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio23_edge_high(&mut self) -> GPIO23_EDGE_HIGH_W {
        GPIO23_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio23_edge_low(&mut self) -> GPIO23_EDGE_LOW_W {
        GPIO23_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio23_level_high(&mut self) -> GPIO23_LEVEL_HIGH_W {
        GPIO23_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio23_level_low(&mut self) -> GPIO23_LEVEL_LOW_W {
        GPIO23_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio22_edge_high(&mut self) -> GPIO22_EDGE_HIGH_W {
        GPIO22_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio22_edge_low(&mut self) -> GPIO22_EDGE_LOW_W {
        GPIO22_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpio22_level_high(&mut self) -> GPIO22_LEVEL_HIGH_W {
        GPIO22_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpio22_level_low(&mut self) -> GPIO22_LEVEL_LOW_W {
        GPIO22_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio21_edge_high(&mut self) -> GPIO21_EDGE_HIGH_W {
        GPIO21_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio21_edge_low(&mut self) -> GPIO21_EDGE_LOW_W {
        GPIO21_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio21_level_high(&mut self) -> GPIO21_LEVEL_HIGH_W {
        GPIO21_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio21_level_low(&mut self) -> GPIO21_LEVEL_LOW_W {
        GPIO21_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio20_edge_high(&mut self) -> GPIO20_EDGE_HIGH_W {
        GPIO20_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio20_edge_low(&mut self) -> GPIO20_EDGE_LOW_W {
        GPIO20_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio20_level_high(&mut self) -> GPIO20_LEVEL_HIGH_W {
        GPIO20_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio20_level_low(&mut self) -> GPIO20_LEVEL_LOW_W {
        GPIO20_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio19_edge_high(&mut self) -> GPIO19_EDGE_HIGH_W {
        GPIO19_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio19_edge_low(&mut self) -> GPIO19_EDGE_LOW_W {
        GPIO19_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio19_level_high(&mut self) -> GPIO19_LEVEL_HIGH_W {
        GPIO19_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio19_level_low(&mut self) -> GPIO19_LEVEL_LOW_W {
        GPIO19_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio18_edge_high(&mut self) -> GPIO18_EDGE_HIGH_W {
        GPIO18_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio18_edge_low(&mut self) -> GPIO18_EDGE_LOW_W {
        GPIO18_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio18_level_high(&mut self) -> GPIO18_LEVEL_HIGH_W {
        GPIO18_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio18_level_low(&mut self) -> GPIO18_LEVEL_LOW_W {
        GPIO18_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio17_edge_high(&mut self) -> GPIO17_EDGE_HIGH_W {
        GPIO17_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio17_edge_low(&mut self) -> GPIO17_EDGE_LOW_W {
        GPIO17_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio17_level_high(&mut self) -> GPIO17_LEVEL_HIGH_W {
        GPIO17_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio17_level_low(&mut self) -> GPIO17_LEVEL_LOW_W {
        GPIO17_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio16_edge_high(&mut self) -> GPIO16_EDGE_HIGH_W {
        GPIO16_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio16_edge_low(&mut self) -> GPIO16_EDGE_LOW_W {
        GPIO16_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio16_level_high(&mut self) -> GPIO16_LEVEL_HIGH_W {
        GPIO16_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio16_level_low(&mut self) -> GPIO16_LEVEL_LOW_W {
        GPIO16_LEVEL_LOW_W { w: self }
    }
}
