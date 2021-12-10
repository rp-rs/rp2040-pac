#[doc = "Register `SIE_STATUS` reader"]
pub struct R(crate::R<SIE_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE_STATUS` writer"]
pub struct W(crate::W<SIE_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SIE_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_SEQ_ERROR` reader - Data Sequence Error.  

 The device can raise a sequence error in the following conditions:  

 * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM  

 The host can raise a data sequence error in the following conditions:  

 * An IN packet from the device has the wrong data PID"]
pub struct DATA_SEQ_ERROR_R(crate::FieldReader<bool, bool>);
impl DATA_SEQ_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_SEQ_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_SEQ_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_SEQ_ERROR` writer - Data Sequence Error.  

 The device can raise a sequence error in the following conditions:  

 * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM  

 The host can raise a data sequence error in the following conditions:  

 * An IN packet from the device has the wrong data PID"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `ACK_REC` reader - ACK received. Raised by both host and device."]
pub struct ACK_REC_R(crate::FieldReader<bool, bool>);
impl ACK_REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_REC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_REC` writer - ACK received. Raised by both host and device."]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `STALL_REC` reader - Host: STALL received"]
pub struct STALL_REC_R(crate::FieldReader<bool, bool>);
impl STALL_REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_REC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_REC` writer - Host: STALL received"]
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `NAK_REC` reader - Host: NAK received"]
pub struct NAK_REC_R(crate::FieldReader<bool, bool>);
impl NAK_REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAK_REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_REC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK_REC` writer - Host: NAK received"]
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `RX_TIMEOUT` reader - RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
pub struct RX_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl RX_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TIMEOUT` writer - RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `RX_OVERFLOW` reader - RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
pub struct RX_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl RX_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OVERFLOW` writer - RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `BIT_STUFF_ERROR` reader - Bit Stuff Error. Raised by the Serial RX engine."]
pub struct BIT_STUFF_ERROR_R(crate::FieldReader<bool, bool>);
impl BIT_STUFF_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT_STUFF_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_STUFF_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_STUFF_ERROR` writer - Bit Stuff Error. Raised by the Serial RX engine."]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CRC_ERROR` reader - CRC Error. Raised by the Serial RX engine."]
pub struct CRC_ERROR_R(crate::FieldReader<bool, bool>);
impl CRC_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_ERROR` writer - CRC Error. Raised by the Serial RX engine."]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `BUS_RESET` reader - Device: bus reset received"]
pub struct BUS_RESET_R(crate::FieldReader<bool, bool>);
impl BUS_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_RESET` writer - Device: bus reset received"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TRANS_COMPLETE` reader - Transaction complete.  

 Raised by device if:  

 * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register  

 Raised by host if:  

 * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
pub struct TRANS_COMPLETE_R(crate::FieldReader<bool, bool>);
impl TRANS_COMPLETE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_COMPLETE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_COMPLETE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE` writer - Transaction complete.  

 Raised by device if:  

 * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register  

 Raised by host if:  

 * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SETUP_REC` reader - Device: Setup packet received"]
pub struct SETUP_REC_R(crate::FieldReader<bool, bool>);
impl SETUP_REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_REC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP_REC` writer - Device: Setup packet received"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CONNECTED` reader - Device: connected"]
pub struct CONNECTED_R(crate::FieldReader<bool, bool>);
impl CONNECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONNECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONNECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONNECTED` writer - Device: connected"]
pub struct CONNECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECTED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RESUME` reader - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
pub struct RESUME_R(crate::FieldReader<bool, bool>);
impl RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `VBUS_OVER_CURR` reader - VBUS over current detected"]
pub struct VBUS_OVER_CURR_R(crate::FieldReader<bool, bool>);
impl VBUS_OVER_CURR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_OVER_CURR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_OVER_CURR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEED` reader - Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
pub struct SPEED_R(crate::FieldReader<u8, u8>);
impl SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEED` writer - Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SUSPENDED` reader - Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
pub struct SUSPENDED_R(crate::FieldReader<bool, bool>);
impl SUSPENDED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSPENDED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPENDED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPENDED` writer - Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
pub struct SUSPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPENDED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "USB bus line state  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE_STATE_A {
    #[doc = "0: SE0"]
    SE0 = 0,
    #[doc = "1: J"]
    J = 1,
    #[doc = "2: K"]
    K = 2,
    #[doc = "3: SE1"]
    SE1 = 3,
}
impl From<LINE_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LINE_STATE` reader - USB bus line state"]
pub struct LINE_STATE_R(crate::FieldReader<u8, LINE_STATE_A>);
impl LINE_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LINE_STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE_STATE_A {
        match self.bits {
            0 => LINE_STATE_A::SE0,
            1 => LINE_STATE_A::J,
            2 => LINE_STATE_A::K,
            3 => LINE_STATE_A::SE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        **self == LINE_STATE_A::SE0
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        **self == LINE_STATE_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        **self == LINE_STATE_A::K
    }
    #[doc = "Checks if the value of the field is `SE1`"]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        **self == LINE_STATE_A::SE1
    }
}
impl core::ops::Deref for LINE_STATE_R {
    type Target = crate::FieldReader<u8, LINE_STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS_DETECTED` reader - Device: VBUS Detected"]
pub struct VBUS_DETECTED_R(crate::FieldReader<bool, bool>);
impl VBUS_DETECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_DETECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_DETECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Data Sequence Error.  

 The device can raise a sequence error in the following conditions:  

 * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM  

 The host can raise a data sequence error in the following conditions:  

 * An IN packet from the device has the wrong data PID"]
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
    #[doc = "Bit 18 - Transaction complete.  

 Raised by device if:  

 * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register  

 Raised by host if:  

 * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
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
    #[doc = "Bit 31 - Data Sequence Error.  

 The device can raise a sequence error in the following conditions:  

 * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM  

 The host can raise a data sequence error in the following conditions:  

 * An IN packet from the device has the wrong data PID"]
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
    #[doc = "Bit 18 - Transaction complete.  

 Raised by device if:  

 * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register  

 Raised by host if:  

 * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    #[inline(always)]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W {
        TRANS_COMPLETE_W { w: self }
    }
    #[doc = "Bit 17 - Device: Setup packet received"]
    #[inline(always)]
    pub fn setup_rec(&mut self) -> SETUP_REC_W {
        SETUP_REC_W { w: self }
    }
    #[doc = "Bit 16 - Device: connected"]
    #[inline(always)]
    pub fn connected(&mut self) -> CONNECTED_W {
        CONNECTED_W { w: self }
    }
    #[doc = "Bit 11 - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bits 8:9 - Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 4 - Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
    #[inline(always)]
    pub fn suspended(&mut self) -> SUSPENDED_W {
        SUSPENDED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SIE status register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sie_status](index.html) module"]
pub struct SIE_STATUS_SPEC;
impl crate::RegisterSpec for SIE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie_status::R](R) reader structure"]
impl crate::Readable for SIE_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie_status::W](W) writer structure"]
impl crate::Writable for SIE_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIE_STATUS to value 0"]
impl crate::Resettable for SIE_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
