#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Cache Flush control"]
    pub flush: FLUSH,
    #[doc = "0x08 - Cache Status"]
    pub stat: STAT,
    #[doc = "0x0c - Cache Hit counter\\n A 32 bit saturating counter that increments upon each cache hit,\\n i.e. when an XIP access is serviced directly from cached data.\\n Write any value to clear."]
    pub ctr_hit: CTR_HIT,
    #[doc = "0x10 - Cache Access counter\\n A 32 bit saturating counter that increments upon each XIP access,\\n whether the cache is hit or not. This includes noncacheable accesses.\\n Write any value to clear."]
    pub ctr_acc: CTR_ACC,
    #[doc = "0x14 - FIFO stream address"]
    pub stream_addr: STREAM_ADDR,
    #[doc = "0x18 - FIFO stream control"]
    pub stream_ctr: STREAM_CTR,
    #[doc = "0x1c - FIFO stream data\\n Streamed data is buffered here, for retrieval by the system DMA.\\n This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing\\n the DMA to bus stalls caused by other XIP traffic."]
    pub stream_fifo: STREAM_FIFO,
}
#[doc = "Cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Cache control"]
pub mod ctrl;
#[doc = "Cache Flush control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flush](flush) module"]
pub type FLUSH = crate::Reg<u32, _FLUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLUSH;
#[doc = "`read()` method returns [flush::R](flush::R) reader structure"]
impl crate::Readable for FLUSH {}
#[doc = "`write(|w| ..)` method takes [flush::W](flush::W) writer structure"]
impl crate::Writable for FLUSH {}
#[doc = "Cache Flush control"]
pub mod flush;
#[doc = "Cache Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "Cache Status"]
pub mod stat;
#[doc = "Cache Hit counter\\n A 32 bit saturating counter that increments upon each cache hit,\\n i.e. when an XIP access is serviced directly from cached data.\\n Write any value to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_hit](ctr_hit) module"]
pub type CTR_HIT = crate::Reg<u32, _CTR_HIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_HIT;
#[doc = "`read()` method returns [ctr_hit::R](ctr_hit::R) reader structure"]
impl crate::Readable for CTR_HIT {}
#[doc = "`write(|w| ..)` method takes [ctr_hit::W](ctr_hit::W) writer structure"]
impl crate::Writable for CTR_HIT {}
#[doc = "Cache Hit counter\\n A 32 bit saturating counter that increments upon each cache hit,\\n i.e. when an XIP access is serviced directly from cached data.\\n Write any value to clear."]
pub mod ctr_hit;
#[doc = "Cache Access counter\\n A 32 bit saturating counter that increments upon each XIP access,\\n whether the cache is hit or not. This includes noncacheable accesses.\\n Write any value to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_acc](ctr_acc) module"]
pub type CTR_ACC = crate::Reg<u32, _CTR_ACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_ACC;
#[doc = "`read()` method returns [ctr_acc::R](ctr_acc::R) reader structure"]
impl crate::Readable for CTR_ACC {}
#[doc = "`write(|w| ..)` method takes [ctr_acc::W](ctr_acc::W) writer structure"]
impl crate::Writable for CTR_ACC {}
#[doc = "Cache Access counter\\n A 32 bit saturating counter that increments upon each XIP access,\\n whether the cache is hit or not. This includes noncacheable accesses.\\n Write any value to clear."]
pub mod ctr_acc;
#[doc = "FIFO stream address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stream_addr](stream_addr) module"]
pub type STREAM_ADDR = crate::Reg<u32, _STREAM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STREAM_ADDR;
#[doc = "`read()` method returns [stream_addr::R](stream_addr::R) reader structure"]
impl crate::Readable for STREAM_ADDR {}
#[doc = "`write(|w| ..)` method takes [stream_addr::W](stream_addr::W) writer structure"]
impl crate::Writable for STREAM_ADDR {}
#[doc = "FIFO stream address"]
pub mod stream_addr;
#[doc = "FIFO stream control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stream_ctr](stream_ctr) module"]
pub type STREAM_CTR = crate::Reg<u32, _STREAM_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STREAM_CTR;
#[doc = "`read()` method returns [stream_ctr::R](stream_ctr::R) reader structure"]
impl crate::Readable for STREAM_CTR {}
#[doc = "`write(|w| ..)` method takes [stream_ctr::W](stream_ctr::W) writer structure"]
impl crate::Writable for STREAM_CTR {}
#[doc = "FIFO stream control"]
pub mod stream_ctr;
#[doc = "FIFO stream data\\n Streamed data is buffered here, for retrieval by the system DMA.\\n This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing\\n the DMA to bus stalls caused by other XIP traffic.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stream_fifo](stream_fifo) module"]
pub type STREAM_FIFO = crate::Reg<u32, _STREAM_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STREAM_FIFO;
#[doc = "`read()` method returns [stream_fifo::R](stream_fifo::R) reader structure"]
impl crate::Readable for STREAM_FIFO {}
#[doc = "FIFO stream data\\n Streamed data is buffered here, for retrieval by the system DMA.\\n This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing\\n the DMA to bus stalls caused by other XIP traffic."]
pub mod stream_fifo;
