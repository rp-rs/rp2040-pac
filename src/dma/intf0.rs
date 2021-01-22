#[doc = "Reader of register INTF0"]
pub type R = crate::R<u32, super::INTF0>;
#[doc = "Writer for register INTF0"]
pub type W = crate::W<u32, super::INTF0>;
#[doc = "Register INTF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTF0`"]
pub type INTF0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTF0`"]
pub struct INTF0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTF0_W<'a> {
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
    pub fn intf0(&self) -> INTF0_R {
        INTF0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub fn intf0(&mut self) -> INTF0_W {
        INTF0_W { w: self }
    }
}
