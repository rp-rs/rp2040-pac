#[doc = "Register `EP_BUFFER_CONTROL%s` reader"]
pub type R = crate::R<EP_BUFFER_CONTROL_SPEC>;
#[doc = "Register `EP_BUFFER_CONTROL%s` writer"]
pub type W = crate::W<EP_BUFFER_CONTROL_SPEC>;
#[doc = "Field `LENGTH_0` reader - The length of the data in buffer 0."]
pub type LENGTH_0_R = crate::FieldReader<u16>;
#[doc = "Field `LENGTH_0` writer - The length of the data in buffer 0."]
pub type LENGTH_0_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AVAILABLE_0` reader - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub type AVAILABLE_0_R = crate::BitReader;
#[doc = "Field `AVAILABLE_0` writer - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub type AVAILABLE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Reply with a stall (valid for both buffers)."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - Reply with a stall (valid for both buffers)."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Reset the buffer selector to buffer 0."]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Reset the buffer selector to buffer 0."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID_0` reader - The data pid of buffer 0."]
pub type PID_0_R = crate::BitReader;
#[doc = "Field `PID_0` writer - The data pid of buffer 0."]
pub type PID_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_0` reader - Buffer 0 is the last buffer of the transfer."]
pub type LAST_0_R = crate::BitReader;
#[doc = "Field `LAST_0` writer - Buffer 0 is the last buffer of the transfer."]
pub type LAST_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL_0` reader - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub type FULL_0_R = crate::BitReader;
#[doc = "Field `FULL_0` writer - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub type FULL_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LENGTH_1` reader - The length of the data in buffer 1."]
pub type LENGTH_1_R = crate::FieldReader<u16>;
#[doc = "Field `LENGTH_1` writer - The length of the data in buffer 1."]
pub type LENGTH_1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AVAILABLE_1` reader - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub type AVAILABLE_1_R = crate::BitReader;
#[doc = "Field `AVAILABLE_1` writer - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
pub type AVAILABLE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint. For a non Isochronous endpoint the offset is always 64 bytes.  

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
impl crate::FieldSpec for DOUBLE_BUFFER_ISO_OFFSET_A {
    type Ux = u8;
}
#[doc = "Field `DOUBLE_BUFFER_ISO_OFFSET` reader - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint. For a non Isochronous endpoint the offset is always 64 bytes."]
pub type DOUBLE_BUFFER_ISO_OFFSET_R = crate::FieldReader<DOUBLE_BUFFER_ISO_OFFSET_A>;
impl DOUBLE_BUFFER_ISO_OFFSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DOUBLE_BUFFER_ISO_OFFSET_A {
        match self.bits {
            0 => DOUBLE_BUFFER_ISO_OFFSET_A::_128,
            1 => DOUBLE_BUFFER_ISO_OFFSET_A::_256,
            2 => DOUBLE_BUFFER_ISO_OFFSET_A::_512,
            3 => DOUBLE_BUFFER_ISO_OFFSET_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == DOUBLE_BUFFER_ISO_OFFSET_A::_128
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == DOUBLE_BUFFER_ISO_OFFSET_A::_256
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == DOUBLE_BUFFER_ISO_OFFSET_A::_512
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == DOUBLE_BUFFER_ISO_OFFSET_A::_1024
    }
}
#[doc = "Field `DOUBLE_BUFFER_ISO_OFFSET` writer - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint. For a non Isochronous endpoint the offset is always 64 bytes."]
pub type DOUBLE_BUFFER_ISO_OFFSET_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, DOUBLE_BUFFER_ISO_OFFSET_A>;
impl<'a, REG> DOUBLE_BUFFER_ISO_OFFSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(DOUBLE_BUFFER_ISO_OFFSET_A::_128)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(DOUBLE_BUFFER_ISO_OFFSET_A::_256)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(DOUBLE_BUFFER_ISO_OFFSET_A::_512)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(DOUBLE_BUFFER_ISO_OFFSET_A::_1024)
    }
}
#[doc = "Field `PID_1` reader - The data pid of buffer 1."]
pub type PID_1_R = crate::BitReader;
#[doc = "Field `PID_1` writer - The data pid of buffer 1."]
pub type PID_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_1` reader - Buffer 1 is the last buffer of the transfer."]
pub type LAST_1_R = crate::BitReader;
#[doc = "Field `LAST_1` writer - Buffer 1 is the last buffer of the transfer."]
pub type LAST_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL_1` reader - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub type FULL_1_R = crate::BitReader;
#[doc = "Field `FULL_1` writer - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
pub type FULL_1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 27:28 - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint. For a non Isochronous endpoint the offset is always 64 bytes."]
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
    pub fn length_0(&mut self) -> LENGTH_0_W<EP_BUFFER_CONTROL_SPEC> {
        LENGTH_0_W::new(self, 0)
    }
    #[doc = "Bit 10 - Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    #[must_use]
    pub fn available_0(&mut self) -> AVAILABLE_0_W<EP_BUFFER_CONTROL_SPEC> {
        AVAILABLE_0_W::new(self, 10)
    }
    #[doc = "Bit 11 - Reply with a stall (valid for both buffers)."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<EP_BUFFER_CONTROL_SPEC> {
        STALL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Reset the buffer selector to buffer 0."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<EP_BUFFER_CONTROL_SPEC> {
        RESET_W::new(self, 12)
    }
    #[doc = "Bit 13 - The data pid of buffer 0."]
    #[inline(always)]
    #[must_use]
    pub fn pid_0(&mut self) -> PID_0_W<EP_BUFFER_CONTROL_SPEC> {
        PID_0_W::new(self, 13)
    }
    #[doc = "Bit 14 - Buffer 0 is the last buffer of the transfer."]
    #[inline(always)]
    #[must_use]
    pub fn last_0(&mut self) -> LAST_0_W<EP_BUFFER_CONTROL_SPEC> {
        LAST_0_W::new(self, 14)
    }
    #[doc = "Bit 15 - Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    #[must_use]
    pub fn full_0(&mut self) -> FULL_0_W<EP_BUFFER_CONTROL_SPEC> {
        FULL_0_W::new(self, 15)
    }
    #[doc = "Bits 16:25 - The length of the data in buffer 1."]
    #[inline(always)]
    #[must_use]
    pub fn length_1(&mut self) -> LENGTH_1_W<EP_BUFFER_CONTROL_SPEC> {
        LENGTH_1_W::new(self, 16)
    }
    #[doc = "Bit 26 - Buffer 1 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    #[must_use]
    pub fn available_1(&mut self) -> AVAILABLE_1_W<EP_BUFFER_CONTROL_SPEC> {
        AVAILABLE_1_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint. For a non Isochronous endpoint the offset is always 64 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn double_buffer_iso_offset(
        &mut self,
    ) -> DOUBLE_BUFFER_ISO_OFFSET_W<EP_BUFFER_CONTROL_SPEC> {
        DOUBLE_BUFFER_ISO_OFFSET_W::new(self, 27)
    }
    #[doc = "Bit 29 - The data pid of buffer 1."]
    #[inline(always)]
    #[must_use]
    pub fn pid_1(&mut self) -> PID_1_W<EP_BUFFER_CONTROL_SPEC> {
        PID_1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Buffer 1 is the last buffer of the transfer."]
    #[inline(always)]
    #[must_use]
    pub fn last_1(&mut self) -> LAST_1_W<EP_BUFFER_CONTROL_SPEC> {
        LAST_1_W::new(self, 30)
    }
    #[doc = "Bit 31 - Buffer 1 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    #[must_use]
    pub fn full_1(&mut self) -> FULL_1_W<EP_BUFFER_CONTROL_SPEC> {
        FULL_1_W::new(self, 31)
    }
}
#[doc = "-  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_buffer_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_buffer_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP_BUFFER_CONTROL_SPEC;
impl crate::RegisterSpec for EP_BUFFER_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_buffer_control::R`](R) reader structure"]
impl crate::Readable for EP_BUFFER_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep_buffer_control::W`](W) writer structure"]
impl crate::Writable for EP_BUFFER_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP_BUFFER_CONTROL%s to value 0"]
impl crate::Resettable for EP_BUFFER_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
