#[doc = "Reader of register IC_RX_TL"]
pub type R = crate::R<u32, super::IC_RX_TL>;
#[doc = "Writer for register IC_RX_TL"]
pub type W = crate::W<u32, super::IC_RX_TL>;
#[doc = "Register IC_RX_TL `reset()`'s with value 0"]
impl crate::ResetValue for super::IC_RX_TL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_TL`"]
pub type RX_TL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_TL`"]
pub struct RX_TL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Threshold Level.\\n\\n Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries."]
    #[inline(always)]
    pub fn rx_tl(&self) -> RX_TL_R {
        RX_TL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO Threshold Level.\\n\\n Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries."]
    #[inline(always)]
    pub fn rx_tl(&mut self) -> RX_TL_W {
        RX_TL_W { w: self }
    }
}
