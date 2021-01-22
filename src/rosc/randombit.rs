#[doc = "Reader of register RANDOMBIT"]
pub type R = crate::R<u32, super::RANDOMBIT>;
#[doc = "Reader of field `RANDOMBIT`"]
pub type RANDOMBIT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn randombit(&self) -> RANDOMBIT_R {
        RANDOMBIT_R::new((self.bits & 0x01) != 0)
    }
}
