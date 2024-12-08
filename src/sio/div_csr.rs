#[doc = "Register `DIV_CSR` reader"]
pub type R = crate::R<DIV_CSR_SPEC>;
#[doc = "Register `DIV_CSR` writer"]
pub type W = crate::W<DIV_CSR_SPEC>;
#[doc = "Field `READY` reader - Reads as 0 when a calculation is in progress, 1 otherwise. Writing an operand (xDIVIDEND, xDIVISOR) will immediately start a new calculation, no matter if one is already in progress. Writing to a result register will immediately terminate any in-progress calculation and set the READY and DIRTY flags."]
pub type READY_R = crate::BitReader;
#[doc = "Field `DIRTY` reader - Changes to 1 when any register is written, and back to 0 when QUOTIENT is read. Software can use this flag to make save/restore more efficient (skip if not DIRTY). If the flag is used in this way, it's recommended to either read QUOTIENT only, or REMAINDER and then QUOTIENT, to prevent data loss on context switch."]
pub type DIRTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reads as 0 when a calculation is in progress, 1 otherwise. Writing an operand (xDIVIDEND, xDIVISOR) will immediately start a new calculation, no matter if one is already in progress. Writing to a result register will immediately terminate any in-progress calculation and set the READY and DIRTY flags."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Changes to 1 when any register is written, and back to 0 when QUOTIENT is read. Software can use this flag to make save/restore more efficient (skip if not DIRTY). If the flag is used in this way, it's recommended to either read QUOTIENT only, or REMAINDER and then QUOTIENT, to prevent data loss on context switch."]
    #[inline(always)]
    pub fn dirty(&self) -> DIRTY_R {
        DIRTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "Control and status register for divider.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_CSR_SPEC;
impl crate::RegisterSpec for DIV_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_csr::R`](R) reader structure"]
impl crate::Readable for DIV_CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_csr::W`](W) writer structure"]
impl crate::Writable for DIV_CSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_CSR to value 0x01"]
impl crate::Resettable for DIV_CSR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
