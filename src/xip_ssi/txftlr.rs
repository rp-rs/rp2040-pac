#[doc = "Reader of register TXFTLR"]
pub type R = crate::R<u32, super::TXFTLR>;
#[doc = "Writer for register TXFTLR"]
pub type W = crate::W<u32, super::TXFTLR>;
#[doc = "Register TXFTLR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFTLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TFT`"]
pub type TFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFT`"]
pub struct TFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO threshold"]
    #[inline(always)]
    pub fn tft(&self) -> TFT_R {
        TFT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit FIFO threshold"]
    #[inline(always)]
    pub fn tft(&mut self) -> TFT_W {
        TFT_W { w: self }
    }
}
