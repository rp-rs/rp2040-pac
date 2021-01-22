#[doc = "Reader of register RXUICR"]
pub type R = crate::R<u32, super::RXUICR>;
#[doc = "Reader of field `RXUICR`"]
pub type RXUICR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clear-on-read receive FIFO underflow interrupt"]
    #[inline(always)]
    pub fn rxuicr(&self) -> RXUICR_R {
        RXUICR_R::new((self.bits & 0x01) != 0)
    }
}
