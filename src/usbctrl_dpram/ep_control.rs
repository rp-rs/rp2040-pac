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
#[doc = "Field `BUFFER_ADDRESS` reader - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
pub type BUFFER_ADDRESS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BUFFER_ADDRESS` writer - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
pub type BUFFER_ADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP_CONTROL_SPEC, u16, u16, 16, O>;
#[doc = "Field `INTERRUPT_ON_NAK` reader - Trigger an interrupt if a NAK is sent. Intended for debug only."]
pub type INTERRUPT_ON_NAK_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT_ON_NAK` writer - Trigger an interrupt if a NAK is sent. Intended for debug only."]
pub type INTERRUPT_ON_NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CONTROL_SPEC, bool, O>;
#[doc = "Field `INTERRUPT_ON_STALL` reader - Trigger an interrupt if a STALL is sent. Intended for debug only."]
pub type INTERRUPT_ON_STALL_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT_ON_STALL` writer - Trigger an interrupt if a STALL is sent. Intended for debug only."]
pub type INTERRUPT_ON_STALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP_CONTROL_SPEC, bool, O>;
#[doc = "Field `ENDPOINT_TYPE` reader - "]
pub type ENDPOINT_TYPE_R = crate::FieldReader<u8, ENDPOINT_TYPE_A>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ENDPOINT_TYPE_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `ENDPOINT_TYPE` writer - "]
pub type ENDPOINT_TYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EP_CONTROL_SPEC, u8, ENDPOINT_TYPE_A, 2, O>;
impl<'a, const O: u8> ENDPOINT_TYPE_W<'a, O> {
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
}
#[doc = "Field `INTERRUPT_PER_DOUBLE_BUFF` reader - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
pub type INTERRUPT_PER_DOUBLE_BUFF_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT_PER_DOUBLE_BUFF` writer - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
pub type INTERRUPT_PER_DOUBLE_BUFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP_CONTROL_SPEC, bool, O>;
#[doc = "Field `INTERRUPT_PER_BUFF` reader - Trigger an interrupt each time a buffer is done."]
pub type INTERRUPT_PER_BUFF_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT_PER_BUFF` writer - Trigger an interrupt each time a buffer is done."]
pub type INTERRUPT_PER_BUFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP_CONTROL_SPEC, bool, O>;
#[doc = "Field `DOUBLE_BUFFERED` reader - This endpoint is double buffered."]
pub type DOUBLE_BUFFERED_R = crate::BitReader<bool>;
#[doc = "Field `DOUBLE_BUFFERED` writer - This endpoint is double buffered."]
pub type DOUBLE_BUFFERED_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CONTROL_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
    #[inline(always)]
    pub fn buffer_address(&self) -> BUFFER_ADDRESS_R {
        BUFFER_ADDRESS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Trigger an interrupt if a NAK is sent. Intended for debug only."]
    #[inline(always)]
    pub fn interrupt_on_nak(&self) -> INTERRUPT_ON_NAK_R {
        INTERRUPT_ON_NAK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Trigger an interrupt if a STALL is sent. Intended for debug only."]
    #[inline(always)]
    pub fn interrupt_on_stall(&self) -> INTERRUPT_ON_STALL_R {
        INTERRUPT_ON_STALL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn endpoint_type(&self) -> ENDPOINT_TYPE_R {
        ENDPOINT_TYPE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
    #[inline(always)]
    pub fn interrupt_per_double_buff(&self) -> INTERRUPT_PER_DOUBLE_BUFF_R {
        INTERRUPT_PER_DOUBLE_BUFF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Trigger an interrupt each time a buffer is done."]
    #[inline(always)]
    pub fn interrupt_per_buff(&self) -> INTERRUPT_PER_BUFF_R {
        INTERRUPT_PER_BUFF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This endpoint is double buffered."]
    #[inline(always)]
    pub fn double_buffered(&self) -> DOUBLE_BUFFERED_R {
        DOUBLE_BUFFERED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
    #[inline(always)]
    #[must_use]
    pub fn buffer_address(&mut self) -> BUFFER_ADDRESS_W<0> {
        BUFFER_ADDRESS_W::new(self)
    }
    #[doc = "Bit 16 - Trigger an interrupt if a NAK is sent. Intended for debug only."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_on_nak(&mut self) -> INTERRUPT_ON_NAK_W<16> {
        INTERRUPT_ON_NAK_W::new(self)
    }
    #[doc = "Bit 17 - Trigger an interrupt if a STALL is sent. Intended for debug only."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_on_stall(&mut self) -> INTERRUPT_ON_STALL_W<17> {
        INTERRUPT_ON_STALL_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn endpoint_type(&mut self) -> ENDPOINT_TYPE_W<26> {
        ENDPOINT_TYPE_W::new(self)
    }
    #[doc = "Bit 28 - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_per_double_buff(&mut self) -> INTERRUPT_PER_DOUBLE_BUFF_W<28> {
        INTERRUPT_PER_DOUBLE_BUFF_W::new(self)
    }
    #[doc = "Bit 29 - Trigger an interrupt each time a buffer is done."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_per_buff(&mut self) -> INTERRUPT_PER_BUFF_W<29> {
        INTERRUPT_PER_BUFF_W::new(self)
    }
    #[doc = "Bit 30 - This endpoint is double buffered."]
    #[inline(always)]
    #[must_use]
    pub fn double_buffered(&mut self) -> DOUBLE_BUFFERED_W<30> {
        DOUBLE_BUFFERED_W::new(self)
    }
    #[doc = "Bit 31 - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP_CONTROL%s to value 0"]
impl crate::Resettable for EP_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
