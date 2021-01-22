#[doc = "Reader of register EP_STATUS_STALL_NAK"]
pub type R = crate::R<u32, super::EP_STATUS_STALL_NAK>;
#[doc = "Writer for register EP_STATUS_STALL_NAK"]
pub type W = crate::W<u32, super::EP_STATUS_STALL_NAK>;
#[doc = "Register EP_STATUS_STALL_NAK `reset()`'s with value 0"]
impl crate::ResetValue for super::EP_STATUS_STALL_NAK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP15_OUT`"]
pub type EP15_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP15_OUT`"]
pub struct EP15_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP15_OUT_W<'a> {
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
#[doc = "Reader of field `EP15_IN`"]
pub type EP15_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP15_IN`"]
pub struct EP15_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP15_IN_W<'a> {
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
#[doc = "Reader of field `EP14_OUT`"]
pub type EP14_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP14_OUT`"]
pub struct EP14_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP14_OUT_W<'a> {
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
#[doc = "Reader of field `EP14_IN`"]
pub type EP14_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP14_IN`"]
pub struct EP14_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP14_IN_W<'a> {
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
#[doc = "Reader of field `EP13_OUT`"]
pub type EP13_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP13_OUT`"]
pub struct EP13_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP13_OUT_W<'a> {
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
#[doc = "Reader of field `EP13_IN`"]
pub type EP13_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP13_IN`"]
pub struct EP13_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP13_IN_W<'a> {
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
#[doc = "Reader of field `EP12_OUT`"]
pub type EP12_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP12_OUT`"]
pub struct EP12_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP12_OUT_W<'a> {
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
#[doc = "Reader of field `EP12_IN`"]
pub type EP12_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP12_IN`"]
pub struct EP12_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP12_IN_W<'a> {
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
#[doc = "Reader of field `EP11_OUT`"]
pub type EP11_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP11_OUT`"]
pub struct EP11_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP11_OUT_W<'a> {
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
#[doc = "Reader of field `EP11_IN`"]
pub type EP11_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP11_IN`"]
pub struct EP11_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP11_IN_W<'a> {
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
#[doc = "Reader of field `EP10_OUT`"]
pub type EP10_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP10_OUT`"]
pub struct EP10_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP10_OUT_W<'a> {
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
#[doc = "Reader of field `EP10_IN`"]
pub type EP10_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP10_IN`"]
pub struct EP10_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP10_IN_W<'a> {
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
#[doc = "Reader of field `EP9_OUT`"]
pub type EP9_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP9_OUT`"]
pub struct EP9_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP9_OUT_W<'a> {
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
#[doc = "Reader of field `EP9_IN`"]
pub type EP9_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP9_IN`"]
pub struct EP9_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP9_IN_W<'a> {
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
#[doc = "Reader of field `EP8_OUT`"]
pub type EP8_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP8_OUT`"]
pub struct EP8_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_OUT_W<'a> {
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
#[doc = "Reader of field `EP8_IN`"]
pub type EP8_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP8_IN`"]
pub struct EP8_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_IN_W<'a> {
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
#[doc = "Reader of field `EP7_OUT`"]
pub type EP7_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7_OUT`"]
pub struct EP7_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_OUT_W<'a> {
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
#[doc = "Reader of field `EP7_IN`"]
pub type EP7_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7_IN`"]
pub struct EP7_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_IN_W<'a> {
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
#[doc = "Reader of field `EP6_OUT`"]
pub type EP6_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6_OUT`"]
pub struct EP6_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_OUT_W<'a> {
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
#[doc = "Reader of field `EP6_IN`"]
pub type EP6_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6_IN`"]
pub struct EP6_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_IN_W<'a> {
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
#[doc = "Reader of field `EP5_OUT`"]
pub type EP5_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5_OUT`"]
pub struct EP5_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_OUT_W<'a> {
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
#[doc = "Reader of field `EP5_IN`"]
pub type EP5_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5_IN`"]
pub struct EP5_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_IN_W<'a> {
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
#[doc = "Reader of field `EP4_OUT`"]
pub type EP4_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4_OUT`"]
pub struct EP4_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_OUT_W<'a> {
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
#[doc = "Reader of field `EP4_IN`"]
pub type EP4_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4_IN`"]
pub struct EP4_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_IN_W<'a> {
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
#[doc = "Reader of field `EP3_OUT`"]
pub type EP3_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3_OUT`"]
pub struct EP3_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_OUT_W<'a> {
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
#[doc = "Reader of field `EP3_IN`"]
pub type EP3_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3_IN`"]
pub struct EP3_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_IN_W<'a> {
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
#[doc = "Reader of field `EP2_OUT`"]
pub type EP2_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2_OUT`"]
pub struct EP2_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_OUT_W<'a> {
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
#[doc = "Reader of field `EP2_IN`"]
pub type EP2_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2_IN`"]
pub struct EP2_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_IN_W<'a> {
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
#[doc = "Reader of field `EP1_OUT`"]
pub type EP1_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_OUT`"]
pub struct EP1_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_OUT_W<'a> {
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
#[doc = "Reader of field `EP1_IN`"]
pub type EP1_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_IN`"]
pub struct EP1_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_IN_W<'a> {
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
#[doc = "Reader of field `EP0_OUT`"]
pub type EP0_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_OUT`"]
pub struct EP0_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_OUT_W<'a> {
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
#[doc = "Reader of field `EP0_IN`"]
pub type EP0_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_IN`"]
pub struct EP0_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_IN_W<'a> {
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
    pub fn ep15_out(&self) -> EP15_OUT_R {
        EP15_OUT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep15_in(&self) -> EP15_IN_R {
        EP15_IN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ep14_out(&self) -> EP14_OUT_R {
        EP14_OUT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ep14_in(&self) -> EP14_IN_R {
        EP14_IN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ep13_out(&self) -> EP13_OUT_R {
        EP13_OUT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ep13_in(&self) -> EP13_IN_R {
        EP13_IN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ep12_out(&self) -> EP12_OUT_R {
        EP12_OUT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ep12_in(&self) -> EP12_IN_R {
        EP12_IN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep11_out(&self) -> EP11_OUT_R {
        EP11_OUT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep11_in(&self) -> EP11_IN_R {
        EP11_IN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep10_out(&self) -> EP10_OUT_R {
        EP10_OUT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep10_in(&self) -> EP10_IN_R {
        EP10_IN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep9_out(&self) -> EP9_OUT_R {
        EP9_OUT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep9_in(&self) -> EP9_IN_R {
        EP9_IN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep8_out(&self) -> EP8_OUT_R {
        EP8_OUT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep8_in(&self) -> EP8_IN_R {
        EP8_IN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep7_out(&self) -> EP7_OUT_R {
        EP7_OUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep7_in(&self) -> EP7_IN_R {
        EP7_IN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep6_out(&self) -> EP6_OUT_R {
        EP6_OUT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep6_in(&self) -> EP6_IN_R {
        EP6_IN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep5_out(&self) -> EP5_OUT_R {
        EP5_OUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep5_in(&self) -> EP5_IN_R {
        EP5_IN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep4_out(&self) -> EP4_OUT_R {
        EP4_OUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep4_in(&self) -> EP4_IN_R {
        EP4_IN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep3_out(&self) -> EP3_OUT_R {
        EP3_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep3_in(&self) -> EP3_IN_R {
        EP3_IN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep2_out(&self) -> EP2_OUT_R {
        EP2_OUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep2_in(&self) -> EP2_IN_R {
        EP2_IN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ep1_out(&self) -> EP1_OUT_R {
        EP1_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ep1_in(&self) -> EP1_IN_R {
        EP1_IN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&self) -> EP0_OUT_R {
        EP0_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&self) -> EP0_IN_R {
        EP0_IN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep15_out(&mut self) -> EP15_OUT_W {
        EP15_OUT_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep15_in(&mut self) -> EP15_IN_W {
        EP15_IN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ep14_out(&mut self) -> EP14_OUT_W {
        EP14_OUT_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ep14_in(&mut self) -> EP14_IN_W {
        EP14_IN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ep13_out(&mut self) -> EP13_OUT_W {
        EP13_OUT_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ep13_in(&mut self) -> EP13_IN_W {
        EP13_IN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ep12_out(&mut self) -> EP12_OUT_W {
        EP12_OUT_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ep12_in(&mut self) -> EP12_IN_W {
        EP12_IN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep11_out(&mut self) -> EP11_OUT_W {
        EP11_OUT_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep11_in(&mut self) -> EP11_IN_W {
        EP11_IN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep10_out(&mut self) -> EP10_OUT_W {
        EP10_OUT_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep10_in(&mut self) -> EP10_IN_W {
        EP10_IN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep9_out(&mut self) -> EP9_OUT_W {
        EP9_OUT_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep9_in(&mut self) -> EP9_IN_W {
        EP9_IN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep8_out(&mut self) -> EP8_OUT_W {
        EP8_OUT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep8_in(&mut self) -> EP8_IN_W {
        EP8_IN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep7_out(&mut self) -> EP7_OUT_W {
        EP7_OUT_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep7_in(&mut self) -> EP7_IN_W {
        EP7_IN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep6_out(&mut self) -> EP6_OUT_W {
        EP6_OUT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep6_in(&mut self) -> EP6_IN_W {
        EP6_IN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep5_out(&mut self) -> EP5_OUT_W {
        EP5_OUT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep5_in(&mut self) -> EP5_IN_W {
        EP5_IN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep4_out(&mut self) -> EP4_OUT_W {
        EP4_OUT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep4_in(&mut self) -> EP4_IN_W {
        EP4_IN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep3_out(&mut self) -> EP3_OUT_W {
        EP3_OUT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep3_in(&mut self) -> EP3_IN_W {
        EP3_IN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep2_out(&mut self) -> EP2_OUT_W {
        EP2_OUT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep2_in(&mut self) -> EP2_IN_W {
        EP2_IN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ep1_out(&mut self) -> EP1_OUT_W {
        EP1_OUT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ep1_in(&mut self) -> EP1_IN_W {
        EP1_IN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&mut self) -> EP0_OUT_W {
        EP0_OUT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&mut self) -> EP0_IN_W {
        EP0_IN_W { w: self }
    }
}
