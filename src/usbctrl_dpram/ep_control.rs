#[doc = "Register `EP_CONTROL%s` reader"]
pub struct R(crate::R<EP_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP_CONTROL%s` writer"]
pub struct W(crate::W<EP_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP_CONTROL_SPEC>;
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
impl From<crate::W<EP_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `DOUBLE_BUFFERED` reader - This endpoint is double buffered."]
pub struct DOUBLE_BUFFERED_R(crate::FieldReader<bool, bool>);
impl DOUBLE_BUFFERED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUBLE_BUFFERED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUBLE_BUFFERED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUBLE_BUFFERED` writer - This endpoint is double buffered."]
pub struct DOUBLE_BUFFERED_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLE_BUFFERED_W<'a> {
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
#[doc = "Field `INTERRUPT_PER_BUFF` reader - Trigger an interrupt each time a buffer is done."]
pub struct INTERRUPT_PER_BUFF_R(crate::FieldReader<bool, bool>);
impl INTERRUPT_PER_BUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERRUPT_PER_BUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRUPT_PER_BUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRUPT_PER_BUFF` writer - Trigger an interrupt each time a buffer is done."]
pub struct INTERRUPT_PER_BUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_PER_BUFF_W<'a> {
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
#[doc = "Field `INTERRUPT_PER_DOUBLE_BUFF` reader - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
pub struct INTERRUPT_PER_DOUBLE_BUFF_R(crate::FieldReader<bool, bool>);
impl INTERRUPT_PER_DOUBLE_BUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERRUPT_PER_DOUBLE_BUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRUPT_PER_DOUBLE_BUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRUPT_PER_DOUBLE_BUFF` writer - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
pub struct INTERRUPT_PER_DOUBLE_BUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_PER_DOUBLE_BUFF_W<'a> {
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
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENDPOINT_TYPE_A {
    #[doc = "0: `0`"]
    CONTROL = 0,
    #[doc = "1: `1`"]
    ISOCHRONOUS = 1,
    #[doc = "2: `10`"]
    BULK = 2,
    #[doc = "3: `11`"]
    INTERRUPT = 3,
}
impl From<ENDPOINT_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: ENDPOINT_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ENDPOINT_TYPE` reader - "]
pub struct ENDPOINT_TYPE_R(crate::FieldReader<u8, ENDPOINT_TYPE_A>);
impl ENDPOINT_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ENDPOINT_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDPOINT_TYPE_A {
        match self.bits {
            0 => ENDPOINT_TYPE_A::CONTROL,
            1 => ENDPOINT_TYPE_A::ISOCHRONOUS,
            2 => ENDPOINT_TYPE_A::BULK,
            3 => ENDPOINT_TYPE_A::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        **self == ENDPOINT_TYPE_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        **self == ENDPOINT_TYPE_A::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        **self == ENDPOINT_TYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        **self == ENDPOINT_TYPE_A::INTERRUPT
    }
}
impl core::ops::Deref for ENDPOINT_TYPE_R {
    type Target = crate::FieldReader<u8, ENDPOINT_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDPOINT_TYPE` writer - "]
pub struct ENDPOINT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPOINT_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDPOINT_TYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(ENDPOINT_TYPE_A::CONTROL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(ENDPOINT_TYPE_A::ISOCHRONOUS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(ENDPOINT_TYPE_A::BULK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(ENDPOINT_TYPE_A::INTERRUPT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `INTERRUPT_ON_STALL` reader - Trigger an interrupt if a STALL is sent. Intended for debug only."]
pub struct INTERRUPT_ON_STALL_R(crate::FieldReader<bool, bool>);
impl INTERRUPT_ON_STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERRUPT_ON_STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRUPT_ON_STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRUPT_ON_STALL` writer - Trigger an interrupt if a STALL is sent. Intended for debug only."]
pub struct INTERRUPT_ON_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_ON_STALL_W<'a> {
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
#[doc = "Field `INTERRUPT_ON_NAK` reader - Trigger an interrupt if a NAK is sent. Intended for debug only."]
pub struct INTERRUPT_ON_NAK_R(crate::FieldReader<bool, bool>);
impl INTERRUPT_ON_NAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERRUPT_ON_NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRUPT_ON_NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRUPT_ON_NAK` writer - Trigger an interrupt if a NAK is sent. Intended for debug only."]
pub struct INTERRUPT_ON_NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_ON_NAK_W<'a> {
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
#[doc = "Field `BUFFER_ADDRESS` reader - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
pub struct BUFFER_ADDRESS_R(crate::FieldReader<u16, u16>);
impl BUFFER_ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BUFFER_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFFER_ADDRESS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFFER_ADDRESS` writer - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
pub struct BUFFER_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This endpoint is double buffered."]
    #[inline(always)]
    pub fn double_buffered(&self) -> DOUBLE_BUFFERED_R {
        DOUBLE_BUFFERED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Trigger an interrupt each time a buffer is done."]
    #[inline(always)]
    pub fn interrupt_per_buff(&self) -> INTERRUPT_PER_BUFF_R {
        INTERRUPT_PER_BUFF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
    #[inline(always)]
    pub fn interrupt_per_double_buff(&self) -> INTERRUPT_PER_DOUBLE_BUFF_R {
        INTERRUPT_PER_DOUBLE_BUFF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn endpoint_type(&self) -> ENDPOINT_TYPE_R {
        ENDPOINT_TYPE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Trigger an interrupt if a STALL is sent. Intended for debug only."]
    #[inline(always)]
    pub fn interrupt_on_stall(&self) -> INTERRUPT_ON_STALL_R {
        INTERRUPT_ON_STALL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Trigger an interrupt if a NAK is sent. Intended for debug only."]
    #[inline(always)]
    pub fn interrupt_on_nak(&self) -> INTERRUPT_ON_NAK_R {
        INTERRUPT_ON_NAK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
    #[inline(always)]
    pub fn buffer_address(&self) -> BUFFER_ADDRESS_R {
        BUFFER_ADDRESS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 30 - This endpoint is double buffered."]
    #[inline(always)]
    pub fn double_buffered(&mut self) -> DOUBLE_BUFFERED_W {
        DOUBLE_BUFFERED_W { w: self }
    }
    #[doc = "Bit 29 - Trigger an interrupt each time a buffer is done."]
    #[inline(always)]
    pub fn interrupt_per_buff(&mut self) -> INTERRUPT_PER_BUFF_W {
        INTERRUPT_PER_BUFF_W { w: self }
    }
    #[doc = "Bit 28 - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
    #[inline(always)]
    pub fn interrupt_per_double_buff(&mut self) -> INTERRUPT_PER_DOUBLE_BUFF_W {
        INTERRUPT_PER_DOUBLE_BUFF_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn endpoint_type(&mut self) -> ENDPOINT_TYPE_W {
        ENDPOINT_TYPE_W { w: self }
    }
    #[doc = "Bit 17 - Trigger an interrupt if a STALL is sent. Intended for debug only."]
    #[inline(always)]
    pub fn interrupt_on_stall(&mut self) -> INTERRUPT_ON_STALL_W {
        INTERRUPT_ON_STALL_W { w: self }
    }
    #[doc = "Bit 16 - Trigger an interrupt if a NAK is sent. Intended for debug only."]
    #[inline(always)]
    pub fn interrupt_on_nak(&mut self) -> INTERRUPT_ON_NAK_W {
        INTERRUPT_ON_NAK_W { w: self }
    }
    #[doc = "Bits 0:15 - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
    #[inline(always)]
    pub fn buffer_address(&mut self) -> BUFFER_ADDRESS_W {
        BUFFER_ADDRESS_W { w: self }
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

For information about available fields see [ep_control](index.html) module"]
pub struct EP_CONTROL_SPEC;
impl crate::RegisterSpec for EP_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep_control::R](R) reader structure"]
impl crate::Readable for EP_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep_control::W](W) writer structure"]
impl crate::Writable for EP_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP_CONTROL%s to value 0"]
impl crate::Resettable for EP_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
