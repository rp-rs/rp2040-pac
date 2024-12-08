#[doc = "Register `STREAM_FIFO` reader"]
pub type R = crate::R<STREAM_FIFO_SPEC>;
#[doc = "Register `STREAM_FIFO` writer"]
pub type W = crate::W<STREAM_FIFO_SPEC>;
#[doc = "Field `STREAM_FIFO` reader - Streamed data is buffered here, for retrieval by the system DMA. This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing the DMA to bus stalls caused by other XIP traffic.  

The field is **modified** in some way after a read operation."]
pub type STREAM_FIFO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Streamed data is buffered here, for retrieval by the system DMA. This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing the DMA to bus stalls caused by other XIP traffic."]
    #[inline(always)]
    pub fn stream_fifo(&self) -> STREAM_FIFO_R {
        STREAM_FIFO_R::new(self.bits)
    }
}
impl W {}
#[doc = "FIFO stream data  

You can [`read`](crate::generic::Reg::read) this register and get [`stream_fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stream_fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STREAM_FIFO_SPEC;
impl crate::RegisterSpec for STREAM_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stream_fifo::R`](R) reader structure"]
impl crate::Readable for STREAM_FIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stream_fifo::W`](W) writer structure"]
impl crate::Writable for STREAM_FIFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STREAM_FIFO to value 0"]
impl crate::Resettable for STREAM_FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
