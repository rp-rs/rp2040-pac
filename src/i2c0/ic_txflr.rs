#[doc = "Register `IC_TXFLR` reader"]
pub type R = crate::R<IC_TXFLR_SPEC>;
#[doc = "Register `IC_TXFLR` writer"]
pub type W = crate::W<IC_TXFLR_SPEC>;
#[doc = "Field `TXFLR` reader - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO. Reset value: 0x0"]
pub type TXFLR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO. Reset value: 0x0"]
    #[inline(always)]
    pub fn txflr(&self) -> TXFLR_R {
        TXFLR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO.  

You can [`read`](crate::generic::Reg::read) this register and get [`ic_txflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_txflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_TXFLR_SPEC;
impl crate::RegisterSpec for IC_TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_txflr::R`](R) reader structure"]
impl crate::Readable for IC_TXFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_txflr::W`](W) writer structure"]
impl crate::Writable for IC_TXFLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_TXFLR to value 0"]
impl crate::Resettable for IC_TXFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
