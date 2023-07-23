#[doc = "Register `EP_BUFFER_CONTROL%s` reader"]
pub struct R(crate::R<EP_BUFFER_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP_BUFFER_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP_BUFFER_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP_BUFFER_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP_BUFFER_CONTROL%s` writer"]
pub struct W(crate::W<EP_BUFFER_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP_BUFFER_CONTROL_SPEC>;
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
impl From<crate::W<EP_BUFFER_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP_BUFFER_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENGTH_0` reader - The length of the data in buffer 0."]
pub type LENGTH_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LENGTH_0` writer - The length of the data in buffer 0."]
pub type LENGTH_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, u16, u16, 10, O>;
#[doc = "Field `AVAILABLE_0` reader - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub type AVAILABLE_0_R = crate::BitReader<bool>;
#[doc = "Field `AVAILABLE_0` writer - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub type AVAILABLE_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `STALL` reader - Reply with a stall (valid for both buffers)."]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Reply with a stall (valid for both buffers)."]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Reset the buffer selector to buffer 0."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Reset the buffer selector to buffer 0."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `PID_0` reader - The data pid of buffer 0."]
pub type PID_0_R = crate::BitReader<bool>;
#[doc = "Field `PID_0` writer - The data pid of buffer 0."]
pub type PID_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `LAST_0` reader - Buffer 0 is the last buffer of the transfer."]
pub type LAST_0_R = crate::BitReader<bool>;
#[doc = "Field `LAST_0` writer - Buffer 0 is the last buffer of the transfer."]
pub type LAST_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `FULL_0` reader - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub type FULL_0_R = crate::BitReader<bool>;
#[doc = "Field `FULL_0` writer - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub type FULL_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `LENGTH_1` reader - The length of the data in buffer 1."]
pub type LENGTH_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LENGTH_1` writer - The length of the data in buffer 1."]
pub type LENGTH_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, u16, u16, 10, O>;
#[doc = "Field `AVAILABLE_1` reader - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub type AVAILABLE_1_R = crate::BitReader<bool>;
#[doc = "Field `AVAILABLE_1` writer - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub type AVAILABLE_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `DOUBLE_BUFFER_ISO_OFFSET` reader - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes."]
pub type DOUBLE_BUFFER_ISO_OFFSET_R = crate::FieldReader<u8, DOUBLE_BUFFER_ISO_OFFSET_A>;
#[doc = "The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOUBLE_BUFFER_ISO_OFFSET_A {
    #[doc = "0: `0`"]
    _128 = 0,
    #[doc = "1: `1`"]
    _256 = 1,
    #[doc = "2: `10`"]
    _512 = 2,
    #[doc = "3: `11`"]
    _1024 = 3,
}
impl From<DOUBLE_BUFFER_ISO_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: DOUBLE_BUFFER_ISO_OFFSET_A) -> Self {
        variant as _
    }
}
impl DOUBLE_BUFFER_ISO_OFFSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUBLE_BUFFER_ISO_OFFSET_A {
        match self.bits {
            0 => DOUBLE_BUFFER_ISO_OFFSET_A::_128,
            1 => DOUBLE_BUFFER_ISO_OFFSET_A::_256,
            2 => DOUBLE_BUFFER_ISO_OFFSET_A::_512,
            3 => DOUBLE_BUFFER_ISO_OFFSET_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == DOUBLE_BUFFER_ISO_OFFSET_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == DOUBLE_BUFFER_ISO_OFFSET_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == DOUBLE_BUFFER_ISO_OFFSET_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == DOUBLE_BUFFER_ISO_OFFSET_A::_1024
    }
}
#[doc = "Field `DOUBLE_BUFFER_ISO_OFFSET` writer - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes."]
pub type DOUBLE_BUFFER_ISO_OFFSET_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EP_BUFFER_CONTROL_SPEC, u8, DOUBLE_BUFFER_ISO_OFFSET_A, 2, O>;
impl<'a, const O: u8> DOUBLE_BUFFER_ISO_OFFSET_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(DOUBLE_BUFFER_ISO_OFFSET_A::_128)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(DOUBLE_BUFFER_ISO_OFFSET_A::_256)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(DOUBLE_BUFFER_ISO_OFFSET_A::_512)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(DOUBLE_BUFFER_ISO_OFFSET_A::_1024)
    }
}
#[doc = "Field `PID_1` reader - The data pid of buffer 1."]
pub type PID_1_R = crate::BitReader<bool>;
#[doc = "Field `PID_1` writer - The data pid of buffer 1."]
pub type PID_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `LAST_1` reader - Buffer 1 is the last buffer of the transfer."]
pub type LAST_1_R = crate::BitReader<bool>;
#[doc = "Field `LAST_1` writer - Buffer 1 is the last buffer of the transfer."]
pub type LAST_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
#[doc = "Field `FULL_1` reader - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub type FULL_1_R = crate::BitReader<bool>;
#[doc = "Field `FULL_1` writer - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub type FULL_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_BUFFER_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - The length of the data in buffer 0."]
    #[inline(always)]
    pub fn length_0(&self) -> LENGTH_0_R {
        LENGTH_0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    pub fn available_0(&self) -> AVAILABLE_0_R {
        AVAILABLE_0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reply with a stall (valid for both buffers)."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset the buffer selector to buffer 0."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The data pid of buffer 0."]
    #[inline(always)]
    pub fn pid_0(&self) -> PID_0_R {
        PID_0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Buffer 0 is the last buffer of the transfer."]
    #[inline(always)]
    pub fn last_0(&self) -> LAST_0_R {
        LAST_0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    pub fn full_0(&self) -> FULL_0_R {
        FULL_0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - The length of the data in buffer 1."]
    #[inline(always)]
    pub fn length_1(&self) -> LENGTH_1_R {
        LENGTH_1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    pub fn available_1(&self) -> AVAILABLE_1_R {
        AVAILABLE_1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes."]
    #[inline(always)]
    pub fn double_buffer_iso_offset(&self) -> DOUBLE_BUFFER_ISO_OFFSET_R {
        DOUBLE_BUFFER_ISO_OFFSET_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - The data pid of buffer 1."]
    #[inline(always)]
    pub fn pid_1(&self) -> PID_1_R {
        PID_1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Buffer 1 is the last buffer of the transfer."]
    #[inline(always)]
    pub fn last_1(&self) -> LAST_1_R {
        LAST_1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    pub fn full_1(&self) -> FULL_1_R {
        FULL_1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - The length of the data in buffer 0."]
    #[inline(always)]
    #[must_use]
    pub fn length_0(&mut self) -> LENGTH_0_W<0> {
        LENGTH_0_W::new(self)
    }
    #[doc = "Bit 10 - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    #[must_use]
    pub fn available_0(&mut self) -> AVAILABLE_0_W<10> {
        AVAILABLE_0_W::new(self)
    }
    #[doc = "Bit 11 - Reply with a stall (valid for both buffers)."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<11> {
        STALL_W::new(self)
    }
    #[doc = "Bit 12 - Reset the buffer selector to buffer 0."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<12> {
        RESET_W::new(self)
    }
    #[doc = "Bit 13 - The data pid of buffer 0."]
    #[inline(always)]
    #[must_use]
    pub fn pid_0(&mut self) -> PID_0_W<13> {
        PID_0_W::new(self)
    }
    #[doc = "Bit 14 - Buffer 0 is the last buffer of the transfer."]
    #[inline(always)]
    #[must_use]
    pub fn last_0(&mut self) -> LAST_0_W<14> {
        LAST_0_W::new(self)
    }
    #[doc = "Bit 15 - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    #[must_use]
    pub fn full_0(&mut self) -> FULL_0_W<15> {
        FULL_0_W::new(self)
    }
    #[doc = "Bits 16:25 - The length of the data in buffer 1."]
    #[inline(always)]
    #[must_use]
    pub fn length_1(&mut self) -> LENGTH_1_W<16> {
        LENGTH_1_W::new(self)
    }
    #[doc = "Bit 26 - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    #[must_use]
    pub fn available_1(&mut self) -> AVAILABLE_1_W<26> {
        AVAILABLE_1_W::new(self)
    }
    #[doc = "Bits 27:28 - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn double_buffer_iso_offset(&mut self) -> DOUBLE_BUFFER_ISO_OFFSET_W<27> {
        DOUBLE_BUFFER_ISO_OFFSET_W::new(self)
    }
    #[doc = "Bit 29 - The data pid of buffer 1."]
    #[inline(always)]
    #[must_use]
    pub fn pid_1(&mut self) -> PID_1_W<29> {
        PID_1_W::new(self)
    }
    #[doc = "Bit 30 - Buffer 1 is the last buffer of the transfer."]
    #[inline(always)]
    #[must_use]
    pub fn last_1(&mut self) -> LAST_1_W<30> {
        LAST_1_W::new(self)
    }
    #[doc = "Bit 31 - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    #[must_use]
    pub fn full_1(&mut self) -> FULL_1_W<31> {
        FULL_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "-  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ep_buffer_control](index.html) module"]
pub struct EP_BUFFER_CONTROL_SPEC;
impl crate::RegisterSpec for EP_BUFFER_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep_buffer_control::R](R) reader structure"]
impl crate::Readable for EP_BUFFER_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep_buffer_control::W](W) writer structure"]
impl crate::Writable for EP_BUFFER_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP_BUFFER_CONTROL%s to value 0"]
impl crate::Resettable for EP_BUFFER_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
