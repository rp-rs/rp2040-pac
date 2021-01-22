#[doc = "Reader of register UARTILPR"]
pub type R = crate::R<u32, super::UARTILPR>;
#[doc = "Writer for register UARTILPR"]
pub type W = crate::W<u32, super::UARTILPR>;
#[doc = "Register UARTILPR `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTILPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ILPDVSR`"]
pub type ILPDVSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILPDVSR`"]
pub struct ILPDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> ILPDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
    #[inline(always)]
    pub fn ilpdvsr(&self) -> ILPDVSR_R {
        ILPDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
    #[inline(always)]
    pub fn ilpdvsr(&mut self) -> ILPDVSR_W {
        ILPDVSR_W { w: self }
    }
}
