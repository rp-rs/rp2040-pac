#[doc = "Reader of register IC_CLR_RX_UNDER"]
pub type R = crate::R<u32, super::IC_CLR_RX_UNDER>;
#[doc = "Reader of field `CLR_RX_UNDER`"]
pub type CLR_RX_UNDER_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rx_under(&self) -> CLR_RX_UNDER_R {
        CLR_RX_UNDER_R::new((self.bits & 0x01) != 0)
    }
}
