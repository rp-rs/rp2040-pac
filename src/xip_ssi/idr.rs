#[doc = "Reader of register IDR"]
pub type R = crate::R<u32, super::IDR>;
#[doc = "Reader of field `IDCODE`"]
pub type IDCODE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral dentification code"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
