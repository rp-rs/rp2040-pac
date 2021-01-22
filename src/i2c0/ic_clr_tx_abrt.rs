#[doc = "Reader of register IC_CLR_TX_ABRT"]
pub type R = crate::R<u32, super::IC_CLR_TX_ABRT>;
#[doc = "Reader of field `CLR_TX_ABRT`"]
pub type CLR_TX_ABRT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_tx_abrt(&self) -> CLR_TX_ABRT_R {
        CLR_TX_ABRT_R::new((self.bits & 0x01) != 0)
    }
}
