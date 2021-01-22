#[doc = "Reader of register SSPPCELLID1"]
pub type R = crate::R<u32, super::SSPPCELLID1>;
#[doc = "Reader of field `SSPPCELLID1`"]
pub type SSPPCELLID1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn ssppcellid1(&self) -> SSPPCELLID1_R {
        SSPPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
