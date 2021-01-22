#[doc = "Reader of register RXFTLR"]
pub type R = crate::R<u32, super::RXFTLR>;
#[doc = "Writer for register RXFTLR"]
pub type W = crate::W<u32, super::RXFTLR>;
#[doc = "Register RXFTLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFTLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFT`"]
pub type RFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFT`"]
pub struct RFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive FIFO threshold"]
    #[inline(always)]
    pub fn rft(&self) -> RFT_R {
        RFT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO threshold"]
    #[inline(always)]
    pub fn rft(&mut self) -> RFT_W {
        RFT_W { w: self }
    }
}
