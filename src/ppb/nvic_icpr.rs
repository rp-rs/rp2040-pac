#[doc = "Reader of register NVIC_ICPR"]
pub type R = crate::R<u32, super::NVIC_ICPR>;
#[doc = "Writer for register NVIC_ICPR"]
pub type W = crate::W<u32, super::NVIC_ICPR>;
#[doc = "Register NVIC_ICPR `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ICPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRPEND`"]
pub type CLRPEND_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLRPEND`"]
pub struct CLRPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-pending bits.\\n Write:\\n 0 = No effect.\\n 1 = Removes pending state and interrupt.\\n Read:\\n 0 = Interrupt is not pending.\\n 1 = Interrupt is pending."]
    #[inline(always)]
    pub fn clrpend(&self) -> CLRPEND_R {
        CLRPEND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-pending bits.\\n Write:\\n 0 = No effect.\\n 1 = Removes pending state and interrupt.\\n Read:\\n 0 = Interrupt is not pending.\\n 1 = Interrupt is pending."]
    #[inline(always)]
    pub fn clrpend(&mut self) -> CLRPEND_W {
        CLRPEND_W { w: self }
    }
}
