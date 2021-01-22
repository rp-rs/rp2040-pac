#[doc = "Reader of register SIE_STATUS"]
pub type R = crate::R<u32, super::SIE_STATUS>;
#[doc = "Writer for register SIE_STATUS"]
pub type W = crate::W<u32, super::SIE_STATUS>;
#[doc = "Register SIE_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::SIE_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_SEQ_ERROR`"]
pub type DATA_SEQ_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_SEQ_ERROR`"]
pub struct DATA_SEQ_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SEQ_ERROR_W<'a> {
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
#[doc = "Reader of field `ACK_REC`"]
pub type ACK_REC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK_REC`"]
pub struct ACK_REC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_REC_W<'a> {
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
#[doc = "Reader of field `STALL_REC`"]
pub type STALL_REC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL_REC`"]
pub struct STALL_REC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_REC_W<'a> {
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
#[doc = "Reader of field `NAK_REC`"]
pub type NAK_REC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAK_REC`"]
pub struct NAK_REC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_REC_W<'a> {
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
#[doc = "Reader of field `RX_TIMEOUT`"]
pub type RX_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TIMEOUT`"]
pub struct RX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TIMEOUT_W<'a> {
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
#[doc = "Reader of field `RX_OVERFLOW`"]
pub type RX_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_OVERFLOW`"]
pub struct RX_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVERFLOW_W<'a> {
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
#[doc = "Reader of field `BIT_STUFF_ERROR`"]
pub type BIT_STUFF_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIT_STUFF_ERROR`"]
pub struct BIT_STUFF_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_STUFF_ERROR_W<'a> {
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
#[doc = "Reader of field `CRC_ERROR`"]
pub type CRC_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_ERROR`"]
pub struct CRC_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SETUP_REC`"]
pub type SETUP_REC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUP_REC`"]
pub struct SETUP_REC_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_REC_W<'a> {
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
#[doc = "Reader of field `CONNECTED`"]
pub type CONNECTED_R = crate::R<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `VBUS_OVER_CURR`"]
pub type VBUS_OVER_CURR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, u8>;
#[doc = "Reader of field `SUSPENDED`"]
pub type SUSPENDED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINE_STATE`"]
pub type LINE_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `VBUS_DETECTED`"]
pub type VBUS_DETECTED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - Data Sequence Error.\\n\\n The device can raise a sequence error in the following conditions:\\n\\n * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM\\n\\n The host can raise a data sequence error in the following conditions:\\n\\n * An IN packet from the device has the wrong data PID"]
    #[inline(always)]
    pub fn data_seq_error(&self) -> DATA_SEQ_ERROR_R {
        DATA_SEQ_ERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ACK received. Raised by both host and device."]
    #[inline(always)]
    pub fn ack_rec(&self) -> ACK_REC_R {
        ACK_REC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Host: STALL received"]
    #[inline(always)]
    pub fn stall_rec(&self) -> STALL_REC_R {
        STALL_REC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Host: NAK received"]
    #[inline(always)]
    pub fn nak_rec(&self) -> NAK_REC_R {
        NAK_REC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
    #[inline(always)]
    pub fn rx_timeout(&self) -> RX_TIMEOUT_R {
        RX_TIMEOUT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bit Stuff Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub fn bit_stuff_error(&self) -> BIT_STUFF_ERROR_R {
        BIT_STUFF_ERROR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CRC Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub fn crc_error(&self) -> CRC_ERROR_R {
        CRC_ERROR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Device: bus reset received"]
    #[inline(always)]
    pub fn bus_reset(&self) -> BUS_RESET_R {
        BUS_RESET_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transaction complete.\\n\\n Raised by device if:\\n\\n * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register\\n\\n Raised by host if:\\n\\n * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Device: Setup packet received"]
    #[inline(always)]
    pub fn setup_rec(&self) -> SETUP_REC_R {
        SETUP_REC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Device: connected"]
    #[inline(always)]
    pub fn connected(&self) -> CONNECTED_R {
        CONNECTED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VBUS over current detected"]
    #[inline(always)]
    pub fn vbus_over_curr(&self) -> VBUS_OVER_CURR_R {
        VBUS_OVER_CURR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
    #[inline(always)]
    pub fn suspended(&self) -> SUSPENDED_R {
        SUSPENDED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - USB bus line state"]
    #[inline(always)]
    pub fn line_state(&self) -> LINE_STATE_R {
        LINE_STATE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Device: VBUS Detected"]
    #[inline(always)]
    pub fn vbus_detected(&self) -> VBUS_DETECTED_R {
        VBUS_DETECTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Data Sequence Error.\\n\\n The device can raise a sequence error in the following conditions:\\n\\n * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM\\n\\n The host can raise a data sequence error in the following conditions:\\n\\n * An IN packet from the device has the wrong data PID"]
    #[inline(always)]
    pub fn data_seq_error(&mut self) -> DATA_SEQ_ERROR_W {
        DATA_SEQ_ERROR_W { w: self }
    }
    #[doc = "Bit 30 - ACK received. Raised by both host and device."]
    #[inline(always)]
    pub fn ack_rec(&mut self) -> ACK_REC_W {
        ACK_REC_W { w: self }
    }
    #[doc = "Bit 29 - Host: STALL received"]
    #[inline(always)]
    pub fn stall_rec(&mut self) -> STALL_REC_W {
        STALL_REC_W { w: self }
    }
    #[doc = "Bit 28 - Host: NAK received"]
    #[inline(always)]
    pub fn nak_rec(&mut self) -> NAK_REC_W {
        NAK_REC_W { w: self }
    }
    #[doc = "Bit 27 - RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
    #[inline(always)]
    pub fn rx_timeout(&mut self) -> RX_TIMEOUT_W {
        RX_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 26 - RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
    #[inline(always)]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W {
        RX_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 25 - Bit Stuff Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub fn bit_stuff_error(&mut self) -> BIT_STUFF_ERROR_W {
        BIT_STUFF_ERROR_W { w: self }
    }
    #[doc = "Bit 24 - CRC Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub fn crc_error(&mut self) -> CRC_ERROR_W {
        CRC_ERROR_W { w: self }
    }
    #[doc = "Bit 19 - Device: bus reset received"]
    #[inline(always)]
    pub fn bus_reset(&mut self) -> BUS_RESET_W {
        BUS_RESET_W { w: self }
    }
    #[doc = "Bit 18 - Transaction complete.\\n\\n Raised by device if:\\n\\n * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register\\n\\n Raised by host if:\\n\\n * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    #[inline(always)]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W {
        TRANS_COMPLETE_W { w: self }
    }
    #[doc = "Bit 17 - Device: Setup packet received"]
    #[inline(always)]
    pub fn setup_rec(&mut self) -> SETUP_REC_W {
        SETUP_REC_W { w: self }
    }
    #[doc = "Bit 11 - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
}
