#[doc = "Register `DIV_CSR` reader"]
pub struct R(crate::R<DIV_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIRTY` reader - Changes to 1 when any register is written, and back to 0 when QUOTIENT is read.  
 Software can use this flag to make save/restore more efficient (skip if not DIRTY).  
 If the flag is used in this way, it's recommended to either read QUOTIENT only,  
 or REMAINDER and then QUOTIENT, to prevent data loss on context switch."]
pub struct DIRTY_R(crate::FieldReader<bool, bool>);
impl DIRTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READY` reader - Reads as 0 when a calculation is in progress, 1 otherwise.  
 Writing an operand (xDIVIDEND, xDIVISOR) will immediately start a new calculation, no  
 matter if one is already in progress.  
 Writing to a result register will immediately terminate any in-progress calculation  
 and set the READY and DIRTY flags."]
pub struct READY_R(crate::FieldReader<bool, bool>);
impl READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Changes to 1 when any register is written, and back to 0 when QUOTIENT is read.  
 Software can use this flag to make save/restore more efficient (skip if not DIRTY).  
 If the flag is used in this way, it's recommended to either read QUOTIENT only,  
 or REMAINDER and then QUOTIENT, to prevent data loss on context switch."]
    #[inline(always)]
    pub fn dirty(&self) -> DIRTY_R {
        DIRTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Reads as 0 when a calculation is in progress, 1 otherwise.  
 Writing an operand (xDIVIDEND, xDIVISOR) will immediately start a new calculation, no  
 matter if one is already in progress.  
 Writing to a result register will immediately terminate any in-progress calculation  
 and set the READY and DIRTY flags."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Control and status register for divider.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [div_csr](index.html) module"]
pub struct DIV_CSR_SPEC;
impl crate::RegisterSpec for DIV_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_csr::R](R) reader structure"]
impl crate::Readable for DIV_CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIV_CSR to value 0x01"]
impl crate::Resettable for DIV_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
