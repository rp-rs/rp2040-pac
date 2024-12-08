#[doc = "Register `IC_RXFLR` reader"]
pub type R = crate::R<IC_RXFLR_SPEC>;
#[doc = "Register `IC_RXFLR` writer"]
pub type W = crate::W<IC_RXFLR_SPEC>;
#[doc = "Field `RXFLR` reader - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO. Reset value: 0x0"]
pub type RXFLR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO. Reset value: 0x0"]
    #[inline(always)]
    pub fn rxflr(&self) -> RXFLR_R {
        RXFLR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO.  

You can [`read`](crate::generic::Reg::read) this register and get [`ic_rxflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_rxflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_RXFLR_SPEC;
impl crate::RegisterSpec for IC_RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_rxflr::R`](R) reader structure"]
impl crate::Readable for IC_RXFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_rxflr::W`](W) writer structure"]
impl crate::Writable for IC_RXFLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_RXFLR to value 0"]
impl crate::Resettable for IC_RXFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
