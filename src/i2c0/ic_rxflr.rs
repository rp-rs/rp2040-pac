#[doc = "Reader of register IC_RXFLR"]
pub type R = crate::R<u32, super::IC_RXFLR>;
#[doc = "Reader of field `RXFLR`"]
pub type RXFLR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn rxflr(&self) -> RXFLR_R {
        RXFLR_R::new((self.bits & 0x1f) as u8)
    }
}
