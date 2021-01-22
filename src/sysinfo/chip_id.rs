#[doc = "Reader of register CHIP_ID"]
pub type R = crate::R<u32, super::CHIP_ID>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `PART`"]
pub type PART_R = crate::R<u16, u16>;
#[doc = "Reader of field `MANUFACTURER`"]
pub type MANUFACTURER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27"]
    #[inline(always)]
    pub fn part(&self) -> PART_R {
        PART_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn manufacturer(&self) -> MANUFACTURER_R {
        MANUFACTURER_R::new((self.bits & 0x0fff) as u16)
    }
}
