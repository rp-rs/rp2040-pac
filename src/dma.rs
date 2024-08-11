#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [CH; 12],
    _reserved1: [u8; 0x0100],
    intr: INTR,
    inte0: INTE0,
    intf0: INTF0,
    ints0: INTS0,
    intr1: INTR1,
    inte1: INTE1,
    intf1: INTF1,
    ints1: INTS1,
    timer0: TIMER0,
    timer1: TIMER1,
    timer2: TIMER2,
    timer3: TIMER3,
    multi_chan_trigger: MULTI_CHAN_TRIGGER,
    sniff_ctrl: SNIFF_CTRL,
    sniff_data: SNIFF_DATA,
    _reserved16: [u8; 0x04],
    fifo_levels: FIFO_LEVELS,
    chan_abort: CHAN_ABORT,
    n_channels: N_CHANNELS,
    _reserved19: [u8; 0x03b4],
    ch0_dbg_ctdreq: CH0_DBG_CTDREQ,
    ch0_dbg_tcr: CH0_DBG_TCR,
    _reserved21: [u8; 0x38],
    ch1_dbg_ctdreq: CH1_DBG_CTDREQ,
    ch1_dbg_tcr: CH1_DBG_TCR,
    _reserved23: [u8; 0x38],
    ch2_dbg_ctdreq: CH2_DBG_CTDREQ,
    ch2_dbg_tcr: CH2_DBG_TCR,
    _reserved25: [u8; 0x38],
    ch3_dbg_ctdreq: CH3_DBG_CTDREQ,
    ch3_dbg_tcr: CH3_DBG_TCR,
    _reserved27: [u8; 0x38],
    ch4_dbg_ctdreq: CH4_DBG_CTDREQ,
    ch4_dbg_tcr: CH4_DBG_TCR,
    _reserved29: [u8; 0x38],
    ch5_dbg_ctdreq: CH5_DBG_CTDREQ,
    ch5_dbg_tcr: CH5_DBG_TCR,
    _reserved31: [u8; 0x38],
    ch6_dbg_ctdreq: CH6_DBG_CTDREQ,
    ch6_dbg_tcr: CH6_DBG_TCR,
    _reserved33: [u8; 0x38],
    ch7_dbg_ctdreq: CH7_DBG_CTDREQ,
    ch7_dbg_tcr: CH7_DBG_TCR,
    _reserved35: [u8; 0x38],
    ch8_dbg_ctdreq: CH8_DBG_CTDREQ,
    ch8_dbg_tcr: CH8_DBG_TCR,
    _reserved37: [u8; 0x38],
    ch9_dbg_ctdreq: CH9_DBG_CTDREQ,
    ch9_dbg_tcr: CH9_DBG_TCR,
    _reserved39: [u8; 0x38],
    ch10_dbg_ctdreq: CH10_DBG_CTDREQ,
    ch10_dbg_tcr: CH10_DBG_TCR,
    _reserved41: [u8; 0x38],
    ch11_dbg_ctdreq: CH11_DBG_CTDREQ,
    ch11_dbg_tcr: CH11_DBG_TCR,
}
impl RegisterBlock {
    #[doc = "0x00..0x300 - Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x300 - Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x400 - Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x404 - Interrupt Enables for IRQ 0"]
    #[inline(always)]
    pub const fn inte0(&self) -> &INTE0 {
        &self.inte0
    }
    #[doc = "0x408 - Force Interrupts"]
    #[inline(always)]
    pub const fn intf0(&self) -> &INTF0 {
        &self.intf0
    }
    #[doc = "0x40c - Interrupt Status for IRQ 0"]
    #[inline(always)]
    pub const fn ints0(&self) -> &INTS0 {
        &self.ints0
    }
    #[doc = "0x410 - Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr1(&self) -> &INTR1 {
        &self.intr1
    }
    #[doc = "0x414 - Interrupt Enables for IRQ 1"]
    #[inline(always)]
    pub const fn inte1(&self) -> &INTE1 {
        &self.inte1
    }
    #[doc = "0x418 - Force Interrupts for IRQ 1"]
    #[inline(always)]
    pub const fn intf1(&self) -> &INTF1 {
        &self.intf1
    }
    #[doc = "0x41c - Interrupt Status (masked) for IRQ 1"]
    #[inline(always)]
    pub const fn ints1(&self) -> &INTS1 {
        &self.ints1
    }
    #[doc = "0x420 - Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer0(&self) -> &TIMER0 {
        &self.timer0
    }
    #[doc = "0x424 - Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer1(&self) -> &TIMER1 {
        &self.timer1
    }
    #[doc = "0x428 - Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer2(&self) -> &TIMER2 {
        &self.timer2
    }
    #[doc = "0x42c - Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer3(&self) -> &TIMER3 {
        &self.timer3
    }
    #[doc = "0x430 - Trigger one or more channels simultaneously"]
    #[inline(always)]
    pub const fn multi_chan_trigger(&self) -> &MULTI_CHAN_TRIGGER {
        &self.multi_chan_trigger
    }
    #[doc = "0x434 - Sniffer Control"]
    #[inline(always)]
    pub const fn sniff_ctrl(&self) -> &SNIFF_CTRL {
        &self.sniff_ctrl
    }
    #[doc = "0x438 - Data accumulator for sniff hardware"]
    #[inline(always)]
    pub const fn sniff_data(&self) -> &SNIFF_DATA {
        &self.sniff_data
    }
    #[doc = "0x440 - Debug RAF, WAF, TDF levels"]
    #[inline(always)]
    pub const fn fifo_levels(&self) -> &FIFO_LEVELS {
        &self.fifo_levels
    }
    #[doc = "0x444 - Abort an in-progress transfer sequence on one or more channels"]
    #[inline(always)]
    pub const fn chan_abort(&self) -> &CHAN_ABORT {
        &self.chan_abort
    }
    #[doc = "0x448 - The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    #[inline(always)]
    pub const fn n_channels(&self) -> &N_CHANNELS {
        &self.n_channels
    }
    #[doc = "0x800 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch0_dbg_ctdreq(&self) -> &CH0_DBG_CTDREQ {
        &self.ch0_dbg_ctdreq
    }
    #[doc = "0x804 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch0_dbg_tcr(&self) -> &CH0_DBG_TCR {
        &self.ch0_dbg_tcr
    }
    #[doc = "0x840 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch1_dbg_ctdreq(&self) -> &CH1_DBG_CTDREQ {
        &self.ch1_dbg_ctdreq
    }
    #[doc = "0x844 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch1_dbg_tcr(&self) -> &CH1_DBG_TCR {
        &self.ch1_dbg_tcr
    }
    #[doc = "0x880 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch2_dbg_ctdreq(&self) -> &CH2_DBG_CTDREQ {
        &self.ch2_dbg_ctdreq
    }
    #[doc = "0x884 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch2_dbg_tcr(&self) -> &CH2_DBG_TCR {
        &self.ch2_dbg_tcr
    }
    #[doc = "0x8c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch3_dbg_ctdreq(&self) -> &CH3_DBG_CTDREQ {
        &self.ch3_dbg_ctdreq
    }
    #[doc = "0x8c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch3_dbg_tcr(&self) -> &CH3_DBG_TCR {
        &self.ch3_dbg_tcr
    }
    #[doc = "0x900 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch4_dbg_ctdreq(&self) -> &CH4_DBG_CTDREQ {
        &self.ch4_dbg_ctdreq
    }
    #[doc = "0x904 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch4_dbg_tcr(&self) -> &CH4_DBG_TCR {
        &self.ch4_dbg_tcr
    }
    #[doc = "0x940 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch5_dbg_ctdreq(&self) -> &CH5_DBG_CTDREQ {
        &self.ch5_dbg_ctdreq
    }
    #[doc = "0x944 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch5_dbg_tcr(&self) -> &CH5_DBG_TCR {
        &self.ch5_dbg_tcr
    }
    #[doc = "0x980 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch6_dbg_ctdreq(&self) -> &CH6_DBG_CTDREQ {
        &self.ch6_dbg_ctdreq
    }
    #[doc = "0x984 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch6_dbg_tcr(&self) -> &CH6_DBG_TCR {
        &self.ch6_dbg_tcr
    }
    #[doc = "0x9c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch7_dbg_ctdreq(&self) -> &CH7_DBG_CTDREQ {
        &self.ch7_dbg_ctdreq
    }
    #[doc = "0x9c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch7_dbg_tcr(&self) -> &CH7_DBG_TCR {
        &self.ch7_dbg_tcr
    }
    #[doc = "0xa00 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch8_dbg_ctdreq(&self) -> &CH8_DBG_CTDREQ {
        &self.ch8_dbg_ctdreq
    }
    #[doc = "0xa04 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch8_dbg_tcr(&self) -> &CH8_DBG_TCR {
        &self.ch8_dbg_tcr
    }
    #[doc = "0xa40 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch9_dbg_ctdreq(&self) -> &CH9_DBG_CTDREQ {
        &self.ch9_dbg_ctdreq
    }
    #[doc = "0xa44 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch9_dbg_tcr(&self) -> &CH9_DBG_TCR {
        &self.ch9_dbg_tcr
    }
    #[doc = "0xa80 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch10_dbg_ctdreq(&self) -> &CH10_DBG_CTDREQ {
        &self.ch10_dbg_ctdreq
    }
    #[doc = "0xa84 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch10_dbg_tcr(&self) -> &CH10_DBG_TCR {
        &self.ch10_dbg_tcr
    }
    #[doc = "0xac0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch11_dbg_ctdreq(&self) -> &CH11_DBG_CTDREQ {
        &self.ch11_dbg_ctdreq
    }
    #[doc = "0xac4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch11_dbg_tcr(&self) -> &CH11_DBG_TCR {
        &self.ch11_dbg_tcr
    }
}
#[doc = "Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
pub mod ch;
#[doc = "INTR (rw) register accessor: Interrupt Status (raw)  

