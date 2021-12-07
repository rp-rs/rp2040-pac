#[doc = "Register `IC_TXFLR` reader"]
pub struct R(crate::R<IC_TXFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_TXFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_TXFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_TXFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXFLR` reader - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.  

 Reset value: 0x0"]
pub struct TXFLR_R(crate::FieldReader<u8, u8>);
impl TXFLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn txflr(&self) -> TXFLR_R {
        TXFLR_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_txflr](index.html) module"]
pub struct IC_TXFLR_SPEC;
impl crate::RegisterSpec for IC_TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_txflr::R](R) reader structure"]
impl crate::Readable for IC_TXFLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_TXFLR to value 0"]
impl crate::Resettable for IC_TXFLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
