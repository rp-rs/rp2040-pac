#[doc = "Reader of register SSPPCELLID2"]
pub type R = crate::R<u32, super::SSPPCELLID2>;
#[doc = "Reader of field `SSPPCELLID2`"]
pub type SSPPCELLID2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x05"]
    #[inline(always)]
    pub fn ssppcellid2(&self) -> SSPPCELLID2_R {
        SSPPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
