#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
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
impl W {}
#[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault.  

You can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
