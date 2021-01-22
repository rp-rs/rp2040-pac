#[doc = "Reader of register INTE0"]
pub type R = crate::R<u32, super::INTE0>;
#[doc = "Writer for register INTE0"]
pub type W = crate::W<u32, super::INTE0>;
#[doc = "Register INTE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTE0`"]
pub type INTE0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTE0`"]
pub struct INTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
    #[inline(always)]
    pub fn inte0(&self) -> INTE0_R {
        INTE0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
    #[inline(always)]
    pub fn inte0(&mut self) -> INTE0_W {
        INTE0_W { w: self }
    }
}
