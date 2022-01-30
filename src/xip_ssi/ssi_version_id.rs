#[doc = "Register `SSI_VERSION_ID` reader"]
pub struct R(crate::R<SSI_VERSION_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSI_VERSION_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSI_VERSION_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSI_VERSION_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSI_COMP_VERSION` reader - SNPS component version (format X.YY)"]
pub struct SSI_COMP_VERSION_R(crate::FieldReader<u32, u32>);
impl SSI_COMP_VERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SSI_COMP_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSI_COMP_VERSION_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - SNPS component version (format X.YY)"]
    #[inline(always)]
    pub fn ssi_comp_version(&self) -> SSI_COMP_VERSION_R {
        SSI_COMP_VERSION_R::new(self.bits)
    }
}
#[doc = "Version ID  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ssi_version_id](index.html) module"]
pub struct SSI_VERSION_ID_SPEC;
impl crate::RegisterSpec for SSI_VERSION_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssi_version_id::R](R) reader structure"]
impl crate::Readable for SSI_VERSION_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSI_VERSION_ID to value 0x3430_312a"]
impl crate::Resettable for SSI_VERSION_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3430_312a
    }
}
