#[doc = "Reader of register SSPPCELLID0"]
pub type R = crate::R<u32, super::SSPPCELLID0>;
#[doc = "Reader of field `SSPPCELLID0`"]
pub type SSPPCELLID0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x0D"]
    #[inline(always)]
    pub fn ssppcellid0(&self) -> SSPPCELLID0_R {
        SSPPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
