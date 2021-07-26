#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Cache Flush control"]
    pub flush: crate::Reg<flush::FLUSH_SPEC>,
    #[doc = "0x08 - Cache Status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x0c - Cache Hit counter  
 A 32 bit saturating counter that increments upon each cache hit,  
 i.e. when an XIP access is serviced directly from cached data.  
 Write any value to clear."]
    pub ctr_hit: crate::Reg<ctr_hit::CTR_HIT_SPEC>,
    #[doc = "0x10 - Cache Access counter  
 A 32 bit saturating counter that increments upon each XIP access,  
 whether the cache is hit or not. This includes noncacheable accesses.  
 Write any value to clear."]
    pub ctr_acc: crate::Reg<ctr_acc::CTR_ACC_SPEC>,
    #[doc = "0x14 - FIFO stream address"]
    pub stream_addr: crate::Reg<stream_addr::STREAM_ADDR_SPEC>,
    #[doc = "0x18 - FIFO stream control"]
    pub stream_ctr: crate::Reg<stream_ctr::STREAM_CTR_SPEC>,
    #[doc = "0x1c - FIFO stream data  
 Streamed data is buffered here, for retrieval by the system DMA.  
 This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing  
 the DMA to bus stalls caused by other XIP traffic."]
    pub stream_fifo: crate::Reg<stream_fifo::STREAM_FIFO_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Cache control"]
pub mod ctrl;
#[doc = "FLUSH register accessor: an alias for `Reg<FLUSH_SPEC>`"]
pub type FLUSH = crate::Reg<flush::FLUSH_SPEC>;
#[doc = "Cache Flush control"]
pub mod flush;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Cache Status"]
pub mod stat;
#[doc = "CTR_HIT register accessor: an alias for `Reg<CTR_HIT_SPEC>`"]
pub type CTR_HIT = crate::Reg<ctr_hit::CTR_HIT_SPEC>;
#[doc = "Cache Hit counter  
 A 32 bit saturating counter that increments upon each cache hit,  
 i.e. when an XIP access is serviced directly from cached data.  
 Write any value to clear."]
pub mod ctr_hit;
#[doc = "CTR_ACC register accessor: an alias for `Reg<CTR_ACC_SPEC>`"]
pub type CTR_ACC = crate::Reg<ctr_acc::CTR_ACC_SPEC>;
#[doc = "Cache Access counter  
 A 32 bit saturating counter that increments upon each XIP access,  
 whether the cache is hit or not. This includes noncacheable accesses.  
 Write any value to clear."]
pub mod ctr_acc;
#[doc = "STREAM_ADDR register accessor: an alias for `Reg<STREAM_ADDR_SPEC>`"]
pub type STREAM_ADDR = crate::Reg<stream_addr::STREAM_ADDR_SPEC>;
#[doc = "FIFO stream address"]
pub mod stream_addr;
#[doc = "STREAM_CTR register accessor: an alias for `Reg<STREAM_CTR_SPEC>`"]
pub type STREAM_CTR = crate::Reg<stream_ctr::STREAM_CTR_SPEC>;
#[doc = "FIFO stream control"]
pub mod stream_ctr;
#[doc = "STREAM_FIFO register accessor: an alias for `Reg<STREAM_FIFO_SPEC>`"]
pub type STREAM_FIFO = crate::Reg<stream_fifo::STREAM_FIFO_SPEC>;
#[doc = "FIFO stream data  
 Streamed data is buffered here, for retrieval by the system DMA.  
 This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing  
 the DMA to bus stalls caused by other XIP traffic."]
pub mod stream_fifo;
