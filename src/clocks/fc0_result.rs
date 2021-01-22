#[doc = "Reader of register FC0_RESULT"]
pub type R = crate::R<u32, super::FC0_RESULT>;
#[doc = "Reader of field `KHZ`"]
pub type KHZ_R = crate::R<u32, u32>;
#[doc = "Reader of field `FRAC`"]
pub type FRAC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 5:29"]
    #[inline(always)]
    pub fn khz(&self) -> KHZ_R {
        KHZ_R::new(((self.bits >> 5) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x1f) as u8)
    }
}
