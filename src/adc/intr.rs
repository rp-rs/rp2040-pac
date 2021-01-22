#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Reader of field `FIFO`"]
pub type FIFO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Triggered when the sample FIFO reaches a certain level.\\n This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new((self.bits & 0x01) != 0)
    }
}
