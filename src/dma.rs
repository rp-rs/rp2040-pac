#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x300 - Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
    pub ch: [CH; 12],
    _reserved1: [u8; 0x0100],
    #[doc = "0x400 - Interrupt Status (raw)"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x404 - Interrupt Enables for IRQ 0"]
    pub inte0: crate::Reg<inte0::INTE0_SPEC>,
    #[doc = "0x408 - Force Interrupts"]
    pub intf0: crate::Reg<intf0::INTF0_SPEC>,
    #[doc = "0x40c - Interrupt Status for IRQ 0"]
    pub ints0: crate::Reg<ints0::INTS0_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x414 - Interrupt Enables for IRQ 1"]
    pub inte1: crate::Reg<inte1::INTE1_SPEC>,
    #[doc = "0x418 - Force Interrupts for IRQ 1"]
    pub intf1: crate::Reg<intf1::INTF1_SPEC>,
    #[doc = "0x41c - Interrupt Status (masked) for IRQ 1"]
    pub ints1: crate::Reg<ints1::INTS1_SPEC>,
    #[doc = "0x420 - Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer0: crate::Reg<timer0::TIMER0_SPEC>,
    #[doc = "0x424 - Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer1: crate::Reg<timer1::TIMER1_SPEC>,
    #[doc = "0x428 - Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer2: crate::Reg<timer2::TIMER2_SPEC>,
    #[doc = "0x42c - Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer3: crate::Reg<timer3::TIMER3_SPEC>,
    #[doc = "0x430 - Trigger one or more channels simultaneously"]
    pub multi_chan_trigger: crate::Reg<multi_chan_trigger::MULTI_CHAN_TRIGGER_SPEC>,
    #[doc = "0x434 - Sniffer Control"]
    pub sniff_ctrl: crate::Reg<sniff_ctrl::SNIFF_CTRL_SPEC>,
    #[doc = "0x438 - Data accumulator for sniff hardware  
 Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    pub sniff_data: crate::Reg<sniff_data::SNIFF_DATA_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x440 - Debug RAF, WAF, TDF levels"]
    pub fifo_levels: crate::Reg<fifo_levels::FIFO_LEVELS_SPEC>,
    #[doc = "0x444 - Abort an in-progress transfer sequence on one or more channels"]
    pub chan_abort: crate::Reg<chan_abort::CHAN_ABORT_SPEC>,
    #[doc = "0x448 - The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    pub n_channels: crate::Reg<n_channels::N_CHANNELS_SPEC>,
    _reserved18: [u8; 0x03b4],
    #[doc = "0x800 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch0_dbg_ctdreq: crate::Reg<ch0_dbg_ctdreq::CH0_DBG_CTDREQ_SPEC>,
    #[doc = "0x804 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch0_dbg_tcr: crate::Reg<ch0_dbg_tcr::CH0_DBG_TCR_SPEC>,
    _reserved20: [u8; 0x38],
    #[doc = "0x840 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch1_dbg_ctdreq: crate::Reg<ch1_dbg_ctdreq::CH1_DBG_CTDREQ_SPEC>,
    #[doc = "0x844 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch1_dbg_tcr: crate::Reg<ch1_dbg_tcr::CH1_DBG_TCR_SPEC>,
    _reserved22: [u8; 0x38],
    #[doc = "0x880 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch2_dbg_ctdreq: crate::Reg<ch2_dbg_ctdreq::CH2_DBG_CTDREQ_SPEC>,
    #[doc = "0x884 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch2_dbg_tcr: crate::Reg<ch2_dbg_tcr::CH2_DBG_TCR_SPEC>,
    _reserved24: [u8; 0x38],
    #[doc = "0x8c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch3_dbg_ctdreq: crate::Reg<ch3_dbg_ctdreq::CH3_DBG_CTDREQ_SPEC>,
    #[doc = "0x8c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch3_dbg_tcr: crate::Reg<ch3_dbg_tcr::CH3_DBG_TCR_SPEC>,
    _reserved26: [u8; 0x38],
    #[doc = "0x900 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch4_dbg_ctdreq: crate::Reg<ch4_dbg_ctdreq::CH4_DBG_CTDREQ_SPEC>,
    #[doc = "0x904 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch4_dbg_tcr: crate::Reg<ch4_dbg_tcr::CH4_DBG_TCR_SPEC>,
    _reserved28: [u8; 0x38],
    #[doc = "0x940 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch5_dbg_ctdreq: crate::Reg<ch5_dbg_ctdreq::CH5_DBG_CTDREQ_SPEC>,
    #[doc = "0x944 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch5_dbg_tcr: crate::Reg<ch5_dbg_tcr::CH5_DBG_TCR_SPEC>,
    _reserved30: [u8; 0x38],
    #[doc = "0x980 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch6_dbg_ctdreq: crate::Reg<ch6_dbg_ctdreq::CH6_DBG_CTDREQ_SPEC>,
    #[doc = "0x984 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch6_dbg_tcr: crate::Reg<ch6_dbg_tcr::CH6_DBG_TCR_SPEC>,
    _reserved32: [u8; 0x38],
    #[doc = "0x9c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch7_dbg_ctdreq: crate::Reg<ch7_dbg_ctdreq::CH7_DBG_CTDREQ_SPEC>,
    #[doc = "0x9c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch7_dbg_tcr: crate::Reg<ch7_dbg_tcr::CH7_DBG_TCR_SPEC>,
    _reserved34: [u8; 0x38],
    #[doc = "0xa00 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch8_dbg_ctdreq: crate::Reg<ch8_dbg_ctdreq::CH8_DBG_CTDREQ_SPEC>,
    #[doc = "0xa04 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch8_dbg_tcr: crate::Reg<ch8_dbg_tcr::CH8_DBG_TCR_SPEC>,
    _reserved36: [u8; 0x38],
    #[doc = "0xa40 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch9_dbg_ctdreq: crate::Reg<ch9_dbg_ctdreq::CH9_DBG_CTDREQ_SPEC>,
    #[doc = "0xa44 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch9_dbg_tcr: crate::Reg<ch9_dbg_tcr::CH9_DBG_TCR_SPEC>,
    _reserved38: [u8; 0x38],
    #[doc = "0xa80 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch10_dbg_ctdreq: crate::Reg<ch10_dbg_ctdreq::CH10_DBG_CTDREQ_SPEC>,
    #[doc = "0xa84 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch10_dbg_tcr: crate::Reg<ch10_dbg_tcr::CH10_DBG_TCR_SPEC>,
    _reserved40: [u8; 0x38],
    #[doc = "0xac0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch11_dbg_ctdreq: crate::Reg<ch11_dbg_ctdreq::CH11_DBG_CTDREQ_SPEC>,
    #[doc = "0xac4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch11_dbg_tcr: crate::Reg<ch11_dbg_tcr::CH11_DBG_TCR_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA Channel 0 Read Address pointer  
 This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch_read_addr: crate::Reg<self::ch::ch_read_addr::CH_READ_ADDR_SPEC>,
    #[doc = "0x04 - DMA Channel 0 Write Address pointer  
 This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch_write_addr: crate::Reg<self::ch::ch_write_addr::CH_WRITE_ADDR_SPEC>,
    #[doc = "0x08 - DMA Channel 0 Transfer Count  
 Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).  

 When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.  

 Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.  

 The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch_trans_count: crate::Reg<self::ch::ch_trans_count::CH_TRANS_COUNT_SPEC>,
    #[doc = "0x0c - DMA Channel 0 Control and Status"]
    pub ch_ctrl_trig: crate::Reg<self::ch::ch_ctrl_trig::CH_CTRL_TRIG_SPEC>,
    #[doc = "0x10 - DMA Channel 0 Control and Status"]
    pub ch_al1_ctrl: crate::Reg<self::ch::ch_al1_ctrl::CH_AL1_CTRL_SPEC>,
    #[doc = "0x14 - Alias for channel 0 READ_ADDR register"]
    pub ch_al1_read_addr: crate::Reg<self::ch::ch_al1_read_addr::CH_AL1_READ_ADDR_SPEC>,
    #[doc = "0x18 - Alias for channel 0 WRITE_ADDR register"]
    pub ch_al1_write_addr: crate::Reg<self::ch::ch_al1_write_addr::CH_AL1_WRITE_ADDR_SPEC>,
    #[doc = "0x1c - Alias for channel 0 TRANS_COUNT register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
    pub ch_al1_trans_count_trig:
        crate::Reg<self::ch::ch_al1_trans_count_trig::CH_AL1_TRANS_COUNT_TRIG_SPEC>,
    #[doc = "0x20 - DMA Channel 0 Control and Status"]
    pub ch_al2_ctrl: crate::Reg<self::ch::ch_al2_ctrl::CH_AL2_CTRL_SPEC>,
    #[doc = "0x24 - Alias for channel 0 TRANS_COUNT register"]
    pub ch_al2_trans_count: crate::Reg<self::ch::ch_al2_trans_count::CH_AL2_TRANS_COUNT_SPEC>,
    #[doc = "0x28 - Alias for channel 0 READ_ADDR register"]
    pub ch_al2_read_addr: crate::Reg<self::ch::ch_al2_read_addr::CH_AL2_READ_ADDR_SPEC>,
    #[doc = "0x2c - Alias for channel 0 WRITE_ADDR register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
    pub ch_al2_write_addr_trig:
        crate::Reg<self::ch::ch_al2_write_addr_trig::CH_AL2_WRITE_ADDR_TRIG_SPEC>,
    #[doc = "0x30 - DMA Channel 0 Control and Status"]
    pub ch_al3_ctrl: crate::Reg<self::ch::ch_al3_ctrl::CH_AL3_CTRL_SPEC>,
    #[doc = "0x34 - Alias for channel 0 WRITE_ADDR register"]
    pub ch_al3_write_addr: crate::Reg<self::ch::ch_al3_write_addr::CH_AL3_WRITE_ADDR_SPEC>,
    #[doc = "0x38 - Alias for channel 0 TRANS_COUNT register"]
    pub ch_al3_trans_count: crate::Reg<self::ch::ch_al3_trans_count::CH_AL3_TRANS_COUNT_SPEC>,
    #[doc = "0x3c - Alias for channel 0 READ_ADDR register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel."]
    pub ch_al3_read_addr_trig:
        crate::Reg<self::ch::ch_al3_read_addr_trig::CH_AL3_READ_ADDR_TRIG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
