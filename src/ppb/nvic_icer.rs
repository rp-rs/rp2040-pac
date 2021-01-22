#[doc = "Reader of register NVIC_ICER"]
pub type R = crate::R<u32, super::NVIC_ICER>;
#[doc = "Writer for register NVIC_ICER"]
pub type W = crate::W<u32, super::NVIC_ICER>;
#[doc = "Register NVIC_ICER `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ICER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRENA`"]
pub type CLRENA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLRENA`"]
pub struct CLRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits.\\n Write:\\n 0 = No effect.\\n 1 = Disable interrupt.\\n Read:\\n 0 = Interrupt disabled.\\n 1 = Interrupt enabled."]
    #[inline(always)]
    pub fn clrena(&self) -> CLRENA_R {
        CLRENA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits.\\n Write:\\n 0 = No effect.\\n 1 = Disable interrupt.\\n Read:\\n 0 = Interrupt disabled.\\n 1 = Interrupt enabled."]
    #[inline(always)]
    pub fn clrena(&mut self) -> CLRENA_W {
        CLRENA_W { w: self }
    }
}
