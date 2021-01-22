#[doc = "Reader of register INT_EP_CTRL"]
pub type R = crate::R<u32, super::INT_EP_CTRL>;
#[doc = "Writer for register INT_EP_CTRL"]
pub type W = crate::W<u32, super::INT_EP_CTRL>;
#[doc = "Register INT_EP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT_EP_ACTIVE`"]
pub type INT_EP_ACTIVE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INT_EP_ACTIVE`"]
pub struct INT_EP_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EP_ACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | (((value as u32) & 0x7fff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:15 - Host: Enable interrupt endpoint 1 -> 15"]
    #[inline(always)]
    pub fn int_ep_active(&self) -> INT_EP_ACTIVE_R {
        INT_EP_ACTIVE_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:15 - Host: Enable interrupt endpoint 1 -> 15"]
    #[inline(always)]
    pub fn int_ep_active(&mut self) -> INT_EP_ACTIVE_W {
        INT_EP_ACTIVE_W { w: self }
    }
}
