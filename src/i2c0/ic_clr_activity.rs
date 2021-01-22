#[doc = "Reader of register IC_CLR_ACTIVITY"]
pub type R = crate::R<u32, super::IC_CLR_ACTIVITY>;
#[doc = "Reader of field `CLR_ACTIVITY`"]
pub type CLR_ACTIVITY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_activity(&self) -> CLR_ACTIVITY_R {
        CLR_ACTIVITY_R::new((self.bits & 0x01) != 0)
    }
}
