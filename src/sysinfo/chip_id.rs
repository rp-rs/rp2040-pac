#[doc = "Register `CHIP_ID` reader"]
pub struct R(crate::R<CHIP_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIP_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIP_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIP_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MANUFACTURER` reader - "]
pub type MANUFACTURER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PART` reader - "]
pub type PART_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REVISION` reader - "]
pub type REVISION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn manufacturer(&self) -> MANUFACTURER_R {
        MANUFACTURER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27"]
    #[inline(always)]
    pub fn part(&self) -> PART_R {
        PART_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "JEDEC JEP-106 compliant chip identifier.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [chip_id](index.html) module"]
pub struct CHIP_ID_SPEC;
impl crate::RegisterSpec for CHIP_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chip_id::R](R) reader structure"]
impl crate::Readable for CHIP_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHIP_ID to value 0"]
impl crate::Resettable for CHIP_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
