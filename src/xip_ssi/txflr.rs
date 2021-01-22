#[doc = "Reader of register TXFLR"]
pub type R = crate::R<u32, super::TXFLR>;
#[doc = "Reader of field `TFTFL`"]
pub type TFTFL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO level"]
    #[inline(always)]
    pub fn tftfl(&self) -> TFTFL_R {
        TFTFL_R::new((self.bits & 0xff) as u8)
    }
}
