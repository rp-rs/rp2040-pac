#[doc = "Reader of register IC_TXFLR"]
pub type R = crate::R<u32, super::IC_TXFLR>;
#[doc = "Reader of field `TXFLR`"]
pub type TXFLR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn txflr(&self) -> TXFLR_R {
        TXFLR_R::new((self.bits & 0x1f) as u8)
    }
}
