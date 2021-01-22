#[doc = "Reader of register IC_COMP_TYPE"]
pub type R = crate::R<u32, super::IC_COMP_TYPE>;
#[doc = "Reader of field `IC_COMP_TYPE`"]
pub type IC_COMP_TYPE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number."]
    #[inline(always)]
    pub fn ic_comp_type(&self) -> IC_COMP_TYPE_R {
        IC_COMP_TYPE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