You can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt Status (raw)"]
pub mod intr;
#[doc = "INTE0 (rw) register accessor: Interrupt Enables for IRQ 0  

You can [`read`](crate::generic::Reg::read) this register and get [`inte0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte0`]
module"]
pub type INTE0 = crate::Reg<inte0::INTE0_SPEC>;
#[doc = "Interrupt Enables for IRQ 0"]
pub mod inte0;
#[doc = "INTF0 (rw) register accessor: Force Interrupts  

You can [`read`](crate::generic::Reg::read) this register and get [`intf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf0`]
module"]
pub type INTF0 = crate::Reg<intf0::INTF0_SPEC>;
#[doc = "Force Interrupts"]
pub mod intf0;
#[doc = "INTS0 (rw) register accessor: Interrupt Status for IRQ 0  

You can [`read`](crate::generic::Reg::read) this register and get [`ints0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints0`]
module"]
pub type INTS0 = crate::Reg<ints0::INTS0_SPEC>;
#[doc = "Interrupt Status for IRQ 0"]
pub mod ints0;
#[doc = "INTR1 (rw) register accessor: Interrupt Status (raw)  

You can [`read`](crate::generic::Reg::read) this register and get [`intr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr1`]
module"]
pub type INTR1 = crate::Reg<intr1::INTR1_SPEC>;
#[doc = "Interrupt Status (raw)"]
pub mod intr1;
#[doc = "INTE1 (rw) register accessor: Interrupt Enables for IRQ 1  

