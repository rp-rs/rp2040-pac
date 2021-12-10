#[doc = "Register `INTS` reader"]
pub struct R(crate::R<INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EP_STALL_NAK` reader - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
pub struct EP_STALL_NAK_R(crate::FieldReader<bool, bool>);
impl EP_STALL_NAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_STALL_NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_STALL_NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABORT_DONE` reader - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
pub struct ABORT_DONE_R(crate::FieldReader<bool, bool>);
impl ABORT_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABORT_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABORT_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_SOF` reader - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
pub struct DEV_SOF_R(crate::FieldReader<bool, bool>);
impl DEV_SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP_REQ` reader - Device. Source: SIE_STATUS.SETUP_REC"]
pub struct SETUP_REQ_R(crate::FieldReader<bool, bool>);
impl SETUP_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_RESUME_FROM_HOST` reader - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
pub struct DEV_RESUME_FROM_HOST_R(crate::FieldReader<bool, bool>);
impl DEV_RESUME_FROM_HOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_RESUME_FROM_HOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_RESUME_FROM_HOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_SUSPEND` reader - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
pub struct DEV_SUSPEND_R(crate::FieldReader<bool, bool>);
impl DEV_SUSPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_SUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_SUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_CONN_DIS` reader - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
pub struct DEV_CONN_DIS_R(crate::FieldReader<bool, bool>);
impl DEV_CONN_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_CONN_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_CONN_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_RESET` reader - Source: SIE_STATUS.BUS_RESET"]
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
#[doc = "Field `VBUS_DETECT` reader - Source: SIE_STATUS.VBUS_DETECTED"]
pub struct VBUS_DETECT_R(crate::FieldReader<bool, bool>);
impl VBUS_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` reader - Source: SIE_STATUS.STALL_REC"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_CRC` reader - Source: SIE_STATUS.CRC_ERROR"]
pub struct ERROR_CRC_R(crate::FieldReader<bool, bool>);
impl ERROR_CRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_BIT_STUFF` reader - Source: SIE_STATUS.BIT_STUFF_ERROR"]
pub struct ERROR_BIT_STUFF_R(crate::FieldReader<bool, bool>);
impl ERROR_BIT_STUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_BIT_STUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_BIT_STUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_RX_OVERFLOW` reader - Source: SIE_STATUS.RX_OVERFLOW"]
pub struct ERROR_RX_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl ERROR_RX_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_RX_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_RX_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_RX_TIMEOUT` reader - Source: SIE_STATUS.RX_TIMEOUT"]
pub struct ERROR_RX_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl ERROR_RX_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_RX_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_RX_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_DATA_SEQ` reader - Source: SIE_STATUS.DATA_SEQ_ERROR"]
pub struct ERROR_DATA_SEQ_R(crate::FieldReader<bool, bool>);
impl ERROR_DATA_SEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_DATA_SEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_DATA_SEQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFF_STATUS` reader - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
pub struct BUFF_STATUS_R(crate::FieldReader<bool, bool>);
impl BUFF_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUFF_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFF_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE` reader - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
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
#[doc = "Field `HOST_SOF` reader - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
pub struct HOST_SOF_R(crate::FieldReader<bool, bool>);
impl HOST_SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_RESUME` reader - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
pub struct HOST_RESUME_R(crate::FieldReader<bool, bool>);
impl HOST_RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_CONN_DIS` reader - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
pub struct HOST_CONN_DIS_R(crate::FieldReader<bool, bool>);
impl HOST_CONN_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_CONN_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_CONN_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Bit 11 - Source: SIE_STATUS.VBUS_DETECTED"]
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
#[doc = "Interrupt status after masking & forcing  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ints](index.html) module"]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ints::R](R) reader structure"]
impl crate::Readable for INTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
