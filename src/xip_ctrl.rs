#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    flush: FLUSH,
    stat: STAT,
    ctr_hit: CTR_HIT,
    ctr_acc: CTR_ACC,
    stream_addr: STREAM_ADDR,
    stream_ctr: STREAM_CTR,
    stream_fifo: STREAM_FIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - Cache control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Cache Flush control"]
    #[inline(always)]
    pub const fn flush(&self) -> &FLUSH {
        &self.flush
    }
    #[doc = "0x08 - Cache Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    #[doc = "0x0c - Cache Hit counter"]
    #[inline(always)]
    pub const fn ctr_hit(&self) -> &CTR_HIT {
        &self.ctr_hit
    }
    #[doc = "0x10 - Cache Access counter"]
    #[inline(always)]
    pub const fn ctr_acc(&self) -> &CTR_ACC {
        &self.ctr_acc
    }
    #[doc = "0x14 - FIFO stream address"]
    #[inline(always)]
    pub const fn stream_addr(&self) -> &STREAM_ADDR {
        &self.stream_addr
    }
    #[doc = "0x18 - FIFO stream control"]
    #[inline(always)]
    pub const fn stream_ctr(&self) -> &STREAM_CTR {
        &self.stream_ctr
    }
    #[doc = "0x1c - FIFO stream data"]
    #[inline(always)]
    pub const fn stream_fifo(&self) -> &STREAM_FIFO {
        &self.stream_fifo
    }
}
#[doc = "CTRL (rw) register accessor: Cache control  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Cache control"]
pub mod ctrl;
#[doc = "FLUSH (rw) register accessor: Cache Flush control  

You can [`read`](crate::generic::Reg::read) this register and get [`flush::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flush::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@flush`]
module"]
pub type FLUSH = crate::Reg<flush::FLUSH_SPEC>;
#[doc = "Cache Flush control"]
pub mod flush;
#[doc = "STAT (rw) register accessor: Cache Status  

You can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Cache Status"]
pub mod stat;
#[doc = "CTR_HIT (rw) register accessor: Cache Hit counter  

You can [`read`](crate::generic::Reg::read) this register and get [`ctr_hit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr_hit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctr_hit`]
module"]
pub type CTR_HIT = crate::Reg<ctr_hit::CTR_HIT_SPEC>;
#[doc = "Cache Hit counter"]
pub mod ctr_hit;
#[doc = "CTR_ACC (rw) register accessor: Cache Access counter  

You can [`read`](crate::generic::Reg::read) this register and get [`ctr_acc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr_acc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctr_acc`]
module"]
pub type CTR_ACC = crate::Reg<ctr_acc::CTR_ACC_SPEC>;
#[doc = "Cache Access counter"]
pub mod ctr_acc;
#[doc = "STREAM_ADDR (rw) register accessor: FIFO stream address  

You can [`read`](crate::generic::Reg::read) this register and get [`stream_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stream_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@stream_addr`]
module"]
pub type STREAM_ADDR = crate::Reg<stream_addr::STREAM_ADDR_SPEC>;
#[doc = "FIFO stream address"]
pub mod stream_addr;
#[doc = "STREAM_CTR (rw) register accessor: FIFO stream control  

You can [`read`](crate::generic::Reg::read) this register and get [`stream_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stream_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@stream_ctr`]
module"]
pub type STREAM_CTR = crate::Reg<stream_ctr::STREAM_CTR_SPEC>;
#[doc = "FIFO stream control"]
pub mod stream_ctr;
#[doc = "STREAM_FIFO (rw) register accessor: FIFO stream data  

You can [`read`](crate::generic::Reg::read) this register and get [`stream_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stream_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@stream_fifo`]
module"]
pub type STREAM_FIFO = crate::Reg<stream_fifo::STREAM_FIFO_SPEC>;
#[doc = "FIFO stream data"]
pub mod stream_fifo;
