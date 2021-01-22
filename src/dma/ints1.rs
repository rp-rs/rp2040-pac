#[doc = "Reader of register INTS1"]
pub type R = crate::R<u32, super::INTS1>;
#[doc = "Writer for register INTS1"]
pub type W = crate::W<u32, super::INTS1>;
#[doc = "Register INTS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTS1`"]
pub type INTS1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTS1`"]
pub struct INTS1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.\\n Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    pub fn ints1(&self) -> INTS1_R {
        INTS1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.\\n Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    pub fn ints1(&mut self) -> INTS1_W {
        INTS1_W { w: self }
    }
}
