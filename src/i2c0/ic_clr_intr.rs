#[doc = "Register `IC_CLR_INTR` reader"]
pub struct R(crate::R<IC_CLR_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_INTR` reader - Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.  

 Reset value: 0x0"]
pub struct CLR_INTR_R(crate::FieldReader<bool, bool>);
impl CLR_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_intr(&self) -> CLR_INTR_R {
        CLR_INTR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clear Combined and Individual Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_intr](index.html) module"]
pub struct IC_CLR_INTR_SPEC;
impl crate::RegisterSpec for IC_CLR_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_intr::R](R) reader structure"]
impl crate::Readable for IC_CLR_INTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_INTR to value 0"]
impl crate::Resettable for IC_CLR_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
