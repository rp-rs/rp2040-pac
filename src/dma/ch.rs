#[doc = "DMA Channel 0 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_read_addr](ch_read_addr) module"]
pub type CH_READ_ADDR = crate::Reg<u32, _CH_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_READ_ADDR;
#[doc = "`read()` method returns [ch_read_addr::R](ch_read_addr::R) reader structure"]
impl crate::Readable for CH_READ_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch_read_addr::W](ch_read_addr::W) writer structure"]
impl crate::Writable for CH_READ_ADDR {}
#[doc = "DMA Channel 0 Read Address pointer\\n This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
pub mod ch_read_addr;
#[doc = "DMA Channel 0 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_write_addr](ch_write_addr) module"]
pub type CH_WRITE_ADDR = crate::Reg<u32, _CH_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_WRITE_ADDR;
#[doc = "`read()` method returns [ch_write_addr::R](ch_write_addr::R) reader structure"]
impl crate::Readable for CH_WRITE_ADDR {}
#[doc = "`write(|w| ..)` method takes [ch_write_addr::W](ch_write_addr::W) writer structure"]
impl crate::Writable for CH_WRITE_ADDR {}
#[doc = "DMA Channel 0 Write Address pointer\\n This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub mod ch_write_addr;
#[doc = "DMA Channel 0 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_trans_count](ch_trans_count) module"]
pub type CH_TRANS_COUNT = crate::Reg<u32, _CH_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_TRANS_COUNT;
#[doc = "`read()` method returns [ch_trans_count::R](ch_trans_count::R) reader structure"]
impl crate::Readable for CH_TRANS_COUNT {}
#[doc = "`write(|w| ..)` method takes [ch_trans_count::W](ch_trans_count::W) writer structure"]
impl crate::Writable for CH_TRANS_COUNT {}
#[doc = "DMA Channel 0 Transfer Count\\n Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).\\n\\n When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.\\n\\n Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.\\n\\n The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub mod ch_trans_count;
#[doc = "DMA Channel 0 Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ctrl_trig](ch_ctrl_trig) module"]
pub type CH_CTRL_TRIG = crate::Reg<u32, _CH_CTRL_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_CTRL_TRIG;
#[doc = "`read()` method returns [ch_ctrl_trig::R](ch_ctrl_trig::R) reader structure"]
impl crate::Readable for CH_CTRL_TRIG {}
#[doc = "`write(|w| ..)` method takes [ch_ctrl_trig::W](ch_ctrl_trig::W) writer structure"]
impl crate::Writable for CH_CTRL_TRIG {}
#[doc = "DMA Channel 0 Control and Status"]
pub mod ch_ctrl_trig;
#[doc = "Alias for channel 0 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al1_ctrl](ch_al1_ctrl) module"]
pub type CH_AL1_CTRL = crate::Reg<u32, _CH_AL1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL1_CTRL;
#[doc = "`read()` method returns [ch_al1_ctrl::R](ch_al1_ctrl::R) reader structure"]
impl crate::Readable for CH_AL1_CTRL {}
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch_al1_ctrl;
#[doc = "Alias for channel 0 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al1_read_addr](ch_al1_read_addr) module"]
pub type CH_AL1_READ_ADDR = crate::Reg<u32, _CH_AL1_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL1_READ_ADDR;
#[doc = "`read()` method returns [ch_al1_read_addr::R](ch_al1_read_addr::R) reader structure"]
impl crate::Readable for CH_AL1_READ_ADDR {}
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch_al1_read_addr;
#[doc = "Alias for channel 0 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al1_write_addr](ch_al1_write_addr) module"]
pub type CH_AL1_WRITE_ADDR = crate::Reg<u32, _CH_AL1_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL1_WRITE_ADDR;
#[doc = "`read()` method returns [ch_al1_write_addr::R](ch_al1_write_addr::R) reader structure"]
impl crate::Readable for CH_AL1_WRITE_ADDR {}
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch_al1_write_addr;
#[doc = "Alias for channel 0 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al1_trans_count_trig](ch_al1_trans_count_trig) module"]
pub type CH_AL1_TRANS_COUNT_TRIG = crate::Reg<u32, _CH_AL1_TRANS_COUNT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL1_TRANS_COUNT_TRIG;
#[doc = "`read()` method returns [ch_al1_trans_count_trig::R](ch_al1_trans_count_trig::R) reader structure"]
impl crate::Readable for CH_AL1_TRANS_COUNT_TRIG {}
#[doc = "Alias for channel 0 TRANS_COUNT register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch_al1_trans_count_trig;
#[doc = "Alias for channel 0 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al2_ctrl](ch_al2_ctrl) module"]
pub type CH_AL2_CTRL = crate::Reg<u32, _CH_AL2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL2_CTRL;
#[doc = "`read()` method returns [ch_al2_ctrl::R](ch_al2_ctrl::R) reader structure"]
impl crate::Readable for CH_AL2_CTRL {}
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch_al2_ctrl;
#[doc = "Alias for channel 0 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al2_trans_count](ch_al2_trans_count) module"]
pub type CH_AL2_TRANS_COUNT = crate::Reg<u32, _CH_AL2_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL2_TRANS_COUNT;
#[doc = "`read()` method returns [ch_al2_trans_count::R](ch_al2_trans_count::R) reader structure"]
impl crate::Readable for CH_AL2_TRANS_COUNT {}
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch_al2_trans_count;
#[doc = "Alias for channel 0 READ_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al2_read_addr](ch_al2_read_addr) module"]
pub type CH_AL2_READ_ADDR = crate::Reg<u32, _CH_AL2_READ_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL2_READ_ADDR;
#[doc = "`read()` method returns [ch_al2_read_addr::R](ch_al2_read_addr::R) reader structure"]
impl crate::Readable for CH_AL2_READ_ADDR {}
#[doc = "Alias for channel 0 READ_ADDR register"]
pub mod ch_al2_read_addr;
#[doc = "Alias for channel 0 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al2_write_addr_trig](ch_al2_write_addr_trig) module"]
pub type CH_AL2_WRITE_ADDR_TRIG = crate::Reg<u32, _CH_AL2_WRITE_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL2_WRITE_ADDR_TRIG;
#[doc = "`read()` method returns [ch_al2_write_addr_trig::R](ch_al2_write_addr_trig::R) reader structure"]
impl crate::Readable for CH_AL2_WRITE_ADDR_TRIG {}
#[doc = "Alias for channel 0 WRITE_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch_al2_write_addr_trig;
#[doc = "Alias for channel 0 CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al3_ctrl](ch_al3_ctrl) module"]
pub type CH_AL3_CTRL = crate::Reg<u32, _CH_AL3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL3_CTRL;
#[doc = "`read()` method returns [ch_al3_ctrl::R](ch_al3_ctrl::R) reader structure"]
impl crate::Readable for CH_AL3_CTRL {}
#[doc = "Alias for channel 0 CTRL register"]
pub mod ch_al3_ctrl;
#[doc = "Alias for channel 0 WRITE_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al3_write_addr](ch_al3_write_addr) module"]
pub type CH_AL3_WRITE_ADDR = crate::Reg<u32, _CH_AL3_WRITE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL3_WRITE_ADDR;
#[doc = "`read()` method returns [ch_al3_write_addr::R](ch_al3_write_addr::R) reader structure"]
impl crate::Readable for CH_AL3_WRITE_ADDR {}
#[doc = "Alias for channel 0 WRITE_ADDR register"]
pub mod ch_al3_write_addr;
#[doc = "Alias for channel 0 TRANS_COUNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al3_trans_count](ch_al3_trans_count) module"]
pub type CH_AL3_TRANS_COUNT = crate::Reg<u32, _CH_AL3_TRANS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL3_TRANS_COUNT;
#[doc = "`read()` method returns [ch_al3_trans_count::R](ch_al3_trans_count::R) reader structure"]
impl crate::Readable for CH_AL3_TRANS_COUNT {}
#[doc = "Alias for channel 0 TRANS_COUNT register"]
pub mod ch_al3_trans_count;
#[doc = "Alias for channel 0 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_al3_read_addr_trig](ch_al3_read_addr_trig) module"]
pub type CH_AL3_READ_ADDR_TRIG = crate::Reg<u32, _CH_AL3_READ_ADDR_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_AL3_READ_ADDR_TRIG;
#[doc = "`read()` method returns [ch_al3_read_addr_trig::R](ch_al3_read_addr_trig::R) reader structure"]
impl crate::Readable for CH_AL3_READ_ADDR_TRIG {}
#[doc = "Alias for channel 0 READ_ADDR register\\n This is a trigger register (0xc). Writing a nonzero value will\\n reload the channel counter and start the channel."]
pub mod ch_al3_read_addr_trig;
