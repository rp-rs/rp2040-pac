#[doc = "Reader of register INTE1"]
pub type R = crate::R<u32, super::INTE1>;
#[doc = "Writer for register INTE1"]
pub type W = crate::W<u32, super::INTE1>;
#[doc = "Register INTE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTE1`"]
pub type INTE1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTE1`"]
pub struct INTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    #[inline(always)]
    pub fn inte1(&self) -> INTE1_R {
        INTE1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    #[inline(always)]
    pub fn inte1(&mut self) -> INTE1_W {
        INTE1_W { w: self }
    }
}
