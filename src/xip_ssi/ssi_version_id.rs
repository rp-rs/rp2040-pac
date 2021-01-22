#[doc = "Reader of register SSI_VERSION_ID"]
pub type R = crate::R<u32, super::SSI_VERSION_ID>;
#[doc = "Reader of field `SSI_COMP_VERSION`"]
pub type SSI_COMP_VERSION_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SNPS component version (format X.YY)"]
    #[inline(always)]
    pub fn ssi_comp_version(&self) -> SSI_COMP_VERSION_R {
        SSI_COMP_VERSION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
