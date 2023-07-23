#[doc = "Register `INTF` reader"]
pub struct R(crate::R<INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTF` writer"]
pub struct W(crate::W<INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTF_SPEC>;
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
impl From<crate::W<INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_CONN_DIS` reader - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
pub type HOST_CONN_DIS_R = crate::BitReader<bool>;
#[doc = "Field `HOST_CONN_DIS` writer - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
pub type HOST_CONN_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `HOST_RESUME` reader - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
pub type HOST_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `HOST_RESUME` writer - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
pub type HOST_RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `HOST_SOF` reader - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
pub type HOST_SOF_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SOF` writer - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
pub type HOST_SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `TRANS_COMPLETE` reader - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
pub type TRANS_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE` writer - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
pub type TRANS_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `BUFF_STATUS` reader - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
pub type BUFF_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `BUFF_STATUS` writer - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
pub type BUFF_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `ERROR_DATA_SEQ` reader - Source: SIE_STATUS.DATA_SEQ_ERROR"]
pub type ERROR_DATA_SEQ_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_DATA_SEQ` writer - Source: SIE_STATUS.DATA_SEQ_ERROR"]
pub type ERROR_DATA_SEQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `ERROR_RX_TIMEOUT` reader - Source: SIE_STATUS.RX_TIMEOUT"]
pub type ERROR_RX_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_RX_TIMEOUT` writer - Source: SIE_STATUS.RX_TIMEOUT"]
pub type ERROR_RX_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `ERROR_RX_OVERFLOW` reader - Source: SIE_STATUS.RX_OVERFLOW"]
pub type ERROR_RX_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_RX_OVERFLOW` writer - Source: SIE_STATUS.RX_OVERFLOW"]
pub type ERROR_RX_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `ERROR_BIT_STUFF` reader - Source: SIE_STATUS.BIT_STUFF_ERROR"]
pub type ERROR_BIT_STUFF_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_BIT_STUFF` writer - Source: SIE_STATUS.BIT_STUFF_ERROR"]
pub type ERROR_BIT_STUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `ERROR_CRC` reader - Source: SIE_STATUS.CRC_ERROR"]
pub type ERROR_CRC_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_CRC` writer - Source: SIE_STATUS.CRC_ERROR"]
pub type ERROR_CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `STALL` reader - Source: SIE_STATUS.STALL_REC"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Source: SIE_STATUS.STALL_REC"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `VBUS_DETECT` reader - Source: SIE_STATUS.VBUS_DETECTED"]
pub type VBUS_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_DETECT` writer - Source: SIE_STATUS.VBUS_DETECTED"]
pub type VBUS_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `BUS_RESET` reader - Source: SIE_STATUS.BUS_RESET"]
pub type BUS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `BUS_RESET` writer - Source: SIE_STATUS.BUS_RESET"]
pub type BUS_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `DEV_CONN_DIS` reader - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
pub type DEV_CONN_DIS_R = crate::BitReader<bool>;
#[doc = "Field `DEV_CONN_DIS` writer - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
pub type DEV_CONN_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `DEV_SUSPEND` reader - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
pub type DEV_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `DEV_SUSPEND` writer - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
pub type DEV_SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `DEV_RESUME_FROM_HOST` reader - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
pub type DEV_RESUME_FROM_HOST_R = crate::BitReader<bool>;
#[doc = "Field `DEV_RESUME_FROM_HOST` writer - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
pub type DEV_RESUME_FROM_HOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `SETUP_REQ` reader - Device. Source: SIE_STATUS.SETUP_REC"]
pub type SETUP_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SETUP_REQ` writer - Device. Source: SIE_STATUS.SETUP_REC"]
pub type SETUP_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `DEV_SOF` reader - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
pub type DEV_SOF_R = crate::BitReader<bool>;
#[doc = "Field `DEV_SOF` writer - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
pub type DEV_SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `ABORT_DONE` reader - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
pub type ABORT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ABORT_DONE` writer - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
pub type ABORT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
#[doc = "Field `EP_STALL_NAK` reader - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
pub type EP_STALL_NAK_R = crate::BitReader<bool>;
#[doc = "Field `EP_STALL_NAK` writer - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
pub type EP_STALL_NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    #[inline(always)]
    pub fn host_conn_dis(&self) -> HOST_CONN_DIS_R {
        HOST_CONN_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn host_resume(&self) -> HOST_RESUME_R {
        HOST_RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn host_sof(&self) -> HOST_SOF_R {
        HOST_SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    #[inline(always)]
    pub fn buff_status(&self) -> BUFF_STATUS_R {
        BUFF_STATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source: SIE_STATUS.DATA_SEQ_ERROR"]
    #[inline(always)]
    pub fn error_data_seq(&self) -> ERROR_DATA_SEQ_R {
        ERROR_DATA_SEQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source: SIE_STATUS.RX_TIMEOUT"]
    #[inline(always)]
    pub fn error_rx_timeout(&self) -> ERROR_RX_TIMEOUT_R {
        ERROR_RX_TIMEOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source: SIE_STATUS.RX_OVERFLOW"]
    #[inline(always)]
    pub fn error_rx_overflow(&self) -> ERROR_RX_OVERFLOW_R {
        ERROR_RX_OVERFLOW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source: SIE_STATUS.BIT_STUFF_ERROR"]
    #[inline(always)]
    pub fn error_bit_stuff(&self) -> ERROR_BIT_STUFF_R {
        ERROR_BIT_STUFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source: SIE_STATUS.CRC_ERROR"]
    #[inline(always)]
    pub fn error_crc(&self) -> ERROR_CRC_R {
        ERROR_CRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source: SIE_STATUS.STALL_REC"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source: SIE_STATUS.VBUS_DETECTED"]
    #[inline(always)]
    pub fn vbus_detect(&self) -> VBUS_DETECT_R {
        VBUS_DETECT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Source: SIE_STATUS.BUS_RESET"]
    #[inline(always)]
    pub fn bus_reset(&self) -> BUS_RESET_R {
        BUS_RESET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    #[inline(always)]
    pub fn dev_conn_dis(&self) -> DEV_CONN_DIS_R {
        DEV_CONN_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    #[inline(always)]
    pub fn dev_suspend(&self) -> DEV_SUSPEND_R {
        DEV_SUSPEND_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn dev_resume_from_host(&self) -> DEV_RESUME_FROM_HOST_R {
        DEV_RESUME_FROM_HOST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Device. Source: SIE_STATUS.SETUP_REC"]
    #[inline(always)]
    pub fn setup_req(&self) -> SETUP_REQ_R {
        SETUP_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn dev_sof(&self) -> DEV_SOF_R {
        DEV_SOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    #[inline(always)]
    pub fn abort_done(&self) -> ABORT_DONE_R {
        ABORT_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    #[inline(always)]
    pub fn ep_stall_nak(&self) -> EP_STALL_NAK_R {
        EP_STALL_NAK_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    #[inline(always)]
    #[must_use]
    pub fn host_conn_dis(&mut self) -> HOST_CONN_DIS_W<0> {
        HOST_CONN_DIS_W::new(self)
    }
    #[doc = "Bit 1 - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    #[must_use]
    pub fn host_resume(&mut self) -> HOST_RESUME_W<1> {
        HOST_RESUME_W::new(self)
    }
    #[doc = "Bit 2 - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    #[inline(always)]
    #[must_use]
    pub fn host_sof(&mut self) -> HOST_SOF_W<2> {
        HOST_SOF_W::new(self)
    }
    #[doc = "Bit 3 - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<3> {
        TRANS_COMPLETE_W::new(self)
    }
    #[doc = "Bit 4 - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    #[inline(always)]
    #[must_use]
    pub fn buff_status(&mut self) -> BUFF_STATUS_W<4> {
        BUFF_STATUS_W::new(self)
    }
    #[doc = "Bit 5 - Source: SIE_STATUS.DATA_SEQ_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error_data_seq(&mut self) -> ERROR_DATA_SEQ_W<5> {
        ERROR_DATA_SEQ_W::new(self)
    }
    #[doc = "Bit 6 - Source: SIE_STATUS.RX_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn error_rx_timeout(&mut self) -> ERROR_RX_TIMEOUT_W<6> {
        ERROR_RX_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 7 - Source: SIE_STATUS.RX_OVERFLOW"]
    #[inline(always)]
    #[must_use]
    pub fn error_rx_overflow(&mut self) -> ERROR_RX_OVERFLOW_W<7> {
        ERROR_RX_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Source: SIE_STATUS.BIT_STUFF_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error_bit_stuff(&mut self) -> ERROR_BIT_STUFF_W<8> {
        ERROR_BIT_STUFF_W::new(self)
    }
    #[doc = "Bit 9 - Source: SIE_STATUS.CRC_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error_crc(&mut self) -> ERROR_CRC_W<9> {
        ERROR_CRC_W::new(self)
    }
    #[doc = "Bit 10 - Source: SIE_STATUS.STALL_REC"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<10> {
        STALL_W::new(self)
    }
    #[doc = "Bit 11 - Source: SIE_STATUS.VBUS_DETECTED"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_detect(&mut self) -> VBUS_DETECT_W<11> {
        VBUS_DETECT_W::new(self)
    }
    #[doc = "Bit 12 - Source: SIE_STATUS.BUS_RESET"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset(&mut self) -> BUS_RESET_W<12> {
        BUS_RESET_W::new(self)
    }
    #[doc = "Bit 13 - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    #[inline(always)]
    #[must_use]
    pub fn dev_conn_dis(&mut self) -> DEV_CONN_DIS_W<13> {
        DEV_CONN_DIS_W::new(self)
    }
    #[doc = "Bit 14 - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    #[inline(always)]
    #[must_use]
    pub fn dev_suspend(&mut self) -> DEV_SUSPEND_W<14> {
        DEV_SUSPEND_W::new(self)
    }
    #[doc = "Bit 15 - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    #[must_use]
    pub fn dev_resume_from_host(&mut self) -> DEV_RESUME_FROM_HOST_W<15> {
        DEV_RESUME_FROM_HOST_W::new(self)
    }
    #[doc = "Bit 16 - Device. Source: SIE_STATUS.SETUP_REC"]
    #[inline(always)]
    #[must_use]
    pub fn setup_req(&mut self) -> SETUP_REQ_W<16> {
        SETUP_REQ_W::new(self)
    }
    #[doc = "Bit 17 - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    #[inline(always)]
    #[must_use]
    pub fn dev_sof(&mut self) -> DEV_SOF_W<17> {
        DEV_SOF_W::new(self)
    }
    #[doc = "Bit 18 - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn abort_done(&mut self) -> ABORT_DONE_W<18> {
        ABORT_DONE_W::new(self)
    }
    #[doc = "Bit 19 - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    #[inline(always)]
    #[must_use]
    pub fn ep_stall_nak(&mut self) -> EP_STALL_NAK_W<19> {
        EP_STALL_NAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Force  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intf::W](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
