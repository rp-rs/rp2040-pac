#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Field `UNALIGN_TRP` reader - Always reads as one, indicates that all unaligned accesses generate a HardFault."]
pub type UNALIGN_TRP_R = crate::BitReader;
#[doc = "Field `STKALIGN` reader - Always reads as one, indicates 8-byte stack alignment on exception entry. On exception entry, the processor uses bit\\[9\\]
of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
pub type STKALIGN_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Always reads as one, indicates that all unaligned accesses generate a HardFault."]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Always reads as one, indicates 8-byte stack alignment on exception entry. On exception entry, the processor uses bit\\[9\\]
of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault.  

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
