#[doc = "Register `RXFLR` reader"]
pub struct R(crate::R<RXFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXTFL` reader - Receive FIFO level"]
pub type RXTFL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO level"]
    #[inline(always)]
    pub fn rxtfl(&self) -> RXTFL_R {
        RXTFL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO level  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [rxflr](index.html) module"]
pub struct RXFLR_SPEC;
impl crate::RegisterSpec for RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxflr::R](R) reader structure"]
impl crate::Readable for RXFLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RXFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
