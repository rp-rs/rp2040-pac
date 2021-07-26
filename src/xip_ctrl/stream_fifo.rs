#[doc = "Register `STREAM_FIFO` reader"]
pub struct R(crate::R<STREAM_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STREAM_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STREAM_FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STREAM_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "FIFO stream data  
 Streamed data is buffered here, for retrieval by the system DMA.  
 This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing  
 the DMA to bus stalls caused by other XIP traffic.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [stream_fifo](index.html) module"]
pub struct STREAM_FIFO_SPEC;
impl crate::RegisterSpec for STREAM_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stream_fifo::R](R) reader structure"]
impl crate::Readable for STREAM_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STREAM_FIFO to value 0"]
impl crate::Resettable for STREAM_FIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