You can [`read`](crate::generic::Reg::read) this register and get [`inte1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte1`]
module"]
pub type INTE1 = crate::Reg<inte1::INTE1_SPEC>;
#[doc = "Interrupt Enables for IRQ 1"]
pub mod inte1;
#[doc = "INTF1 (rw) register accessor: Force Interrupts for IRQ 1  

You can [`read`](crate::generic::Reg::read) this register and get [`intf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf1`]
module"]
pub type INTF1 = crate::Reg<intf1::INTF1_SPEC>;
#[doc = "Force Interrupts for IRQ 1"]
pub mod intf1;
#[doc = "INTS1 (rw) register accessor: Interrupt Status (masked) for IRQ 1  

You can [`read`](crate::generic::Reg::read) this register and get [`ints1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints1`]
module"]
pub type INTS1 = crate::Reg<ints1::INTS1_SPEC>;
#[doc = "Interrupt Status (masked) for IRQ 1"]
pub mod ints1;
#[doc = "TIMER0 (rw) register accessor: Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::generic::Reg::read) this register and get [`timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer0`]
module"]
pub type TIMER0 = crate::Reg<timer0::TIMER0_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer0;
#[doc = "TIMER1 (rw) register accessor: Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer1`]
module"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer2`]
module"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer3`]
module"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer3;
#[doc = "MULTI_CHAN_TRIGGER (rw) register accessor: Trigger one or more channels simultaneously  

