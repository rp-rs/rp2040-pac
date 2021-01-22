#[doc = "Reader of register IC_SDA_HOLD"]
pub type R = crate::R<u32, super::IC_SDA_HOLD>;
#[doc = "Writer for register IC_SDA_HOLD"]
pub type W = crate::W<u32, super::IC_SDA_HOLD>;
#[doc = "Register IC_SDA_HOLD `reset()`'s with value 0x01"]
impl crate::ResetValue for super::IC_SDA_HOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `IC_SDA_RX_HOLD`"]
pub type IC_SDA_RX_HOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC_SDA_RX_HOLD`"]
pub struct IC_SDA_RX_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SDA_RX_HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `IC_SDA_TX_HOLD`"]
pub type IC_SDA_TX_HOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IC_SDA_TX_HOLD`"]
pub struct IC_SDA_TX_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SDA_TX_HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.\\n\\n Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
    #[inline(always)]
    pub fn ic_sda_rx_hold(&self) -> IC_SDA_RX_HOLD_R {
        IC_SDA_RX_HOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.\\n\\n Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
    #[inline(always)]
    pub fn ic_sda_tx_hold(&self) -> IC_SDA_TX_HOLD_R {
        IC_SDA_TX_HOLD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.\\n\\n Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
    #[inline(always)]
    pub fn ic_sda_rx_hold(&mut self) -> IC_SDA_RX_HOLD_W {
        IC_SDA_RX_HOLD_W { w: self }
    }
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.\\n\\n Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
    #[inline(always)]
    pub fn ic_sda_tx_hold(&mut self) -> IC_SDA_TX_HOLD_W {
        IC_SDA_TX_HOLD_W { w: self }
    }
}
