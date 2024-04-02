#[doc = "Register `STREAM_FIFO` reader"]
pub type R = crate::R<STREAM_FIFO_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "FIFO stream data  
 Streamed data is buffered here, for retrieval by the system DMA.  
 This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing  
 the DMA to bus stalls caused by other XIP traffic.  

You can [`read`](crate::Reg::read) this register and get [`stream_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STREAM_FIFO_SPEC;
impl crate::RegisterSpec for STREAM_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stream_fifo::R`](R) reader structure"]
impl crate::Readable for STREAM_FIFO_SPEC {}
#[doc = "`reset()` method sets STREAM_FIFO to value 0"]
impl crate::Resettable for STREAM_FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
