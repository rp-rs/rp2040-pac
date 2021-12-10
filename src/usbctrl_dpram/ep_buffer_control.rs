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
#[doc = "Field `FULL_1` reader - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub struct FULL_1_R(crate::FieldReader<bool, bool>);
impl FULL_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULL_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL_1` writer - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub struct FULL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_1_W<'a> {
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
#[doc = "Field `LAST_1` reader - Buffer 1 is the last buffer of the transfer."]
pub struct LAST_1_R(crate::FieldReader<bool, bool>);
impl LAST_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LAST_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LAST_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_1` writer - Buffer 1 is the last buffer of the transfer."]
pub struct LAST_1_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_1_W<'a> {
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
#[doc = "Field `PID_1` reader - The data pid of buffer 1."]
pub struct PID_1_R(crate::FieldReader<bool, bool>);
impl PID_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID_1` writer - The data pid of buffer 1."]
pub struct PID_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_1_W<'a> {
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
#[doc = "The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DOUBLE_BUFFER_ISO_OFFSET` reader - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes."]
pub struct DOUBLE_BUFFER_ISO_OFFSET_R(crate::FieldReader<u8, DOUBLE_BUFFER_ISO_OFFSET_A>);
impl DOUBLE_BUFFER_ISO_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUBLE_BUFFER_ISO_OFFSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DOUBLE_BUFFER_ISO_OFFSET_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == DOUBLE_BUFFER_ISO_OFFSET_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        **self == DOUBLE_BUFFER_ISO_OFFSET_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        **self == DOUBLE_BUFFER_ISO_OFFSET_A::_1024
    }
}
impl core::ops::Deref for DOUBLE_BUFFER_ISO_OFFSET_R {
    type Target = crate::FieldReader<u8, DOUBLE_BUFFER_ISO_OFFSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUBLE_BUFFER_ISO_OFFSET` writer - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes."]
pub struct DOUBLE_BUFFER_ISO_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLE_BUFFER_ISO_OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUBLE_BUFFER_ISO_OFFSET_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `AVAILABLE_1` reader - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub struct AVAILABLE_1_R(crate::FieldReader<bool, bool>);
impl AVAILABLE_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVAILABLE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVAILABLE_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVAILABLE_1` writer - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub struct AVAILABLE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> AVAILABLE_1_W<'a> {
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
#[doc = "Field `LENGTH_1` reader - The length of the data in buffer 1."]
pub struct LENGTH_1_R(crate::FieldReader<u16, u16>);
impl LENGTH_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LENGTH_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENGTH_1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENGTH_1` writer - The length of the data in buffer 1."]
pub struct LENGTH_1_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `FULL_0` reader - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub struct FULL_0_R(crate::FieldReader<bool, bool>);
impl FULL_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULL_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL_0` writer - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub struct FULL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `LAST_0` reader - Buffer 0 is the last buffer of the transfer."]
pub struct LAST_0_R(crate::FieldReader<bool, bool>);
impl LAST_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LAST_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LAST_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_0` writer - Buffer 0 is the last buffer of the transfer."]
pub struct LAST_0_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PID_0` reader - The data pid of buffer 0."]
pub struct PID_0_R(crate::FieldReader<bool, bool>);
impl PID_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID_0` writer - The data pid of buffer 0."]
pub struct PID_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RESET` reader - Reset the buffer selector to buffer 0."]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - Reset the buffer selector to buffer 0."]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `STALL` reader - Reply with a stall (valid for both buffers)."]
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
#[doc = "Field `STALL` writer - Reply with a stall (valid for both buffers)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `AVAILABLE_0` reader - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub struct AVAILABLE_0_R(crate::FieldReader<bool, bool>);
impl AVAILABLE_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVAILABLE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVAILABLE_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVAILABLE_0` writer - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub struct AVAILABLE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> AVAILABLE_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `LENGTH_0` reader - The length of the data in buffer 0."]
pub struct LENGTH_0_R(crate::FieldReader<u16, u16>);
impl LENGTH_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LENGTH_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENGTH_0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENGTH_0` writer - The length of the data in buffer 0."]
pub struct LENGTH_0_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    pub fn full_1(&self) -> FULL_1_R {
        FULL_1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Buffer 1 is the last buffer of the transfer."]
    #[inline(always)]
    pub fn last_1(&self) -> LAST_1_R {
        LAST_1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The data pid of buffer 1."]
    #[inline(always)]
    pub fn pid_1(&self) -> PID_1_R {
        PID_1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes."]
    #[inline(always)]
    pub fn double_buffer_iso_offset(&self) -> DOUBLE_BUFFER_ISO_OFFSET_R {
        DOUBLE_BUFFER_ISO_OFFSET_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    pub fn available_1(&self) -> AVAILABLE_1_R {
        AVAILABLE_1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - The length of the data in buffer 1."]
    #[inline(always)]
    pub fn length_1(&self) -> LENGTH_1_R {
        LENGTH_1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    pub fn full_0(&self) -> FULL_0_R {
        FULL_0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Buffer 0 is the last buffer of the transfer."]
    #[inline(always)]
    pub fn last_0(&self) -> LAST_0_R {
        LAST_0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The data pid of buffer 0."]
    #[inline(always)]
    pub fn pid_0(&self) -> PID_0_R {
        PID_0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reset the buffer selector to buffer 0."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reply with a stall (valid for both buffers)."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    pub fn available_0(&self) -> AVAILABLE_0_R {
        AVAILABLE_0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - The length of the data in buffer 0."]
    #[inline(always)]
    pub fn length_0(&self) -> LENGTH_0_R {
        LENGTH_0_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    pub fn full_1(&mut self) -> FULL_1_W {
        FULL_1_W { w: self }
    }
    #[doc = "Bit 30 - Buffer 1 is the last buffer of the transfer."]
    #[inline(always)]
    pub fn last_1(&mut self) -> LAST_1_W {
        LAST_1_W { w: self }
    }
    #[doc = "Bit 29 - The data pid of buffer 1."]
    #[inline(always)]
    pub fn pid_1(&mut self) -> PID_1_W {
        PID_1_W { w: self }
    }
    #[doc = "Bits 27:28 - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint.  
 For a non Isochronous endpoint the offset is always 64 bytes."]
    #[inline(always)]
    pub fn double_buffer_iso_offset(&mut self) -> DOUBLE_BUFFER_ISO_OFFSET_W {
        DOUBLE_BUFFER_ISO_OFFSET_W { w: self }
    }
    #[doc = "Bit 26 - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    pub fn available_1(&mut self) -> AVAILABLE_1_W {
        AVAILABLE_1_W { w: self }
    }
    #[doc = "Bits 16:25 - The length of the data in buffer 1."]
    #[inline(always)]
    pub fn length_1(&mut self) -> LENGTH_1_W {
        LENGTH_1_W { w: self }
    }
    #[doc = "Bit 15 - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    pub fn full_0(&mut self) -> FULL_0_W {
        FULL_0_W { w: self }
    }
    #[doc = "Bit 14 - Buffer 0 is the last buffer of the transfer."]
    #[inline(always)]
    pub fn last_0(&mut self) -> LAST_0_W {
        LAST_0_W { w: self }
    }
    #[doc = "Bit 13 - The data pid of buffer 0."]
    #[inline(always)]
    pub fn pid_0(&mut self) -> PID_0_W {
        PID_0_W { w: self }
    }
    #[doc = "Bit 12 - Reset the buffer selector to buffer 0."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 11 - Reply with a stall (valid for both buffers)."]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 10 - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    pub fn available_0(&mut self) -> AVAILABLE_0_W {
        AVAILABLE_0_W { w: self }
    }
    #[doc = "Bits 0:9 - The length of the data in buffer 0."]
    #[inline(always)]
    pub fn length_0(&mut self) -> LENGTH_0_W {
        LENGTH_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "%s-%s  

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
}
#[doc = "`reset()` method sets EP_BUFFER_CONTROL%s to value 0"]
impl crate::Resettable for EP_BUFFER_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
