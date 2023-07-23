#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA Channel 0 Read Address pointer  
 This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch_read_addr: CH_READ_ADDR,
    #[doc = "0x04 - DMA Channel 0 Write Address pointer  
 This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch_write_addr: CH_WRITE_ADDR,
    #[doc = "0x08 - DMA Channel 0 Transfer Count  
 Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).  

 When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.  

 Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.  

 The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch_trans_count: CH_TRANS_COUNT,
    #[doc = "0x0c - DMA Channel 0 Control and Status"]
    pub ch_ctrl_trig: CH_CTRL_TRIG,
    #[doc = "0x10 - DMA Channel 0 Control and Status"]
    pub ch_al1_ctrl: CH_AL1_CTRL,
    #[doc = "0x14 - Alias for channel 0 READ_ADDR register"]
    pub ch_al1_read_addr: CH_AL1_READ_ADDR,
    #[doc = "0x18 - Alias for channel 0 WRITE_ADDR register"]
    pub ch_al1_write_addr: CH_AL1_WRITE_ADDR,
    #[doc = "0x1c - Alias for channel 0 TRANS_COUNT register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
    pub ch_al1_trans_count_trig: CH_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x20 - DMA Channel 0 Control and Status"]
    pub ch_al2_ctrl: CH_AL2_CTRL,
    #[doc = "0x24 - Alias for channel 0 TRANS_COUNT register"]
    pub ch_al2_trans_count: CH_AL2_TRANS_COUNT,
    #[doc = "0x28 - Alias for channel 0 READ_ADDR register"]
    pub ch_al2_read_addr: CH_AL2_READ_ADDR,
    #[doc = "0x2c - Alias for channel 0 WRITE_ADDR register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
    pub ch_al2_write_addr_trig: CH_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x30 - DMA Channel 0 Control and Status"]
    pub ch_al3_ctrl: CH_AL3_CTRL,
    #[doc = "0x34 - Alias for channel 0 WRITE_ADDR register"]
    pub ch_al3_write_addr: CH_AL3_WRITE_ADDR,
    #[doc = "0x38 - Alias for channel 0 TRANS_COUNT register"]
    pub ch_al3_trans_count: CH_AL3_TRANS_COUNT,
    #[doc = "0x3c - Alias for channel 0 READ_ADDR register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
    pub ch_al3_read_addr_trig: CH_AL3_READ_ADDR_TRIG,
}
#[doc = "CH_READ_ADDR (rw) register accessor: an alias for `Reg<CH_READ_ADDR_SPEC>`"]
pub type CH_READ_ADDR = crate::Reg<ch_read_addr::CH_READ_ADDR_SPEC>;
#[doc = "DMA Channel 0 Read Address pointer  
 This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch_read_addr;
#[doc = "CH_WRITE_ADDR (rw) register accessor: an alias for `Reg<CH_WRITE_ADDR_SPEC>`"]
pub type CH_WRITE_ADDR = crate::Reg<ch_write_addr::CH_WRITE_ADDR_SPEC>;
#[doc = "DMA Channel 0 Write Address pointer  
 This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch_write_addr;
#[doc = "CH_TRANS_COUNT (rw) register accessor: an alias for `Reg<CH_TRANS_COUNT_SPEC>`"]
pub type CH_TRANS_COUNT = crate::Reg<ch_trans_count::CH_TRANS_COUNT_SPEC>;
#[doc = "DMA Channel 0 Transfer Count  
 Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).  

 When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.  

 Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.  

 The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch_trans_count;
