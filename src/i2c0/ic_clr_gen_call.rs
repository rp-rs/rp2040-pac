#[doc = "Reader of register IC_CLR_GEN_CALL"]
pub type R = crate::R<u32, super::IC_CLR_GEN_CALL>;
#[doc = "Reader of field `CLR_GEN_CALL`"]
pub type CLR_GEN_CALL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_gen_call(&self) -> CLR_GEN_CALL_R {
        CLR_GEN_CALL_R::new((self.bits & 0x01) != 0)
    }
}
