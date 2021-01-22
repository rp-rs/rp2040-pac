#[doc = "Reader of register SSPPERIPHID2"]
pub type R = crate::R<u32, super::SSPPERIPHID2>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `DESIGNER1`"]
pub type DESIGNER1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 4:7 - These bits return the peripheral revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - These bits read back as 0x4"]
    #[inline(always)]
    pub fn designer1(&self) -> DESIGNER1_R {
        DESIGNER1_R::new((self.bits & 0x0f) as u8)
    }
}
