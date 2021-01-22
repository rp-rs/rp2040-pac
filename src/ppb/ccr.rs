#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Reader of field `STKALIGN`"]
pub type STKALIGN_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNALIGN_TRP`"]
pub type UNALIGN_TRP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 9 - Always reads as one, indicates 8-byte stack alignment on exception entry. On exception entry, the processor uses bit\\[9\\]
of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Always reads as one, indicates that all unaligned accesses generate a HardFault."]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