You can [`read`](crate::generic::Reg::read) this register and get [`multi_chan_trigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`multi_chan_trigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@multi_chan_trigger`]
module"]
pub type MULTI_CHAN_TRIGGER = crate::Reg<multi_chan_trigger::MULTI_CHAN_TRIGGER_SPEC>;
#[doc = "Trigger one or more channels simultaneously"]
pub mod multi_chan_trigger;
#[doc = "SNIFF_CTRL (rw) register accessor: Sniffer Control  

You can [`read`](crate::generic::Reg::read) this register and get [`sniff_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sniff_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sniff_ctrl`]
module"]
pub type SNIFF_CTRL = crate::Reg<sniff_ctrl::SNIFF_CTRL_SPEC>;
#[doc = "Sniffer Control"]
pub mod sniff_ctrl;
#[doc = "SNIFF_DATA (rw) register accessor: Data accumulator for sniff hardware  

You can [`read`](crate::generic::Reg::read) this register and get [`sniff_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sniff_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sniff_data`]
module"]
pub type SNIFF_DATA = crate::Reg<sniff_data::SNIFF_DATA_SPEC>;
#[doc = "Data accumulator for sniff hardware"]
pub mod sniff_data;
#[doc = "FIFO_LEVELS (rw) register accessor: Debug RAF, WAF, TDF levels  

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_levels::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_levels::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo_levels`]
module"]
pub type FIFO_LEVELS = crate::Reg<fifo_levels::FIFO_LEVELS_SPEC>;
#[doc = "Debug RAF, WAF, TDF levels"]
pub mod fifo_levels;
#[doc = "CHAN_ABORT (rw) register accessor: Abort an in-progress transfer sequence on one or more channels  

You can [`read`](crate::generic::Reg::read) this register and get [`chan_abort::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chan_abort::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chan_abort`]
module"]
pub type CHAN_ABORT = crate::Reg<chan_abort::CHAN_ABORT_SPEC>;
#[doc = "Abort an in-progress transfer sequence on one or more channels"]
pub mod chan_abort;
#[doc = "N_CHANNELS (rw) register accessor: The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area.  

