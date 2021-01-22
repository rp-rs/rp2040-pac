#[doc = "Reader of register MPU_TYPE"]
pub type R = crate::R<u32, super::MPU_TYPE>;
#[doc = "Reader of field `IREGION`"]
pub type IREGION_R = crate::R<u8, u8>;
#[doc = "Reader of field `DREGION`"]
pub type DREGION_R = crate::R<u8, u8>;
#[doc = "Reader of field `SEPARATE`"]
pub type SEPARATE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 16:23 - Instruction region. Reads as zero as ARMv6-M only supports a unified MPU."]
    #[inline(always)]
    pub fn iregion(&self) -> IREGION_R {
        IREGION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of regions supported by the MPU."]
    #[inline(always)]
    pub fn dregion(&self) -> DREGION_R {
        DREGION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 0 - Indicates support for separate instruction and data address maps. Reads as 0 as ARMv6-M only supports a unified MPU."]
    #[inline(always)]
    pub fn separate(&self) -> SEPARATE_R {
        SEPARATE_R::new((self.bits & 0x01) != 0)
    }
}
