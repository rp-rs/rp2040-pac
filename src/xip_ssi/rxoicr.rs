#[doc = "Reader of register RXOICR"]
pub type R = crate::R<u32, super::RXOICR>;
#[doc = "Reader of field `RXOICR`"]
pub type RXOICR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clear-on-read receive FIFO overflow interrupt"]
    #[inline(always)]
    pub fn rxoicr(&self) -> RXOICR_R {
        RXOICR_R::new((self.bits & 0x01) != 0)
    }
}
