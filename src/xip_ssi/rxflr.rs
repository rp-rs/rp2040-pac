#[doc = "Reader of register RXFLR"]
pub type R = crate::R<u32, super::RXFLR>;
#[doc = "Reader of field `RXTFL`"]
pub type RXTFL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO level"]
    #[inline(always)]
    pub fn rxtfl(&self) -> RXTFL_R {
        RXTFL_R::new((self.bits & 0xff) as u8)
    }
}
