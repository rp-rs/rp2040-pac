#[doc = "Reader of register CLK_SYS_RESUS_STATUS"]
pub type R = crate::R<u32, super::CLK_SYS_RESUS_STATUS>;
#[doc = "Reader of field `RESUSSED`"]
pub type RESUSSED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clock has been resuscitated, correct the error then send ctrl_clear=1"]
    #[inline(always)]
    pub fn resussed(&self) -> RESUSSED_R {
        RESUSSED_R::new((self.bits & 0x01) != 0)
    }
}
