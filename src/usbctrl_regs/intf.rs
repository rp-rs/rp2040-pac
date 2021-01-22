#[doc = "Reader of register INTF"]
pub type R = crate::R<u32, super::INTF>;
#[doc = "Writer for register INTF"]
pub type W = crate::W<u32, super::INTF>;
#[doc = "Register INTF `reset()`'s with value 0"]
impl crate::ResetValue for super::INTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_STALL_NAK`"]
pub type EP_STALL_NAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP_STALL_NAK`"]
pub struct EP_STALL_NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_STALL_NAK_W<'a> {
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
#[doc = "Reader of field `ABORT_DONE`"]
pub type ABORT_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABORT_DONE`"]
pub struct ABORT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_DONE_W<'a> {
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
#[doc = "Reader of field `DEV_SOF`"]
pub type DEV_SOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_SOF`"]
pub struct DEV_SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_SOF_W<'a> {
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
#[doc = "Reader of field `SETUP_REQ`"]
pub type SETUP_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUP_REQ`"]
pub struct SETUP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_REQ_W<'a> {
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
#[doc = "Reader of field `DEV_RESUME_FROM_HOST`"]
pub type DEV_RESUME_FROM_HOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_RESUME_FROM_HOST`"]
pub struct DEV_RESUME_FROM_HOST_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_RESUME_FROM_HOST_W<'a> {
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
#[doc = "Reader of field `DEV_SUSPEND`"]
pub type DEV_SUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_SUSPEND`"]
pub struct DEV_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_SUSPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DEV_CONN_DIS`"]
pub type DEV_CONN_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_CONN_DIS`"]
pub struct DEV_CONN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_CONN_DIS_W<'a> {
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
#[doc = "Reader of field `BUS_RESET`"]
pub type BUS_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_RESET`"]
pub struct BUS_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_RESET_W<'a> {
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
#[doc = "Reader of field `VBUS_DETECT`"]
pub type VBUS_DETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUS_DETECT`"]
pub struct VBUS_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_DETECT_W<'a> {
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
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
#[doc = "Reader of field `ERROR_CRC`"]
pub type ERROR_CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_CRC`"]
pub struct ERROR_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_CRC_W<'a> {
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
#[doc = "Reader of field `ERROR_BIT_STUFF`"]
pub type ERROR_BIT_STUFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_BIT_STUFF`"]
pub struct ERROR_BIT_STUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_BIT_STUFF_W<'a> {
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
#[doc = "Reader of field `ERROR_RX_OVERFLOW`"]
pub type ERROR_RX_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_RX_OVERFLOW`"]
pub struct ERROR_RX_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_RX_OVERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ERROR_RX_TIMEOUT`"]
pub type ERROR_RX_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_RX_TIMEOUT`"]
pub struct ERROR_RX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_RX_TIMEOUT_W<'a> {
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
#[doc = "Reader of field `ERROR_DATA_SEQ`"]
pub type ERROR_DATA_SEQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_DATA_SEQ`"]
pub struct ERROR_DATA_SEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_DATA_SEQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BUFF_STATUS`"]
pub type BUFF_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFF_STATUS`"]
pub struct BUFF_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_STATUS_W<'a> {
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
#[doc = "Reader of field `TRANS_COMPLETE`"]
pub type TRANS_COMPLETE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_COMPLETE`"]
pub struct TRANS_COMPLETE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_COMPLETE_W<'a> {
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
#[doc = "Reader of field `HOST_SOF`"]
pub type HOST_SOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SOF`"]
pub struct HOST_SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SOF_W<'a> {
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
#[doc = "Reader of field `HOST_RESUME`"]
pub type HOST_RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_RESUME`"]
pub struct HOST_RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_RESUME_W<'a> {
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
#[doc = "Reader of field `HOST_CONN_DIS`"]
pub type HOST_CONN_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_CONN_DIS`"]
pub struct HOST_CONN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_CONN_DIS_W<'a> {
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
    #[doc = "Bit 19 - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    #[inline(always)]
    pub fn ep_stall_nak(&self) -> EP_STALL_NAK_R {
        EP_STALL_NAK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    #[inline(always)]
    pub fn abort_done(&self) -> ABORT_DONE_R {
        ABORT_DONE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn dev_sof(&self) -> DEV_SOF_R {
        DEV_SOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Device. Source: SIE_STATUS.SETUP_REC"]
    #[inline(always)]
    pub fn setup_req(&self) -> SETUP_REQ_R {
        SETUP_REQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn dev_resume_from_host(&self) -> DEV_RESUME_FROM_HOST_R {
        DEV_RESUME_FROM_HOST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    #[inline(always)]
    pub fn dev_suspend(&self) -> DEV_SUSPEND_R {
        DEV_SUSPEND_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    #[inline(always)]
    pub fn dev_conn_dis(&self) -> DEV_CONN_DIS_R {
        DEV_CONN_DIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Source: SIE_STATUS.BUS_RESET"]
    #[inline(always)]
    pub fn bus_reset(&self) -> BUS_RESET_R {
        BUS_RESET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source: SIE_STATUS.VBUS_DETECT"]
    #[inline(always)]
    pub fn vbus_detect(&self) -> VBUS_DETECT_R {
        VBUS_DETECT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Source: SIE_STATUS.STALL_REC"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Source: SIE_STATUS.CRC_ERROR"]
    #[inline(always)]
    pub fn error_crc(&self) -> ERROR_CRC_R {
        ERROR_CRC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Source: SIE_STATUS.BIT_STUFF_ERROR"]
    #[inline(always)]
    pub fn error_bit_stuff(&self) -> ERROR_BIT_STUFF_R {
        ERROR_BIT_STUFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Source: SIE_STATUS.RX_OVERFLOW"]
    #[inline(always)]
    pub fn error_rx_overflow(&self) -> ERROR_RX_OVERFLOW_R {
        ERROR_RX_OVERFLOW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source: SIE_STATUS.RX_TIMEOUT"]
    #[inline(always)]
    pub fn error_rx_timeout(&self) -> ERROR_RX_TIMEOUT_R {
        ERROR_RX_TIMEOUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Source: SIE_STATUS.DATA_SEQ_ERROR"]
    #[inline(always)]
    pub fn error_data_seq(&self) -> ERROR_DATA_SEQ_R {
        ERROR_DATA_SEQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    #[inline(always)]
    pub fn buff_status(&self) -> BUFF_STATUS_R {
        BUFF_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn host_sof(&self) -> HOST_SOF_R {
        HOST_SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn host_resume(&self) -> HOST_RESUME_R {
        HOST_RESUME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    #[inline(always)]
    pub fn host_conn_dis(&self) -> HOST_CONN_DIS_R {
        HOST_CONN_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    #[inline(always)]
    pub fn ep_stall_nak(&mut self) -> EP_STALL_NAK_W {
        EP_STALL_NAK_W { w: self }
    }
    #[doc = "Bit 18 - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    #[inline(always)]
    pub fn abort_done(&mut self) -> ABORT_DONE_W {
        ABORT_DONE_W { w: self }
    }
    #[doc = "Bit 17 - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn dev_sof(&mut self) -> DEV_SOF_W {
        DEV_SOF_W { w: self }
    }
    #[doc = "Bit 16 - Device. Source: SIE_STATUS.SETUP_REC"]
    #[inline(always)]
    pub fn setup_req(&mut self) -> SETUP_REQ_W {
        SETUP_REQ_W { w: self }
    }
    #[doc = "Bit 15 - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn dev_resume_from_host(&mut self) -> DEV_RESUME_FROM_HOST_W {
        DEV_RESUME_FROM_HOST_W { w: self }
    }
    #[doc = "Bit 14 - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    #[inline(always)]
    pub fn dev_suspend(&mut self) -> DEV_SUSPEND_W {
        DEV_SUSPEND_W { w: self }
    }
    #[doc = "Bit 13 - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    #[inline(always)]
    pub fn dev_conn_dis(&mut self) -> DEV_CONN_DIS_W {
        DEV_CONN_DIS_W { w: self }
    }
    #[doc = "Bit 12 - Source: SIE_STATUS.BUS_RESET"]
    #[inline(always)]
    pub fn bus_reset(&mut self) -> BUS_RESET_W {
        BUS_RESET_W { w: self }
    }
    #[doc = "Bit 11 - Source: SIE_STATUS.VBUS_DETECT"]
    #[inline(always)]
    pub fn vbus_detect(&mut self) -> VBUS_DETECT_W {
        VBUS_DETECT_W { w: self }
    }
    #[doc = "Bit 10 - Source: SIE_STATUS.STALL_REC"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 9 - Source: SIE_STATUS.CRC_ERROR"]
    #[inline(always)]
    pub fn error_crc(&mut self) -> ERROR_CRC_W {
        ERROR_CRC_W { w: self }
    }
    #[doc = "Bit 8 - Source: SIE_STATUS.BIT_STUFF_ERROR"]
    #[inline(always)]
    pub fn error_bit_stuff(&mut self) -> ERROR_BIT_STUFF_W {
        ERROR_BIT_STUFF_W { w: self }
    }
    #[doc = "Bit 7 - Source: SIE_STATUS.RX_OVERFLOW"]
    #[inline(always)]
    pub fn error_rx_overflow(&mut self) -> ERROR_RX_OVERFLOW_W {
        ERROR_RX_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 6 - Source: SIE_STATUS.RX_TIMEOUT"]
    #[inline(always)]
    pub fn error_rx_timeout(&mut self) -> ERROR_RX_TIMEOUT_W {
        ERROR_RX_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 5 - Source: SIE_STATUS.DATA_SEQ_ERROR"]
    #[inline(always)]
    pub fn error_data_seq(&mut self) -> ERROR_DATA_SEQ_W {
        ERROR_DATA_SEQ_W { w: self }
    }
    #[doc = "Bit 4 - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    #[inline(always)]
    pub fn buff_status(&mut self) -> BUFF_STATUS_W {
        BUFF_STATUS_W { w: self }
    }
    #[doc = "Bit 3 - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    #[inline(always)]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W {
        TRANS_COMPLETE_W { w: self }
    }
    #[doc = "Bit 2 - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn host_sof(&mut self) -> HOST_SOF_W {
        HOST_SOF_W { w: self }
    }
    #[doc = "Bit 1 - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn host_resume(&mut self) -> HOST_RESUME_W {
        HOST_RESUME_W { w: self }
    }
    #[doc = "Bit 0 - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    #[inline(always)]
    pub fn host_conn_dis(&mut self) -> HOST_CONN_DIS_W {
        HOST_CONN_DIS_W { w: self }
    }
}