#[doc = "CH_CTRL_TRIG (rw) register accessor: an alias for `Reg<CH_CTRL_TRIG_SPEC>`"]
pub type CH_CTRL_TRIG = crate::Reg<ch_ctrl_trig::CH_CTRL_TRIG_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_ctrl_trig;
#[doc = "CH_AL1_CTRL (rw) register accessor: an alias for `Reg<CH_AL1_CTRL_SPEC>`"]
pub type CH_AL1_CTRL = crate::Reg<ch_al1_ctrl::CH_AL1_CTRL_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_al1_ctrl;
#[doc = "CH_AL1_READ_ADDR (rw) register accessor: an alias for `Reg<CH_AL1_READ_ADDR_SPEC>`"]
pub type CH_AL1_READ_ADDR = crate::Reg<ch_al1_read_addr::CH_AL1_READ_ADDR_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch_al1_read_addr;
#[doc = "CH_AL1_WRITE_ADDR (rw) register accessor: an alias for `Reg<CH_AL1_WRITE_ADDR_SPEC>`"]
pub type CH_AL1_WRITE_ADDR = crate::Reg<ch_al1_write_addr::CH_AL1_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch_al1_write_addr;
#[doc = "CH_AL1_TRANS_COUNT_TRIG (rw) register accessor: an alias for `Reg<CH_AL1_TRANS_COUNT_TRIG_SPEC>`"]
pub type CH_AL1_TRANS_COUNT_TRIG =
    crate::Reg<ch_al1_trans_count_trig::CH_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
pub mod ch_al1_trans_count_trig;
#[doc = "CH_AL2_CTRL (rw) register accessor: an alias for `Reg<CH_AL2_CTRL_SPEC>`"]
pub type CH_AL2_CTRL = crate::Reg<ch_al2_ctrl::CH_AL2_CTRL_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_al2_ctrl;
#[doc = "CH_AL2_TRANS_COUNT (rw) register accessor: an alias for `Reg<CH_AL2_TRANS_COUNT_SPEC>`"]
pub type CH_AL2_TRANS_COUNT = crate::Reg<ch_al2_trans_count::CH_AL2_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch_al2_trans_count;
#[doc = "CH_AL2_READ_ADDR (rw) register accessor: an alias for `Reg<CH_AL2_READ_ADDR_SPEC>`"]
pub type CH_AL2_READ_ADDR = crate::Reg<ch_al2_read_addr::CH_AL2_READ_ADDR_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch_al2_read_addr;
#[doc = "CH_AL2_WRITE_ADDR_TRIG (rw) register accessor: an alias for `Reg<CH_AL2_WRITE_ADDR_TRIG_SPEC>`"]
pub type CH_AL2_WRITE_ADDR_TRIG = crate::Reg<ch_al2_write_addr_trig::CH_AL2_WRITE_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
pub mod ch_al2_write_addr_trig;
#[doc = "CH_AL3_CTRL (rw) register accessor: an alias for `Reg<CH_AL3_CTRL_SPEC>`"]
pub type CH_AL3_CTRL = crate::Reg<ch_al3_ctrl::CH_AL3_CTRL_SPEC>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_al3_ctrl;
#[doc = "CH_AL3_WRITE_ADDR (rw) register accessor: an alias for `Reg<CH_AL3_WRITE_ADDR_SPEC>`"]
pub type CH_AL3_WRITE_ADDR = crate::Reg<ch_al3_write_addr::CH_AL3_WRITE_ADDR_SPEC>;
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch_al3_write_addr;
#[doc = "CH_AL3_TRANS_COUNT (rw) register accessor: an alias for `Reg<CH_AL3_TRANS_COUNT_SPEC>`"]
pub type CH_AL3_TRANS_COUNT = crate::Reg<ch_al3_trans_count::CH_AL3_TRANS_COUNT_SPEC>;
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch_al3_trans_count;
#[doc = "CH_AL3_READ_ADDR_TRIG (rw) register accessor: an alias for `Reg<CH_AL3_READ_ADDR_TRIG_SPEC>`"]
pub type CH_AL3_READ_ADDR_TRIG = crate::Reg<ch_al3_read_addr_trig::CH_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Alias for channel 0 READ_ADDR register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
pub mod ch_al3_read_addr_trig;