pub mod ch;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt Status (raw)"]
pub mod intr;
#[doc = "INTE0 register accessor: an alias for `Reg<INTE0_SPEC>`"]
pub type INTE0 = crate::Reg<inte0::INTE0_SPEC>;
#[doc = "Interrupt Enables for IRQ 0"]
pub mod inte0;
#[doc = "INTF0 register accessor: an alias for `Reg<INTF0_SPEC>`"]
pub type INTF0 = crate::Reg<intf0::INTF0_SPEC>;
#[doc = "Force Interrupts"]
pub mod intf0;
#[doc = "INTS0 register accessor: an alias for `Reg<INTS0_SPEC>`"]
pub type INTS0 = crate::Reg<ints0::INTS0_SPEC>;
#[doc = "Interrupt Status for IRQ 0"]
pub mod ints0;
#[doc = "INTE1 register accessor: an alias for `Reg<INTE1_SPEC>`"]
pub type INTE1 = crate::Reg<inte1::INTE1_SPEC>;
#[doc = "Interrupt Enables for IRQ 1"]
pub mod inte1;
#[doc = "INTF1 register accessor: an alias for `Reg<INTF1_SPEC>`"]
pub type INTF1 = crate::Reg<intf1::INTF1_SPEC>;
#[doc = "Force Interrupts for IRQ 1"]
pub mod intf1;
#[doc = "INTS1 register accessor: an alias for `Reg<INTS1_SPEC>`"]
pub type INTS1 = crate::Reg<ints1::INTS1_SPEC>;
#[doc = "Interrupt Status (masked) for IRQ 1"]
pub mod ints1;
#[doc = "TIMER0 register accessor: an alias for `Reg<TIMER0_SPEC>`"]
pub type TIMER0 = crate::Reg<timer0::TIMER0_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer0;
#[doc = "TIMER1 register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer1;
#[doc = "TIMER2 register accessor: an alias for `Reg<TIMER2_SPEC>`"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer2;
#[doc = "TIMER3 register accessor: an alias for `Reg<TIMER3_SPEC>`"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer3;
#[doc = "MULTI_CHAN_TRIGGER register accessor: an alias for `Reg<MULTI_CHAN_TRIGGER_SPEC>`"]
pub type MULTI_CHAN_TRIGGER = crate::Reg<multi_chan_trigger::MULTI_CHAN_TRIGGER_SPEC>;
#[doc = "Trigger one or more channels simultaneously"]
pub mod multi_chan_trigger;
#[doc = "SNIFF_CTRL register accessor: an alias for `Reg<SNIFF_CTRL_SPEC>`"]
pub type SNIFF_CTRL = crate::Reg<sniff_ctrl::SNIFF_CTRL_SPEC>;
#[doc = "Sniffer Control"]
pub mod sniff_ctrl;
#[doc = "SNIFF_DATA register accessor: an alias for `Reg<SNIFF_DATA_SPEC>`"]
pub type SNIFF_DATA = crate::Reg<sniff_data::SNIFF_DATA_SPEC>;
#[doc = "Data accumulator for sniff hardware  
 Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
