#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Reader of field `ICR`"]
pub type ICR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clear-on-read all active interrupts"]
    #[inline(always)]
    pub fn icr(&self) -> ICR_R {
        ICR_R::new((self.bits & 0x01) != 0)
    }
}
