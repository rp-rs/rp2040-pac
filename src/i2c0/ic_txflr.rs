#[doc = "Register `IC_TXFLR` reader"]
pub type R = crate::R<IC_TXFLR_SPEC>;
#[doc = "Field `TXFLR` reader - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.  

 Reset value: 0x0"]
pub type TXFLR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn txflr(&self) -> TXFLR_R {
        TXFLR_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO.  

You can [`read`](crate::Reg::read) this register and get [`ic_txflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_TXFLR_SPEC;
impl crate::RegisterSpec for IC_TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_txflr::R`](R) reader structure"]
impl crate::Readable for IC_TXFLR_SPEC {}
#[doc = "`reset()` method sets IC_TXFLR to value 0"]
impl crate::Resettable for IC_TXFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
