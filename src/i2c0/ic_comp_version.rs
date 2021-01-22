#[doc = "Reader of register IC_COMP_VERSION"]
pub type R = crate::R<u32, super::IC_COMP_VERSION>;
#[doc = "Reader of field `IC_COMP_VERSION`"]
pub type IC_COMP_VERSION_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic_comp_version(&self) -> IC_COMP_VERSION_R {
        IC_COMP_VERSION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
