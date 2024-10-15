#[doc = "Register `CHIP_ID` reader"]
pub type R = crate::R<CHIP_ID_SPEC>;
#[doc = "Field `MANUFACTURER` reader - "]
pub type MANUFACTURER_R = crate::FieldReader<u16>;
#[doc = "Field `PART` reader - "]
pub type PART_R = crate::FieldReader<u16>;
#[doc = "Field `REVISION` reader - "]
pub type REVISION_R = crate::FieldReader;
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

You can [`read`](crate::Reg::read) this register and get [`chip_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIP_ID_SPEC;
impl crate::RegisterSpec for CHIP_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chip_id::R`](R) reader structure"]
impl crate::Readable for CHIP_ID_SPEC {}
#[doc = "`reset()` method sets CHIP_ID to value 0"]
impl crate::Resettable for CHIP_ID_SPEC {
    const RESET_VALUE: u32 = 0;
}
