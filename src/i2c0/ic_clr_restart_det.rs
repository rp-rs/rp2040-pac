#[doc = "Reader of register IC_CLR_RESTART_DET"]
pub type R = crate::R<u32, super::IC_CLR_RESTART_DET>;
#[doc = "Reader of field `CLR_RESTART_DET`"]
pub type CLR_RESTART_DET_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_restart_det(&self) -> CLR_RESTART_DET_R {
        CLR_RESTART_DET_R::new((self.bits & 0x01) != 0)
    }
}
