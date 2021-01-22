#[doc = "Reader of register INTF1"]
pub type R = crate::R<u32, super::INTF1>;
#[doc = "Writer for register INTF1"]
pub type W = crate::W<u32, super::INTF1>;
#[doc = "Register INTF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTF1`"]
pub type INTF1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTF1`"]
pub struct INTF1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub fn intf1(&self) -> INTF1_R {
        INTF1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub fn intf1(&mut self) -> INTF1_W {
        INTF1_W { w: self }
    }
}