pub mod sniff_data;
#[doc = "FIFO_LEVELS register accessor: an alias for `Reg<FIFO_LEVELS_SPEC>`"]
pub type FIFO_LEVELS = crate::Reg<fifo_levels::FIFO_LEVELS_SPEC>;
#[doc = "Debug RAF, WAF, TDF levels"]
pub mod fifo_levels;
#[doc = "CHAN_ABORT register accessor: an alias for `Reg<CHAN_ABORT_SPEC>`"]
pub type CHAN_ABORT = crate::Reg<chan_abort::CHAN_ABORT_SPEC>;
#[doc = "Abort an in-progress transfer sequence on one or more channels"]
pub mod chan_abort;
#[doc = "N_CHANNELS register accessor: an alias for `Reg<N_CHANNELS_SPEC>`"]
pub type N_CHANNELS = crate::Reg<n_channels::N_CHANNELS_SPEC>;
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
pub mod n_channels;
#[doc = "CH0_DBG_CTDREQ register accessor: an alias for `Reg<CH0_DBG_CTDREQ_SPEC>`"]
pub type CH0_DBG_CTDREQ = crate::Reg<ch0_dbg_ctdreq::CH0_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch0_dbg_ctdreq;
#[doc = "CH0_DBG_TCR register accessor: an alias for `Reg<CH0_DBG_TCR_SPEC>`"]
pub type CH0_DBG_TCR = crate::Reg<ch0_dbg_tcr::CH0_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch0_dbg_tcr;
#[doc = "CH1_DBG_CTDREQ register accessor: an alias for `Reg<CH1_DBG_CTDREQ_SPEC>`"]
pub type CH1_DBG_CTDREQ = crate::Reg<ch1_dbg_ctdreq::CH1_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch1_dbg_ctdreq;
#[doc = "CH1_DBG_TCR register accessor: an alias for `Reg<CH1_DBG_TCR_SPEC>`"]
pub type CH1_DBG_TCR = crate::Reg<ch1_dbg_tcr::CH1_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch1_dbg_tcr;
#[doc = "CH2_DBG_CTDREQ register accessor: an alias for `Reg<CH2_DBG_CTDREQ_SPEC>`"]
pub type CH2_DBG_CTDREQ = crate::Reg<ch2_dbg_ctdreq::CH2_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch2_dbg_ctdreq;
#[doc = "CH2_DBG_TCR register accessor: an alias for `Reg<CH2_DBG_TCR_SPEC>`"]
pub type CH2_DBG_TCR = crate::Reg<ch2_dbg_tcr::CH2_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch2_dbg_tcr;
#[doc = "CH3_DBG_CTDREQ register accessor: an alias for `Reg<CH3_DBG_CTDREQ_SPEC>`"]
pub type CH3_DBG_CTDREQ = crate::Reg<ch3_dbg_ctdreq::CH3_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch3_dbg_ctdreq;
#[doc = "CH3_DBG_TCR register accessor: an alias for `Reg<CH3_DBG_TCR_SPEC>`"]
pub type CH3_DBG_TCR = crate::Reg<ch3_dbg_tcr::CH3_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch3_dbg_tcr;
#[doc = "CH4_DBG_CTDREQ register accessor: an alias for `Reg<CH4_DBG_CTDREQ_SPEC>`"]
pub type CH4_DBG_CTDREQ = crate::Reg<ch4_dbg_ctdreq::CH4_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch4_dbg_ctdreq;
#[doc = "CH4_DBG_TCR register accessor: an alias for `Reg<CH4_DBG_TCR_SPEC>`"]
pub type CH4_DBG_TCR = crate::Reg<ch4_dbg_tcr::CH4_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch4_dbg_tcr;
#[doc = "CH5_DBG_CTDREQ register accessor: an alias for `Reg<CH5_DBG_CTDREQ_SPEC>`"]
pub type CH5_DBG_CTDREQ = crate::Reg<ch5_dbg_ctdreq::CH5_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch5_dbg_ctdreq;
#[doc = "CH5_DBG_TCR register accessor: an alias for `Reg<CH5_DBG_TCR_SPEC>`"]
pub type CH5_DBG_TCR = crate::Reg<ch5_dbg_tcr::CH5_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch5_dbg_tcr;
#[doc = "CH6_DBG_CTDREQ register accessor: an alias for `Reg<CH6_DBG_CTDREQ_SPEC>`"]
pub type CH6_DBG_CTDREQ = crate::Reg<ch6_dbg_ctdreq::CH6_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch6_dbg_ctdreq;
#[doc = "CH6_DBG_TCR register accessor: an alias for `Reg<CH6_DBG_TCR_SPEC>`"]
pub type CH6_DBG_TCR = crate::Reg<ch6_dbg_tcr::CH6_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch6_dbg_tcr;
#[doc = "CH7_DBG_CTDREQ register accessor: an alias for `Reg<CH7_DBG_CTDREQ_SPEC>`"]
pub type CH7_DBG_CTDREQ = crate::Reg<ch7_dbg_ctdreq::CH7_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch7_dbg_ctdreq;
#[doc = "CH7_DBG_TCR register accessor: an alias for `Reg<CH7_DBG_TCR_SPEC>`"]
pub type CH7_DBG_TCR = crate::Reg<ch7_dbg_tcr::CH7_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch7_dbg_tcr;
#[doc = "CH8_DBG_CTDREQ register accessor: an alias for `Reg<CH8_DBG_CTDREQ_SPEC>`"]
pub type CH8_DBG_CTDREQ = crate::Reg<ch8_dbg_ctdreq::CH8_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch8_dbg_ctdreq;
#[doc = "CH8_DBG_TCR register accessor: an alias for `Reg<CH8_DBG_TCR_SPEC>`"]
pub type CH8_DBG_TCR = crate::Reg<ch8_dbg_tcr::CH8_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch8_dbg_tcr;
#[doc = "CH9_DBG_CTDREQ register accessor: an alias for `Reg<CH9_DBG_CTDREQ_SPEC>`"]
pub type CH9_DBG_CTDREQ = crate::Reg<ch9_dbg_ctdreq::CH9_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch9_dbg_ctdreq;
#[doc = "CH9_DBG_TCR register accessor: an alias for `Reg<CH9_DBG_TCR_SPEC>`"]
pub type CH9_DBG_TCR = crate::Reg<ch9_dbg_tcr::CH9_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch9_dbg_tcr;
#[doc = "CH10_DBG_CTDREQ register accessor: an alias for `Reg<CH10_DBG_CTDREQ_SPEC>`"]
pub type CH10_DBG_CTDREQ = crate::Reg<ch10_dbg_ctdreq::CH10_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch10_dbg_ctdreq;
#[doc = "CH10_DBG_TCR register accessor: an alias for `Reg<CH10_DBG_TCR_SPEC>`"]
pub type CH10_DBG_TCR = crate::Reg<ch10_dbg_tcr::CH10_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch10_dbg_tcr;
#[doc = "CH11_DBG_CTDREQ register accessor: an alias for `Reg<CH11_DBG_CTDREQ_SPEC>`"]
pub type CH11_DBG_CTDREQ = crate::Reg<ch11_dbg_ctdreq::CH11_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch11_dbg_ctdreq;
#[doc = "CH11_DBG_TCR register accessor: an alias for `Reg<CH11_DBG_TCR_SPEC>`"]
pub type CH11_DBG_TCR = crate::Reg<ch11_dbg_tcr::CH11_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch11_dbg_tcr;
