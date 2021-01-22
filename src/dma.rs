#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Channel 0 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch0_read_addr: CH0_READ_ADDR,
    #[doc = "0x04 - DMA Channel 0 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch0_write_addr: CH0_WRITE_ADDR,
    #[doc = "0x08 - DMA Channel 0 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch0_trans_count: CH0_TRANS_COUNT,
    #[doc = "0x0c - DMA Channel 0 Control and Status"]
    pub ch0_ctrl_trig: CH0_CTRL_TRIG,
    #[doc = "0x10 - Alias for channel 0 CTRL register"]
    pub ch0_al1_ctrl: CH0_AL1_CTRL,
    #[doc = "0x14 - Alias for channel 0 READ_ADDR register"]
    pub ch0_al1_read_addr: CH0_AL1_READ_ADDR,
    #[doc = "0x18 - Alias for channel 0 WRITE_ADDR register"]
    pub ch0_al1_write_addr: CH0_AL1_WRITE_ADDR,
    #[doc = "0x1c - Alias for channel 0 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch0_al1_trans_count_trig: CH0_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x20 - Alias for channel 0 CTRL register"]
    pub ch0_al2_ctrl: CH0_AL2_CTRL,
    #[doc = "0x24 - Alias for channel 0 TRANS_COUNT register"]
    pub ch0_al2_trans_count: CH0_AL2_TRANS_COUNT,
    #[doc = "0x28 - Alias for channel 0 READ_ADDR register"]
    pub ch0_al2_read_addr: CH0_AL2_READ_ADDR,
    #[doc = "0x2c - Alias for channel 0 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch0_al2_write_addr_trig: CH0_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x30 - Alias for channel 0 CTRL register"]
    pub ch0_al3_ctrl: CH0_AL3_CTRL,
    #[doc = "0x34 - Alias for channel 0 WRITE_ADDR register"]
    pub ch0_al3_write_addr: CH0_AL3_WRITE_ADDR,
    #[doc = "0x38 - Alias for channel 0 TRANS_COUNT register"]
    pub ch0_al3_trans_count: CH0_AL3_TRANS_COUNT,
    #[doc = "0x3c - Alias for channel 0 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch0_al3_read_addr_trig: CH0_AL3_READ_ADDR_TRIG,
    #[doc = "0x40 - DMA Channel 1 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch1_read_addr: CH1_READ_ADDR,
    #[doc = "0x44 - DMA Channel 1 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch1_write_addr: CH1_WRITE_ADDR,
    #[doc = "0x48 - DMA Channel 1 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch1_trans_count: CH1_TRANS_COUNT,
    #[doc = "0x4c - DMA Channel 1 Control and Status"]
    pub ch1_ctrl_trig: CH1_CTRL_TRIG,
    #[doc = "0x50 - Alias for channel 1 CTRL register"]
    pub ch1_al1_ctrl: CH1_AL1_CTRL,
    #[doc = "0x54 - Alias for channel 1 READ_ADDR register"]
    pub ch1_al1_read_addr: CH1_AL1_READ_ADDR,
    #[doc = "0x58 - Alias for channel 1 WRITE_ADDR register"]
    pub ch1_al1_write_addr: CH1_AL1_WRITE_ADDR,
    #[doc = "0x5c - Alias for channel 1 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch1_al1_trans_count_trig: CH1_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x60 - Alias for channel 1 CTRL register"]
    pub ch1_al2_ctrl: CH1_AL2_CTRL,
    #[doc = "0x64 - Alias for channel 1 TRANS_COUNT register"]
    pub ch1_al2_trans_count: CH1_AL2_TRANS_COUNT,
    #[doc = "0x68 - Alias for channel 1 READ_ADDR register"]
    pub ch1_al2_read_addr: CH1_AL2_READ_ADDR,
    #[doc = "0x6c - Alias for channel 1 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch1_al2_write_addr_trig: CH1_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x70 - Alias for channel 1 CTRL register"]
    pub ch1_al3_ctrl: CH1_AL3_CTRL,
    #[doc = "0x74 - Alias for channel 1 WRITE_ADDR register"]
    pub ch1_al3_write_addr: CH1_AL3_WRITE_ADDR,
    #[doc = "0x78 - Alias for channel 1 TRANS_COUNT register"]
    pub ch1_al3_trans_count: CH1_AL3_TRANS_COUNT,
    #[doc = "0x7c - Alias for channel 1 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch1_al3_read_addr_trig: CH1_AL3_READ_ADDR_TRIG,
    #[doc = "0x80 - DMA Channel 2 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch2_read_addr: CH2_READ_ADDR,
    #[doc = "0x84 - DMA Channel 2 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch2_write_addr: CH2_WRITE_ADDR,
    #[doc = "0x88 - DMA Channel 2 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch2_trans_count: CH2_TRANS_COUNT,
    #[doc = "0x8c - DMA Channel 2 Control and Status"]
    pub ch2_ctrl_trig: CH2_CTRL_TRIG,
    #[doc = "0x90 - Alias for channel 2 CTRL register"]
    pub ch2_al1_ctrl: CH2_AL1_CTRL,
    #[doc = "0x94 - Alias for channel 2 READ_ADDR register"]
    pub ch2_al1_read_addr: CH2_AL1_READ_ADDR,
    #[doc = "0x98 - Alias for channel 2 WRITE_ADDR register"]
    pub ch2_al1_write_addr: CH2_AL1_WRITE_ADDR,
    #[doc = "0x9c - Alias for channel 2 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch2_al1_trans_count_trig: CH2_AL1_TRANS_COUNT_TRIG,
    #[doc = "0xa0 - Alias for channel 2 CTRL register"]
    pub ch2_al2_ctrl: CH2_AL2_CTRL,
    #[doc = "0xa4 - Alias for channel 2 TRANS_COUNT register"]
    pub ch2_al2_trans_count: CH2_AL2_TRANS_COUNT,
    #[doc = "0xa8 - Alias for channel 2 READ_ADDR register"]
    pub ch2_al2_read_addr: CH2_AL2_READ_ADDR,
    #[doc = "0xac - Alias for channel 2 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch2_al2_write_addr_trig: CH2_AL2_WRITE_ADDR_TRIG,
    #[doc = "0xb0 - Alias for channel 2 CTRL register"]
    pub ch2_al3_ctrl: CH2_AL3_CTRL,
    #[doc = "0xb4 - Alias for channel 2 WRITE_ADDR register"]
    pub ch2_al3_write_addr: CH2_AL3_WRITE_ADDR,
    #[doc = "0xb8 - Alias for channel 2 TRANS_COUNT register"]
    pub ch2_al3_trans_count: CH2_AL3_TRANS_COUNT,
    #[doc = "0xbc - Alias for channel 2 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch2_al3_read_addr_trig: CH2_AL3_READ_ADDR_TRIG,
    #[doc = "0xc0 - DMA Channel 3 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch3_read_addr: CH3_READ_ADDR,
    #[doc = "0xc4 - DMA Channel 3 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch3_write_addr: CH3_WRITE_ADDR,
    #[doc = "0xc8 - DMA Channel 3 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch3_trans_count: CH3_TRANS_COUNT,
    #[doc = "0xcc - DMA Channel 3 Control and Status"]
    pub ch3_ctrl_trig: CH3_CTRL_TRIG,
    #[doc = "0xd0 - Alias for channel 3 CTRL register"]
    pub ch3_al1_ctrl: CH3_AL1_CTRL,
    #[doc = "0xd4 - Alias for channel 3 READ_ADDR register"]
    pub ch3_al1_read_addr: CH3_AL1_READ_ADDR,
    #[doc = "0xd8 - Alias for channel 3 WRITE_ADDR register"]
    pub ch3_al1_write_addr: CH3_AL1_WRITE_ADDR,
    #[doc = "0xdc - Alias for channel 3 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch3_al1_trans_count_trig: CH3_AL1_TRANS_COUNT_TRIG,
    #[doc = "0xe0 - Alias for channel 3 CTRL register"]
    pub ch3_al2_ctrl: CH3_AL2_CTRL,
    #[doc = "0xe4 - Alias for channel 3 TRANS_COUNT register"]
    pub ch3_al2_trans_count: CH3_AL2_TRANS_COUNT,
    #[doc = "0xe8 - Alias for channel 3 READ_ADDR register"]
    pub ch3_al2_read_addr: CH3_AL2_READ_ADDR,
    #[doc = "0xec - Alias for channel 3 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch3_al2_write_addr_trig: CH3_AL2_WRITE_ADDR_TRIG,
    #[doc = "0xf0 - Alias for channel 3 CTRL register"]
    pub ch3_al3_ctrl: CH3_AL3_CTRL,
    #[doc = "0xf4 - Alias for channel 3 WRITE_ADDR register"]
    pub ch3_al3_write_addr: CH3_AL3_WRITE_ADDR,
    #[doc = "0xf8 - Alias for channel 3 TRANS_COUNT register"]
    pub ch3_al3_trans_count: CH3_AL3_TRANS_COUNT,
    #[doc = "0xfc - Alias for channel 3 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch3_al3_read_addr_trig: CH3_AL3_READ_ADDR_TRIG,
    #[doc = "0x100 - DMA Channel 4 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch4_read_addr: CH4_READ_ADDR,
    #[doc = "0x104 - DMA Channel 4 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch4_write_addr: CH4_WRITE_ADDR,
    #[doc = "0x108 - DMA Channel 4 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch4_trans_count: CH4_TRANS_COUNT,
    #[doc = "0x10c - DMA Channel 4 Control and Status"]
    pub ch4_ctrl_trig: CH4_CTRL_TRIG,
    #[doc = "0x110 - Alias for channel 4 CTRL register"]
    pub ch4_al1_ctrl: CH4_AL1_CTRL,
    #[doc = "0x114 - Alias for channel 4 READ_ADDR register"]
    pub ch4_al1_read_addr: CH4_AL1_READ_ADDR,
    #[doc = "0x118 - Alias for channel 4 WRITE_ADDR register"]
    pub ch4_al1_write_addr: CH4_AL1_WRITE_ADDR,
    #[doc = "0x11c - Alias for channel 4 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch4_al1_trans_count_trig: CH4_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x120 - Alias for channel 4 CTRL register"]
    pub ch4_al2_ctrl: CH4_AL2_CTRL,
    #[doc = "0x124 - Alias for channel 4 TRANS_COUNT register"]
    pub ch4_al2_trans_count: CH4_AL2_TRANS_COUNT,
    #[doc = "0x128 - Alias for channel 4 READ_ADDR register"]
    pub ch4_al2_read_addr: CH4_AL2_READ_ADDR,
    #[doc = "0x12c - Alias for channel 4 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch4_al2_write_addr_trig: CH4_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x130 - Alias for channel 4 CTRL register"]
    pub ch4_al3_ctrl: CH4_AL3_CTRL,
    #[doc = "0x134 - Alias for channel 4 WRITE_ADDR register"]
    pub ch4_al3_write_addr: CH4_AL3_WRITE_ADDR,
    #[doc = "0x138 - Alias for channel 4 TRANS_COUNT register"]
    pub ch4_al3_trans_count: CH4_AL3_TRANS_COUNT,
    #[doc = "0x13c - Alias for channel 4 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch4_al3_read_addr_trig: CH4_AL3_READ_ADDR_TRIG,
    #[doc = "0x140 - DMA Channel 5 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch5_read_addr: CH5_READ_ADDR,
    #[doc = "0x144 - DMA Channel 5 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch5_write_addr: CH5_WRITE_ADDR,
    #[doc = "0x148 - DMA Channel 5 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch5_trans_count: CH5_TRANS_COUNT,
    #[doc = "0x14c - DMA Channel 5 Control and Status"]
    pub ch5_ctrl_trig: CH5_CTRL_TRIG,
    #[doc = "0x150 - Alias for channel 5 CTRL register"]
    pub ch5_al1_ctrl: CH5_AL1_CTRL,
    #[doc = "0x154 - Alias for channel 5 READ_ADDR register"]
    pub ch5_al1_read_addr: CH5_AL1_READ_ADDR,
    #[doc = "0x158 - Alias for channel 5 WRITE_ADDR register"]
    pub ch5_al1_write_addr: CH5_AL1_WRITE_ADDR,
    #[doc = "0x15c - Alias for channel 5 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch5_al1_trans_count_trig: CH5_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x160 - Alias for channel 5 CTRL register"]
    pub ch5_al2_ctrl: CH5_AL2_CTRL,
    #[doc = "0x164 - Alias for channel 5 TRANS_COUNT register"]
    pub ch5_al2_trans_count: CH5_AL2_TRANS_COUNT,
    #[doc = "0x168 - Alias for channel 5 READ_ADDR register"]
    pub ch5_al2_read_addr: CH5_AL2_READ_ADDR,
    #[doc = "0x16c - Alias for channel 5 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch5_al2_write_addr_trig: CH5_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x170 - Alias for channel 5 CTRL register"]
    pub ch5_al3_ctrl: CH5_AL3_CTRL,
    #[doc = "0x174 - Alias for channel 5 WRITE_ADDR register"]
    pub ch5_al3_write_addr: CH5_AL3_WRITE_ADDR,
    #[doc = "0x178 - Alias for channel 5 TRANS_COUNT register"]
    pub ch5_al3_trans_count: CH5_AL3_TRANS_COUNT,
    #[doc = "0x17c - Alias for channel 5 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch5_al3_read_addr_trig: CH5_AL3_READ_ADDR_TRIG,
    #[doc = "0x180 - DMA Channel 6 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch6_read_addr: CH6_READ_ADDR,
    #[doc = "0x184 - DMA Channel 6 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch6_write_addr: CH6_WRITE_ADDR,
    #[doc = "0x188 - DMA Channel 6 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch6_trans_count: CH6_TRANS_COUNT,
    #[doc = "0x18c - DMA Channel 6 Control and Status"]
    pub ch6_ctrl_trig: CH6_CTRL_TRIG,
    #[doc = "0x190 - Alias for channel 6 CTRL register"]
    pub ch6_al1_ctrl: CH6_AL1_CTRL,
    #[doc = "0x194 - Alias for channel 6 READ_ADDR register"]
    pub ch6_al1_read_addr: CH6_AL1_READ_ADDR,
    #[doc = "0x198 - Alias for channel 6 WRITE_ADDR register"]
    pub ch6_al1_write_addr: CH6_AL1_WRITE_ADDR,
    #[doc = "0x19c - Alias for channel 6 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch6_al1_trans_count_trig: CH6_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x1a0 - Alias for channel 6 CTRL register"]
    pub ch6_al2_ctrl: CH6_AL2_CTRL,
    #[doc = "0x1a4 - Alias for channel 6 TRANS_COUNT register"]
    pub ch6_al2_trans_count: CH6_AL2_TRANS_COUNT,
    #[doc = "0x1a8 - Alias for channel 6 READ_ADDR register"]
    pub ch6_al2_read_addr: CH6_AL2_READ_ADDR,
    #[doc = "0x1ac - Alias for channel 6 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch6_al2_write_addr_trig: CH6_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x1b0 - Alias for channel 6 CTRL register"]
    pub ch6_al3_ctrl: CH6_AL3_CTRL,
    #[doc = "0x1b4 - Alias for channel 6 WRITE_ADDR register"]
    pub ch6_al3_write_addr: CH6_AL3_WRITE_ADDR,
    #[doc = "0x1b8 - Alias for channel 6 TRANS_COUNT register"]
    pub ch6_al3_trans_count: CH6_AL3_TRANS_COUNT,
    #[doc = "0x1bc - Alias for channel 6 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch6_al3_read_addr_trig: CH6_AL3_READ_ADDR_TRIG,
    #[doc = "0x1c0 - DMA Channel 7 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch7_read_addr: CH7_READ_ADDR,
    #[doc = "0x1c4 - DMA Channel 7 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch7_write_addr: CH7_WRITE_ADDR,
    #[doc = "0x1c8 - DMA Channel 7 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch7_trans_count: CH7_TRANS_COUNT,
    #[doc = "0x1cc - DMA Channel 7 Control and Status"]
    pub ch7_ctrl_trig: CH7_CTRL_TRIG,
    #[doc = "0x1d0 - Alias for channel 7 CTRL register"]
    pub ch7_al1_ctrl: CH7_AL1_CTRL,
    #[doc = "0x1d4 - Alias for channel 7 READ_ADDR register"]
    pub ch7_al1_read_addr: CH7_AL1_READ_ADDR,
    #[doc = "0x1d8 - Alias for channel 7 WRITE_ADDR register"]
    pub ch7_al1_write_addr: CH7_AL1_WRITE_ADDR,
    #[doc = "0x1dc - Alias for channel 7 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch7_al1_trans_count_trig: CH7_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x1e0 - Alias for channel 7 CTRL register"]
    pub ch7_al2_ctrl: CH7_AL2_CTRL,
    #[doc = "0x1e4 - Alias for channel 7 TRANS_COUNT register"]
    pub ch7_al2_trans_count: CH7_AL2_TRANS_COUNT,
    #[doc = "0x1e8 - Alias for channel 7 READ_ADDR register"]
    pub ch7_al2_read_addr: CH7_AL2_READ_ADDR,
    #[doc = "0x1ec - Alias for channel 7 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch7_al2_write_addr_trig: CH7_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x1f0 - Alias for channel 7 CTRL register"]
    pub ch7_al3_ctrl: CH7_AL3_CTRL,
    #[doc = "0x1f4 - Alias for channel 7 WRITE_ADDR register"]
    pub ch7_al3_write_addr: CH7_AL3_WRITE_ADDR,
    #[doc = "0x1f8 - Alias for channel 7 TRANS_COUNT register"]
    pub ch7_al3_trans_count: CH7_AL3_TRANS_COUNT,
    #[doc = "0x1fc - Alias for channel 7 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch7_al3_read_addr_trig: CH7_AL3_READ_ADDR_TRIG,
    #[doc = "0x200 - DMA Channel 8 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch8_read_addr: CH8_READ_ADDR,
    #[doc = "0x204 - DMA Channel 8 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch8_write_addr: CH8_WRITE_ADDR,
    #[doc = "0x208 - DMA Channel 8 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch8_trans_count: CH8_TRANS_COUNT,
    #[doc = "0x20c - DMA Channel 8 Control and Status"]
    pub ch8_ctrl_trig: CH8_CTRL_TRIG,
    #[doc = "0x210 - Alias for channel 8 CTRL register"]
    pub ch8_al1_ctrl: CH8_AL1_CTRL,
    #[doc = "0x214 - Alias for channel 8 READ_ADDR register"]
    pub ch8_al1_read_addr: CH8_AL1_READ_ADDR,
    #[doc = "0x218 - Alias for channel 8 WRITE_ADDR register"]
    pub ch8_al1_write_addr: CH8_AL1_WRITE_ADDR,
    #[doc = "0x21c - Alias for channel 8 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch8_al1_trans_count_trig: CH8_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x220 - Alias for channel 8 CTRL register"]
    pub ch8_al2_ctrl: CH8_AL2_CTRL,
    #[doc = "0x224 - Alias for channel 8 TRANS_COUNT register"]
    pub ch8_al2_trans_count: CH8_AL2_TRANS_COUNT,
    #[doc = "0x228 - Alias for channel 8 READ_ADDR register"]
    pub ch8_al2_read_addr: CH8_AL2_READ_ADDR,
    #[doc = "0x22c - Alias for channel 8 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch8_al2_write_addr_trig: CH8_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x230 - Alias for channel 8 CTRL register"]
    pub ch8_al3_ctrl: CH8_AL3_CTRL,
    #[doc = "0x234 - Alias for channel 8 WRITE_ADDR register"]
    pub ch8_al3_write_addr: CH8_AL3_WRITE_ADDR,
    #[doc = "0x238 - Alias for channel 8 TRANS_COUNT register"]
    pub ch8_al3_trans_count: CH8_AL3_TRANS_COUNT,
    #[doc = "0x23c - Alias for channel 8 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch8_al3_read_addr_trig: CH8_AL3_READ_ADDR_TRIG,
    #[doc = "0x240 - DMA Channel 9 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch9_read_addr: CH9_READ_ADDR,
    #[doc = "0x244 - DMA Channel 9 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch9_write_addr: CH9_WRITE_ADDR,
    #[doc = "0x248 - DMA Channel 9 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch9_trans_count: CH9_TRANS_COUNT,
    #[doc = "0x24c - DMA Channel 9 Control and Status"]
    pub ch9_ctrl_trig: CH9_CTRL_TRIG,
    #[doc = "0x250 - Alias for channel 9 CTRL register"]
    pub ch9_al1_ctrl: CH9_AL1_CTRL,
    #[doc = "0x254 - Alias for channel 9 READ_ADDR register"]
    pub ch9_al1_read_addr: CH9_AL1_READ_ADDR,
    #[doc = "0x258 - Alias for channel 9 WRITE_ADDR register"]
    pub ch9_al1_write_addr: CH9_AL1_WRITE_ADDR,
    #[doc = "0x25c - Alias for channel 9 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch9_al1_trans_count_trig: CH9_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x260 - Alias for channel 9 CTRL register"]
    pub ch9_al2_ctrl: CH9_AL2_CTRL,
    #[doc = "0x264 - Alias for channel 9 TRANS_COUNT register"]
    pub ch9_al2_trans_count: CH9_AL2_TRANS_COUNT,
    #[doc = "0x268 - Alias for channel 9 READ_ADDR register"]
    pub ch9_al2_read_addr: CH9_AL2_READ_ADDR,
    #[doc = "0x26c - Alias for channel 9 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch9_al2_write_addr_trig: CH9_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x270 - Alias for channel 9 CTRL register"]
    pub ch9_al3_ctrl: CH9_AL3_CTRL,
    #[doc = "0x274 - Alias for channel 9 WRITE_ADDR register"]
    pub ch9_al3_write_addr: CH9_AL3_WRITE_ADDR,
    #[doc = "0x278 - Alias for channel 9 TRANS_COUNT register"]
    pub ch9_al3_trans_count: CH9_AL3_TRANS_COUNT,
    #[doc = "0x27c - Alias for channel 9 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch9_al3_read_addr_trig: CH9_AL3_READ_ADDR_TRIG,
    #[doc = "0x280 - DMA Channel 10 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch10_read_addr: CH10_READ_ADDR,
    #[doc = "0x284 - DMA Channel 10 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch10_write_addr: CH10_WRITE_ADDR,
    #[doc = "0x288 - DMA Channel 10 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch10_trans_count: CH10_TRANS_COUNT,
    #[doc = "0x28c - DMA Channel 10 Control and Status"]
    pub ch10_ctrl_trig: CH10_CTRL_TRIG,
    #[doc = "0x290 - Alias for channel 10 CTRL register"]
    pub ch10_al1_ctrl: CH10_AL1_CTRL,
    #[doc = "0x294 - Alias for channel 10 READ_ADDR register"]
    pub ch10_al1_read_addr: CH10_AL1_READ_ADDR,
    #[doc = "0x298 - Alias for channel 10 WRITE_ADDR register"]
    pub ch10_al1_write_addr: CH10_AL1_WRITE_ADDR,
    #[doc = "0x29c - Alias for channel 10 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch10_al1_trans_count_trig: CH10_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x2a0 - Alias for channel 10 CTRL register"]
    pub ch10_al2_ctrl: CH10_AL2_CTRL,
    #[doc = "0x2a4 - Alias for channel 10 TRANS_COUNT register"]
    pub ch10_al2_trans_count: CH10_AL2_TRANS_COUNT,
    #[doc = "0x2a8 - Alias for channel 10 READ_ADDR register"]
    pub ch10_al2_read_addr: CH10_AL2_READ_ADDR,
    #[doc = "0x2ac - Alias for channel 10 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch10_al2_write_addr_trig: CH10_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x2b0 - Alias for channel 10 CTRL register"]
    pub ch10_al3_ctrl: CH10_AL3_CTRL,
    #[doc = "0x2b4 - Alias for channel 10 WRITE_ADDR register"]
    pub ch10_al3_write_addr: CH10_AL3_WRITE_ADDR,
    #[doc = "0x2b8 - Alias for channel 10 TRANS_COUNT register"]
    pub ch10_al3_trans_count: CH10_AL3_TRANS_COUNT,
    #[doc = "0x2bc - Alias for channel 10 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch10_al3_read_addr_trig: CH10_AL3_READ_ADDR_TRIG,
    #[doc = "0x2c0 - DMA Channel 11 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch11_read_addr: CH11_READ_ADDR,
    #[doc = "0x2c4 - DMA Channel 11 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch11_write_addr: CH11_WRITE_ADDR,
    #[doc = "0x2c8 - DMA Channel 11 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch11_trans_count: CH11_TRANS_COUNT,
    #[doc = "0x2cc - DMA Channel 11 Control and Status"]
    pub ch11_ctrl_trig: CH11_CTRL_TRIG,
    #[doc = "0x2d0 - Alias for channel 11 CTRL register"]
    pub ch11_al1_ctrl: CH11_AL1_CTRL,
    #[doc = "0x2d4 - Alias for channel 11 READ_ADDR register"]
    pub ch11_al1_read_addr: CH11_AL1_READ_ADDR,
    #[doc = "0x2d8 - Alias for channel 11 WRITE_ADDR register"]
    pub ch11_al1_write_addr: CH11_AL1_WRITE_ADDR,
    #[doc = "0x2dc - Alias for channel 11 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch11_al1_trans_count_trig: CH11_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x2e0 - Alias for channel 11 CTRL register"]
    pub ch11_al2_ctrl: CH11_AL2_CTRL,
    #[doc = "0x2e4 - Alias for channel 11 TRANS_COUNT register"]
    pub ch11_al2_trans_count: CH11_AL2_TRANS_COUNT,
    #[doc = "0x2e8 - Alias for channel 11 READ_ADDR register"]
    pub ch11_al2_read_addr: CH11_AL2_READ_ADDR,
    #[doc = "0x2ec - Alias for channel 11 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch11_al2_write_addr_trig: CH11_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x2f0 - Alias for channel 11 CTRL register"]
    pub ch11_al3_ctrl: CH11_AL3_CTRL,
    #[doc = "0x2f4 - Alias for channel 11 WRITE_ADDR register"]
    pub ch11_al3_write_addr: CH11_AL3_WRITE_ADDR,
    #[doc = "0x2f8 - Alias for channel 11 TRANS_COUNT register"]
    pub ch11_al3_trans_count: CH11_AL3_TRANS_COUNT,
    #[doc = "0x2fc - Alias for channel 11 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch11_al3_read_addr_trig: CH11_AL3_READ_ADDR_TRIG,
    _reserved192: [u8; 256usize],
    #[doc = "0x400 - Interrupt Status (raw)"]
    pub intr: INTR,
    #[doc = "0x404 - Interrupt Enables for IRQ 0"]
    pub inte0: INTE0,
    #[doc = "0x408 - Force Interrupts"]
    pub intf0: INTF0,
    #[doc = "0x40c - Interrupt Status for IRQ 0"]
    pub ints0: INTS0,
    _reserved196: [u8; 4usize],
    #[doc = "0x414 - Interrupt Enables for IRQ 1"]
    pub inte1: INTE1,
    #[doc = "0x418 - Force Interrupts for IRQ 1"]
    pub intf1: INTF1,
    #[doc = "0x41c - Interrupt Status (masked) for IRQ 1"]
    pub ints1: INTS1,
    #[doc = "0x420 - Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer0: TIMER0,
    #[doc = "0x424 - Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer1: TIMER1,
    _reserved201: [u8; 8usize],
    #[doc = "0x430 - Trigger one or more channels simultaneously"]
    pub multi_chan_trigger: MULTI_CHAN_TRIGGER,
    #[doc = "0x434 - Sniffer Control"]
    pub sniff_ctrl: SNIFF_CTRL,
    #[doc = "0x438 - Data accumulator for sniff hardware\\n Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    pub sniff_data: SNIFF_DATA,
    _reserved204: [u8; 4usize],
    #[doc = "0x440 - Debug RAF, WAF, TDF levels"]
    pub fifo_levels: FIFO_LEVELS,
    #[doc = "0x444 - Abort an in-progress transfer sequence on one or more channels"]
    pub chan_abort: CHAN_ABORT,
    #[doc = "0x448 - The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    pub n_channels: N_CHANNELS,
    _reserved207: [u8; 948usize],
    #[doc = "0x800 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch0_dbg_ctdreq: CH0_DBG_CTDREQ,
    #[doc = "0x804 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch0_dbg_tcr: CH0_DBG_TCR,
    _reserved209: [u8; 56usize],
    #[doc = "0x840 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch1_dbg_ctdreq: CH1_DBG_CTDREQ,
    #[doc = "0x844 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch1_dbg_tcr: CH1_DBG_TCR,
    _reserved211: [u8; 56usize],
    #[doc = "0x880 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch2_dbg_ctdreq: CH2_DBG_CTDREQ,
    #[doc = "0x884 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch2_dbg_tcr: CH2_DBG_TCR,
    _reserved213: [u8; 56usize],
    #[doc = "0x8c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch3_dbg_ctdreq: CH3_DBG_CTDREQ,
    #[doc = "0x8c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch3_dbg_tcr: CH3_DBG_TCR,
    _reserved215: [u8; 56usize],
    #[doc = "0x900 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch4_dbg_ctdreq: CH4_DBG_CTDREQ,
    #[doc = "0x904 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch4_dbg_tcr: CH4_DBG_TCR,
    _reserved217: [u8; 56usize],
    #[doc = "0x940 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch5_dbg_ctdreq: CH5_DBG_CTDREQ,
    #[doc = "0x944 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch5_dbg_tcr: CH5_DBG_TCR,
    _reserved219: [u8; 56usize],
    #[doc = "0x980 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch6_dbg_ctdreq: CH6_DBG_CTDREQ,
    #[doc = "0x984 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch6_dbg_tcr: CH6_DBG_TCR,
    _reserved221: [u8; 56usize],
    #[doc = "0x9c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch7_dbg_ctdreq: CH7_DBG_CTDREQ,
    #[doc = "0x9c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch7_dbg_tcr: CH7_DBG_TCR,
    _reserved223: [u8; 56usize],
    #[doc = "0xa00 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch8_dbg_ctdreq: CH8_DBG_CTDREQ,
    #[doc = "0xa04 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch8_dbg_tcr: CH8_DBG_TCR,
    _reserved225: [u8; 56usize],
    #[doc = "0xa40 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch9_dbg_ctdreq: CH9_DBG_CTDREQ,
    #[doc = "0xa44 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch9_dbg_tcr: CH9_DBG_TCR,
    _reserved227: [u8; 56usize],
    #[doc = "0xa80 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch10_dbg_ctdreq: CH10_DBG_CTDREQ,
    #[doc = "0xa84 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch10_dbg_tcr: CH10_DBG_TCR,
    _reserved229: [u8; 56usize],
    #[doc = "0xac0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch11_dbg_ctdreq: CH11_DBG_CTDREQ,
    #[doc = "0xac4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch11_dbg_tcr: CH11_DBG_TCR,
}
#[doc = "DMA Channel 0 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_read_addr](ch0_read_addr) module"]
pub type CH0_READ_ADDR = crate::Reg<u32, _CH0_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_READ_ADDR;
#[doc = "`read()` method returns [ch0_read_addr::R](ch0_read_addr::R) reader structure"]
impl crate::Readable for CH0_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch0_read_addr::W](ch0_read_addr::W) writer structure"]
impl crate::Writable for CH0_READ_ADDR {}
#[doc = "DMA Channel 0 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch0_read_addr;
#[doc = "DMA Channel 0 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_write_addr](ch0_write_addr) module"]
pub type CH0_WRITE_ADDR = crate::Reg<u32, _CH0_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_WRITE_ADDR;
#[doc = "`read()` method returns [ch0_write_addr::R](ch0_write_addr::R) reader structure"]
impl crate::Readable for CH0_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch0_write_addr::W](ch0_write_addr::W) writer structure"]
impl crate::Writable for CH0_WRITE_ADDR {}
#[doc = "DMA Channel 0 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch0_write_addr;
#[doc = "DMA Channel 0 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_trans_count](ch0_trans_count) module"]
pub type CH0_TRANS_COUNT = crate::Reg<u32, _CH0_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_TRANS_COUNT;
#[doc = "`read()` method returns [ch0_trans_count::R](ch0_trans_count::R) reader structure"]
impl crate::Readable for CH0_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch0_trans_count::W](ch0_trans_count::W) writer structure"]
impl crate::Writable for CH0_TRANS_COUNT {}
#[doc = "DMA Channel 0 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch0_trans_count;
#[doc = "DMA Channel 0 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_ctrl_trig](ch0_ctrl_trig) module"]
pub type CH0_CTRL_TRIG = crate::Reg<u32, _CH0_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CTRL_TRIG;
#[doc = "`read()` method returns [ch0_ctrl_trig::R](ch0_ctrl_trig::R) reader structure"]
impl crate::Readable for CH0_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch0_ctrl_trig::W](ch0_ctrl_trig::W) writer structure"]
impl crate::Writable for CH0_CTRL_TRIG {}
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch0_ctrl_trig;
#[doc = "Alias for channel 0 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al1_ctrl](ch0_al1_ctrl) module"]
pub type CH0_AL1_CTRL = crate::Reg<u32, _CH0_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL1_CTRL;
#[doc = "`read()` method returns [ch0_al1_ctrl::R](ch0_al1_ctrl::R) reader structure"]
impl crate::Readable for CH0_AL1_CTRL {}
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch0_al1_ctrl;
#[doc = "Alias for channel 0 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al1_read_addr](ch0_al1_read_addr) module"]
pub type CH0_AL1_READ_ADDR = crate::Reg<u32, _CH0_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch0_al1_read_addr::R](ch0_al1_read_addr::R) reader structure"]
impl crate::Readable for CH0_AL1_READ_ADDR {}
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch0_al1_read_addr;
#[doc = "Alias for channel 0 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al1_write_addr](ch0_al1_write_addr) module"]
pub type CH0_AL1_WRITE_ADDR = crate::Reg<u32, _CH0_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch0_al1_write_addr::R](ch0_al1_write_addr::R) reader structure"]
impl crate::Readable for CH0_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch0_al1_write_addr;
#[doc = "Alias for channel 0 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al1_trans_count_trig](ch0_al1_trans_count_trig) module"]
pub type CH0_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH0_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch0_al1_trans_count_trig::R](ch0_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH0_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 0 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch0_al1_trans_count_trig;
#[doc = "Alias for channel 0 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al2_ctrl](ch0_al2_ctrl) module"]
pub type CH0_AL2_CTRL = crate::Reg<u32, _CH0_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL2_CTRL;
#[doc = "`read()` method returns [ch0_al2_ctrl::R](ch0_al2_ctrl::R) reader structure"]
impl crate::Readable for CH0_AL2_CTRL {}
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch0_al2_ctrl;
#[doc = "Alias for channel 0 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al2_trans_count](ch0_al2_trans_count) module"]
pub type CH0_AL2_TRANS_COUNT = crate::Reg<u32, _CH0_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch0_al2_trans_count::R](ch0_al2_trans_count::R) reader structure"]
impl crate::Readable for CH0_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch0_al2_trans_count;
#[doc = "Alias for channel 0 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al2_read_addr](ch0_al2_read_addr) module"]
pub type CH0_AL2_READ_ADDR = crate::Reg<u32, _CH0_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch0_al2_read_addr::R](ch0_al2_read_addr::R) reader structure"]
impl crate::Readable for CH0_AL2_READ_ADDR {}
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch0_al2_read_addr;
#[doc = "Alias for channel 0 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al2_write_addr_trig](ch0_al2_write_addr_trig) module"]
pub type CH0_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH0_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch0_al2_write_addr_trig::R](ch0_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH0_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 0 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch0_al2_write_addr_trig;
#[doc = "Alias for channel 0 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al3_ctrl](ch0_al3_ctrl) module"]
pub type CH0_AL3_CTRL = crate::Reg<u32, _CH0_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL3_CTRL;
#[doc = "`read()` method returns [ch0_al3_ctrl::R](ch0_al3_ctrl::R) reader structure"]
impl crate::Readable for CH0_AL3_CTRL {}
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch0_al3_ctrl;
#[doc = "Alias for channel 0 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al3_write_addr](ch0_al3_write_addr) module"]
pub type CH0_AL3_WRITE_ADDR = crate::Reg<u32, _CH0_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch0_al3_write_addr::R](ch0_al3_write_addr::R) reader structure"]
impl crate::Readable for CH0_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch0_al3_write_addr;
#[doc = "Alias for channel 0 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al3_trans_count](ch0_al3_trans_count) module"]
pub type CH0_AL3_TRANS_COUNT = crate::Reg<u32, _CH0_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch0_al3_trans_count::R](ch0_al3_trans_count::R) reader structure"]
impl crate::Readable for CH0_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch0_al3_trans_count;
#[doc = "Alias for channel 0 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_al3_read_addr_trig](ch0_al3_read_addr_trig) module"]
pub type CH0_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH0_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch0_al3_read_addr_trig::R](ch0_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH0_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 0 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch0_al3_read_addr_trig;
#[doc = "DMA Channel 1 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_read_addr](ch1_read_addr) module"]
pub type CH1_READ_ADDR = crate::Reg<u32, _CH1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_READ_ADDR;
#[doc = "`read()` method returns [ch1_read_addr::R](ch1_read_addr::R) reader structure"]
impl crate::Readable for CH1_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch1_read_addr::W](ch1_read_addr::W) writer structure"]
impl crate::Writable for CH1_READ_ADDR {}
#[doc = "DMA Channel 1 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch1_read_addr;
#[doc = "DMA Channel 1 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_write_addr](ch1_write_addr) module"]
pub type CH1_WRITE_ADDR = crate::Reg<u32, _CH1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_WRITE_ADDR;
#[doc = "`read()` method returns [ch1_write_addr::R](ch1_write_addr::R) reader structure"]
impl crate::Readable for CH1_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch1_write_addr::W](ch1_write_addr::W) writer structure"]
impl crate::Writable for CH1_WRITE_ADDR {}
#[doc = "DMA Channel 1 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch1_write_addr;
#[doc = "DMA Channel 1 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_trans_count](ch1_trans_count) module"]
pub type CH1_TRANS_COUNT = crate::Reg<u32, _CH1_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_TRANS_COUNT;
#[doc = "`read()` method returns [ch1_trans_count::R](ch1_trans_count::R) reader structure"]
impl crate::Readable for CH1_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch1_trans_count::W](ch1_trans_count::W) writer structure"]
impl crate::Writable for CH1_TRANS_COUNT {}
#[doc = "DMA Channel 1 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch1_trans_count;
#[doc = "DMA Channel 1 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_ctrl_trig](ch1_ctrl_trig) module"]
pub type CH1_CTRL_TRIG = crate::Reg<u32, _CH1_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CTRL_TRIG;
#[doc = "`read()` method returns [ch1_ctrl_trig::R](ch1_ctrl_trig::R) reader structure"]
impl crate::Readable for CH1_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch1_ctrl_trig::W](ch1_ctrl_trig::W) writer structure"]
impl crate::Writable for CH1_CTRL_TRIG {}
#[doc = "DMA Channel 1 Control and Status"]
pub mod ch1_ctrl_trig;
#[doc = "Alias for channel 1 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al1_ctrl](ch1_al1_ctrl) module"]
pub type CH1_AL1_CTRL = crate::Reg<u32, _CH1_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL1_CTRL;
#[doc = "`read()` method returns [ch1_al1_ctrl::R](ch1_al1_ctrl::R) reader structure"]
impl crate::Readable for CH1_AL1_CTRL {}
#[doc = "Alias for channel 1 CTRL register"]
pub mod ch1_al1_ctrl;
#[doc = "Alias for channel 1 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al1_read_addr](ch1_al1_read_addr) module"]
pub type CH1_AL1_READ_ADDR = crate::Reg<u32, _CH1_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch1_al1_read_addr::R](ch1_al1_read_addr::R) reader structure"]
impl crate::Readable for CH1_AL1_READ_ADDR {}
#[doc = "Alias for channel 1 READ_ADDR register"]
pub mod ch1_al1_read_addr;
#[doc = "Alias for channel 1 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al1_write_addr](ch1_al1_write_addr) module"]
pub type CH1_AL1_WRITE_ADDR = crate::Reg<u32, _CH1_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch1_al1_write_addr::R](ch1_al1_write_addr::R) reader structure"]
impl crate::Readable for CH1_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 1 WRITE_ADDR register"]
pub mod ch1_al1_write_addr;
#[doc = "Alias for channel 1 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al1_trans_count_trig](ch1_al1_trans_count_trig) module"]
pub type CH1_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH1_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch1_al1_trans_count_trig::R](ch1_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH1_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 1 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch1_al1_trans_count_trig;
#[doc = "Alias for channel 1 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al2_ctrl](ch1_al2_ctrl) module"]
pub type CH1_AL2_CTRL = crate::Reg<u32, _CH1_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL2_CTRL;
#[doc = "`read()` method returns [ch1_al2_ctrl::R](ch1_al2_ctrl::R) reader structure"]
impl crate::Readable for CH1_AL2_CTRL {}
#[doc = "Alias for channel 1 CTRL register"]
pub mod ch1_al2_ctrl;
#[doc = "Alias for channel 1 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al2_trans_count](ch1_al2_trans_count) module"]
pub type CH1_AL2_TRANS_COUNT = crate::Reg<u32, _CH1_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch1_al2_trans_count::R](ch1_al2_trans_count::R) reader structure"]
impl crate::Readable for CH1_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 1 TRANS_COUNT register"]
pub mod ch1_al2_trans_count;
#[doc = "Alias for channel 1 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al2_read_addr](ch1_al2_read_addr) module"]
pub type CH1_AL2_READ_ADDR = crate::Reg<u32, _CH1_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch1_al2_read_addr::R](ch1_al2_read_addr::R) reader structure"]
impl crate::Readable for CH1_AL2_READ_ADDR {}
#[doc = "Alias for channel 1 READ_ADDR register"]
pub mod ch1_al2_read_addr;
#[doc = "Alias for channel 1 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al2_write_addr_trig](ch1_al2_write_addr_trig) module"]
pub type CH1_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH1_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch1_al2_write_addr_trig::R](ch1_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH1_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 1 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch1_al2_write_addr_trig;
#[doc = "Alias for channel 1 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al3_ctrl](ch1_al3_ctrl) module"]
pub type CH1_AL3_CTRL = crate::Reg<u32, _CH1_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL3_CTRL;
#[doc = "`read()` method returns [ch1_al3_ctrl::R](ch1_al3_ctrl::R) reader structure"]
impl crate::Readable for CH1_AL3_CTRL {}
#[doc = "Alias for channel 1 CTRL register"]
pub mod ch1_al3_ctrl;
#[doc = "Alias for channel 1 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al3_write_addr](ch1_al3_write_addr) module"]
pub type CH1_AL3_WRITE_ADDR = crate::Reg<u32, _CH1_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch1_al3_write_addr::R](ch1_al3_write_addr::R) reader structure"]
impl crate::Readable for CH1_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 1 WRITE_ADDR register"]
pub mod ch1_al3_write_addr;
#[doc = "Alias for channel 1 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al3_trans_count](ch1_al3_trans_count) module"]
pub type CH1_AL3_TRANS_COUNT = crate::Reg<u32, _CH1_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch1_al3_trans_count::R](ch1_al3_trans_count::R) reader structure"]
impl crate::Readable for CH1_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 1 TRANS_COUNT register"]
pub mod ch1_al3_trans_count;
#[doc = "Alias for channel 1 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_al3_read_addr_trig](ch1_al3_read_addr_trig) module"]
pub type CH1_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH1_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch1_al3_read_addr_trig::R](ch1_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH1_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 1 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch1_al3_read_addr_trig;
#[doc = "DMA Channel 2 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_read_addr](ch2_read_addr) module"]
pub type CH2_READ_ADDR = crate::Reg<u32, _CH2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_READ_ADDR;
#[doc = "`read()` method returns [ch2_read_addr::R](ch2_read_addr::R) reader structure"]
impl crate::Readable for CH2_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch2_read_addr::W](ch2_read_addr::W) writer structure"]
impl crate::Writable for CH2_READ_ADDR {}
#[doc = "DMA Channel 2 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch2_read_addr;
#[doc = "DMA Channel 2 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_write_addr](ch2_write_addr) module"]
pub type CH2_WRITE_ADDR = crate::Reg<u32, _CH2_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_WRITE_ADDR;
#[doc = "`read()` method returns [ch2_write_addr::R](ch2_write_addr::R) reader structure"]
impl crate::Readable for CH2_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch2_write_addr::W](ch2_write_addr::W) writer structure"]
impl crate::Writable for CH2_WRITE_ADDR {}
#[doc = "DMA Channel 2 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch2_write_addr;
#[doc = "DMA Channel 2 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_trans_count](ch2_trans_count) module"]
pub type CH2_TRANS_COUNT = crate::Reg<u32, _CH2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_TRANS_COUNT;
#[doc = "`read()` method returns [ch2_trans_count::R](ch2_trans_count::R) reader structure"]
impl crate::Readable for CH2_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch2_trans_count::W](ch2_trans_count::W) writer structure"]
impl crate::Writable for CH2_TRANS_COUNT {}
#[doc = "DMA Channel 2 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch2_trans_count;
#[doc = "DMA Channel 2 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_ctrl_trig](ch2_ctrl_trig) module"]
pub type CH2_CTRL_TRIG = crate::Reg<u32, _CH2_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CTRL_TRIG;
#[doc = "`read()` method returns [ch2_ctrl_trig::R](ch2_ctrl_trig::R) reader structure"]
impl crate::Readable for CH2_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch2_ctrl_trig::W](ch2_ctrl_trig::W) writer structure"]
impl crate::Writable for CH2_CTRL_TRIG {}
#[doc = "DMA Channel 2 Control and Status"]
pub mod ch2_ctrl_trig;
#[doc = "Alias for channel 2 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al1_ctrl](ch2_al1_ctrl) module"]
pub type CH2_AL1_CTRL = crate::Reg<u32, _CH2_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL1_CTRL;
#[doc = "`read()` method returns [ch2_al1_ctrl::R](ch2_al1_ctrl::R) reader structure"]
impl crate::Readable for CH2_AL1_CTRL {}
#[doc = "Alias for channel 2 CTRL register"]
pub mod ch2_al1_ctrl;
#[doc = "Alias for channel 2 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al1_read_addr](ch2_al1_read_addr) module"]
pub type CH2_AL1_READ_ADDR = crate::Reg<u32, _CH2_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch2_al1_read_addr::R](ch2_al1_read_addr::R) reader structure"]
impl crate::Readable for CH2_AL1_READ_ADDR {}
#[doc = "Alias for channel 2 READ_ADDR register"]
pub mod ch2_al1_read_addr;
#[doc = "Alias for channel 2 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al1_write_addr](ch2_al1_write_addr) module"]
pub type CH2_AL1_WRITE_ADDR = crate::Reg<u32, _CH2_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch2_al1_write_addr::R](ch2_al1_write_addr::R) reader structure"]
impl crate::Readable for CH2_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 2 WRITE_ADDR register"]
pub mod ch2_al1_write_addr;
#[doc = "Alias for channel 2 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al1_trans_count_trig](ch2_al1_trans_count_trig) module"]
pub type CH2_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH2_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch2_al1_trans_count_trig::R](ch2_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH2_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 2 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch2_al1_trans_count_trig;
#[doc = "Alias for channel 2 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al2_ctrl](ch2_al2_ctrl) module"]
pub type CH2_AL2_CTRL = crate::Reg<u32, _CH2_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL2_CTRL;
#[doc = "`read()` method returns [ch2_al2_ctrl::R](ch2_al2_ctrl::R) reader structure"]
impl crate::Readable for CH2_AL2_CTRL {}
#[doc = "Alias for channel 2 CTRL register"]
pub mod ch2_al2_ctrl;
#[doc = "Alias for channel 2 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al2_trans_count](ch2_al2_trans_count) module"]
pub type CH2_AL2_TRANS_COUNT = crate::Reg<u32, _CH2_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch2_al2_trans_count::R](ch2_al2_trans_count::R) reader structure"]
impl crate::Readable for CH2_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 2 TRANS_COUNT register"]
pub mod ch2_al2_trans_count;
#[doc = "Alias for channel 2 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al2_read_addr](ch2_al2_read_addr) module"]
pub type CH2_AL2_READ_ADDR = crate::Reg<u32, _CH2_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch2_al2_read_addr::R](ch2_al2_read_addr::R) reader structure"]
impl crate::Readable for CH2_AL2_READ_ADDR {}
#[doc = "Alias for channel 2 READ_ADDR register"]
pub mod ch2_al2_read_addr;
#[doc = "Alias for channel 2 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al2_write_addr_trig](ch2_al2_write_addr_trig) module"]
pub type CH2_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH2_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch2_al2_write_addr_trig::R](ch2_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH2_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 2 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch2_al2_write_addr_trig;
#[doc = "Alias for channel 2 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al3_ctrl](ch2_al3_ctrl) module"]
pub type CH2_AL3_CTRL = crate::Reg<u32, _CH2_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL3_CTRL;
#[doc = "`read()` method returns [ch2_al3_ctrl::R](ch2_al3_ctrl::R) reader structure"]
impl crate::Readable for CH2_AL3_CTRL {}
#[doc = "Alias for channel 2 CTRL register"]
pub mod ch2_al3_ctrl;
#[doc = "Alias for channel 2 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al3_write_addr](ch2_al3_write_addr) module"]
pub type CH2_AL3_WRITE_ADDR = crate::Reg<u32, _CH2_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch2_al3_write_addr::R](ch2_al3_write_addr::R) reader structure"]
impl crate::Readable for CH2_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 2 WRITE_ADDR register"]
pub mod ch2_al3_write_addr;
#[doc = "Alias for channel 2 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al3_trans_count](ch2_al3_trans_count) module"]
pub type CH2_AL3_TRANS_COUNT = crate::Reg<u32, _CH2_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch2_al3_trans_count::R](ch2_al3_trans_count::R) reader structure"]
impl crate::Readable for CH2_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 2 TRANS_COUNT register"]
pub mod ch2_al3_trans_count;
#[doc = "Alias for channel 2 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_al3_read_addr_trig](ch2_al3_read_addr_trig) module"]
pub type CH2_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH2_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch2_al3_read_addr_trig::R](ch2_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH2_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 2 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch2_al3_read_addr_trig;
#[doc = "DMA Channel 3 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_read_addr](ch3_read_addr) module"]
pub type CH3_READ_ADDR = crate::Reg<u32, _CH3_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_READ_ADDR;
#[doc = "`read()` method returns [ch3_read_addr::R](ch3_read_addr::R) reader structure"]
impl crate::Readable for CH3_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch3_read_addr::W](ch3_read_addr::W) writer structure"]
impl crate::Writable for CH3_READ_ADDR {}
#[doc = "DMA Channel 3 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch3_read_addr;
#[doc = "DMA Channel 3 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_write_addr](ch3_write_addr) module"]
pub type CH3_WRITE_ADDR = crate::Reg<u32, _CH3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_WRITE_ADDR;
#[doc = "`read()` method returns [ch3_write_addr::R](ch3_write_addr::R) reader structure"]
impl crate::Readable for CH3_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch3_write_addr::W](ch3_write_addr::W) writer structure"]
impl crate::Writable for CH3_WRITE_ADDR {}
#[doc = "DMA Channel 3 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch3_write_addr;
#[doc = "DMA Channel 3 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_trans_count](ch3_trans_count) module"]
pub type CH3_TRANS_COUNT = crate::Reg<u32, _CH3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_TRANS_COUNT;
#[doc = "`read()` method returns [ch3_trans_count::R](ch3_trans_count::R) reader structure"]
impl crate::Readable for CH3_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch3_trans_count::W](ch3_trans_count::W) writer structure"]
impl crate::Writable for CH3_TRANS_COUNT {}
#[doc = "DMA Channel 3 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch3_trans_count;
#[doc = "DMA Channel 3 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_ctrl_trig](ch3_ctrl_trig) module"]
pub type CH3_CTRL_TRIG = crate::Reg<u32, _CH3_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CTRL_TRIG;
#[doc = "`read()` method returns [ch3_ctrl_trig::R](ch3_ctrl_trig::R) reader structure"]
impl crate::Readable for CH3_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch3_ctrl_trig::W](ch3_ctrl_trig::W) writer structure"]
impl crate::Writable for CH3_CTRL_TRIG {}
#[doc = "DMA Channel 3 Control and Status"]
pub mod ch3_ctrl_trig;
#[doc = "Alias for channel 3 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al1_ctrl](ch3_al1_ctrl) module"]
pub type CH3_AL1_CTRL = crate::Reg<u32, _CH3_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL1_CTRL;
#[doc = "`read()` method returns [ch3_al1_ctrl::R](ch3_al1_ctrl::R) reader structure"]
impl crate::Readable for CH3_AL1_CTRL {}
#[doc = "Alias for channel 3 CTRL register"]
pub mod ch3_al1_ctrl;
#[doc = "Alias for channel 3 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al1_read_addr](ch3_al1_read_addr) module"]
pub type CH3_AL1_READ_ADDR = crate::Reg<u32, _CH3_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch3_al1_read_addr::R](ch3_al1_read_addr::R) reader structure"]
impl crate::Readable for CH3_AL1_READ_ADDR {}
#[doc = "Alias for channel 3 READ_ADDR register"]
pub mod ch3_al1_read_addr;
#[doc = "Alias for channel 3 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al1_write_addr](ch3_al1_write_addr) module"]
pub type CH3_AL1_WRITE_ADDR = crate::Reg<u32, _CH3_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch3_al1_write_addr::R](ch3_al1_write_addr::R) reader structure"]
impl crate::Readable for CH3_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 3 WRITE_ADDR register"]
pub mod ch3_al1_write_addr;
#[doc = "Alias for channel 3 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al1_trans_count_trig](ch3_al1_trans_count_trig) module"]
pub type CH3_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH3_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch3_al1_trans_count_trig::R](ch3_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH3_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 3 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch3_al1_trans_count_trig;
#[doc = "Alias for channel 3 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al2_ctrl](ch3_al2_ctrl) module"]
pub type CH3_AL2_CTRL = crate::Reg<u32, _CH3_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL2_CTRL;
#[doc = "`read()` method returns [ch3_al2_ctrl::R](ch3_al2_ctrl::R) reader structure"]
impl crate::Readable for CH3_AL2_CTRL {}
#[doc = "Alias for channel 3 CTRL register"]
pub mod ch3_al2_ctrl;
#[doc = "Alias for channel 3 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al2_trans_count](ch3_al2_trans_count) module"]
pub type CH3_AL2_TRANS_COUNT = crate::Reg<u32, _CH3_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch3_al2_trans_count::R](ch3_al2_trans_count::R) reader structure"]
impl crate::Readable for CH3_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 3 TRANS_COUNT register"]
pub mod ch3_al2_trans_count;
#[doc = "Alias for channel 3 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al2_read_addr](ch3_al2_read_addr) module"]
pub type CH3_AL2_READ_ADDR = crate::Reg<u32, _CH3_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch3_al2_read_addr::R](ch3_al2_read_addr::R) reader structure"]
impl crate::Readable for CH3_AL2_READ_ADDR {}
#[doc = "Alias for channel 3 READ_ADDR register"]
pub mod ch3_al2_read_addr;
#[doc = "Alias for channel 3 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al2_write_addr_trig](ch3_al2_write_addr_trig) module"]
pub type CH3_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH3_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch3_al2_write_addr_trig::R](ch3_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH3_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 3 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch3_al2_write_addr_trig;
#[doc = "Alias for channel 3 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al3_ctrl](ch3_al3_ctrl) module"]
pub type CH3_AL3_CTRL = crate::Reg<u32, _CH3_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL3_CTRL;
#[doc = "`read()` method returns [ch3_al3_ctrl::R](ch3_al3_ctrl::R) reader structure"]
impl crate::Readable for CH3_AL3_CTRL {}
#[doc = "Alias for channel 3 CTRL register"]
pub mod ch3_al3_ctrl;
#[doc = "Alias for channel 3 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al3_write_addr](ch3_al3_write_addr) module"]
pub type CH3_AL3_WRITE_ADDR = crate::Reg<u32, _CH3_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch3_al3_write_addr::R](ch3_al3_write_addr::R) reader structure"]
impl crate::Readable for CH3_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 3 WRITE_ADDR register"]
pub mod ch3_al3_write_addr;
#[doc = "Alias for channel 3 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al3_trans_count](ch3_al3_trans_count) module"]
pub type CH3_AL3_TRANS_COUNT = crate::Reg<u32, _CH3_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch3_al3_trans_count::R](ch3_al3_trans_count::R) reader structure"]
impl crate::Readable for CH3_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 3 TRANS_COUNT register"]
pub mod ch3_al3_trans_count;
#[doc = "Alias for channel 3 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_al3_read_addr_trig](ch3_al3_read_addr_trig) module"]
pub type CH3_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH3_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch3_al3_read_addr_trig::R](ch3_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH3_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 3 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch3_al3_read_addr_trig;
#[doc = "DMA Channel 4 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_read_addr](ch4_read_addr) module"]
pub type CH4_READ_ADDR = crate::Reg<u32, _CH4_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_READ_ADDR;
#[doc = "`read()` method returns [ch4_read_addr::R](ch4_read_addr::R) reader structure"]
impl crate::Readable for CH4_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch4_read_addr::W](ch4_read_addr::W) writer structure"]
impl crate::Writable for CH4_READ_ADDR {}
#[doc = "DMA Channel 4 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch4_read_addr;
#[doc = "DMA Channel 4 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_write_addr](ch4_write_addr) module"]
pub type CH4_WRITE_ADDR = crate::Reg<u32, _CH4_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_WRITE_ADDR;
#[doc = "`read()` method returns [ch4_write_addr::R](ch4_write_addr::R) reader structure"]
impl crate::Readable for CH4_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch4_write_addr::W](ch4_write_addr::W) writer structure"]
impl crate::Writable for CH4_WRITE_ADDR {}
#[doc = "DMA Channel 4 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch4_write_addr;
#[doc = "DMA Channel 4 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_trans_count](ch4_trans_count) module"]
pub type CH4_TRANS_COUNT = crate::Reg<u32, _CH4_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_TRANS_COUNT;
#[doc = "`read()` method returns [ch4_trans_count::R](ch4_trans_count::R) reader structure"]
impl crate::Readable for CH4_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch4_trans_count::W](ch4_trans_count::W) writer structure"]
impl crate::Writable for CH4_TRANS_COUNT {}
#[doc = "DMA Channel 4 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch4_trans_count;
#[doc = "DMA Channel 4 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_ctrl_trig](ch4_ctrl_trig) module"]
pub type CH4_CTRL_TRIG = crate::Reg<u32, _CH4_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CTRL_TRIG;
#[doc = "`read()` method returns [ch4_ctrl_trig::R](ch4_ctrl_trig::R) reader structure"]
impl crate::Readable for CH4_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch4_ctrl_trig::W](ch4_ctrl_trig::W) writer structure"]
impl crate::Writable for CH4_CTRL_TRIG {}
#[doc = "DMA Channel 4 Control and Status"]
pub mod ch4_ctrl_trig;
#[doc = "Alias for channel 4 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al1_ctrl](ch4_al1_ctrl) module"]
pub type CH4_AL1_CTRL = crate::Reg<u32, _CH4_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL1_CTRL;
#[doc = "`read()` method returns [ch4_al1_ctrl::R](ch4_al1_ctrl::R) reader structure"]
impl crate::Readable for CH4_AL1_CTRL {}
#[doc = "Alias for channel 4 CTRL register"]
pub mod ch4_al1_ctrl;
#[doc = "Alias for channel 4 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al1_read_addr](ch4_al1_read_addr) module"]
pub type CH4_AL1_READ_ADDR = crate::Reg<u32, _CH4_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch4_al1_read_addr::R](ch4_al1_read_addr::R) reader structure"]
impl crate::Readable for CH4_AL1_READ_ADDR {}
#[doc = "Alias for channel 4 READ_ADDR register"]
pub mod ch4_al1_read_addr;
#[doc = "Alias for channel 4 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al1_write_addr](ch4_al1_write_addr) module"]
pub type CH4_AL1_WRITE_ADDR = crate::Reg<u32, _CH4_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch4_al1_write_addr::R](ch4_al1_write_addr::R) reader structure"]
impl crate::Readable for CH4_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 4 WRITE_ADDR register"]
pub mod ch4_al1_write_addr;
#[doc = "Alias for channel 4 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al1_trans_count_trig](ch4_al1_trans_count_trig) module"]
pub type CH4_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH4_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch4_al1_trans_count_trig::R](ch4_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH4_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 4 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch4_al1_trans_count_trig;
#[doc = "Alias for channel 4 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al2_ctrl](ch4_al2_ctrl) module"]
pub type CH4_AL2_CTRL = crate::Reg<u32, _CH4_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL2_CTRL;
#[doc = "`read()` method returns [ch4_al2_ctrl::R](ch4_al2_ctrl::R) reader structure"]
impl crate::Readable for CH4_AL2_CTRL {}
#[doc = "Alias for channel 4 CTRL register"]
pub mod ch4_al2_ctrl;
#[doc = "Alias for channel 4 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al2_trans_count](ch4_al2_trans_count) module"]
pub type CH4_AL2_TRANS_COUNT = crate::Reg<u32, _CH4_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch4_al2_trans_count::R](ch4_al2_trans_count::R) reader structure"]
impl crate::Readable for CH4_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 4 TRANS_COUNT register"]
pub mod ch4_al2_trans_count;
#[doc = "Alias for channel 4 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al2_read_addr](ch4_al2_read_addr) module"]
pub type CH4_AL2_READ_ADDR = crate::Reg<u32, _CH4_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch4_al2_read_addr::R](ch4_al2_read_addr::R) reader structure"]
impl crate::Readable for CH4_AL2_READ_ADDR {}
#[doc = "Alias for channel 4 READ_ADDR register"]
pub mod ch4_al2_read_addr;
#[doc = "Alias for channel 4 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al2_write_addr_trig](ch4_al2_write_addr_trig) module"]
pub type CH4_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH4_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch4_al2_write_addr_trig::R](ch4_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH4_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 4 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch4_al2_write_addr_trig;
#[doc = "Alias for channel 4 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al3_ctrl](ch4_al3_ctrl) module"]
pub type CH4_AL3_CTRL = crate::Reg<u32, _CH4_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL3_CTRL;
#[doc = "`read()` method returns [ch4_al3_ctrl::R](ch4_al3_ctrl::R) reader structure"]
impl crate::Readable for CH4_AL3_CTRL {}
#[doc = "Alias for channel 4 CTRL register"]
pub mod ch4_al3_ctrl;
#[doc = "Alias for channel 4 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al3_write_addr](ch4_al3_write_addr) module"]
pub type CH4_AL3_WRITE_ADDR = crate::Reg<u32, _CH4_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch4_al3_write_addr::R](ch4_al3_write_addr::R) reader structure"]
impl crate::Readable for CH4_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 4 WRITE_ADDR register"]
pub mod ch4_al3_write_addr;
#[doc = "Alias for channel 4 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al3_trans_count](ch4_al3_trans_count) module"]
pub type CH4_AL3_TRANS_COUNT = crate::Reg<u32, _CH4_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch4_al3_trans_count::R](ch4_al3_trans_count::R) reader structure"]
impl crate::Readable for CH4_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 4 TRANS_COUNT register"]
pub mod ch4_al3_trans_count;
#[doc = "Alias for channel 4 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_al3_read_addr_trig](ch4_al3_read_addr_trig) module"]
pub type CH4_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH4_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch4_al3_read_addr_trig::R](ch4_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH4_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 4 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch4_al3_read_addr_trig;
#[doc = "DMA Channel 5 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_read_addr](ch5_read_addr) module"]
pub type CH5_READ_ADDR = crate::Reg<u32, _CH5_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_READ_ADDR;
#[doc = "`read()` method returns [ch5_read_addr::R](ch5_read_addr::R) reader structure"]
impl crate::Readable for CH5_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch5_read_addr::W](ch5_read_addr::W) writer structure"]
impl crate::Writable for CH5_READ_ADDR {}
#[doc = "DMA Channel 5 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch5_read_addr;
#[doc = "DMA Channel 5 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_write_addr](ch5_write_addr) module"]
pub type CH5_WRITE_ADDR = crate::Reg<u32, _CH5_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_WRITE_ADDR;
#[doc = "`read()` method returns [ch5_write_addr::R](ch5_write_addr::R) reader structure"]
impl crate::Readable for CH5_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch5_write_addr::W](ch5_write_addr::W) writer structure"]
impl crate::Writable for CH5_WRITE_ADDR {}
#[doc = "DMA Channel 5 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch5_write_addr;
#[doc = "DMA Channel 5 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_trans_count](ch5_trans_count) module"]
pub type CH5_TRANS_COUNT = crate::Reg<u32, _CH5_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_TRANS_COUNT;
#[doc = "`read()` method returns [ch5_trans_count::R](ch5_trans_count::R) reader structure"]
impl crate::Readable for CH5_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch5_trans_count::W](ch5_trans_count::W) writer structure"]
impl crate::Writable for CH5_TRANS_COUNT {}
#[doc = "DMA Channel 5 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch5_trans_count;
#[doc = "DMA Channel 5 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_ctrl_trig](ch5_ctrl_trig) module"]
pub type CH5_CTRL_TRIG = crate::Reg<u32, _CH5_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CTRL_TRIG;
#[doc = "`read()` method returns [ch5_ctrl_trig::R](ch5_ctrl_trig::R) reader structure"]
impl crate::Readable for CH5_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch5_ctrl_trig::W](ch5_ctrl_trig::W) writer structure"]
impl crate::Writable for CH5_CTRL_TRIG {}
#[doc = "DMA Channel 5 Control and Status"]
pub mod ch5_ctrl_trig;
#[doc = "Alias for channel 5 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al1_ctrl](ch5_al1_ctrl) module"]
pub type CH5_AL1_CTRL = crate::Reg<u32, _CH5_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL1_CTRL;
#[doc = "`read()` method returns [ch5_al1_ctrl::R](ch5_al1_ctrl::R) reader structure"]
impl crate::Readable for CH5_AL1_CTRL {}
#[doc = "Alias for channel 5 CTRL register"]
pub mod ch5_al1_ctrl;
#[doc = "Alias for channel 5 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al1_read_addr](ch5_al1_read_addr) module"]
pub type CH5_AL1_READ_ADDR = crate::Reg<u32, _CH5_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch5_al1_read_addr::R](ch5_al1_read_addr::R) reader structure"]
impl crate::Readable for CH5_AL1_READ_ADDR {}
#[doc = "Alias for channel 5 READ_ADDR register"]
pub mod ch5_al1_read_addr;
#[doc = "Alias for channel 5 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al1_write_addr](ch5_al1_write_addr) module"]
pub type CH5_AL1_WRITE_ADDR = crate::Reg<u32, _CH5_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch5_al1_write_addr::R](ch5_al1_write_addr::R) reader structure"]
impl crate::Readable for CH5_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 5 WRITE_ADDR register"]
pub mod ch5_al1_write_addr;
#[doc = "Alias for channel 5 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al1_trans_count_trig](ch5_al1_trans_count_trig) module"]
pub type CH5_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH5_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch5_al1_trans_count_trig::R](ch5_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH5_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 5 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch5_al1_trans_count_trig;
#[doc = "Alias for channel 5 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al2_ctrl](ch5_al2_ctrl) module"]
pub type CH5_AL2_CTRL = crate::Reg<u32, _CH5_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL2_CTRL;
#[doc = "`read()` method returns [ch5_al2_ctrl::R](ch5_al2_ctrl::R) reader structure"]
impl crate::Readable for CH5_AL2_CTRL {}
#[doc = "Alias for channel 5 CTRL register"]
pub mod ch5_al2_ctrl;
#[doc = "Alias for channel 5 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al2_trans_count](ch5_al2_trans_count) module"]
pub type CH5_AL2_TRANS_COUNT = crate::Reg<u32, _CH5_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch5_al2_trans_count::R](ch5_al2_trans_count::R) reader structure"]
impl crate::Readable for CH5_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 5 TRANS_COUNT register"]
pub mod ch5_al2_trans_count;
#[doc = "Alias for channel 5 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al2_read_addr](ch5_al2_read_addr) module"]
pub type CH5_AL2_READ_ADDR = crate::Reg<u32, _CH5_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch5_al2_read_addr::R](ch5_al2_read_addr::R) reader structure"]
impl crate::Readable for CH5_AL2_READ_ADDR {}
#[doc = "Alias for channel 5 READ_ADDR register"]
pub mod ch5_al2_read_addr;
#[doc = "Alias for channel 5 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al2_write_addr_trig](ch5_al2_write_addr_trig) module"]
pub type CH5_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH5_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch5_al2_write_addr_trig::R](ch5_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH5_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 5 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch5_al2_write_addr_trig;
#[doc = "Alias for channel 5 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al3_ctrl](ch5_al3_ctrl) module"]
pub type CH5_AL3_CTRL = crate::Reg<u32, _CH5_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL3_CTRL;
#[doc = "`read()` method returns [ch5_al3_ctrl::R](ch5_al3_ctrl::R) reader structure"]
impl crate::Readable for CH5_AL3_CTRL {}
#[doc = "Alias for channel 5 CTRL register"]
pub mod ch5_al3_ctrl;
#[doc = "Alias for channel 5 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al3_write_addr](ch5_al3_write_addr) module"]
pub type CH5_AL3_WRITE_ADDR = crate::Reg<u32, _CH5_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch5_al3_write_addr::R](ch5_al3_write_addr::R) reader structure"]
impl crate::Readable for CH5_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 5 WRITE_ADDR register"]
pub mod ch5_al3_write_addr;
#[doc = "Alias for channel 5 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al3_trans_count](ch5_al3_trans_count) module"]
pub type CH5_AL3_TRANS_COUNT = crate::Reg<u32, _CH5_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch5_al3_trans_count::R](ch5_al3_trans_count::R) reader structure"]
impl crate::Readable for CH5_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 5 TRANS_COUNT register"]
pub mod ch5_al3_trans_count;
#[doc = "Alias for channel 5 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_al3_read_addr_trig](ch5_al3_read_addr_trig) module"]
pub type CH5_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH5_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch5_al3_read_addr_trig::R](ch5_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH5_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 5 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch5_al3_read_addr_trig;
#[doc = "DMA Channel 6 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_read_addr](ch6_read_addr) module"]
pub type CH6_READ_ADDR = crate::Reg<u32, _CH6_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_READ_ADDR;
#[doc = "`read()` method returns [ch6_read_addr::R](ch6_read_addr::R) reader structure"]
impl crate::Readable for CH6_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch6_read_addr::W](ch6_read_addr::W) writer structure"]
impl crate::Writable for CH6_READ_ADDR {}
#[doc = "DMA Channel 6 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch6_read_addr;
#[doc = "DMA Channel 6 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_write_addr](ch6_write_addr) module"]
pub type CH6_WRITE_ADDR = crate::Reg<u32, _CH6_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_WRITE_ADDR;
#[doc = "`read()` method returns [ch6_write_addr::R](ch6_write_addr::R) reader structure"]
impl crate::Readable for CH6_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch6_write_addr::W](ch6_write_addr::W) writer structure"]
impl crate::Writable for CH6_WRITE_ADDR {}
#[doc = "DMA Channel 6 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch6_write_addr;
#[doc = "DMA Channel 6 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_trans_count](ch6_trans_count) module"]
pub type CH6_TRANS_COUNT = crate::Reg<u32, _CH6_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_TRANS_COUNT;
#[doc = "`read()` method returns [ch6_trans_count::R](ch6_trans_count::R) reader structure"]
impl crate::Readable for CH6_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch6_trans_count::W](ch6_trans_count::W) writer structure"]
impl crate::Writable for CH6_TRANS_COUNT {}
#[doc = "DMA Channel 6 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch6_trans_count;
#[doc = "DMA Channel 6 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_ctrl_trig](ch6_ctrl_trig) module"]
pub type CH6_CTRL_TRIG = crate::Reg<u32, _CH6_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CTRL_TRIG;
#[doc = "`read()` method returns [ch6_ctrl_trig::R](ch6_ctrl_trig::R) reader structure"]
impl crate::Readable for CH6_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch6_ctrl_trig::W](ch6_ctrl_trig::W) writer structure"]
impl crate::Writable for CH6_CTRL_TRIG {}
#[doc = "DMA Channel 6 Control and Status"]
pub mod ch6_ctrl_trig;
#[doc = "Alias for channel 6 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al1_ctrl](ch6_al1_ctrl) module"]
pub type CH6_AL1_CTRL = crate::Reg<u32, _CH6_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL1_CTRL;
#[doc = "`read()` method returns [ch6_al1_ctrl::R](ch6_al1_ctrl::R) reader structure"]
impl crate::Readable for CH6_AL1_CTRL {}
#[doc = "Alias for channel 6 CTRL register"]
pub mod ch6_al1_ctrl;
#[doc = "Alias for channel 6 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al1_read_addr](ch6_al1_read_addr) module"]
pub type CH6_AL1_READ_ADDR = crate::Reg<u32, _CH6_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch6_al1_read_addr::R](ch6_al1_read_addr::R) reader structure"]
impl crate::Readable for CH6_AL1_READ_ADDR {}
#[doc = "Alias for channel 6 READ_ADDR register"]
pub mod ch6_al1_read_addr;
#[doc = "Alias for channel 6 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al1_write_addr](ch6_al1_write_addr) module"]
pub type CH6_AL1_WRITE_ADDR = crate::Reg<u32, _CH6_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch6_al1_write_addr::R](ch6_al1_write_addr::R) reader structure"]
impl crate::Readable for CH6_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 6 WRITE_ADDR register"]
pub mod ch6_al1_write_addr;
#[doc = "Alias for channel 6 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al1_trans_count_trig](ch6_al1_trans_count_trig) module"]
pub type CH6_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH6_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch6_al1_trans_count_trig::R](ch6_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH6_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 6 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch6_al1_trans_count_trig;
#[doc = "Alias for channel 6 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al2_ctrl](ch6_al2_ctrl) module"]
pub type CH6_AL2_CTRL = crate::Reg<u32, _CH6_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL2_CTRL;
#[doc = "`read()` method returns [ch6_al2_ctrl::R](ch6_al2_ctrl::R) reader structure"]
impl crate::Readable for CH6_AL2_CTRL {}
#[doc = "Alias for channel 6 CTRL register"]
pub mod ch6_al2_ctrl;
#[doc = "Alias for channel 6 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al2_trans_count](ch6_al2_trans_count) module"]
pub type CH6_AL2_TRANS_COUNT = crate::Reg<u32, _CH6_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch6_al2_trans_count::R](ch6_al2_trans_count::R) reader structure"]
impl crate::Readable for CH6_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 6 TRANS_COUNT register"]
pub mod ch6_al2_trans_count;
#[doc = "Alias for channel 6 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al2_read_addr](ch6_al2_read_addr) module"]
pub type CH6_AL2_READ_ADDR = crate::Reg<u32, _CH6_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch6_al2_read_addr::R](ch6_al2_read_addr::R) reader structure"]
impl crate::Readable for CH6_AL2_READ_ADDR {}
#[doc = "Alias for channel 6 READ_ADDR register"]
pub mod ch6_al2_read_addr;
#[doc = "Alias for channel 6 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al2_write_addr_trig](ch6_al2_write_addr_trig) module"]
pub type CH6_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH6_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch6_al2_write_addr_trig::R](ch6_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH6_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 6 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch6_al2_write_addr_trig;
#[doc = "Alias for channel 6 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al3_ctrl](ch6_al3_ctrl) module"]
pub type CH6_AL3_CTRL = crate::Reg<u32, _CH6_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL3_CTRL;
#[doc = "`read()` method returns [ch6_al3_ctrl::R](ch6_al3_ctrl::R) reader structure"]
impl crate::Readable for CH6_AL3_CTRL {}
#[doc = "Alias for channel 6 CTRL register"]
pub mod ch6_al3_ctrl;
#[doc = "Alias for channel 6 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al3_write_addr](ch6_al3_write_addr) module"]
pub type CH6_AL3_WRITE_ADDR = crate::Reg<u32, _CH6_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch6_al3_write_addr::R](ch6_al3_write_addr::R) reader structure"]
impl crate::Readable for CH6_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 6 WRITE_ADDR register"]
pub mod ch6_al3_write_addr;
#[doc = "Alias for channel 6 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al3_trans_count](ch6_al3_trans_count) module"]
pub type CH6_AL3_TRANS_COUNT = crate::Reg<u32, _CH6_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch6_al3_trans_count::R](ch6_al3_trans_count::R) reader structure"]
impl crate::Readable for CH6_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 6 TRANS_COUNT register"]
pub mod ch6_al3_trans_count;
#[doc = "Alias for channel 6 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_al3_read_addr_trig](ch6_al3_read_addr_trig) module"]
pub type CH6_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH6_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch6_al3_read_addr_trig::R](ch6_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH6_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 6 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch6_al3_read_addr_trig;
#[doc = "DMA Channel 7 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_read_addr](ch7_read_addr) module"]
pub type CH7_READ_ADDR = crate::Reg<u32, _CH7_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_READ_ADDR;
#[doc = "`read()` method returns [ch7_read_addr::R](ch7_read_addr::R) reader structure"]
impl crate::Readable for CH7_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch7_read_addr::W](ch7_read_addr::W) writer structure"]
impl crate::Writable for CH7_READ_ADDR {}
#[doc = "DMA Channel 7 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch7_read_addr;
#[doc = "DMA Channel 7 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_write_addr](ch7_write_addr) module"]
pub type CH7_WRITE_ADDR = crate::Reg<u32, _CH7_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_WRITE_ADDR;
#[doc = "`read()` method returns [ch7_write_addr::R](ch7_write_addr::R) reader structure"]
impl crate::Readable for CH7_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch7_write_addr::W](ch7_write_addr::W) writer structure"]
impl crate::Writable for CH7_WRITE_ADDR {}
#[doc = "DMA Channel 7 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch7_write_addr;
#[doc = "DMA Channel 7 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_trans_count](ch7_trans_count) module"]
pub type CH7_TRANS_COUNT = crate::Reg<u32, _CH7_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_TRANS_COUNT;
#[doc = "`read()` method returns [ch7_trans_count::R](ch7_trans_count::R) reader structure"]
impl crate::Readable for CH7_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch7_trans_count::W](ch7_trans_count::W) writer structure"]
impl crate::Writable for CH7_TRANS_COUNT {}
#[doc = "DMA Channel 7 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch7_trans_count;
#[doc = "DMA Channel 7 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_ctrl_trig](ch7_ctrl_trig) module"]
pub type CH7_CTRL_TRIG = crate::Reg<u32, _CH7_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CTRL_TRIG;
#[doc = "`read()` method returns [ch7_ctrl_trig::R](ch7_ctrl_trig::R) reader structure"]
impl crate::Readable for CH7_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch7_ctrl_trig::W](ch7_ctrl_trig::W) writer structure"]
impl crate::Writable for CH7_CTRL_TRIG {}
#[doc = "DMA Channel 7 Control and Status"]
pub mod ch7_ctrl_trig;
#[doc = "Alias for channel 7 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al1_ctrl](ch7_al1_ctrl) module"]
pub type CH7_AL1_CTRL = crate::Reg<u32, _CH7_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL1_CTRL;
#[doc = "`read()` method returns [ch7_al1_ctrl::R](ch7_al1_ctrl::R) reader structure"]
impl crate::Readable for CH7_AL1_CTRL {}
#[doc = "Alias for channel 7 CTRL register"]
pub mod ch7_al1_ctrl;
#[doc = "Alias for channel 7 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al1_read_addr](ch7_al1_read_addr) module"]
pub type CH7_AL1_READ_ADDR = crate::Reg<u32, _CH7_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch7_al1_read_addr::R](ch7_al1_read_addr::R) reader structure"]
impl crate::Readable for CH7_AL1_READ_ADDR {}
#[doc = "Alias for channel 7 READ_ADDR register"]
pub mod ch7_al1_read_addr;
#[doc = "Alias for channel 7 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al1_write_addr](ch7_al1_write_addr) module"]
pub type CH7_AL1_WRITE_ADDR = crate::Reg<u32, _CH7_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch7_al1_write_addr::R](ch7_al1_write_addr::R) reader structure"]
impl crate::Readable for CH7_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 7 WRITE_ADDR register"]
pub mod ch7_al1_write_addr;
#[doc = "Alias for channel 7 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al1_trans_count_trig](ch7_al1_trans_count_trig) module"]
pub type CH7_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH7_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch7_al1_trans_count_trig::R](ch7_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH7_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 7 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch7_al1_trans_count_trig;
#[doc = "Alias for channel 7 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al2_ctrl](ch7_al2_ctrl) module"]
pub type CH7_AL2_CTRL = crate::Reg<u32, _CH7_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL2_CTRL;
#[doc = "`read()` method returns [ch7_al2_ctrl::R](ch7_al2_ctrl::R) reader structure"]
impl crate::Readable for CH7_AL2_CTRL {}
#[doc = "Alias for channel 7 CTRL register"]
pub mod ch7_al2_ctrl;
#[doc = "Alias for channel 7 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al2_trans_count](ch7_al2_trans_count) module"]
pub type CH7_AL2_TRANS_COUNT = crate::Reg<u32, _CH7_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch7_al2_trans_count::R](ch7_al2_trans_count::R) reader structure"]
impl crate::Readable for CH7_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 7 TRANS_COUNT register"]
pub mod ch7_al2_trans_count;
#[doc = "Alias for channel 7 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al2_read_addr](ch7_al2_read_addr) module"]
pub type CH7_AL2_READ_ADDR = crate::Reg<u32, _CH7_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch7_al2_read_addr::R](ch7_al2_read_addr::R) reader structure"]
impl crate::Readable for CH7_AL2_READ_ADDR {}
#[doc = "Alias for channel 7 READ_ADDR register"]
pub mod ch7_al2_read_addr;
#[doc = "Alias for channel 7 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al2_write_addr_trig](ch7_al2_write_addr_trig) module"]
pub type CH7_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH7_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch7_al2_write_addr_trig::R](ch7_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH7_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 7 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch7_al2_write_addr_trig;
#[doc = "Alias for channel 7 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al3_ctrl](ch7_al3_ctrl) module"]
pub type CH7_AL3_CTRL = crate::Reg<u32, _CH7_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL3_CTRL;
#[doc = "`read()` method returns [ch7_al3_ctrl::R](ch7_al3_ctrl::R) reader structure"]
impl crate::Readable for CH7_AL3_CTRL {}
#[doc = "Alias for channel 7 CTRL register"]
pub mod ch7_al3_ctrl;
#[doc = "Alias for channel 7 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al3_write_addr](ch7_al3_write_addr) module"]
pub type CH7_AL3_WRITE_ADDR = crate::Reg<u32, _CH7_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch7_al3_write_addr::R](ch7_al3_write_addr::R) reader structure"]
impl crate::Readable for CH7_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 7 WRITE_ADDR register"]
pub mod ch7_al3_write_addr;
#[doc = "Alias for channel 7 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al3_trans_count](ch7_al3_trans_count) module"]
pub type CH7_AL3_TRANS_COUNT = crate::Reg<u32, _CH7_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch7_al3_trans_count::R](ch7_al3_trans_count::R) reader structure"]
impl crate::Readable for CH7_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 7 TRANS_COUNT register"]
pub mod ch7_al3_trans_count;
#[doc = "Alias for channel 7 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_al3_read_addr_trig](ch7_al3_read_addr_trig) module"]
pub type CH7_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH7_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch7_al3_read_addr_trig::R](ch7_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH7_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 7 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch7_al3_read_addr_trig;
#[doc = "DMA Channel 8 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_read_addr](ch8_read_addr) module"]
pub type CH8_READ_ADDR = crate::Reg<u32, _CH8_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_READ_ADDR;
#[doc = "`read()` method returns [ch8_read_addr::R](ch8_read_addr::R) reader structure"]
impl crate::Readable for CH8_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch8_read_addr::W](ch8_read_addr::W) writer structure"]
impl crate::Writable for CH8_READ_ADDR {}
#[doc = "DMA Channel 8 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch8_read_addr;
#[doc = "DMA Channel 8 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_write_addr](ch8_write_addr) module"]
pub type CH8_WRITE_ADDR = crate::Reg<u32, _CH8_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_WRITE_ADDR;
#[doc = "`read()` method returns [ch8_write_addr::R](ch8_write_addr::R) reader structure"]
impl crate::Readable for CH8_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch8_write_addr::W](ch8_write_addr::W) writer structure"]
impl crate::Writable for CH8_WRITE_ADDR {}
#[doc = "DMA Channel 8 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch8_write_addr;
#[doc = "DMA Channel 8 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_trans_count](ch8_trans_count) module"]
pub type CH8_TRANS_COUNT = crate::Reg<u32, _CH8_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_TRANS_COUNT;
#[doc = "`read()` method returns [ch8_trans_count::R](ch8_trans_count::R) reader structure"]
impl crate::Readable for CH8_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch8_trans_count::W](ch8_trans_count::W) writer structure"]
impl crate::Writable for CH8_TRANS_COUNT {}
#[doc = "DMA Channel 8 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch8_trans_count;
#[doc = "DMA Channel 8 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_ctrl_trig](ch8_ctrl_trig) module"]
pub type CH8_CTRL_TRIG = crate::Reg<u32, _CH8_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_CTRL_TRIG;
#[doc = "`read()` method returns [ch8_ctrl_trig::R](ch8_ctrl_trig::R) reader structure"]
impl crate::Readable for CH8_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch8_ctrl_trig::W](ch8_ctrl_trig::W) writer structure"]
impl crate::Writable for CH8_CTRL_TRIG {}
#[doc = "DMA Channel 8 Control and Status"]
pub mod ch8_ctrl_trig;
#[doc = "Alias for channel 8 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al1_ctrl](ch8_al1_ctrl) module"]
pub type CH8_AL1_CTRL = crate::Reg<u32, _CH8_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL1_CTRL;
#[doc = "`read()` method returns [ch8_al1_ctrl::R](ch8_al1_ctrl::R) reader structure"]
impl crate::Readable for CH8_AL1_CTRL {}
#[doc = "Alias for channel 8 CTRL register"]
pub mod ch8_al1_ctrl;
#[doc = "Alias for channel 8 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al1_read_addr](ch8_al1_read_addr) module"]
pub type CH8_AL1_READ_ADDR = crate::Reg<u32, _CH8_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch8_al1_read_addr::R](ch8_al1_read_addr::R) reader structure"]
impl crate::Readable for CH8_AL1_READ_ADDR {}
#[doc = "Alias for channel 8 READ_ADDR register"]
pub mod ch8_al1_read_addr;
#[doc = "Alias for channel 8 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al1_write_addr](ch8_al1_write_addr) module"]
pub type CH8_AL1_WRITE_ADDR = crate::Reg<u32, _CH8_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch8_al1_write_addr::R](ch8_al1_write_addr::R) reader structure"]
impl crate::Readable for CH8_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 8 WRITE_ADDR register"]
pub mod ch8_al1_write_addr;
#[doc = "Alias for channel 8 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al1_trans_count_trig](ch8_al1_trans_count_trig) module"]
pub type CH8_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH8_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch8_al1_trans_count_trig::R](ch8_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH8_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 8 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch8_al1_trans_count_trig;
#[doc = "Alias for channel 8 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al2_ctrl](ch8_al2_ctrl) module"]
pub type CH8_AL2_CTRL = crate::Reg<u32, _CH8_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL2_CTRL;
#[doc = "`read()` method returns [ch8_al2_ctrl::R](ch8_al2_ctrl::R) reader structure"]
impl crate::Readable for CH8_AL2_CTRL {}
#[doc = "Alias for channel 8 CTRL register"]
pub mod ch8_al2_ctrl;
#[doc = "Alias for channel 8 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al2_trans_count](ch8_al2_trans_count) module"]
pub type CH8_AL2_TRANS_COUNT = crate::Reg<u32, _CH8_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch8_al2_trans_count::R](ch8_al2_trans_count::R) reader structure"]
impl crate::Readable for CH8_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 8 TRANS_COUNT register"]
pub mod ch8_al2_trans_count;
#[doc = "Alias for channel 8 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al2_read_addr](ch8_al2_read_addr) module"]
pub type CH8_AL2_READ_ADDR = crate::Reg<u32, _CH8_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch8_al2_read_addr::R](ch8_al2_read_addr::R) reader structure"]
impl crate::Readable for CH8_AL2_READ_ADDR {}
#[doc = "Alias for channel 8 READ_ADDR register"]
pub mod ch8_al2_read_addr;
#[doc = "Alias for channel 8 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al2_write_addr_trig](ch8_al2_write_addr_trig) module"]
pub type CH8_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH8_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch8_al2_write_addr_trig::R](ch8_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH8_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 8 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch8_al2_write_addr_trig;
#[doc = "Alias for channel 8 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al3_ctrl](ch8_al3_ctrl) module"]
pub type CH8_AL3_CTRL = crate::Reg<u32, _CH8_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL3_CTRL;
#[doc = "`read()` method returns [ch8_al3_ctrl::R](ch8_al3_ctrl::R) reader structure"]
impl crate::Readable for CH8_AL3_CTRL {}
#[doc = "Alias for channel 8 CTRL register"]
pub mod ch8_al3_ctrl;
#[doc = "Alias for channel 8 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al3_write_addr](ch8_al3_write_addr) module"]
pub type CH8_AL3_WRITE_ADDR = crate::Reg<u32, _CH8_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch8_al3_write_addr::R](ch8_al3_write_addr::R) reader structure"]
impl crate::Readable for CH8_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 8 WRITE_ADDR register"]
pub mod ch8_al3_write_addr;
#[doc = "Alias for channel 8 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al3_trans_count](ch8_al3_trans_count) module"]
pub type CH8_AL3_TRANS_COUNT = crate::Reg<u32, _CH8_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch8_al3_trans_count::R](ch8_al3_trans_count::R) reader structure"]
impl crate::Readable for CH8_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 8 TRANS_COUNT register"]
pub mod ch8_al3_trans_count;
#[doc = "Alias for channel 8 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_al3_read_addr_trig](ch8_al3_read_addr_trig) module"]
pub type CH8_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH8_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch8_al3_read_addr_trig::R](ch8_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH8_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 8 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch8_al3_read_addr_trig;
#[doc = "DMA Channel 9 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_read_addr](ch9_read_addr) module"]
pub type CH9_READ_ADDR = crate::Reg<u32, _CH9_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_READ_ADDR;
#[doc = "`read()` method returns [ch9_read_addr::R](ch9_read_addr::R) reader structure"]
impl crate::Readable for CH9_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch9_read_addr::W](ch9_read_addr::W) writer structure"]
impl crate::Writable for CH9_READ_ADDR {}
#[doc = "DMA Channel 9 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch9_read_addr;
#[doc = "DMA Channel 9 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_write_addr](ch9_write_addr) module"]
pub type CH9_WRITE_ADDR = crate::Reg<u32, _CH9_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_WRITE_ADDR;
#[doc = "`read()` method returns [ch9_write_addr::R](ch9_write_addr::R) reader structure"]
impl crate::Readable for CH9_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch9_write_addr::W](ch9_write_addr::W) writer structure"]
impl crate::Writable for CH9_WRITE_ADDR {}
#[doc = "DMA Channel 9 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch9_write_addr;
#[doc = "DMA Channel 9 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_trans_count](ch9_trans_count) module"]
pub type CH9_TRANS_COUNT = crate::Reg<u32, _CH9_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_TRANS_COUNT;
#[doc = "`read()` method returns [ch9_trans_count::R](ch9_trans_count::R) reader structure"]
impl crate::Readable for CH9_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch9_trans_count::W](ch9_trans_count::W) writer structure"]
impl crate::Writable for CH9_TRANS_COUNT {}
#[doc = "DMA Channel 9 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch9_trans_count;
#[doc = "DMA Channel 9 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_ctrl_trig](ch9_ctrl_trig) module"]
pub type CH9_CTRL_TRIG = crate::Reg<u32, _CH9_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_CTRL_TRIG;
#[doc = "`read()` method returns [ch9_ctrl_trig::R](ch9_ctrl_trig::R) reader structure"]
impl crate::Readable for CH9_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch9_ctrl_trig::W](ch9_ctrl_trig::W) writer structure"]
impl crate::Writable for CH9_CTRL_TRIG {}
#[doc = "DMA Channel 9 Control and Status"]
pub mod ch9_ctrl_trig;
#[doc = "Alias for channel 9 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al1_ctrl](ch9_al1_ctrl) module"]
pub type CH9_AL1_CTRL = crate::Reg<u32, _CH9_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL1_CTRL;
#[doc = "`read()` method returns [ch9_al1_ctrl::R](ch9_al1_ctrl::R) reader structure"]
impl crate::Readable for CH9_AL1_CTRL {}
#[doc = "Alias for channel 9 CTRL register"]
pub mod ch9_al1_ctrl;
#[doc = "Alias for channel 9 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al1_read_addr](ch9_al1_read_addr) module"]
pub type CH9_AL1_READ_ADDR = crate::Reg<u32, _CH9_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch9_al1_read_addr::R](ch9_al1_read_addr::R) reader structure"]
impl crate::Readable for CH9_AL1_READ_ADDR {}
#[doc = "Alias for channel 9 READ_ADDR register"]
pub mod ch9_al1_read_addr;
#[doc = "Alias for channel 9 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al1_write_addr](ch9_al1_write_addr) module"]
pub type CH9_AL1_WRITE_ADDR = crate::Reg<u32, _CH9_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch9_al1_write_addr::R](ch9_al1_write_addr::R) reader structure"]
impl crate::Readable for CH9_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 9 WRITE_ADDR register"]
pub mod ch9_al1_write_addr;
#[doc = "Alias for channel 9 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al1_trans_count_trig](ch9_al1_trans_count_trig) module"]
pub type CH9_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH9_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch9_al1_trans_count_trig::R](ch9_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH9_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 9 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch9_al1_trans_count_trig;
#[doc = "Alias for channel 9 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al2_ctrl](ch9_al2_ctrl) module"]
pub type CH9_AL2_CTRL = crate::Reg<u32, _CH9_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL2_CTRL;
#[doc = "`read()` method returns [ch9_al2_ctrl::R](ch9_al2_ctrl::R) reader structure"]
impl crate::Readable for CH9_AL2_CTRL {}
#[doc = "Alias for channel 9 CTRL register"]
pub mod ch9_al2_ctrl;
#[doc = "Alias for channel 9 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al2_trans_count](ch9_al2_trans_count) module"]
pub type CH9_AL2_TRANS_COUNT = crate::Reg<u32, _CH9_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch9_al2_trans_count::R](ch9_al2_trans_count::R) reader structure"]
impl crate::Readable for CH9_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 9 TRANS_COUNT register"]
pub mod ch9_al2_trans_count;
#[doc = "Alias for channel 9 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al2_read_addr](ch9_al2_read_addr) module"]
pub type CH9_AL2_READ_ADDR = crate::Reg<u32, _CH9_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch9_al2_read_addr::R](ch9_al2_read_addr::R) reader structure"]
impl crate::Readable for CH9_AL2_READ_ADDR {}
#[doc = "Alias for channel 9 READ_ADDR register"]
pub mod ch9_al2_read_addr;
#[doc = "Alias for channel 9 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al2_write_addr_trig](ch9_al2_write_addr_trig) module"]
pub type CH9_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH9_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch9_al2_write_addr_trig::R](ch9_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH9_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 9 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch9_al2_write_addr_trig;
#[doc = "Alias for channel 9 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al3_ctrl](ch9_al3_ctrl) module"]
pub type CH9_AL3_CTRL = crate::Reg<u32, _CH9_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL3_CTRL;
#[doc = "`read()` method returns [ch9_al3_ctrl::R](ch9_al3_ctrl::R) reader structure"]
impl crate::Readable for CH9_AL3_CTRL {}
#[doc = "Alias for channel 9 CTRL register"]
pub mod ch9_al3_ctrl;
#[doc = "Alias for channel 9 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al3_write_addr](ch9_al3_write_addr) module"]
pub type CH9_AL3_WRITE_ADDR = crate::Reg<u32, _CH9_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch9_al3_write_addr::R](ch9_al3_write_addr::R) reader structure"]
impl crate::Readable for CH9_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 9 WRITE_ADDR register"]
pub mod ch9_al3_write_addr;
#[doc = "Alias for channel 9 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al3_trans_count](ch9_al3_trans_count) module"]
pub type CH9_AL3_TRANS_COUNT = crate::Reg<u32, _CH9_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch9_al3_trans_count::R](ch9_al3_trans_count::R) reader structure"]
impl crate::Readable for CH9_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 9 TRANS_COUNT register"]
pub mod ch9_al3_trans_count;
#[doc = "Alias for channel 9 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_al3_read_addr_trig](ch9_al3_read_addr_trig) module"]
pub type CH9_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH9_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch9_al3_read_addr_trig::R](ch9_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH9_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 9 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch9_al3_read_addr_trig;
#[doc = "DMA Channel 10 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_read_addr](ch10_read_addr) module"]
pub type CH10_READ_ADDR = crate::Reg<u32, _CH10_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_READ_ADDR;
#[doc = "`read()` method returns [ch10_read_addr::R](ch10_read_addr::R) reader structure"]
impl crate::Readable for CH10_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch10_read_addr::W](ch10_read_addr::W) writer structure"]
impl crate::Writable for CH10_READ_ADDR {}
#[doc = "DMA Channel 10 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch10_read_addr;
#[doc = "DMA Channel 10 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_write_addr](ch10_write_addr) module"]
pub type CH10_WRITE_ADDR = crate::Reg<u32, _CH10_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_WRITE_ADDR;
#[doc = "`read()` method returns [ch10_write_addr::R](ch10_write_addr::R) reader structure"]
impl crate::Readable for CH10_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch10_write_addr::W](ch10_write_addr::W) writer structure"]
impl crate::Writable for CH10_WRITE_ADDR {}
#[doc = "DMA Channel 10 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch10_write_addr;
#[doc = "DMA Channel 10 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_trans_count](ch10_trans_count) module"]
pub type CH10_TRANS_COUNT = crate::Reg<u32, _CH10_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_TRANS_COUNT;
#[doc = "`read()` method returns [ch10_trans_count::R](ch10_trans_count::R) reader structure"]
impl crate::Readable for CH10_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch10_trans_count::W](ch10_trans_count::W) writer structure"]
impl crate::Writable for CH10_TRANS_COUNT {}
#[doc = "DMA Channel 10 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch10_trans_count;
#[doc = "DMA Channel 10 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_ctrl_trig](ch10_ctrl_trig) module"]
pub type CH10_CTRL_TRIG = crate::Reg<u32, _CH10_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_CTRL_TRIG;
#[doc = "`read()` method returns [ch10_ctrl_trig::R](ch10_ctrl_trig::R) reader structure"]
impl crate::Readable for CH10_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch10_ctrl_trig::W](ch10_ctrl_trig::W) writer structure"]
impl crate::Writable for CH10_CTRL_TRIG {}
#[doc = "DMA Channel 10 Control and Status"]
pub mod ch10_ctrl_trig;
#[doc = "Alias for channel 10 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al1_ctrl](ch10_al1_ctrl) module"]
pub type CH10_AL1_CTRL = crate::Reg<u32, _CH10_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL1_CTRL;
#[doc = "`read()` method returns [ch10_al1_ctrl::R](ch10_al1_ctrl::R) reader structure"]
impl crate::Readable for CH10_AL1_CTRL {}
#[doc = "Alias for channel 10 CTRL register"]
pub mod ch10_al1_ctrl;
#[doc = "Alias for channel 10 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al1_read_addr](ch10_al1_read_addr) module"]
pub type CH10_AL1_READ_ADDR = crate::Reg<u32, _CH10_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch10_al1_read_addr::R](ch10_al1_read_addr::R) reader structure"]
impl crate::Readable for CH10_AL1_READ_ADDR {}
#[doc = "Alias for channel 10 READ_ADDR register"]
pub mod ch10_al1_read_addr;
#[doc = "Alias for channel 10 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al1_write_addr](ch10_al1_write_addr) module"]
pub type CH10_AL1_WRITE_ADDR = crate::Reg<u32, _CH10_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch10_al1_write_addr::R](ch10_al1_write_addr::R) reader structure"]
impl crate::Readable for CH10_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 10 WRITE_ADDR register"]
pub mod ch10_al1_write_addr;
#[doc = "Alias for channel 10 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al1_trans_count_trig](ch10_al1_trans_count_trig) module"]
pub type CH10_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH10_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch10_al1_trans_count_trig::R](ch10_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH10_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 10 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch10_al1_trans_count_trig;
#[doc = "Alias for channel 10 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al2_ctrl](ch10_al2_ctrl) module"]
pub type CH10_AL2_CTRL = crate::Reg<u32, _CH10_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL2_CTRL;
#[doc = "`read()` method returns [ch10_al2_ctrl::R](ch10_al2_ctrl::R) reader structure"]
impl crate::Readable for CH10_AL2_CTRL {}
#[doc = "Alias for channel 10 CTRL register"]
pub mod ch10_al2_ctrl;
#[doc = "Alias for channel 10 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al2_trans_count](ch10_al2_trans_count) module"]
pub type CH10_AL2_TRANS_COUNT = crate::Reg<u32, _CH10_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch10_al2_trans_count::R](ch10_al2_trans_count::R) reader structure"]
impl crate::Readable for CH10_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 10 TRANS_COUNT register"]
pub mod ch10_al2_trans_count;
#[doc = "Alias for channel 10 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al2_read_addr](ch10_al2_read_addr) module"]
pub type CH10_AL2_READ_ADDR = crate::Reg<u32, _CH10_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch10_al2_read_addr::R](ch10_al2_read_addr::R) reader structure"]
impl crate::Readable for CH10_AL2_READ_ADDR {}
#[doc = "Alias for channel 10 READ_ADDR register"]
pub mod ch10_al2_read_addr;
#[doc = "Alias for channel 10 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al2_write_addr_trig](ch10_al2_write_addr_trig) module"]
pub type CH10_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH10_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch10_al2_write_addr_trig::R](ch10_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH10_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 10 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch10_al2_write_addr_trig;
#[doc = "Alias for channel 10 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al3_ctrl](ch10_al3_ctrl) module"]
pub type CH10_AL3_CTRL = crate::Reg<u32, _CH10_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL3_CTRL;
#[doc = "`read()` method returns [ch10_al3_ctrl::R](ch10_al3_ctrl::R) reader structure"]
impl crate::Readable for CH10_AL3_CTRL {}
#[doc = "Alias for channel 10 CTRL register"]
pub mod ch10_al3_ctrl;
#[doc = "Alias for channel 10 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al3_write_addr](ch10_al3_write_addr) module"]
pub type CH10_AL3_WRITE_ADDR = crate::Reg<u32, _CH10_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch10_al3_write_addr::R](ch10_al3_write_addr::R) reader structure"]
impl crate::Readable for CH10_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 10 WRITE_ADDR register"]
pub mod ch10_al3_write_addr;
#[doc = "Alias for channel 10 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al3_trans_count](ch10_al3_trans_count) module"]
pub type CH10_AL3_TRANS_COUNT = crate::Reg<u32, _CH10_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch10_al3_trans_count::R](ch10_al3_trans_count::R) reader structure"]
impl crate::Readable for CH10_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 10 TRANS_COUNT register"]
pub mod ch10_al3_trans_count;
#[doc = "Alias for channel 10 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_al3_read_addr_trig](ch10_al3_read_addr_trig) module"]
pub type CH10_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH10_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch10_al3_read_addr_trig::R](ch10_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH10_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 10 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch10_al3_read_addr_trig;
#[doc = "DMA Channel 11 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_read_addr](ch11_read_addr) module"]
pub type CH11_READ_ADDR = crate::Reg<u32, _CH11_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_READ_ADDR;
#[doc = "`read()` method returns [ch11_read_addr::R](ch11_read_addr::R) reader structure"]
impl crate::Readable for CH11_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch11_read_addr::W](ch11_read_addr::W) writer structure"]
impl crate::Writable for CH11_READ_ADDR {}
#[doc = "DMA Channel 11 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch11_read_addr;
#[doc = "DMA Channel 11 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_write_addr](ch11_write_addr) module"]
pub type CH11_WRITE_ADDR = crate::Reg<u32, _CH11_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_WRITE_ADDR;
#[doc = "`read()` method returns [ch11_write_addr::R](ch11_write_addr::R) reader structure"]
impl crate::Readable for CH11_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch11_write_addr::W](ch11_write_addr::W) writer structure"]
impl crate::Writable for CH11_WRITE_ADDR {}
#[doc = "DMA Channel 11 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch11_write_addr;
#[doc = "DMA Channel 11 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_trans_count](ch11_trans_count) module"]
pub type CH11_TRANS_COUNT = crate::Reg<u32, _CH11_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_TRANS_COUNT;
#[doc = "`read()` method returns [ch11_trans_count::R](ch11_trans_count::R) reader structure"]
impl crate::Readable for CH11_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch11_trans_count::W](ch11_trans_count::W) writer structure"]
impl crate::Writable for CH11_TRANS_COUNT {}
#[doc = "DMA Channel 11 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch11_trans_count;
#[doc = "DMA Channel 11 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_ctrl_trig](ch11_ctrl_trig) module"]
pub type CH11_CTRL_TRIG = crate::Reg<u32, _CH11_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_CTRL_TRIG;
#[doc = "`read()` method returns [ch11_ctrl_trig::R](ch11_ctrl_trig::R) reader structure"]
impl crate::Readable for CH11_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch11_ctrl_trig::W](ch11_ctrl_trig::W) writer structure"]
impl crate::Writable for CH11_CTRL_TRIG {}
#[doc = "DMA Channel 11 Control and Status"]
pub mod ch11_ctrl_trig;
#[doc = "Alias for channel 11 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al1_ctrl](ch11_al1_ctrl) module"]
pub type CH11_AL1_CTRL = crate::Reg<u32, _CH11_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL1_CTRL;
#[doc = "`read()` method returns [ch11_al1_ctrl::R](ch11_al1_ctrl::R) reader structure"]
impl crate::Readable for CH11_AL1_CTRL {}
#[doc = "Alias for channel 11 CTRL register"]
pub mod ch11_al1_ctrl;
#[doc = "Alias for channel 11 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al1_read_addr](ch11_al1_read_addr) module"]
pub type CH11_AL1_READ_ADDR = crate::Reg<u32, _CH11_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch11_al1_read_addr::R](ch11_al1_read_addr::R) reader structure"]
impl crate::Readable for CH11_AL1_READ_ADDR {}
#[doc = "Alias for channel 11 READ_ADDR register"]
pub mod ch11_al1_read_addr;
#[doc = "Alias for channel 11 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al1_write_addr](ch11_al1_write_addr) module"]
pub type CH11_AL1_WRITE_ADDR = crate::Reg<u32, _CH11_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch11_al1_write_addr::R](ch11_al1_write_addr::R) reader structure"]
impl crate::Readable for CH11_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 11 WRITE_ADDR register"]
pub mod ch11_al1_write_addr;
#[doc = "Alias for channel 11 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al1_trans_count_trig](ch11_al1_trans_count_trig) module"]
pub type CH11_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH11_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch11_al1_trans_count_trig::R](ch11_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH11_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 11 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch11_al1_trans_count_trig;
#[doc = "Alias for channel 11 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al2_ctrl](ch11_al2_ctrl) module"]
pub type CH11_AL2_CTRL = crate::Reg<u32, _CH11_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL2_CTRL;
#[doc = "`read()` method returns [ch11_al2_ctrl::R](ch11_al2_ctrl::R) reader structure"]
impl crate::Readable for CH11_AL2_CTRL {}
#[doc = "Alias for channel 11 CTRL register"]
pub mod ch11_al2_ctrl;
#[doc = "Alias for channel 11 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al2_trans_count](ch11_al2_trans_count) module"]
pub type CH11_AL2_TRANS_COUNT = crate::Reg<u32, _CH11_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch11_al2_trans_count::R](ch11_al2_trans_count::R) reader structure"]
impl crate::Readable for CH11_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 11 TRANS_COUNT register"]
pub mod ch11_al2_trans_count;
#[doc = "Alias for channel 11 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al2_read_addr](ch11_al2_read_addr) module"]
pub type CH11_AL2_READ_ADDR = crate::Reg<u32, _CH11_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch11_al2_read_addr::R](ch11_al2_read_addr::R) reader structure"]
impl crate::Readable for CH11_AL2_READ_ADDR {}
#[doc = "Alias for channel 11 READ_ADDR register"]
pub mod ch11_al2_read_addr;
#[doc = "Alias for channel 11 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al2_write_addr_trig](ch11_al2_write_addr_trig) module"]
pub type CH11_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH11_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch11_al2_write_addr_trig::R](ch11_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH11_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 11 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch11_al2_write_addr_trig;
#[doc = "Alias for channel 11 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al3_ctrl](ch11_al3_ctrl) module"]
pub type CH11_AL3_CTRL = crate::Reg<u32, _CH11_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL3_CTRL;
#[doc = "`read()` method returns [ch11_al3_ctrl::R](ch11_al3_ctrl::R) reader structure"]
impl crate::Readable for CH11_AL3_CTRL {}
#[doc = "Alias for channel 11 CTRL register"]
pub mod ch11_al3_ctrl;
#[doc = "Alias for channel 11 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al3_write_addr](ch11_al3_write_addr) module"]
pub type CH11_AL3_WRITE_ADDR = crate::Reg<u32, _CH11_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch11_al3_write_addr::R](ch11_al3_write_addr::R) reader structure"]
impl crate::Readable for CH11_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 11 WRITE_ADDR register"]
pub mod ch11_al3_write_addr;
#[doc = "Alias for channel 11 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al3_trans_count](ch11_al3_trans_count) module"]
pub type CH11_AL3_TRANS_COUNT = crate::Reg<u32, _CH11_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch11_al3_trans_count::R](ch11_al3_trans_count::R) reader structure"]
impl crate::Readable for CH11_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 11 TRANS_COUNT register"]
pub mod ch11_al3_trans_count;
#[doc = "Alias for channel 11 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_al3_read_addr_trig](ch11_al3_read_addr_trig) module"]
pub type CH11_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH11_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch11_al3_read_addr_trig::R](ch11_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH11_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 11 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch11_al3_read_addr_trig;
#[doc = "Interrupt Status (raw)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "Interrupt Status (raw)"]
pub mod intr;
#[doc = "Interrupt Enables for IRQ 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inte0](inte0) module"]
pub type INTE0 = crate::Reg<u32, _INTE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTE0;
#[doc = "`read()` method returns [inte0::R](inte0::R) reader structure"]
impl crate::Readable for INTE0 {}
#[doc = "`write(|w| ..)` method takes [inte0::W](inte0::W) writer structure"]
impl crate::Writable for INTE0 {}
#[doc = "Interrupt Enables for IRQ 0"]
pub mod inte0;
#[doc = "Force Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf0](intf0) module"]
pub type INTF0 = crate::Reg<u32, _INTF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF0;
#[doc = "`read()` method returns [intf0::R](intf0::R) reader structure"]
impl crate::Readable for INTF0 {}
#[doc = "`write(|w| ..)` method takes [intf0::W](intf0::W) writer structure"]
impl crate::Writable for INTF0 {}
#[doc = "Force Interrupts"]
pub mod intf0;
#[doc = "Interrupt Status for IRQ 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ints0](ints0) module"]
pub type INTS0 = crate::Reg<u32, _INTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTS0;
#[doc = "`read()` method returns [ints0::R](ints0::R) reader structure"]
impl crate::Readable for INTS0 {}
#[doc = "`write(|w| ..)` method takes [ints0::W](ints0::W) writer structure"]
impl crate::Writable for INTS0 {}
#[doc = "Interrupt Status for IRQ 0"]
pub mod ints0;
#[doc = "Interrupt Enables for IRQ 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inte1](inte1) module"]
pub type INTE1 = crate::Reg<u32, _INTE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTE1;
#[doc = "`read()` method returns [inte1::R](inte1::R) reader structure"]
impl crate::Readable for INTE1 {}
#[doc = "`write(|w| ..)` method takes [inte1::W](inte1::W) writer structure"]
impl crate::Writable for INTE1 {}
#[doc = "Interrupt Enables for IRQ 1"]
pub mod inte1;
#[doc = "Force Interrupts for IRQ 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf1](intf1) module"]
pub type INTF1 = crate::Reg<u32, _INTF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF1;
#[doc = "`read()` method returns [intf1::R](intf1::R) reader structure"]
impl crate::Readable for INTF1 {}
#[doc = "`write(|w| ..)` method takes [intf1::W](intf1::W) writer structure"]
impl crate::Writable for INTF1 {}
#[doc = "Force Interrupts for IRQ 1"]
pub mod intf1;
#[doc = "Interrupt Status (masked) for IRQ 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ints1](ints1) module"]
pub type INTS1 = crate::Reg<u32, _INTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTS1;
#[doc = "`read()` method returns [ints1::R](ints1::R) reader structure"]
impl crate::Readable for INTS1 {}
#[doc = "`write(|w| ..)` method takes [ints1::W](ints1::W) writer structure"]
impl crate::Writable for INTS1 {}
#[doc = "Interrupt Status (masked) for IRQ 1"]
pub mod ints1;
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0](timer0) module"]
pub type TIMER0 = crate::Reg<u32, _TIMER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER0;
#[doc = "`read()` method returns [timer0::R](timer0::R) reader structure"]
impl crate::Readable for TIMER0 {}
#[doc = "`write(|w| ..)` method takes [timer0::W](timer0::W) writer structure"]
impl crate::Writable for TIMER0 {}
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer0;
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1](timer1) module"]
pub type TIMER1 = crate::Reg<u32, _TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1;
#[doc = "`read()` method returns [timer1::R](timer1::R) reader structure"]
impl crate::Readable for TIMER1 {}
#[doc = "`write(|w| ..)` method takes [timer1::W](timer1::W) writer structure"]
impl crate::Writable for TIMER1 {}
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer1;
#[doc = "Trigger one or more channels simultaneously\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [multi_chan_trigger](multi_chan_trigger) module"]
pub type MULTI_CHAN_TRIGGER = crate::Reg<u32, _MULTI_CHAN_TRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MULTI_CHAN_TRIGGER;
#[doc = "`read()` method returns [multi_chan_trigger::R](multi_chan_trigger::R) reader structure"]
impl crate::Readable for MULTI_CHAN_TRIGGER {}
#[doc = "`write(|w| ..)` method takes [multi_chan_trigger::W](multi_chan_trigger::W) writer structure"]
impl crate::Writable for MULTI_CHAN_TRIGGER {}
#[doc = "Trigger one or more channels simultaneously"]
pub mod multi_chan_trigger;
#[doc = "Sniffer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sniff_ctrl](sniff_ctrl) module"]
pub type SNIFF_CTRL = crate::Reg<u32, _SNIFF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNIFF_CTRL;
#[doc = "`read()` method returns [sniff_ctrl::R](sniff_ctrl::R) reader structure"]
impl crate::Readable for SNIFF_CTRL {}
#[doc = "`write(|w| ..)` method takes [sniff_ctrl::W](sniff_ctrl::W) writer structure"]
impl crate::Writable for SNIFF_CTRL {}
#[doc = "Sniffer Control"]
pub mod sniff_ctrl;
#[doc = "Data accumulator for sniff hardware\\n Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sniff_data](sniff_data) module"]
pub type SNIFF_DATA = crate::Reg<u32, _SNIFF_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNIFF_DATA;
#[doc = "`read()` method returns [sniff_data::R](sniff_data::R) reader structure"]
impl crate::Readable for SNIFF_DATA {}
#[doc = "`write(|w| ..)` method takes [sniff_data::W](sniff_data::W) writer structure"]
impl crate::Writable for SNIFF_DATA {}
#[doc = "Data accumulator for sniff hardware\\n Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
pub mod sniff_data;
#[doc = "Debug RAF, WAF, TDF levels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_levels](fifo_levels) module"]
pub type FIFO_LEVELS = crate::Reg<u32, _FIFO_LEVELS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_LEVELS;
#[doc = "`read()` method returns [fifo_levels::R](fifo_levels::R) reader structure"]
impl crate::Readable for FIFO_LEVELS {}
#[doc = "Debug RAF, WAF, TDF levels"]
pub mod fifo_levels;
#[doc = "Abort an in-progress transfer sequence on one or more channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_abort](chan_abort) module"]
pub type CHAN_ABORT = crate::Reg<u32, _CHAN_ABORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_ABORT;
#[doc = "`read()` method returns [chan_abort::R](chan_abort::R) reader structure"]
impl crate::Readable for CHAN_ABORT {}
#[doc = "`write(|w| ..)` method takes [chan_abort::W](chan_abort::W) writer structure"]
impl crate::Writable for CHAN_ABORT {}
#[doc = "Abort an in-progress transfer sequence on one or more channels"]
pub mod chan_abort;
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [n_channels](n_channels) module"]
pub type N_CHANNELS = crate::Reg<u32, _N_CHANNELS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _N_CHANNELS;
#[doc = "`read()` method returns [n_channels::R](n_channels::R) reader structure"]
impl crate::Readable for N_CHANNELS {}
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
pub mod n_channels;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_dbg_ctdreq](ch0_dbg_ctdreq) module"]
pub type CH0_DBG_CTDREQ = crate::Reg<u32, _CH0_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_DBG_CTDREQ;
#[doc = "`read()` method returns [ch0_dbg_ctdreq::R](ch0_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH0_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch0_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_dbg_tcr](ch0_dbg_tcr) module"]
pub type CH0_DBG_TCR = crate::Reg<u32, _CH0_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_DBG_TCR;
#[doc = "`read()` method returns [ch0_dbg_tcr::R](ch0_dbg_tcr::R) reader structure"]
impl crate::Readable for CH0_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch0_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_dbg_ctdreq](ch1_dbg_ctdreq) module"]
pub type CH1_DBG_CTDREQ = crate::Reg<u32, _CH1_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_DBG_CTDREQ;
#[doc = "`read()` method returns [ch1_dbg_ctdreq::R](ch1_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH1_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch1_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_dbg_tcr](ch1_dbg_tcr) module"]
pub type CH1_DBG_TCR = crate::Reg<u32, _CH1_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_DBG_TCR;
#[doc = "`read()` method returns [ch1_dbg_tcr::R](ch1_dbg_tcr::R) reader structure"]
impl crate::Readable for CH1_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch1_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_dbg_ctdreq](ch2_dbg_ctdreq) module"]
pub type CH2_DBG_CTDREQ = crate::Reg<u32, _CH2_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_DBG_CTDREQ;
#[doc = "`read()` method returns [ch2_dbg_ctdreq::R](ch2_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH2_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch2_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_dbg_tcr](ch2_dbg_tcr) module"]
pub type CH2_DBG_TCR = crate::Reg<u32, _CH2_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_DBG_TCR;
#[doc = "`read()` method returns [ch2_dbg_tcr::R](ch2_dbg_tcr::R) reader structure"]
impl crate::Readable for CH2_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch2_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_dbg_ctdreq](ch3_dbg_ctdreq) module"]
pub type CH3_DBG_CTDREQ = crate::Reg<u32, _CH3_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_DBG_CTDREQ;
#[doc = "`read()` method returns [ch3_dbg_ctdreq::R](ch3_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH3_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch3_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_dbg_tcr](ch3_dbg_tcr) module"]
pub type CH3_DBG_TCR = crate::Reg<u32, _CH3_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_DBG_TCR;
#[doc = "`read()` method returns [ch3_dbg_tcr::R](ch3_dbg_tcr::R) reader structure"]
impl crate::Readable for CH3_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch3_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_dbg_ctdreq](ch4_dbg_ctdreq) module"]
pub type CH4_DBG_CTDREQ = crate::Reg<u32, _CH4_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_DBG_CTDREQ;
#[doc = "`read()` method returns [ch4_dbg_ctdreq::R](ch4_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH4_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch4_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_dbg_tcr](ch4_dbg_tcr) module"]
pub type CH4_DBG_TCR = crate::Reg<u32, _CH4_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_DBG_TCR;
#[doc = "`read()` method returns [ch4_dbg_tcr::R](ch4_dbg_tcr::R) reader structure"]
impl crate::Readable for CH4_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch4_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_dbg_ctdreq](ch5_dbg_ctdreq) module"]
pub type CH5_DBG_CTDREQ = crate::Reg<u32, _CH5_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_DBG_CTDREQ;
#[doc = "`read()` method returns [ch5_dbg_ctdreq::R](ch5_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH5_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch5_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_dbg_tcr](ch5_dbg_tcr) module"]
pub type CH5_DBG_TCR = crate::Reg<u32, _CH5_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_DBG_TCR;
#[doc = "`read()` method returns [ch5_dbg_tcr::R](ch5_dbg_tcr::R) reader structure"]
impl crate::Readable for CH5_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch5_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_dbg_ctdreq](ch6_dbg_ctdreq) module"]
pub type CH6_DBG_CTDREQ = crate::Reg<u32, _CH6_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_DBG_CTDREQ;
#[doc = "`read()` method returns [ch6_dbg_ctdreq::R](ch6_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH6_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch6_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_dbg_tcr](ch6_dbg_tcr) module"]
pub type CH6_DBG_TCR = crate::Reg<u32, _CH6_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_DBG_TCR;
#[doc = "`read()` method returns [ch6_dbg_tcr::R](ch6_dbg_tcr::R) reader structure"]
impl crate::Readable for CH6_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch6_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_dbg_ctdreq](ch7_dbg_ctdreq) module"]
pub type CH7_DBG_CTDREQ = crate::Reg<u32, _CH7_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_DBG_CTDREQ;
#[doc = "`read()` method returns [ch7_dbg_ctdreq::R](ch7_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH7_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch7_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_dbg_tcr](ch7_dbg_tcr) module"]
pub type CH7_DBG_TCR = crate::Reg<u32, _CH7_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_DBG_TCR;
#[doc = "`read()` method returns [ch7_dbg_tcr::R](ch7_dbg_tcr::R) reader structure"]
impl crate::Readable for CH7_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch7_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_dbg_ctdreq](ch8_dbg_ctdreq) module"]
pub type CH8_DBG_CTDREQ = crate::Reg<u32, _CH8_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_DBG_CTDREQ;
#[doc = "`read()` method returns [ch8_dbg_ctdreq::R](ch8_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH8_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch8_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_dbg_tcr](ch8_dbg_tcr) module"]
pub type CH8_DBG_TCR = crate::Reg<u32, _CH8_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_DBG_TCR;
#[doc = "`read()` method returns [ch8_dbg_tcr::R](ch8_dbg_tcr::R) reader structure"]
impl crate::Readable for CH8_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch8_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_dbg_ctdreq](ch9_dbg_ctdreq) module"]
pub type CH9_DBG_CTDREQ = crate::Reg<u32, _CH9_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_DBG_CTDREQ;
#[doc = "`read()` method returns [ch9_dbg_ctdreq::R](ch9_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH9_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch9_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_dbg_tcr](ch9_dbg_tcr) module"]
pub type CH9_DBG_TCR = crate::Reg<u32, _CH9_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_DBG_TCR;
#[doc = "`read()` method returns [ch9_dbg_tcr::R](ch9_dbg_tcr::R) reader structure"]
impl crate::Readable for CH9_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch9_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_dbg_ctdreq](ch10_dbg_ctdreq) module"]
pub type CH10_DBG_CTDREQ = crate::Reg<u32, _CH10_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_DBG_CTDREQ;
#[doc = "`read()` method returns [ch10_dbg_ctdreq::R](ch10_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH10_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch10_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_dbg_tcr](ch10_dbg_tcr) module"]
pub type CH10_DBG_TCR = crate::Reg<u32, _CH10_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_DBG_TCR;
#[doc = "`read()` method returns [ch10_dbg_tcr::R](ch10_dbg_tcr::R) reader structure"]
impl crate::Readable for CH10_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch10_dbg_tcr;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_dbg_ctdreq](ch11_dbg_ctdreq) module"]
pub type CH11_DBG_CTDREQ = crate::Reg<u32, _CH11_DBG_CTDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_DBG_CTDREQ;
#[doc = "`read()` method returns [ch11_dbg_ctdreq::R](ch11_dbg_ctdreq::R) reader structure"]
impl crate::Readable for CH11_DBG_CTDREQ {}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch11_dbg_ctdreq;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_dbg_tcr](ch11_dbg_tcr) module"]
pub type CH11_DBG_TCR = crate::Reg<u32, _CH11_DBG_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_DBG_TCR;
#[doc = "`read()` method returns [ch11_dbg_tcr::R](ch11_dbg_tcr::R) reader structure"]
impl crate::Readable for CH11_DBG_TCR {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch11_dbg_tcr;
