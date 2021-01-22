#[doc = "Reader of register UARTFBRD"]
pub type R = crate::R<u32, super::UARTFBRD>;
#[doc = "Writer for register UARTFBRD"]
pub type W = crate::W<u32, super::UARTFBRD>;
#[doc = "Register UARTFBRD `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTFBRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAUD_DIVFRAC`"]
pub type BAUD_DIVFRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAUD_DIVFRAC`"]
pub struct BAUD_DIVFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_DIVFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divfrac(&self) -> BAUD_DIVFRAC_R {
        BAUD_DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divfrac(&mut self) -> BAUD_DIVFRAC_W {
        BAUD_DIVFRAC_W { w: self }
    }
}
