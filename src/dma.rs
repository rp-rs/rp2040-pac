#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
    pub ch: [CH; 12],
    _reserved1: [u8; 256usize],
    #[doc = "0x400 - Interrupt Status (raw)"]
    pub intr: INTR,
    #[doc = "0x404 - Interrupt Enables for IRQ 0"]
    pub inte0: INTE0,
    #[doc = "0x408 - Force Interrupts"]
    pub intf0: INTF0,
    #[doc = "0x40c - Interrupt Status for IRQ 0"]
    pub ints0: INTS0,
    _reserved5: [u8; 4usize],
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
    #[doc = "0x428 - Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer2: TIMER2,
    #[doc = "0x42c - Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub timer3: TIMER3,
    #[doc = "0x430 - Trigger one or more channels simultaneously"]
    pub multi_chan_trigger: MULTI_CHAN_TRIGGER,
    #[doc = "0x434 - Sniffer Control"]
    pub sniff_ctrl: SNIFF_CTRL,
    #[doc = "0x438 - Data accumulator for sniff hardware\\n Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    pub sniff_data: SNIFF_DATA,
    _reserved15: [u8; 4usize],
    #[doc = "0x440 - Debug RAF, WAF, TDF levels"]
    pub fifo_levels: FIFO_LEVELS,
    #[doc = "0x444 - Abort an in-progress transfer sequence on one or more channels"]
    pub chan_abort: CHAN_ABORT,
    #[doc = "0x448 - The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    pub n_channels: N_CHANNELS,
    _reserved18: [u8; 948usize],
    #[doc = "0x800 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch0_dbg_ctdreq: CH0_DBG_CTDREQ,
    #[doc = "0x804 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch0_dbg_tcr: CH0_DBG_TCR,
    _reserved20: [u8; 56usize],
    #[doc = "0x840 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch1_dbg_ctdreq: CH1_DBG_CTDREQ,
    #[doc = "0x844 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch1_dbg_tcr: CH1_DBG_TCR,
    _reserved22: [u8; 56usize],
    #[doc = "0x880 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch2_dbg_ctdreq: CH2_DBG_CTDREQ,
    #[doc = "0x884 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch2_dbg_tcr: CH2_DBG_TCR,
    _reserved24: [u8; 56usize],
    #[doc = "0x8c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch3_dbg_ctdreq: CH3_DBG_CTDREQ,
    #[doc = "0x8c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch3_dbg_tcr: CH3_DBG_TCR,
    _reserved26: [u8; 56usize],
    #[doc = "0x900 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch4_dbg_ctdreq: CH4_DBG_CTDREQ,
    #[doc = "0x904 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch4_dbg_tcr: CH4_DBG_TCR,
    _reserved28: [u8; 56usize],
    #[doc = "0x940 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch5_dbg_ctdreq: CH5_DBG_CTDREQ,
    #[doc = "0x944 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch5_dbg_tcr: CH5_DBG_TCR,
    _reserved30: [u8; 56usize],
    #[doc = "0x980 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch6_dbg_ctdreq: CH6_DBG_CTDREQ,
    #[doc = "0x984 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch6_dbg_tcr: CH6_DBG_TCR,
    _reserved32: [u8; 56usize],
    #[doc = "0x9c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch7_dbg_ctdreq: CH7_DBG_CTDREQ,
    #[doc = "0x9c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch7_dbg_tcr: CH7_DBG_TCR,
    _reserved34: [u8; 56usize],
    #[doc = "0xa00 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch8_dbg_ctdreq: CH8_DBG_CTDREQ,
    #[doc = "0xa04 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch8_dbg_tcr: CH8_DBG_TCR,
    _reserved36: [u8; 56usize],
    #[doc = "0xa40 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch9_dbg_ctdreq: CH9_DBG_CTDREQ,
    #[doc = "0xa44 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch9_dbg_tcr: CH9_DBG_TCR,
    _reserved38: [u8; 56usize],
    #[doc = "0xa80 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch10_dbg_ctdreq: CH10_DBG_CTDREQ,
    #[doc = "0xa84 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch10_dbg_tcr: CH10_DBG_TCR,
    _reserved40: [u8; 56usize],
    #[doc = "0xac0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub ch11_dbg_ctdreq: CH11_DBG_CTDREQ,
    #[doc = "0xac4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub ch11_dbg_tcr: CH11_DBG_TCR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA Channel 0 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub ch_read_addr: self::ch::CH_READ_ADDR,
    #[doc = "0x04 - DMA Channel 0 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub ch_write_addr: self::ch::CH_WRITE_ADDR,
    #[doc = "0x08 - DMA Channel 0 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub ch_trans_count: self::ch::CH_TRANS_COUNT,
    #[doc = "0x0c - DMA Channel 0 Control and Status"]
    pub ch_ctrl_trig: self::ch::CH_CTRL_TRIG,
    #[doc = "0x10 - Alias for channel 0 CTRL register"]
    pub ch_al1_ctrl: self::ch::CH_AL1_CTRL,
    #[doc = "0x14 - Alias for channel 0 READ_ADDR register"]
    pub ch_al1_read_addr: self::ch::CH_AL1_READ_ADDR,
    #[doc = "0x18 - Alias for channel 0 WRITE_ADDR register"]
    pub ch_al1_write_addr: self::ch::CH_AL1_WRITE_ADDR,
    #[doc = "0x1c - Alias for channel 0 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch_al1_trans_count_trig: self::ch::CH_AL1_TRANS_COUNT_TRIG,
    #[doc = "0x20 - Alias for channel 0 CTRL register"]
    pub ch_al2_ctrl: self::ch::CH_AL2_CTRL,
    #[doc = "0x24 - Alias for channel 0 TRANS_COUNT register"]
    pub ch_al2_trans_count: self::ch::CH_AL2_TRANS_COUNT,
    #[doc = "0x28 - Alias for channel 0 READ_ADDR register"]
    pub ch_al2_read_addr: self::ch::CH_AL2_READ_ADDR,
    #[doc = "0x2c - Alias for channel 0 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch_al2_write_addr_trig: self::ch::CH_AL2_WRITE_ADDR_TRIG,
    #[doc = "0x30 - Alias for channel 0 CTRL register"]
    pub ch_al3_ctrl: self::ch::CH_AL3_CTRL,
    #[doc = "0x34 - Alias for channel 0 WRITE_ADDR register"]
    pub ch_al3_write_addr: self::ch::CH_AL3_WRITE_ADDR,
    #[doc = "0x38 - Alias for channel 0 TRANS_COUNT register"]
    pub ch_al3_trans_count: self::ch::CH_AL3_TRANS_COUNT,
    #[doc = "0x3c - Alias for channel 0 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
    pub ch_al3_read_addr_trig: self::ch::CH_AL3_READ_ADDR_TRIG,
}
#[doc = r"Register block"]
#[doc = "Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
pub mod ch;
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
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2](timer2) module"]
pub type TIMER2 = crate::Reg<u32, _TIMER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2;
#[doc = "`read()` method returns [timer2::R](timer2::R) reader structure"]
impl crate::Readable for TIMER2 {}
#[doc = "`write(|w| ..)` method takes [timer2::W](timer2::W) writer structure"]
impl crate::Writable for TIMER2 {}
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer2;
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer3](timer3) module"]
pub type TIMER3 = crate::Reg<u32, _TIMER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER3;
#[doc = "`read()` method returns [timer3::R](timer3::R) reader structure"]
impl crate::Readable for TIMER3 {}
#[doc = "`write(|w| ..)` method takes [timer3::W](timer3::W) writer structure"]
impl crate::Writable for TIMER3 {}
#[doc = "Pacing (X/Y) Fractional Timer\\n The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer3;
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
