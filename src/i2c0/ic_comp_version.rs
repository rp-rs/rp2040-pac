#[doc = "Register `IC_COMP_VERSION` reader"]
pub struct R(crate::R<IC_COMP_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_COMP_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_COMP_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_COMP_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IC_COMP_VERSION` reader - "]
pub type IC_COMP_VERSION_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic_comp_version(&self) -> IC_COMP_VERSION_R {
        IC_COMP_VERSION_R::new(self.bits)
    }
}
#[doc = "I2C Component Version Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_comp_version](index.html) module"]
pub struct IC_COMP_VERSION_SPEC;
impl crate::RegisterSpec for IC_COMP_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_comp_version::R](R) reader structure"]
impl crate::Readable for IC_COMP_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_COMP_VERSION to value 0x3230_312a"]
impl crate::Resettable for IC_COMP_VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x3230_312a;
}
