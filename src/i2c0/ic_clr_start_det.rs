#[doc = "Reader of register IC_CLR_START_DET"]
pub type R = crate::R<u32, super::IC_CLR_START_DET>;
#[doc = "Reader of field `CLR_START_DET`"]
pub type CLR_START_DET_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_start_det(&self) -> CLR_START_DET_R {
        CLR_START_DET_R::new((self.bits & 0x01) != 0)
    }
}
