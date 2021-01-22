#[doc = "Reader of register SSPPCELLID3"]
pub type R = crate::R<u32, super::SSPPCELLID3>;
#[doc = "Reader of field `SSPPCELLID3`"]
pub type SSPPCELLID3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn ssppcellid3(&self) -> SSPPCELLID3_R {
        SSPPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
