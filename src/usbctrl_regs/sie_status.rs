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
#[doc = "Field `VBUS_DETECTED` reader - Device: VBUS Detected"]
pub type VBUS_DETECTED_R = crate::BitReader<bool>;
#[doc = "Field `LINE_STATE` reader - USB bus line state"]
pub type LINE_STATE_R = crate::FieldReader<u8, LINE_STATE_A>;
#[doc = "USB bus line state  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LINE_STATE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == LINE_STATE_A::SE0
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == LINE_STATE_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == LINE_STATE_A::K
    }
    #[doc = "Checks if the value of the field is `SE1`"]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == LINE_STATE_A::SE1
    }
}
#[doc = "Field `SUSPENDED` reader - Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
pub type SUSPENDED_R = crate::BitReader<bool>;
#[doc = "Field `SUSPENDED` writer - Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
pub type SUSPENDED_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPEED` writer - Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
pub type SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIE_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `VBUS_OVER_CURR` reader - VBUS over current detected"]
pub type VBUS_OVER_CURR_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` reader - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `CONNECTED` reader - Device: connected"]
pub type CONNECTED_R = crate::BitReader<bool>;
#[doc = "Field `CONNECTED` writer - Device: connected"]
pub type CONNECTED_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `SETUP_REC` reader - Device: Setup packet received"]
pub type SETUP_REC_R = crate::BitReader<bool>;
#[doc = "Field `SETUP_REC` writer - Device: Setup packet received"]
pub type SETUP_REC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `TRANS_COMPLETE` reader - Transaction complete.  

 Raised by device if:  

 * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register  

 Raised by host if:  

 * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
pub type TRANS_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE` writer - Transaction complete.  

 Raised by device if:  

 * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register  

 Raised by host if:  

 * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
pub type TRANS_COMPLETE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `BUS_RESET` reader - Device: bus reset received"]
pub type BUS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `BUS_RESET` writer - Device: bus reset received"]
pub type BUS_RESET_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `CRC_ERROR` reader - CRC Error. Raised by the Serial RX engine."]
pub type CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `CRC_ERROR` writer - CRC Error. Raised by the Serial RX engine."]
pub type CRC_ERROR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `BIT_STUFF_ERROR` reader - Bit Stuff Error. Raised by the Serial RX engine."]
pub type BIT_STUFF_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `BIT_STUFF_ERROR` writer - Bit Stuff Error. Raised by the Serial RX engine."]
pub type BIT_STUFF_ERROR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `RX_OVERFLOW` reader - RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
pub type RX_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERFLOW` writer - RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
pub type RX_OVERFLOW_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `RX_TIMEOUT` reader - RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
pub type RX_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `RX_TIMEOUT` writer - RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
pub type RX_TIMEOUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `NAK_REC` reader - Host: NAK received"]
pub type NAK_REC_R = crate::BitReader<bool>;
#[doc = "Field `NAK_REC` writer - Host: NAK received"]
pub type NAK_REC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `STALL_REC` reader - Host: STALL received"]
pub type STALL_REC_R = crate::BitReader<bool>;
#[doc = "Field `STALL_REC` writer - Host: STALL received"]
pub type STALL_REC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `ACK_REC` reader - ACK received. Raised by both host and device."]
pub type ACK_REC_R = crate::BitReader<bool>;
#[doc = "Field `ACK_REC` writer - ACK received. Raised by both host and device."]
pub type ACK_REC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
#[doc = "Field `DATA_SEQ_ERROR` reader - Data Sequence Error.  

 The device can raise a sequence error in the following conditions:  

 * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM  

 The host can raise a data sequence error in the following conditions:  

 * An IN packet from the device has the wrong data PID"]
pub type DATA_SEQ_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `DATA_SEQ_ERROR` writer - Data Sequence Error.  

 The device can raise a sequence error in the following conditions:  

 * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM  

 The host can raise a data sequence error in the following conditions:  

 * An IN packet from the device has the wrong data PID"]
