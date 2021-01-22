#[doc = "Reader of register IC_CLR_RD_REQ"]
pub type R = crate::R<u32, super::IC_CLR_RD_REQ>;
#[doc = "Reader of field `CLR_RD_REQ`"]
pub type CLR_RD_REQ_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_rd_req(&self) -> CLR_RD_REQ_R {
        CLR_RD_REQ_R::new((self.bits & 0x01) != 0)
    }
}
