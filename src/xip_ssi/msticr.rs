#[doc = "Reader of register MSTICR"]
pub type R = crate::R<u32, super::MSTICR>;
#[doc = "Reader of field `MSTICR`"]
pub type MSTICR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clear-on-read multi-master contention interrupt"]
    #[inline(always)]
    pub fn msticr(&self) -> MSTICR_R {
        MSTICR_R::new((self.bits & 0x01) != 0)
    }
}
