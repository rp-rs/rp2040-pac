#[doc = "Reader of register IC_CLR_INTR"]
pub type R = crate::R<u32, super::IC_CLR_INTR>;
#[doc = "Reader of field `CLR_INTR`"]
pub type CLR_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_intr(&self) -> CLR_INTR_R {
        CLR_INTR_R::new((self.bits & 0x01) != 0)
    }
}
