#[doc = "Reader of register INTS"]
pub type R = crate::R<u32, super::INTS>;
#[doc = "Reader of field `CLK_SYS_RESUS`"]
pub type CLK_SYS_RESUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_resus(&self) -> CLK_SYS_RESUS_R {
        CLK_SYS_RESUS_R::new((self.bits & 0x01) != 0)
    }
}
