#[doc = "Reader of register FIFO_ST"]
pub type R = crate::R<u32, super::FIFO_ST>;
#[doc = "Writer for register FIFO_ST"]
pub type W = crate::W<u32, super::FIFO_ST>;
#[doc = "Register FIFO_ST `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FIFO_ST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `ROE`"]
pub type ROE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROE`"]
pub struct ROE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WOF`"]
pub type WOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOF`"]
pub struct WOF_W<'a> {
    w: &'a mut W,
}
impl<'a> WOF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VLD`"]
pub type VLD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    #[inline(always)]
    pub fn roe(&self) -> ROE_R {
        ROE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
    #[inline(always)]
    pub fn wof(&self) -> WOF_R {
        WOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    #[inline(always)]
    pub fn roe(&mut self) -> ROE_W {
        ROE_W { w: self }
    }
    #[doc = "Bit 2 - Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
    #[inline(always)]
    pub fn wof(&mut self) -> WOF_W {
        WOF_W { w: self }
    }
}
