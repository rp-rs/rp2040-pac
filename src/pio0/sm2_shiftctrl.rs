#[doc = "Reader of register SM2_SHIFTCTRL"]
pub type R = crate::R<u32, super::SM2_SHIFTCTRL>;
#[doc = "Writer for register SM2_SHIFTCTRL"]
pub type W = crate::W<u32, super::SM2_SHIFTCTRL>;
#[doc = "Register SM2_SHIFTCTRL `reset()`'s with value 0x000c_0000"]
impl crate::ResetValue for super::SM2_SHIFTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000c_0000
    }
}
#[doc = "Reader of field `FJOIN_RX`"]
pub type FJOIN_RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FJOIN_RX`"]
pub struct FJOIN_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> FJOIN_RX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `FJOIN_TX`"]
pub type FJOIN_TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FJOIN_TX`"]
pub struct FJOIN_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> FJOIN_TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PULL_THRESH`"]
pub type PULL_THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PULL_THRESH`"]
pub struct PULL_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `PUSH_THRESH`"]
pub type PUSH_THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUSH_THRESH`"]
pub struct PUSH_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSH_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `OUT_SHIFTDIR`"]
pub type OUT_SHIFTDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_SHIFTDIR`"]
pub struct OUT_SHIFTDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SHIFTDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `IN_SHIFTDIR`"]
pub type IN_SHIFTDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_SHIFTDIR`"]
pub struct IN_SHIFTDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SHIFTDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `AUTOPULL`"]
pub type AUTOPULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOPULL`"]
pub struct AUTOPULL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `AUTOPUSH`"]
pub type AUTOPUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOPUSH`"]
pub struct AUTOPUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.\\n TX FIFO is disabled as a result (always reads as both full and empty).\\n FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_rx(&self) -> FJOIN_RX_R {
        FJOIN_RX_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.\\n RX FIFO is disabled as a result (always reads as both full and empty).\\n FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_tx(&self) -> FJOIN_TX_R {
        FJOIN_TX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Number of bits shifted out of TXSR before autopull or conditional pull.\\n Write 0 for value of 32."]
    #[inline(always)]
    pub fn pull_thresh(&self) -> PULL_THRESH_R {
        PULL_THRESH_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of bits shifted into RXSR before autopush or conditional push.\\n Write 0 for value of 32."]
    #[inline(always)]
    pub fn push_thresh(&self) -> PUSH_THRESH_R {
        PUSH_THRESH_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 19 - 1 = shift out of output shift register to right. 0 = to left."]
    #[inline(always)]
    pub fn out_shiftdir(&self) -> OUT_SHIFTDIR_R {
        OUT_SHIFTDIR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 1 = shift input shift register to right (data enters from left). 0 = to left."]
    #[inline(always)]
    pub fn in_shiftdir(&self) -> IN_SHIFTDIR_R {
        IN_SHIFTDIR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pull automatically when the output shift register is emptied"]
    #[inline(always)]
    pub fn autopull(&self) -> AUTOPULL_R {
        AUTOPULL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Push automatically when the input shift register is filled"]
    #[inline(always)]
    pub fn autopush(&self) -> AUTOPUSH_R {
        AUTOPUSH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.\\n TX FIFO is disabled as a result (always reads as both full and empty).\\n FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_rx(&mut self) -> FJOIN_RX_W {
        FJOIN_RX_W { w: self }
    }
    #[doc = "Bit 30 - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.\\n RX FIFO is disabled as a result (always reads as both full and empty).\\n FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_tx(&mut self) -> FJOIN_TX_W {
        FJOIN_TX_W { w: self }
    }
    #[doc = "Bits 25:29 - Number of bits shifted out of TXSR before autopull or conditional pull.\\n Write 0 for value of 32."]
    #[inline(always)]
    pub fn pull_thresh(&mut self) -> PULL_THRESH_W {
        PULL_THRESH_W { w: self }
    }
    #[doc = "Bits 20:24 - Number of bits shifted into RXSR before autopush or conditional push.\\n Write 0 for value of 32."]
    #[inline(always)]
    pub fn push_thresh(&mut self) -> PUSH_THRESH_W {
        PUSH_THRESH_W { w: self }
    }
    #[doc = "Bit 19 - 1 = shift out of output shift register to right. 0 = to left."]
    #[inline(always)]
    pub fn out_shiftdir(&mut self) -> OUT_SHIFTDIR_W {
        OUT_SHIFTDIR_W { w: self }
    }
    #[doc = "Bit 18 - 1 = shift input shift register to right (data enters from left). 0 = to left."]
    #[inline(always)]
    pub fn in_shiftdir(&mut self) -> IN_SHIFTDIR_W {
        IN_SHIFTDIR_W { w: self }
    }
    #[doc = "Bit 17 - Pull automatically when the output shift register is emptied"]
    #[inline(always)]
    pub fn autopull(&mut self) -> AUTOPULL_W {
        AUTOPULL_W { w: self }
    }
    #[doc = "Bit 16 - Push automatically when the input shift register is filled"]
    #[inline(always)]
    pub fn autopush(&mut self) -> AUTOPUSH_W {
        AUTOPUSH_W { w: self }
    }
}
