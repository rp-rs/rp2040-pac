#[doc = "Reader of register SSPPERIPHID0"]
pub type R = crate::R<u32, super::SSPPERIPHID0>;
#[doc = "Reader of field `PARTNUMBER0`"]
pub type PARTNUMBER0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x22"]
    #[inline(always)]
    pub fn partnumber0(&self) -> PARTNUMBER0_R {
        PARTNUMBER0_R::new((self.bits & 0xff) as u8)
    }
}
