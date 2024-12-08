#[repr(C)]
#[doc = "Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
pub struct CH {
    ch_read_addr: CH_READ_ADDR,
    ch_write_addr: CH_WRITE_ADDR,
    ch_trans_count: CH_TRANS_COUNT,
    ch_ctrl_trig: CH_CTRL_TRIG,
    ch_al1_ctrl: CH_AL1_CTRL,
    ch_al1_read_addr: CH_AL1_READ_ADDR,
    ch_al1_write_addr: CH_AL1_WRITE_ADDR,
    ch_al1_trans_count_trig: CH_AL1_TRANS_COUNT_TRIG,
    ch_al2_ctrl: CH_AL2_CTRL,
    ch_al2_trans_count: CH_AL2_TRANS_COUNT,
    ch_al2_read_addr: CH_AL2_READ_ADDR,
    ch_al2_write_addr_trig: CH_AL2_WRITE_ADDR_TRIG,
    ch_al3_ctrl: CH_AL3_CTRL,
    ch_al3_write_addr: CH_AL3_WRITE_ADDR,
    ch_al3_trans_count: CH_AL3_TRANS_COUNT,
    ch_al3_read_addr_trig: CH_AL3_READ_ADDR_TRIG,
}
impl CH {
    #[doc = "0x00 - DMA Channel 0 Read Address pointer"]
    #[inline(always)]
    pub const fn ch_read_addr(&self) -> &CH_READ_ADDR {
        &self.ch_read_addr
    }
    #[doc = "0x04 - DMA Channel 0 Write Address pointer"]
    #[inline(always)]
    pub const fn ch_write_addr(&self) -> &CH_WRITE_ADDR {
        &self.ch_write_addr
    }
    #[doc = "0x08 - DMA Channel 0 Transfer Count"]
    #[inline(always)]
    pub const fn ch_trans_count(&self) -> &CH_TRANS_COUNT {
        &self.ch_trans_count
    }
    #[doc = "0x0c - DMA Channel 0 Control and Status"]
    #[inline(always)]
    pub const fn ch_ctrl_trig(&self) -> &CH_CTRL_TRIG {
        &self.ch_ctrl_trig
    }
    #[doc = "0x10 - DMA Channel 0 Control and Status"]
    #[inline(always)]
    pub const fn ch_al1_ctrl(&self) -> &CH_AL1_CTRL {
        &self.ch_al1_ctrl
    }
    #[doc = "0x14 - Alias for channel 0 READ_ADDR register"]
    #[inline(always)]
    pub const fn ch_al1_read_addr(&self) -> &CH_AL1_READ_ADDR {
        &self.ch_al1_read_addr
    }
    #[doc = "0x18 - Alias for channel 0 WRITE_ADDR register"]
    #[inline(always)]
    pub const fn ch_al1_write_addr(&self) -> &CH_AL1_WRITE_ADDR {
        &self.ch_al1_write_addr
    }
    #[doc = "0x1c - Alias for channel 0 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn ch_al1_trans_count_trig(&self) -> &CH_AL1_TRANS_COUNT_TRIG {
        &self.ch_al1_trans_count_trig
    }
    #[doc = "0x20 - DMA Channel 0 Control and Status"]
    #[inline(always)]
    pub const fn ch_al2_ctrl(&self) -> &CH_AL2_CTRL {
        &self.ch_al2_ctrl
    }
    #[doc = "0x24 - Alias for channel 0 TRANS_COUNT register"]
    #[inline(always)]
    pub const fn ch_al2_trans_count(&self) -> &CH_AL2_TRANS_COUNT {
        &self.ch_al2_trans_count
    }
    #[doc = "0x28 - Alias for channel 0 READ_ADDR register"]
    #[inline(always)]
    pub const fn ch_al2_read_addr(&self) -> &CH_AL2_READ_ADDR {
        &self.ch_al2_read_addr
    }
    #[doc = "0x2c - Alias for channel 0 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn ch_al2_write_addr_trig(&self) -> &CH_AL2_WRITE_ADDR_TRIG {
        &self.ch_al2_write_addr_trig
    }
    #[doc = "0x30 - DMA Channel 0 Control and Status"]
    #[inline(always)]
    pub const fn ch_al3_ctrl(&self) -> &CH_AL3_CTRL {
        &self.ch_al3_ctrl
    }
    #[doc = "0x34 - Alias for channel 0 WRITE_ADDR register"]
    #[inline(always)]
    pub const fn ch_al3_write_addr(&self) -> &CH_AL3_WRITE_ADDR {
        &self.ch_al3_write_addr
    }
    #[doc = "0x38 - Alias for channel 0 TRANS_COUNT register"]
    #[inline(always)]
    pub const fn ch_al3_trans_count(&self) -> &CH_AL3_TRANS_COUNT {
        &self.ch_al3_trans_count
    }
    #[doc = "0x3c - Alias for channel 0 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn ch_al3_read_addr_trig(&self) -> &CH_AL3_READ_ADDR_TRIG {
        &self.ch_al3_read_addr_trig
    }
}
#[doc = "CH_READ_ADDR (rw) register accessor: DMA Channel 0 Read Address pointer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_read_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_read_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_read_addr`]
module"]
pub type CH_READ_ADDR = crate::Reg<ch_read_addr::CH_READ_ADDR_SPEC>;
#[doc = "DMA Channel 0 Read Address pointer"]
pub mod ch_read_addr;
#[doc = "CH_WRITE_ADDR (rw) register accessor: DMA Channel 0 Write Address pointer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_write_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_write_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_write_addr`]
module"]
pub type CH_WRITE_ADDR = crate::Reg<ch_write_addr::CH_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 0 Write Address pointer"]
pub mod ch_write_addr;
#[doc = "CH_TRANS_COUNT (rw) register accessor: DMA Channel 0 Transfer Count  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_trans_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_trans_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_trans_count`]
module"]
pub type CH_TRANS_COUNT = crate::Reg<ch_trans_count::CH_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 0 Transfer Count"]
pub mod ch_trans_count;
#[doc = "CH_CTRL_TRIG (rw) register accessor: DMA Channel 0 Control and Status  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_ctrl_trig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ctrl_trig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_ctrl_trig`]
module"]
pub type CH_CTRL_TRIG = crate::Reg<ch_ctrl_trig::CH_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_ctrl_trig;
#[doc = "CH_AL1_CTRL (rw) register accessor: DMA Channel 0 Control and Status  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al1_ctrl`]
module"]
pub type CH_AL1_CTRL = crate::Reg<ch_al1_ctrl::CH_AL1_CTRL_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_al1_ctrl;
#[doc = "CH_AL1_READ_ADDR (rw) register accessor: Alias for channel 0 READ_ADDR register  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al1_read_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al1_read_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al1_read_addr`]
module"]
pub type CH_AL1_READ_ADDR = crate::Reg<ch_al1_read_addr::CH_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch_al1_read_addr;
#[doc = "CH_AL1_WRITE_ADDR (rw) register accessor: Alias for channel 0 WRITE_ADDR register  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al1_write_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al1_write_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al1_write_addr`]
module"]
pub type CH_AL1_WRITE_ADDR = crate::Reg<ch_al1_write_addr::CH_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch_al1_write_addr;
#[doc = "CH_AL1_TRANS_COUNT_TRIG (rw) register accessor: Alias for channel 0 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al1_trans_count_trig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al1_trans_count_trig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al1_trans_count_trig`]
module"]
pub type CH_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch_al1_trans_count_trig::CH_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
pub mod ch_al1_trans_count_trig;
#[doc = "CH_AL2_CTRL (rw) register accessor: DMA Channel 0 Control and Status  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al2_ctrl`]
module"]
pub type CH_AL2_CTRL = crate::Reg<ch_al2_ctrl::CH_AL2_CTRL_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_al2_ctrl;
#[doc = "CH_AL2_TRANS_COUNT (rw) register accessor: Alias for channel 0 TRANS_COUNT register  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al2_trans_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al2_trans_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al2_trans_count`]
module"]
pub type CH_AL2_TRANS_COUNT = crate::Reg<ch_al2_trans_count::CH_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch_al2_trans_count;
#[doc = "CH_AL2_READ_ADDR (rw) register accessor: Alias for channel 0 READ_ADDR register  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al2_read_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al2_read_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al2_read_addr`]
module"]
pub type CH_AL2_READ_ADDR = crate::Reg<ch_al2_read_addr::CH_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch_al2_read_addr;
#[doc = "CH_AL2_WRITE_ADDR_TRIG (rw) register accessor: Alias for channel 0 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al2_write_addr_trig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al2_write_addr_trig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al2_write_addr_trig`]
module"]
pub type CH_AL2_WRITE_ADDR_TRIG = crate::Reg<ch_al2_write_addr_trig::CH_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
pub mod ch_al2_write_addr_trig;
#[doc = "CH_AL3_CTRL (rw) register accessor: DMA Channel 0 Control and Status  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al3_ctrl`]
module"]
pub type CH_AL3_CTRL = crate::Reg<ch_al3_ctrl::CH_AL3_CTRL_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_al3_ctrl;
#[doc = "CH_AL3_WRITE_ADDR (rw) register accessor: Alias for channel 0 WRITE_ADDR register  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al3_write_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al3_write_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al3_write_addr`]
module"]
pub type CH_AL3_WRITE_ADDR = crate::Reg<ch_al3_write_addr::CH_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch_al3_write_addr;
#[doc = "CH_AL3_TRANS_COUNT (rw) register accessor: Alias for channel 0 TRANS_COUNT register  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al3_trans_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al3_trans_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al3_trans_count`]
module"]
pub type CH_AL3_TRANS_COUNT = crate::Reg<ch_al3_trans_count::CH_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch_al3_trans_count;
#[doc = "CH_AL3_READ_ADDR_TRIG (rw) register accessor: Alias for channel 0 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al3_read_addr_trig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al3_read_addr_trig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch_al3_read_addr_trig`]
module"]
pub type CH_AL3_READ_ADDR_TRIG = crate::Reg<ch_al3_read_addr_trig::CH_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
pub mod ch_al3_read_addr_trig;
