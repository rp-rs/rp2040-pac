#[doc = "Reader of register TXOICR"]
pub type R = crate::R<u32, super::TXOICR>;
#[doc = "Reader of field `TXOICR`"]
pub type TXOICR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clear-on-read transmit FIFO overflow interrupt"]
    #[inline(always)]
    pub fn txoicr(&self) -> TXOICR_R {
        TXOICR_R::new((self.bits & 0x01) != 0)
    }
}
