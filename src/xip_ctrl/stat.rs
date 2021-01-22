#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `FIFO_FULL`"]
pub type FIFO_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFO_EMPTY`"]
pub type FIFO_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLUSH_READY`"]
pub type FLUSH_READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - When 1, indicates the XIP streaming FIFO is completely full.\\n The streaming FIFO is 2 entries deep, so the full and empty\\n flag allow its level to be ascertained."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, indicates the XIP streaming FIFO is completely empty."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Reads as 0 while a cache flush is in progress, and 1 otherwise.\\n The cache is flushed whenever the XIP block is reset, and also\\n when requested via the FLUSH register."]
    #[inline(always)]
    pub fn flush_ready(&self) -> FLUSH_READY_R {
        FLUSH_READY_R::new((self.bits & 0x01) != 0)
    }
}