pub type DATA_SEQ_ERROR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SIE_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Device: VBUS Detected"]
    #[inline(always)]
    pub fn vbus_detected(&self) -> VBUS_DETECTED_R {
        VBUS_DETECTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - USB bus line state"]
    #[inline(always)]
    pub fn line_state(&self) -> LINE_STATE_R {
        LINE_STATE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
    #[inline(always)]
    pub fn suspended(&self) -> SUSPENDED_R {
        SUSPENDED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - VBUS over current detected"]
    #[inline(always)]
    pub fn vbus_over_curr(&self) -> VBUS_OVER_CURR_R {
        VBUS_OVER_CURR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Device: connected"]
    #[inline(always)]
    pub fn connected(&self) -> CONNECTED_R {
        CONNECTED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device: Setup packet received"]
    #[inline(always)]
    pub fn setup_rec(&self) -> SETUP_REC_R {
        SETUP_REC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transaction complete.  

 Raised by device if:  

 * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register  

 Raised by host if:  

 * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Device: bus reset received"]
    #[inline(always)]
    pub fn bus_reset(&self) -> BUS_RESET_R {
        BUS_RESET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - CRC Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub fn crc_error(&self) -> CRC_ERROR_R {
        CRC_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bit Stuff Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub fn bit_stuff_error(&self) -> BIT_STUFF_ERROR_R {
        BIT_STUFF_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
    #[inline(always)]
    pub fn rx_timeout(&self) -> RX_TIMEOUT_R {
        RX_TIMEOUT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Host: NAK received"]
    #[inline(always)]
    pub fn nak_rec(&self) -> NAK_REC_R {
        NAK_REC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Host: STALL received"]
    #[inline(always)]
    pub fn stall_rec(&self) -> STALL_REC_R {
        STALL_REC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ACK received. Raised by both host and device."]
    #[inline(always)]
    pub fn ack_rec(&self) -> ACK_REC_R {
        ACK_REC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Sequence Error.  

 The device can raise a sequence error in the following conditions:  

 * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM  

 The host can raise a data sequence error in the following conditions:  

 * An IN packet from the device has the wrong data PID"]
    #[inline(always)]
    pub fn data_seq_error(&self) -> DATA_SEQ_ERROR_R {
        DATA_SEQ_ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
    #[inline(always)]
    #[must_use]
    pub fn suspended(&mut self) -> SUSPENDED_W<4> {
        SUSPENDED_W::new(self)
    }
    #[doc = "Bits 8:9 - Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<8> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 11 - Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<11> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 16 - Device: connected"]
    #[inline(always)]
    #[must_use]
    pub fn connected(&mut self) -> CONNECTED_W<16> {
        CONNECTED_W::new(self)
    }
    #[doc = "Bit 17 - Device: Setup packet received"]
    #[inline(always)]
    #[must_use]
    pub fn setup_rec(&mut self) -> SETUP_REC_W<17> {
        SETUP_REC_W::new(self)
    }
    #[doc = "Bit 18 - Transaction complete.  

 Raised by device if:  

 * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register  

 Raised by host if:  

 * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<18> {
        TRANS_COMPLETE_W::new(self)
    }
    #[doc = "Bit 19 - Device: bus reset received"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset(&mut self) -> BUS_RESET_W<19> {
        BUS_RESET_W::new(self)
    }
    #[doc = "Bit 24 - CRC Error. Raised by the Serial RX engine."]
    #[inline(always)]
    #[must_use]
    pub fn crc_error(&mut self) -> CRC_ERROR_W<24> {
        CRC_ERROR_W::new(self)
    }
    #[doc = "Bit 25 - Bit Stuff Error. Raised by the Serial RX engine."]
    #[inline(always)]
    #[must_use]
    pub fn bit_stuff_error(&mut self) -> BIT_STUFF_ERROR_W<25> {
        BIT_STUFF_ERROR_W::new(self)
    }
    #[doc = "Bit 26 - RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W<26> {
        RX_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 27 - RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
    #[inline(always)]
    #[must_use]
    pub fn rx_timeout(&mut self) -> RX_TIMEOUT_W<27> {
        RX_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 28 - Host: NAK received"]
    #[inline(always)]
    #[must_use]
    pub fn nak_rec(&mut self) -> NAK_REC_W<28> {
        NAK_REC_W::new(self)
    }
    #[doc = "Bit 29 - Host: STALL received"]
    #[inline(always)]
    #[must_use]
    pub fn stall_rec(&mut self) -> STALL_REC_W<29> {
        STALL_REC_W::new(self)
    }
    #[doc = "Bit 30 - ACK received. Raised by both host and device."]
    #[inline(always)]
    #[must_use]
    pub fn ack_rec(&mut self) -> ACK_REC_W<30> {
        ACK_REC_W::new(self)
    }
    #[doc = "Bit 31 - Data Sequence Error.  

 The device can raise a sequence error in the following conditions:  

 * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM  

 The host can raise a data sequence error in the following conditions:  

 * An IN packet from the device has the wrong data PID"]
    #[inline(always)]
    #[must_use]
    pub fn data_seq_error(&mut self) -> DATA_SEQ_ERROR_W<31> {
        DATA_SEQ_ERROR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff0f_0b10;
}
#[doc = "`reset()` method sets SIE_STATUS to value 0"]
impl crate::Resettable for SIE_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
