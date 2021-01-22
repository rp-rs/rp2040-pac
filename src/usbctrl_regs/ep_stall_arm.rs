#[doc = "Reader of register EP_STALL_ARM"]
pub type R = crate::R<u32, super::EP_STALL_ARM>;
#[doc = "Writer for register EP_STALL_ARM"]
pub type W = crate::W<u32, super::EP_STALL_ARM>;
#[doc = "Register EP_STALL_ARM `reset()`'s with value 0"]
impl crate::ResetValue for super::EP_STALL_ARM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
