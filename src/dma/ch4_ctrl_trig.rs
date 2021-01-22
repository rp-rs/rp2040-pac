#[doc = "Reader of register CH4_CTRL_TRIG"]
pub type R = crate::R<u32, super::CH4_CTRL_TRIG>;
#[doc = "Writer for register CH4_CTRL_TRIG"]
pub type W = crate::W<u32, super::CH4_CTRL_TRIG>;
#[doc = "Register CH4_CTRL_TRIG `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::CH4_CTRL_TRIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Reader of field `AHB_ERROR`"]
pub type AHB_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `READ_ERROR`"]
pub type READ_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_ERROR`"]
pub struct READ_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_ERROR_W<'a> {
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
#[doc = "Reader of field `WRITE_ERROR`"]
pub type WRITE_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE_ERROR`"]
pub struct WRITE_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ERROR_W<'a> {
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SNIFF_EN`"]
pub type SNIFF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SNIFF_EN`"]
pub struct SNIFF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SNIFF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `BSWAP`"]
pub type BSWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSWAP`"]
pub struct BSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BSWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `IRQ_QUIET`"]
pub type IRQ_QUIET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_QUIET`"]
pub struct IRQ_QUIET_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_QUIET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Select a Transfer Request signal.\\n The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system).\\n 0x0 to 0x3a -> select DREQ n as TREQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TREQ_SEL_A {
    #[doc = "59: Select Timer 0 as TREQ"]
    TIMER0 = 59,
    #[doc = "60: Select Timer 1 as TREQ"]
    TIMER1 = 60,
    #[doc = "61: Select Timer 2 as TREQ (Optional)"]
    TIMER2 = 61,
    #[doc = "62: Select Timer 3 as TREQ (Optional)"]
    TIMER3 = 62,
    #[doc = "63: Permanent request, for unpaced transfers."]
    PERMANENT = 63,
}
impl From<TREQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TREQ_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TREQ_SEL`"]
pub type TREQ_SEL_R = crate::R<u8, TREQ_SEL_A>;
impl TREQ_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TREQ_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            59 => Val(TREQ_SEL_A::TIMER0),
            60 => Val(TREQ_SEL_A::TIMER1),
            61 => Val(TREQ_SEL_A::TIMER2),
            62 => Val(TREQ_SEL_A::TIMER3),
            63 => Val(TREQ_SEL_A::PERMANENT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == TREQ_SEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == TREQ_SEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == TREQ_SEL_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == TREQ_SEL_A::TIMER3
    }
    #[doc = "Checks if the value of the field is `PERMANENT`"]
    #[inline(always)]
    pub fn is_permanent(&self) -> bool {
        *self == TREQ_SEL_A::PERMANENT
    }
}
#[doc = "Write proxy for field `TREQ_SEL`"]
pub struct TREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TREQ_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TREQ_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select Timer 0 as TREQ"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(TREQ_SEL_A::TIMER0)
    }
    #[doc = "Select Timer 1 as TREQ"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(TREQ_SEL_A::TIMER1)
    }
    #[doc = "Select Timer 2 as TREQ (Optional)"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut W {
        self.variant(TREQ_SEL_A::TIMER2)
    }
    #[doc = "Select Timer 3 as TREQ (Optional)"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut W {
        self.variant(TREQ_SEL_A::TIMER3)
    }
    #[doc = "Permanent request, for unpaced transfers."]
    #[inline(always)]
    pub fn permanent(self) -> &'a mut W {
        self.variant(TREQ_SEL_A::PERMANENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 15)) | (((value as u32) & 0x3f) << 15);
        self.w
    }
}
#[doc = "Reader of field `CHAIN_TO`"]
pub type CHAIN_TO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHAIN_TO`"]
pub struct CHAIN_TO_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIN_TO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `RING_SEL`"]
pub type RING_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RING_SEL`"]
pub struct RING_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RING_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers.\\n\\n Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RING_SIZE_A {
    #[doc = "0: `0`"]
    RING_NONE = 0,
}
impl From<RING_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RING_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RING_SIZE`"]
pub type RING_SIZE_R = crate::R<u8, RING_SIZE_A>;
impl RING_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RING_SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RING_SIZE_A::RING_NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RING_NONE`"]
    #[inline(always)]
    pub fn is_ring_none(&self) -> bool {
        *self == RING_SIZE_A::RING_NONE
    }
}
#[doc = "Write proxy for field `RING_SIZE`"]
pub struct RING_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RING_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RING_SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ring_none(self) -> &'a mut W {
        self.variant(RING_SIZE_A::RING_NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `INCR_WRITE`"]
pub type INCR_WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCR_WRITE`"]
pub struct INCR_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> INCR_WRITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `INCR_READ`"]
pub type INCR_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCR_READ`"]
pub struct INCR_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> INCR_READ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA_SIZE_A {
    #[doc = "0: `0`"]
    SIZE_BYTE = 0,
    #[doc = "1: `1`"]
    SIZE_HALFWORD = 1,
    #[doc = "2: `10`"]
    SIZE_WORD = 2,
}
impl From<DATA_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATA_SIZE`"]
pub type DATA_SIZE_R = crate::R<u8, DATA_SIZE_A>;
impl DATA_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATA_SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATA_SIZE_A::SIZE_BYTE),
            1 => Val(DATA_SIZE_A::SIZE_HALFWORD),
            2 => Val(DATA_SIZE_A::SIZE_WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_BYTE`"]
    #[inline(always)]
    pub fn is_size_byte(&self) -> bool {
        *self == DATA_SIZE_A::SIZE_BYTE
    }
    #[doc = "Checks if the value of the field is `SIZE_HALFWORD`"]
    #[inline(always)]
    pub fn is_size_halfword(&self) -> bool {
        *self == DATA_SIZE_A::SIZE_HALFWORD
    }
    #[doc = "Checks if the value of the field is `SIZE_WORD`"]
    #[inline(always)]
    pub fn is_size_word(&self) -> bool {
        *self == DATA_SIZE_A::SIZE_WORD
    }
}
#[doc = "Write proxy for field `DATA_SIZE`"]
pub struct DATA_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn size_byte(self) -> &'a mut W {
        self.variant(DATA_SIZE_A::SIZE_BYTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn size_halfword(self) -> &'a mut W {
        self.variant(DATA_SIZE_A::SIZE_HALFWORD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn size_word(self) -> &'a mut W {
        self.variant(DATA_SIZE_A::SIZE_WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `HIGH_PRIORITY`"]
pub type HIGH_PRIORITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIGH_PRIORITY`"]
pub struct HIGH_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_PRIORITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Logical OR of the READ_ERROR and WRITE_ERROR flags. The channel halts when it encounters any bus error, and always raises its channel IRQ flag."]
    #[inline(always)]
    pub fn ahb_error(&self) -> AHB_ERROR_R {
        AHB_ERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - If 1, the channel received a read bus error. Write one to clear.\\n READ_ADDR shows the approximate address where the bus error was encountered (will not to be earlier, or more than 3 transfers later)"]
    #[inline(always)]
    pub fn read_error(&self) -> READ_ERROR_R {
        READ_ERROR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - If 1, the channel received a write bus error. Write one to clear.\\n WRITE_ADDR shows the approximate address where the bus error was encountered (will not to be earlier, or more than 5 transfers later)"]
    #[inline(always)]
    pub fn write_error(&self) -> WRITE_ERROR_R {
        WRITE_ERROR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused.\\n\\n To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected.\\n\\n This allows checksum to be enabled or disabled on a per-control- block basis."]
    #[inline(always)]
    pub fn sniff_en(&self) -> SNIFF_EN_R {
        SNIFF_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Apply byte-swap transformation to DMA data.\\n For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    #[inline(always)]
    pub fn bswap(&self) -> BSWAP_R {
        BSWAP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain.\\n\\n This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    #[inline(always)]
    pub fn irq_quiet(&self) -> IRQ_QUIET_R {
        IRQ_QUIET_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 15:20 - Select a Transfer Request signal.\\n The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system).\\n 0x0 to 0x3a -> select DREQ n as TREQ"]
    #[inline(always)]
    pub fn treq_sel(&self) -> TREQ_SEL_R {
        TREQ_SEL_R::new(((self.bits >> 15) & 0x3f) as u8)
    }
    #[doc = "Bits 11:14 - When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_.\\n Reset value is equal to channel number (4)."]
    #[inline(always)]
    pub fn chain_to(&self) -> CHAIN_TO_R {
        CHAIN_TO_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Select whether RING_SIZE applies to read or write addresses.\\n If 0, read addresses are wrapped on a (1 << RING_SIZE) boundary. If 1, write addresses are wrapped."]
    #[inline(always)]
    pub fn ring_sel(&self) -> RING_SEL_R {
        RING_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers.\\n\\n Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    #[inline(always)]
    pub fn ring_size(&self) -> RING_SIZE_R {
        RING_SIZE_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address.\\n\\n Generally this should be disabled for memory-to-peripheral transfers."]
    #[inline(always)]
    pub fn incr_write(&self) -> INCR_WRITE_R {
        INCR_WRITE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address.\\n\\n Generally this should be disabled for peripheral-to-memory transfers."]
    #[inline(always)]
    pub fn incr_read(&self) -> INCR_READ_R {
        INCR_READ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels.\\n\\n This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
    #[inline(always)]
    pub fn high_priority(&self) -> HIGH_PRIORITY_R {
        HIGH_PRIORITY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA Channel Enable.\\n When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - If 1, the channel received a read bus error. Write one to clear.\\n READ_ADDR shows the approximate address where the bus error was encountered (will not to be earlier, or more than 3 transfers later)"]
    #[inline(always)]
    pub fn read_error(&mut self) -> READ_ERROR_W {
        READ_ERROR_W { w: self }
    }
    #[doc = "Bit 29 - If 1, the channel received a write bus error. Write one to clear.\\n WRITE_ADDR shows the approximate address where the bus error was encountered (will not to be earlier, or more than 5 transfers later)"]
    #[inline(always)]
    pub fn write_error(&mut self) -> WRITE_ERROR_W {
        WRITE_ERROR_W { w: self }
    }
    #[doc = "Bit 23 - If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected.\\n\\n This allows checksum to be enabled or disabled on a per-control- block basis."]
    #[inline(always)]
    pub fn sniff_en(&mut self) -> SNIFF_EN_W {
        SNIFF_EN_W { w: self }
    }
    #[doc = "Bit 22 - Apply byte-swap transformation to DMA data.\\n For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    #[inline(always)]
    pub fn bswap(&mut self) -> BSWAP_W {
        BSWAP_W { w: self }
    }
    #[doc = "Bit 21 - In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain.\\n\\n This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    #[inline(always)]
    pub fn irq_quiet(&mut self) -> IRQ_QUIET_W {
        IRQ_QUIET_W { w: self }
    }
    #[doc = "Bits 15:20 - Select a Transfer Request signal.\\n The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system).\\n 0x0 to 0x3a -> select DREQ n as TREQ"]
    #[inline(always)]
    pub fn treq_sel(&mut self) -> TREQ_SEL_W {
        TREQ_SEL_W { w: self }
    }
    #[doc = "Bits 11:14 - When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_.\\n Reset value is equal to channel number (4)."]
    #[inline(always)]
    pub fn chain_to(&mut self) -> CHAIN_TO_W {
        CHAIN_TO_W { w: self }
    }
    #[doc = "Bit 10 - Select whether RING_SIZE applies to read or write addresses.\\n If 0, read addresses are wrapped on a (1 << RING_SIZE) boundary. If 1, write addresses are wrapped."]
    #[inline(always)]
    pub fn ring_sel(&mut self) -> RING_SEL_W {
        RING_SEL_W { w: self }
    }
    #[doc = "Bits 6:9 - Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers.\\n\\n Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    #[inline(always)]
    pub fn ring_size(&mut self) -> RING_SIZE_W {
        RING_SIZE_W { w: self }
    }
    #[doc = "Bit 5 - If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address.\\n\\n Generally this should be disabled for memory-to-peripheral transfers."]
    #[inline(always)]
    pub fn incr_write(&mut self) -> INCR_WRITE_W {
        INCR_WRITE_W { w: self }
    }
    #[doc = "Bit 4 - If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address.\\n\\n Generally this should be disabled for peripheral-to-memory transfers."]
    #[inline(always)]
    pub fn incr_read(&mut self) -> INCR_READ_W {
        INCR_READ_W { w: self }
    }
    #[doc = "Bits 2:3 - Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    #[inline(always)]
    pub fn data_size(&mut self) -> DATA_SIZE_W {
        DATA_SIZE_W { w: self }
    }
    #[doc = "Bit 1 - HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels.\\n\\n This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
    #[inline(always)]
    pub fn high_priority(&mut self) -> HIGH_PRIORITY_W {
        HIGH_PRIORITY_W { w: self }
    }
    #[doc = "Bit 0 - DMA Channel Enable.\\n When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
