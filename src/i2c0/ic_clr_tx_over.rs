#[doc = "Reader of register IC_CLR_TX_OVER"]
pub type R = crate::R<u32, super::IC_CLR_TX_OVER>;
#[doc = "Reader of field `CLR_TX_OVER`"]
pub type CLR_TX_OVER_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_tx_over(&self) -> CLR_TX_OVER_R {
        CLR_TX_OVER_R::new((self.bits & 0x01) != 0)
    }
}
