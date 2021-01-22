#[doc = "Reader of register SOF_RD"]
pub type R = crate::R<u32, super::SOF_RD>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x07ff) as u16)
    }
}
