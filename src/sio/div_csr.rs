#[doc = "Reader of register DIV_CSR"]
pub type R = crate::R<u32, super::DIV_CSR>;
#[doc = "Reader of field `DIRTY`"]
pub type DIRTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Changes to 1 when any register is written, and back to 0 when QUOTIENT is read.\\n Software can use this flag to make save/restore more efficient (skip if not DIRTY).\\n If the flag is used in this way, it's recommended to either read QUOTIENT only,\\n or REMAINDER and then QUOTIENT, to prevent data loss on context switch."]
    #[inline(always)]
    pub fn dirty(&self) -> DIRTY_R {
        DIRTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Reads as 0 when a calculation is in progress, 1 otherwise.\\n Writing an operand (xDIVIDEND, xDIVISOR) will immediately start a new calculation, no\\n matter if one is already in progress.\\n Writing to a result register will immediately terminate any in-progress calculation\\n and set the READY and DIRTY flags."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
}
