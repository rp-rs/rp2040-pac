#[doc = "Reader of register INTS0"]
pub type R = crate::R<u32, super::INTS0>;
#[doc = "Writer for register INTS0"]
pub type W = crate::W<u32, super::INTS0>;
#[doc = "Register INTS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTS0`"]
pub type INTS0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTS0`"]
pub struct INTS0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted.\\n Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    pub fn ints0(&self) -> INTS0_R {
        INTS0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted.\\n Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    pub fn ints0(&mut self) -> INTS0_W {
        INTS0_W { w: self }
    }
}