You can [`read`](crate::generic::Reg::read) this register and get [`n_channels::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_channels::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@n_channels`]
module"]
pub type N_CHANNELS = crate::Reg<n_channels::N_CHANNELS_SPEC>;
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
pub mod n_channels;
#[doc = "CH0_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch0_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch0_dbg_ctdreq`]
module"]
pub type CH0_DBG_CTDREQ = crate::Reg<ch0_dbg_ctdreq::CH0_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch0_dbg_ctdreq;
#[doc = "CH0_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch0_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch0_dbg_tcr`]
module"]
pub type CH0_DBG_TCR = crate::Reg<ch0_dbg_tcr::CH0_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch0_dbg_tcr;
#[doc = "CH1_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch1_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch1_dbg_ctdreq`]
module"]
pub type CH1_DBG_CTDREQ = crate::Reg<ch1_dbg_ctdreq::CH1_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch1_dbg_ctdreq;
#[doc = "CH1_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch1_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch1_dbg_tcr`]
module"]
pub type CH1_DBG_TCR = crate::Reg<ch1_dbg_tcr::CH1_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch1_dbg_tcr;
#[doc = "CH2_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch2_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch2_dbg_ctdreq`]
module"]
pub type CH2_DBG_CTDREQ = crate::Reg<ch2_dbg_ctdreq::CH2_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch2_dbg_ctdreq;
#[doc = "CH2_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch2_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch2_dbg_tcr`]
module"]
pub type CH2_DBG_TCR = crate::Reg<ch2_dbg_tcr::CH2_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch2_dbg_tcr;
#[doc = "CH3_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch3_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch3_dbg_ctdreq`]
module"]
pub type CH3_DBG_CTDREQ = crate::Reg<ch3_dbg_ctdreq::CH3_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch3_dbg_ctdreq;
#[doc = "CH3_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch3_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch3_dbg_tcr`]
module"]
pub type CH3_DBG_TCR = crate::Reg<ch3_dbg_tcr::CH3_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch3_dbg_tcr;
#[doc = "CH4_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch4_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch4_dbg_ctdreq`]
module"]
pub type CH4_DBG_CTDREQ = crate::Reg<ch4_dbg_ctdreq::CH4_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch4_dbg_ctdreq;
#[doc = "CH4_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch4_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch4_dbg_tcr`]
module"]
pub type CH4_DBG_TCR = crate::Reg<ch4_dbg_tcr::CH4_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch4_dbg_tcr;
#[doc = "CH5_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch5_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch5_dbg_ctdreq`]
module"]
pub type CH5_DBG_CTDREQ = crate::Reg<ch5_dbg_ctdreq::CH5_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch5_dbg_ctdreq;
#[doc = "CH5_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch5_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch5_dbg_tcr`]
module"]
pub type CH5_DBG_TCR = crate::Reg<ch5_dbg_tcr::CH5_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch5_dbg_tcr;
#[doc = "CH6_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch6_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch6_dbg_ctdreq`]
module"]
pub type CH6_DBG_CTDREQ = crate::Reg<ch6_dbg_ctdreq::CH6_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch6_dbg_ctdreq;
#[doc = "CH6_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch6_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch6_dbg_tcr`]
module"]
pub type CH6_DBG_TCR = crate::Reg<ch6_dbg_tcr::CH6_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch6_dbg_tcr;
#[doc = "CH7_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch7_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch7_dbg_ctdreq`]
module"]
pub type CH7_DBG_CTDREQ = crate::Reg<ch7_dbg_ctdreq::CH7_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch7_dbg_ctdreq;
#[doc = "CH7_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch7_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch7_dbg_tcr`]
module"]
pub type CH7_DBG_TCR = crate::Reg<ch7_dbg_tcr::CH7_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch7_dbg_tcr;
#[doc = "CH8_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch8_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch8_dbg_ctdreq`]
module"]
pub type CH8_DBG_CTDREQ = crate::Reg<ch8_dbg_ctdreq::CH8_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch8_dbg_ctdreq;
#[doc = "CH8_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch8_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch8_dbg_tcr`]
module"]
pub type CH8_DBG_TCR = crate::Reg<ch8_dbg_tcr::CH8_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch8_dbg_tcr;
#[doc = "CH9_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch9_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch9_dbg_ctdreq`]
module"]
pub type CH9_DBG_CTDREQ = crate::Reg<ch9_dbg_ctdreq::CH9_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch9_dbg_ctdreq;
#[doc = "CH9_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch9_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch9_dbg_tcr`]
module"]
pub type CH9_DBG_TCR = crate::Reg<ch9_dbg_tcr::CH9_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch9_dbg_tcr;
#[doc = "CH10_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch10_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch10_dbg_ctdreq`]
module"]
pub type CH10_DBG_CTDREQ = crate::Reg<ch10_dbg_ctdreq::CH10_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch10_dbg_ctdreq;
#[doc = "CH10_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch10_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch10_dbg_tcr`]
module"]
pub type CH10_DBG_TCR = crate::Reg<ch10_dbg_tcr::CH10_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch10_dbg_tcr;
#[doc = "CH11_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch11_dbg_ctdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_dbg_ctdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch11_dbg_ctdreq`]
module"]
pub type CH11_DBG_CTDREQ = crate::Reg<ch11_dbg_ctdreq::CH11_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch11_dbg_ctdreq;
#[doc = "CH11_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::generic::Reg::read) this register and get [`ch11_dbg_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_dbg_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch11_dbg_tcr`]
module"]
pub type CH11_DBG_TCR = crate::Reg<ch11_dbg_tcr::CH11_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch11_dbg_tcr;
