#[doc = "Register `CPUID` reader"]
pub struct R(crate::R<CPUID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REVISION` reader - Minor revision number m in the rnpm revision status:  
 0x1 = Patch 1."]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PARTNO` reader - Number of processor within family: 0xC60 = Cortex-M0+"]
pub type PARTNO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARCHITECTURE` reader - Constant that defines the architecture of the processor:  
 0xC = ARMv6-M architecture."]
pub type ARCHITECTURE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VARIANT` reader - Major revision number n in the rnpm revision status:  
 0x0 = Revision 0."]
pub type VARIANT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMPLEMENTER` reader - Implementor code: 0x41 = ARM"]
pub type IMPLEMENTER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Minor revision number m in the rnpm revision status:  
 0x1 = Patch 1."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Number of processor within family: 0xC60 = Cortex-M0+"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Constant that defines the architecture of the processor:  
 0xC = ARMv6-M architecture."]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Major revision number n in the rnpm revision status:  
 0x0 = Revision 0."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementor code: 0x41 = ARM"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [cpuid](index.html) module"]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuid::R](R) reader structure"]
impl crate::Readable for CPUID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPUID to value 0x410c_c601"]
impl crate::Resettable for CPUID_SPEC {
    const RESET_VALUE: Self::Ux = 0x410c_c601;
}
