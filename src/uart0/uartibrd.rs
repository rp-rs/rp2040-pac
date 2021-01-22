#[doc = "Reader of register UARTIBRD"]
pub type R = crate::R<u32, super::UARTIBRD>;
#[doc = "Writer for register UARTIBRD"]
pub type W = crate::W<u32, super::UARTIBRD>;
#[doc = "Register UARTIBRD `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTIBRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAUD_DIVINT`"]
pub type BAUD_DIVINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BAUD_DIVINT`"]
pub struct BAUD_DIVINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_DIVINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divint(&self) -> BAUD_DIVINT_R {
        BAUD_DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divint(&mut self) -> BAUD_DIVINT_W {
        BAUD_DIVINT_W { w: self }
    }
}
