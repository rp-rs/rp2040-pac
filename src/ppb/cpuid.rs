#[doc = "Reader of register CPUID"]
pub type R = crate::R<u32, super::CPUID>;
#[doc = "Reader of field `IMPLEMENTER`"]
pub type IMPLEMENTER_R = crate::R<u8, u8>;
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ARCHITECTURE`"]
pub type ARCHITECTURE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PARTNO`"]
pub type PARTNO_R = crate::R<u16, u16>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - Implementor code: 0x41 = ARM"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - Major revision number n in the rnpm revision status:\\n 0x0 = Revision 0."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Constant that defines the architecture of the processor:\\n 0xC = ARMv6-M architecture."]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Number of processor within family: 0xC60 = Cortex-M0+"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - Minor revision number m in the rnpm revision status:\\n 0x1 = Patch 1."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
}
