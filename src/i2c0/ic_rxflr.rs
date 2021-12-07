#[doc = "Register `IC_RXFLR` reader"]
pub struct R(crate::R<IC_RXFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_RXFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_RXFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_RXFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFLR` reader - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO.  

 Reset value: 0x0"]
pub struct RXFLR_R(crate::FieldReader<u8, u8>);
impl RXFLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn rxflr(&self) -> RXFLR_R {
        RXFLR_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_rxflr](index.html) module"]
pub struct IC_RXFLR_SPEC;
impl crate::RegisterSpec for IC_RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_rxflr::R](R) reader structure"]
impl crate::Readable for IC_RXFLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_RXFLR to value 0"]
impl crate::Resettable for IC_RXFLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
