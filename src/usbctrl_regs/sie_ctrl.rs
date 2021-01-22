#[doc = "Reader of register SIE_CTRL"]
pub type R = crate::R<u32, super::SIE_CTRL>;
#[doc = "Writer for register SIE_CTRL"]
pub type W = crate::W<u32, super::SIE_CTRL>;
#[doc = "Register SIE_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SIE_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP0_INT_STALL`"]
pub type EP0_INT_STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INT_STALL`"]
pub struct EP0_INT_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_STALL_W<'a> {
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
#[doc = "Reader of field `EP0_DOUBLE_BUF`"]
pub type EP0_DOUBLE_BUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_DOUBLE_BUF`"]
pub struct EP0_DOUBLE_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_DOUBLE_BUF_W<'a> {
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
#[doc = "Reader of field `EP0_INT_1BUF`"]
pub type EP0_INT_1BUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INT_1BUF`"]
pub struct EP0_INT_1BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_1BUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `EP0_INT_2BUF`"]
pub type EP0_INT_2BUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INT_2BUF`"]
pub struct EP0_INT_2BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_2BUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `EP0_INT_NAK`"]
pub type EP0_INT_NAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INT_NAK`"]
pub struct EP0_INT_NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_NAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DIRECT_EN`"]
pub type DIRECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRECT_EN`"]
pub struct DIRECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DIRECT_DP`"]
pub type DIRECT_DP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRECT_DP`"]
pub struct DIRECT_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECT_DP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DIRECT_DM`"]
pub type DIRECT_DM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRECT_DM`"]
pub struct DIRECT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECT_DM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TRANSCEIVER_PD`"]
pub type TRANSCEIVER_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANSCEIVER_PD`"]
pub struct TRANSCEIVER_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSCEIVER_PD_W<'a> {
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
#[doc = "Reader of field `RPU_OPT`"]
pub type RPU_OPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPU_OPT`"]
pub struct RPU_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RPU_OPT_W<'a> {
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
#[doc = "Reader of field `PULLUP_EN`"]
pub type PULLUP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PULLUP_EN`"]
pub struct PULLUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLUP_EN_W<'a> {
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
#[doc = "Reader of field `PULLDOWN_EN`"]
pub type PULLDOWN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PULLDOWN_EN`"]
pub struct PULLDOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLDOWN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RESET_BUS`"]
pub type RESET_BUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_BUS`"]
pub struct RESET_BUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_BUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `VBUS_EN`"]
pub type VBUS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUS_EN`"]
pub struct VBUS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `KEEP_ALIVE_EN`"]
pub type KEEP_ALIVE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEEP_ALIVE_EN`"]
pub struct KEEP_ALIVE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEP_ALIVE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SOF_EN`"]
pub type SOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_EN`"]
pub struct SOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SOF_SYNC`"]
pub type SOF_SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_SYNC`"]
pub struct SOF_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_SYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PREAMBLE_EN`"]
pub type PREAMBLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREAMBLE_EN`"]
pub struct PREAMBLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREAMBLE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `STOP_TRANS`"]
pub type STOP_TRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_TRANS`"]
pub struct STOP_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_TRANS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RECEIVE_DATA`"]
pub type RECEIVE_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RECEIVE_DATA`"]
pub struct RECEIVE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_DATA_W<'a> {
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
#[doc = "Reader of field `SEND_DATA`"]
pub type SEND_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_DATA`"]
pub struct SEND_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_DATA_W<'a> {
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
#[doc = "Reader of field `SEND_SETUP`"]
pub type SEND_SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_SETUP`"]
pub struct SEND_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_SETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `START_TRANS`"]
pub type START_TRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_TRANS`"]
pub struct START_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> START_TRANS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    pub fn ep0_int_stall(&self) -> EP0_INT_STALL_R {
        EP0_INT_STALL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    pub fn ep0_double_buf(&self) -> EP0_DOUBLE_BUF_R {
        EP0_DOUBLE_BUF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_1buf(&self) -> EP0_INT_1BUF_R {
        EP0_INT_1BUF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_2buf(&self) -> EP0_INT_2BUF_R {
        EP0_INT_2BUF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    pub fn ep0_int_nak(&self) -> EP0_INT_NAK_R {
        EP0_INT_NAK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Direct bus drive enable"]
    #[inline(always)]
    pub fn direct_en(&self) -> DIRECT_EN_R {
        DIRECT_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Direct control of DP"]
    #[inline(always)]
    pub fn direct_dp(&self) -> DIRECT_DP_R {
        DIRECT_DP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Direct control of DM"]
    #[inline(always)]
    pub fn direct_dm(&self) -> DIRECT_DM_R {
        DIRECT_DM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Power down bus transceiver"]
    #[inline(always)]
    pub fn transceiver_pd(&self) -> TRANSCEIVER_PD_R {
        TRANSCEIVER_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    pub fn rpu_opt(&self) -> RPU_OPT_R {
        RPU_OPT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Device: Enable pull up resistor"]
    #[inline(always)]
    pub fn pullup_en(&self) -> PULLUP_EN_R {
        PULLUP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Host: Enable pull down resistors"]
    #[inline(always)]
    pub fn pulldown_en(&self) -> PULLDOWN_EN_R {
        PULLDOWN_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Host: Reset bus"]
    #[inline(always)]
    pub fn reset_bus(&self) -> RESET_BUS_R {
        RESET_BUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Host: Enable VBUS"]
    #[inline(always)]
    pub fn vbus_en(&self) -> VBUS_EN_R {
        VBUS_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    pub fn keep_alive_en(&self) -> KEEP_ALIVE_EN_R {
        KEEP_ALIVE_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    pub fn sof_en(&self) -> SOF_EN_R {
        SOF_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    pub fn sof_sync(&self) -> SOF_SYNC_R {
        SOF_SYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    pub fn preamble_en(&self) -> PREAMBLE_EN_R {
        PREAMBLE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Host: Stop transaction"]
    #[inline(always)]
    pub fn stop_trans(&self) -> STOP_TRANS_R {
        STOP_TRANS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Host: Receive transaction (IN to host)"]
    #[inline(always)]
    pub fn receive_data(&self) -> RECEIVE_DATA_R {
        RECEIVE_DATA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Host: Send transaction (OUT from host)"]
    #[inline(always)]
    pub fn send_data(&self) -> SEND_DATA_R {
        SEND_DATA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host: Send Setup packet"]
    #[inline(always)]
    pub fn send_setup(&self) -> SEND_SETUP_R {
        SEND_SETUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Host: Start transaction"]
    #[inline(always)]
    pub fn start_trans(&self) -> START_TRANS_R {
        START_TRANS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    pub fn ep0_int_stall(&mut self) -> EP0_INT_STALL_W {
        EP0_INT_STALL_W { w: self }
    }
    #[doc = "Bit 30 - Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    pub fn ep0_double_buf(&mut self) -> EP0_DOUBLE_BUF_W {
        EP0_DOUBLE_BUF_W { w: self }
    }
    #[doc = "Bit 29 - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_1buf(&mut self) -> EP0_INT_1BUF_W {
        EP0_INT_1BUF_W { w: self }
    }
    #[doc = "Bit 28 - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_2buf(&mut self) -> EP0_INT_2BUF_W {
        EP0_INT_2BUF_W { w: self }
    }
    #[doc = "Bit 27 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    pub fn ep0_int_nak(&mut self) -> EP0_INT_NAK_W {
        EP0_INT_NAK_W { w: self }
    }
    #[doc = "Bit 26 - Direct bus drive enable"]
    #[inline(always)]
    pub fn direct_en(&mut self) -> DIRECT_EN_W {
        DIRECT_EN_W { w: self }
    }
    #[doc = "Bit 25 - Direct control of DP"]
    #[inline(always)]
    pub fn direct_dp(&mut self) -> DIRECT_DP_W {
        DIRECT_DP_W { w: self }
    }
    #[doc = "Bit 24 - Direct control of DM"]
    #[inline(always)]
    pub fn direct_dm(&mut self) -> DIRECT_DM_W {
        DIRECT_DM_W { w: self }
    }
    #[doc = "Bit 18 - Power down bus transceiver"]
    #[inline(always)]
    pub fn transceiver_pd(&mut self) -> TRANSCEIVER_PD_W {
        TRANSCEIVER_PD_W { w: self }
    }
    #[doc = "Bit 17 - Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    pub fn rpu_opt(&mut self) -> RPU_OPT_W {
        RPU_OPT_W { w: self }
    }
    #[doc = "Bit 16 - Device: Enable pull up resistor"]
    #[inline(always)]
    pub fn pullup_en(&mut self) -> PULLUP_EN_W {
        PULLUP_EN_W { w: self }
    }
    #[doc = "Bit 15 - Host: Enable pull down resistors"]
    #[inline(always)]
    pub fn pulldown_en(&mut self) -> PULLDOWN_EN_W {
        PULLDOWN_EN_W { w: self }
    }
    #[doc = "Bit 13 - Host: Reset bus"]
    #[inline(always)]
    pub fn reset_bus(&mut self) -> RESET_BUS_W {
        RESET_BUS_W { w: self }
    }
    #[doc = "Bit 12 - Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 11 - Host: Enable VBUS"]
    #[inline(always)]
    pub fn vbus_en(&mut self) -> VBUS_EN_W {
        VBUS_EN_W { w: self }
    }
    #[doc = "Bit 10 - Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    pub fn keep_alive_en(&mut self) -> KEEP_ALIVE_EN_W {
        KEEP_ALIVE_EN_W { w: self }
    }
    #[doc = "Bit 9 - Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    pub fn sof_en(&mut self) -> SOF_EN_W {
        SOF_EN_W { w: self }
    }
    #[doc = "Bit 8 - Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    pub fn sof_sync(&mut self) -> SOF_SYNC_W {
        SOF_SYNC_W { w: self }
    }
    #[doc = "Bit 6 - Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    pub fn preamble_en(&mut self) -> PREAMBLE_EN_W {
        PREAMBLE_EN_W { w: self }
    }
    #[doc = "Bit 4 - Host: Stop transaction"]
    #[inline(always)]
    pub fn stop_trans(&mut self) -> STOP_TRANS_W {
        STOP_TRANS_W { w: self }
    }
    #[doc = "Bit 3 - Host: Receive transaction (IN to host)"]
    #[inline(always)]
    pub fn receive_data(&mut self) -> RECEIVE_DATA_W {
        RECEIVE_DATA_W { w: self }
    }
    #[doc = "Bit 2 - Host: Send transaction (OUT from host)"]
    #[inline(always)]
    pub fn send_data(&mut self) -> SEND_DATA_W {
        SEND_DATA_W { w: self }
    }
    #[doc = "Bit 1 - Host: Send Setup packet"]
    #[inline(always)]
    pub fn send_setup(&mut self) -> SEND_SETUP_W {
        SEND_SETUP_W { w: self }
    }
    #[doc = "Bit 0 - Host: Start transaction"]
    #[inline(always)]
    pub fn start_trans(&mut self) -> START_TRANS_W {
        START_TRANS_W { w: self }
    }
}
