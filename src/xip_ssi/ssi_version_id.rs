#[doc = "Register `SSI_VERSION_ID` reader"]
pub type R = crate::R<SSI_VERSION_ID_SPEC>;
#[doc = "Register `SSI_VERSION_ID` writer"]
pub type W = crate::W<SSI_VERSION_ID_SPEC>;
#[doc = "Field `SSI_COMP_VERSION` reader - SNPS component version (format X.YY)"]
pub type SSI_COMP_VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SNPS component version (format X.YY)"]
    #[inline(always)]
    pub fn ssi_comp_version(&self) -> SSI_COMP_VERSION_R {
        SSI_COMP_VERSION_R::new(self.bits)
    }
}
impl W {}
#[doc = "Version ID  

You can [`read`](crate::generic::Reg::read) this register and get [`ssi_version_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssi_version_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSI_VERSION_ID_SPEC;
impl crate::RegisterSpec for SSI_VERSION_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssi_version_id::R`](R) reader structure"]
impl crate::Readable for SSI_VERSION_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssi_version_id::W`](W) writer structure"]
impl crate::Writable for SSI_VERSION_ID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSI_VERSION_ID to value 0x3430_312a"]
impl crate::Resettable for SSI_VERSION_ID_SPEC {
    const RESET_VALUE: u32 = 0x3430_312a;
}
