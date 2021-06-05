#[doc = "Reader of register EP6_OUT_CONTROL"]
pub type R = crate::R<u32, super::EP6_OUT_CONTROL>;
#[doc = "Writer for register EP6_OUT_CONTROL"]
pub type W = crate::W<u32, super::EP6_OUT_CONTROL>;
#[doc = "Register EP6_OUT_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::EP6_OUT_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `DOUBLE_BUFFERED`"]
pub type DOUBLE_BUFFERED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUBLE_BUFFERED`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `INTERRUPT_PER_BUFF`"]
pub type INTERRUPT_PER_BUFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERRUPT_PER_BUFF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `INTERRUPT_PER_DOUBLE_BUFF`"]
pub type INTERRUPT_PER_DOUBLE_BUFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERRUPT_PER_DOUBLE_BUFF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
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
#[doc = "Reader of field `ENDPOINT_TYPE`"]
pub type ENDPOINT_TYPE_R = crate::R<u8, ENDPOINT_TYPE_A>;
impl ENDPOINT_TYPE_R {
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
        *self == ENDPOINT_TYPE_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == ENDPOINT_TYPE_A::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == ENDPOINT_TYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == ENDPOINT_TYPE_A::INTERRUPT
    }
}
#[doc = "Write proxy for field `ENDPOINT_TYPE`"]
pub struct ENDPOINT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPOINT_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDPOINT_TYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `INTERRUPT_ON_STALL`"]
pub type INTERRUPT_ON_STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERRUPT_ON_STALL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `INTERRUPT_ON_NAK`"]
pub type INTERRUPT_ON_NAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERRUPT_ON_NAK`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `BUFFER_ADDRESS`"]
pub type BUFFER_ADDRESS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUFFER_ADDRESS`"]
pub struct BUFFER_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
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
}
