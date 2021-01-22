#[doc = "Reader of register IC_TX_TL"]
pub type R = crate::R<u32, super::IC_TX_TL>;
#[doc = "Writer for register IC_TX_TL"]
pub type W = crate::W<u32, super::IC_TX_TL>;
#[doc = "Register IC_TX_TL `reset()`'s with value 0"]
impl crate::ResetValue for super::IC_TX_TL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_TL`"]
pub type TX_TL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_TL`"]
pub struct TX_TL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Threshold Level.\\n\\n Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
    #[inline(always)]
    pub fn tx_tl(&self) -> TX_TL_R {
        TX_TL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit FIFO Threshold Level.\\n\\n Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
    #[inline(always)]
    pub fn tx_tl(&mut self) -> TX_TL_W {
        TX_TL_W { w: self }
    }
}
