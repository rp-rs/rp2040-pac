#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
    pub ic_con: IC_CON,
    #[doc = "0x04 - I2C Target Address Register\\n\\n This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.\\n\\n Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
    pub ic_tar: IC_TAR,
    #[doc = "0x08 - I2C Slave Address Register"]
    pub ic_sar: IC_SAR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.\\n\\n The size of the register changes as follows:\\n\\n Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
    pub ic_data_cmd: IC_DATA_CMD,
    #[doc = "0x14 - Standard Speed I2C Clock SCL High Count Register"]
    pub ic_ss_scl_hcnt: IC_SS_SCL_HCNT,
    #[doc = "0x18 - Standard Speed I2C Clock SCL Low Count Register"]
    pub ic_ss_scl_lcnt: IC_SS_SCL_LCNT,
    #[doc = "0x1c - Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
    pub ic_fs_scl_hcnt: IC_FS_SCL_HCNT,
    #[doc = "0x20 - Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
    pub ic_fs_scl_lcnt: IC_FS_SCL_LCNT,
    _reserved8: [u8; 8usize],
    #[doc = "0x2c - I2C Interrupt Status Register\\n\\n Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
    pub ic_intr_stat: IC_INTR_STAT,
    #[doc = "0x30 - I2C Interrupt Mask Register.\\n\\n These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
    pub ic_intr_mask: IC_INTR_MASK,
    #[doc = "0x34 - I2C Raw Interrupt Status Register\\n\\n Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
    pub ic_raw_intr_stat: IC_RAW_INTR_STAT,
    #[doc = "0x38 - I2C Receive FIFO Threshold Register"]
    pub ic_rx_tl: IC_RX_TL,
    #[doc = "0x3c - I2C Transmit FIFO Threshold Register"]
    pub ic_tx_tl: IC_TX_TL,
    #[doc = "0x40 - Clear Combined and Individual Interrupt Register"]
    pub ic_clr_intr: IC_CLR_INTR,
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    pub ic_clr_rx_under: IC_CLR_RX_UNDER,
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    pub ic_clr_rx_over: IC_CLR_RX_OVER,
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    pub ic_clr_tx_over: IC_CLR_TX_OVER,
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    pub ic_clr_rd_req: IC_CLR_RD_REQ,
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    pub ic_clr_tx_abrt: IC_CLR_TX_ABRT,
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    pub ic_clr_rx_done: IC_CLR_RX_DONE,
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    pub ic_clr_activity: IC_CLR_ACTIVITY,
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    pub ic_clr_stop_det: IC_CLR_STOP_DET,
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    pub ic_clr_start_det: IC_CLR_START_DET,
    #[doc = "0x68 - Clear GEN_CALL Interrupt Register"]
    pub ic_clr_gen_call: IC_CLR_GEN_CALL,
    #[doc = "0x6c - I2C Enable Register"]
    pub ic_enable: IC_ENABLE,
    #[doc = "0x70 - I2C Status Register\\n\\n This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.\\n\\n When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
    pub ic_status: IC_STATUS,
    #[doc = "0x74 - I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
    pub ic_txflr: IC_TXFLR,
    #[doc = "0x78 - I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
    pub ic_rxflr: IC_RXFLR,
    #[doc = "0x7c - I2C SDA Hold Time Length Register\\n\\n The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).\\n\\n The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.\\n\\n Writes to this register succeed only when IC_ENABLE\\[0\\]=0.\\n\\n The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode one cycle in master mode, seven cycles in slave mode for the value to be implemented.\\n\\n The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles."]
    pub ic_sda_hold: IC_SDA_HOLD,
    #[doc = "0x80 - I2C Transmit Abort Source Register\\n\\n This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).\\n\\n Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
    pub ic_tx_abrt_source: IC_TX_ABRT_SOURCE,
    #[doc = "0x84 - Generate Slave Data NACK Register\\n\\n The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.\\n\\n A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
    pub ic_slv_data_nack_only: IC_SLV_DATA_NACK_ONLY,
    #[doc = "0x88 - DMA Control Register\\n\\n The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
    pub ic_dma_cr: IC_DMA_CR,
    #[doc = "0x8c - DMA Transmit Data Level Register"]
    pub ic_dma_tdlr: IC_DMA_TDLR,
    #[doc = "0x90 - I2C Receive Data Level Register"]
    pub ic_dma_rdlr: IC_DMA_RDLR,
    #[doc = "0x94 - I2C SDA Setup Register\\n\\n This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.\\n\\n Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.\\n\\n Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
    pub ic_sda_setup: IC_SDA_SETUP,
    #[doc = "0x98 - I2C ACK General Call Register\\n\\n The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.\\n\\n This register is applicable only when the DW_apb_i2c is in slave mode."]
    pub ic_ack_general_call: IC_ACK_GENERAL_CALL,
    #[doc = "0x9c - I2C Enable Status Register\\n\\n The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE\\[0\\]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled.\\n\\n If IC_ENABLE\\[0\\]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.\\n\\n If IC_ENABLE\\[0\\]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.\\n\\n Note: When IC_ENABLE\\[0\\]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
    pub ic_enable_status: IC_ENABLE_STATUS,
    #[doc = "0xa0 - I2C SS, FS or FM+ spike suppression limit\\n\\n This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
    pub ic_fs_spklen: IC_FS_SPKLEN,
    _reserved38: [u8; 4usize],
    #[doc = "0xa8 - Clear RESTART_DET Interrupt Register"]
    pub ic_clr_restart_det: IC_CLR_RESTART_DET,
    _reserved39: [u8; 72usize],
    #[doc = "0xf4 - Component Parameter Register 1\\n\\n Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
    pub ic_comp_param_1: IC_COMP_PARAM_1,
    #[doc = "0xf8 - I2C Component Version Register"]
    pub ic_comp_version: IC_COMP_VERSION,
    #[doc = "0xfc - I2C Component Type Register"]
    pub ic_comp_type: IC_COMP_TYPE,
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_con](ic_con) module"]
pub type IC_CON = crate::Reg<u32, _IC_CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CON;
#[doc = "`read()` method returns [ic_con::R](ic_con::R) reader structure"]
impl crate::Readable for IC_CON {}
#[doc = "`write(|w| ..)` method takes [ic_con::W](ic_con::W) writer structure"]
impl crate::Writable for IC_CON {}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
pub mod ic_con;
#[doc = "I2C Target Address Register\\n\\n This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.\\n\\n Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_tar](ic_tar) module"]
pub type IC_TAR = crate::Reg<u32, _IC_TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_TAR;
#[doc = "`read()` method returns [ic_tar::R](ic_tar::R) reader structure"]
impl crate::Readable for IC_TAR {}
#[doc = "`write(|w| ..)` method takes [ic_tar::W](ic_tar::W) writer structure"]
impl crate::Writable for IC_TAR {}
#[doc = "I2C Target Address Register\\n\\n This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.\\n\\n Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
pub mod ic_tar;
#[doc = "I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_sar](ic_sar) module"]
pub type IC_SAR = crate::Reg<u32, _IC_SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_SAR;
#[doc = "`read()` method returns [ic_sar::R](ic_sar::R) reader structure"]
impl crate::Readable for IC_SAR {}
#[doc = "`write(|w| ..)` method takes [ic_sar::W](ic_sar::W) writer structure"]
impl crate::Writable for IC_SAR {}
#[doc = "I2C Slave Address Register"]
pub mod ic_sar;
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.\\n\\n The size of the register changes as follows:\\n\\n Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_data_cmd](ic_data_cmd) module"]
pub type IC_DATA_CMD = crate::Reg<u32, _IC_DATA_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_DATA_CMD;
#[doc = "`read()` method returns [ic_data_cmd::R](ic_data_cmd::R) reader structure"]
impl crate::Readable for IC_DATA_CMD {}
#[doc = "`write(|w| ..)` method takes [ic_data_cmd::W](ic_data_cmd::W) writer structure"]
impl crate::Writable for IC_DATA_CMD {}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.\\n\\n The size of the register changes as follows:\\n\\n Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
pub mod ic_data_cmd;
#[doc = "Standard Speed I2C Clock SCL High Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_ss_scl_hcnt](ic_ss_scl_hcnt) module"]
pub type IC_SS_SCL_HCNT = crate::Reg<u32, _IC_SS_SCL_HCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_SS_SCL_HCNT;
#[doc = "`read()` method returns [ic_ss_scl_hcnt::R](ic_ss_scl_hcnt::R) reader structure"]
impl crate::Readable for IC_SS_SCL_HCNT {}
#[doc = "`write(|w| ..)` method takes [ic_ss_scl_hcnt::W](ic_ss_scl_hcnt::W) writer structure"]
impl crate::Writable for IC_SS_SCL_HCNT {}
#[doc = "Standard Speed I2C Clock SCL High Count Register"]
pub mod ic_ss_scl_hcnt;
#[doc = "Standard Speed I2C Clock SCL Low Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_ss_scl_lcnt](ic_ss_scl_lcnt) module"]
pub type IC_SS_SCL_LCNT = crate::Reg<u32, _IC_SS_SCL_LCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_SS_SCL_LCNT;
#[doc = "`read()` method returns [ic_ss_scl_lcnt::R](ic_ss_scl_lcnt::R) reader structure"]
impl crate::Readable for IC_SS_SCL_LCNT {}
#[doc = "`write(|w| ..)` method takes [ic_ss_scl_lcnt::W](ic_ss_scl_lcnt::W) writer structure"]
impl crate::Writable for IC_SS_SCL_LCNT {}
#[doc = "Standard Speed I2C Clock SCL Low Count Register"]
pub mod ic_ss_scl_lcnt;
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_fs_scl_hcnt](ic_fs_scl_hcnt) module"]
pub type IC_FS_SCL_HCNT = crate::Reg<u32, _IC_FS_SCL_HCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_FS_SCL_HCNT;
#[doc = "`read()` method returns [ic_fs_scl_hcnt::R](ic_fs_scl_hcnt::R) reader structure"]
impl crate::Readable for IC_FS_SCL_HCNT {}
#[doc = "`write(|w| ..)` method takes [ic_fs_scl_hcnt::W](ic_fs_scl_hcnt::W) writer structure"]
impl crate::Writable for IC_FS_SCL_HCNT {}
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
pub mod ic_fs_scl_hcnt;
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_fs_scl_lcnt](ic_fs_scl_lcnt) module"]
pub type IC_FS_SCL_LCNT = crate::Reg<u32, _IC_FS_SCL_LCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_FS_SCL_LCNT;
#[doc = "`read()` method returns [ic_fs_scl_lcnt::R](ic_fs_scl_lcnt::R) reader structure"]
impl crate::Readable for IC_FS_SCL_LCNT {}
#[doc = "`write(|w| ..)` method takes [ic_fs_scl_lcnt::W](ic_fs_scl_lcnt::W) writer structure"]
impl crate::Writable for IC_FS_SCL_LCNT {}
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
pub mod ic_fs_scl_lcnt;
#[doc = "I2C Interrupt Status Register\\n\\n Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_intr_stat](ic_intr_stat) module"]
pub type IC_INTR_STAT = crate::Reg<u32, _IC_INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_INTR_STAT;
#[doc = "`read()` method returns [ic_intr_stat::R](ic_intr_stat::R) reader structure"]
impl crate::Readable for IC_INTR_STAT {}
#[doc = "I2C Interrupt Status Register\\n\\n Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
pub mod ic_intr_stat;
#[doc = "I2C Interrupt Mask Register.\\n\\n These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_intr_mask](ic_intr_mask) module"]
pub type IC_INTR_MASK = crate::Reg<u32, _IC_INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_INTR_MASK;
#[doc = "`read()` method returns [ic_intr_mask::R](ic_intr_mask::R) reader structure"]
impl crate::Readable for IC_INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [ic_intr_mask::W](ic_intr_mask::W) writer structure"]
impl crate::Writable for IC_INTR_MASK {}
#[doc = "I2C Interrupt Mask Register.\\n\\n These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
pub mod ic_intr_mask;
#[doc = "I2C Raw Interrupt Status Register\\n\\n Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_raw_intr_stat](ic_raw_intr_stat) module"]
pub type IC_RAW_INTR_STAT = crate::Reg<u32, _IC_RAW_INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_RAW_INTR_STAT;
#[doc = "`read()` method returns [ic_raw_intr_stat::R](ic_raw_intr_stat::R) reader structure"]
impl crate::Readable for IC_RAW_INTR_STAT {}
#[doc = "I2C Raw Interrupt Status Register\\n\\n Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
pub mod ic_raw_intr_stat;
#[doc = "I2C Receive FIFO Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_rx_tl](ic_rx_tl) module"]
pub type IC_RX_TL = crate::Reg<u32, _IC_RX_TL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_RX_TL;
#[doc = "`read()` method returns [ic_rx_tl::R](ic_rx_tl::R) reader structure"]
impl crate::Readable for IC_RX_TL {}
#[doc = "`write(|w| ..)` method takes [ic_rx_tl::W](ic_rx_tl::W) writer structure"]
impl crate::Writable for IC_RX_TL {}
#[doc = "I2C Receive FIFO Threshold Register"]
pub mod ic_rx_tl;
#[doc = "I2C Transmit FIFO Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_tx_tl](ic_tx_tl) module"]
pub type IC_TX_TL = crate::Reg<u32, _IC_TX_TL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_TX_TL;
#[doc = "`read()` method returns [ic_tx_tl::R](ic_tx_tl::R) reader structure"]
impl crate::Readable for IC_TX_TL {}
#[doc = "`write(|w| ..)` method takes [ic_tx_tl::W](ic_tx_tl::W) writer structure"]
impl crate::Writable for IC_TX_TL {}
#[doc = "I2C Transmit FIFO Threshold Register"]
pub mod ic_tx_tl;
#[doc = "Clear Combined and Individual Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_intr](ic_clr_intr) module"]
pub type IC_CLR_INTR = crate::Reg<u32, _IC_CLR_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_INTR;
#[doc = "`read()` method returns [ic_clr_intr::R](ic_clr_intr::R) reader structure"]
impl crate::Readable for IC_CLR_INTR {}
#[doc = "Clear Combined and Individual Interrupt Register"]
pub mod ic_clr_intr;
#[doc = "Clear RX_UNDER Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_rx_under](ic_clr_rx_under) module"]
pub type IC_CLR_RX_UNDER = crate::Reg<u32, _IC_CLR_RX_UNDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_RX_UNDER;
#[doc = "`read()` method returns [ic_clr_rx_under::R](ic_clr_rx_under::R) reader structure"]
impl crate::Readable for IC_CLR_RX_UNDER {}
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod ic_clr_rx_under;
#[doc = "Clear RX_OVER Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_rx_over](ic_clr_rx_over) module"]
pub type IC_CLR_RX_OVER = crate::Reg<u32, _IC_CLR_RX_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_RX_OVER;
#[doc = "`read()` method returns [ic_clr_rx_over::R](ic_clr_rx_over::R) reader structure"]
impl crate::Readable for IC_CLR_RX_OVER {}
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod ic_clr_rx_over;
#[doc = "Clear TX_OVER Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_tx_over](ic_clr_tx_over) module"]
pub type IC_CLR_TX_OVER = crate::Reg<u32, _IC_CLR_TX_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_TX_OVER;
#[doc = "`read()` method returns [ic_clr_tx_over::R](ic_clr_tx_over::R) reader structure"]
impl crate::Readable for IC_CLR_TX_OVER {}
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod ic_clr_tx_over;
#[doc = "Clear RD_REQ Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_rd_req](ic_clr_rd_req) module"]
pub type IC_CLR_RD_REQ = crate::Reg<u32, _IC_CLR_RD_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_RD_REQ;
#[doc = "`read()` method returns [ic_clr_rd_req::R](ic_clr_rd_req::R) reader structure"]
impl crate::Readable for IC_CLR_RD_REQ {}
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod ic_clr_rd_req;
#[doc = "Clear TX_ABRT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_tx_abrt](ic_clr_tx_abrt) module"]
pub type IC_CLR_TX_ABRT = crate::Reg<u32, _IC_CLR_TX_ABRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_TX_ABRT;
#[doc = "`read()` method returns [ic_clr_tx_abrt::R](ic_clr_tx_abrt::R) reader structure"]
impl crate::Readable for IC_CLR_TX_ABRT {}
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod ic_clr_tx_abrt;
#[doc = "Clear RX_DONE Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_rx_done](ic_clr_rx_done) module"]
pub type IC_CLR_RX_DONE = crate::Reg<u32, _IC_CLR_RX_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_RX_DONE;
#[doc = "`read()` method returns [ic_clr_rx_done::R](ic_clr_rx_done::R) reader structure"]
impl crate::Readable for IC_CLR_RX_DONE {}
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod ic_clr_rx_done;
#[doc = "Clear ACTIVITY Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_activity](ic_clr_activity) module"]
pub type IC_CLR_ACTIVITY = crate::Reg<u32, _IC_CLR_ACTIVITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_ACTIVITY;
#[doc = "`read()` method returns [ic_clr_activity::R](ic_clr_activity::R) reader structure"]
impl crate::Readable for IC_CLR_ACTIVITY {}
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod ic_clr_activity;
#[doc = "Clear STOP_DET Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_stop_det](ic_clr_stop_det) module"]
pub type IC_CLR_STOP_DET = crate::Reg<u32, _IC_CLR_STOP_DET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_STOP_DET;
#[doc = "`read()` method returns [ic_clr_stop_det::R](ic_clr_stop_det::R) reader structure"]
impl crate::Readable for IC_CLR_STOP_DET {}
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod ic_clr_stop_det;
#[doc = "Clear START_DET Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_start_det](ic_clr_start_det) module"]
pub type IC_CLR_START_DET = crate::Reg<u32, _IC_CLR_START_DET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_START_DET;
#[doc = "`read()` method returns [ic_clr_start_det::R](ic_clr_start_det::R) reader structure"]
impl crate::Readable for IC_CLR_START_DET {}
#[doc = "Clear START_DET Interrupt Register"]
pub mod ic_clr_start_det;
#[doc = "Clear GEN_CALL Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_gen_call](ic_clr_gen_call) module"]
pub type IC_CLR_GEN_CALL = crate::Reg<u32, _IC_CLR_GEN_CALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_GEN_CALL;
#[doc = "`read()` method returns [ic_clr_gen_call::R](ic_clr_gen_call::R) reader structure"]
impl crate::Readable for IC_CLR_GEN_CALL {}
#[doc = "Clear GEN_CALL Interrupt Register"]
pub mod ic_clr_gen_call;
#[doc = "I2C Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_enable](ic_enable) module"]
pub type IC_ENABLE = crate::Reg<u32, _IC_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_ENABLE;
#[doc = "`read()` method returns [ic_enable::R](ic_enable::R) reader structure"]
impl crate::Readable for IC_ENABLE {}
#[doc = "`write(|w| ..)` method takes [ic_enable::W](ic_enable::W) writer structure"]
impl crate::Writable for IC_ENABLE {}
#[doc = "I2C Enable Register"]
pub mod ic_enable;
#[doc = "I2C Status Register\\n\\n This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.\\n\\n When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_status](ic_status) module"]
pub type IC_STATUS = crate::Reg<u32, _IC_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_STATUS;
#[doc = "`read()` method returns [ic_status::R](ic_status::R) reader structure"]
impl crate::Readable for IC_STATUS {}
#[doc = "I2C Status Register\\n\\n This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.\\n\\n When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
pub mod ic_status;
#[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_txflr](ic_txflr) module"]
pub type IC_TXFLR = crate::Reg<u32, _IC_TXFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_TXFLR;
#[doc = "`read()` method returns [ic_txflr::R](ic_txflr::R) reader structure"]
impl crate::Readable for IC_TXFLR {}
#[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
pub mod ic_txflr;
#[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_rxflr](ic_rxflr) module"]
pub type IC_RXFLR = crate::Reg<u32, _IC_RXFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_RXFLR;
#[doc = "`read()` method returns [ic_rxflr::R](ic_rxflr::R) reader structure"]
impl crate::Readable for IC_RXFLR {}
#[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
pub mod ic_rxflr;
#[doc = "I2C SDA Hold Time Length Register\\n\\n The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).\\n\\n The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.\\n\\n Writes to this register succeed only when IC_ENABLE\\[0\\]=0.\\n\\n The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode one cycle in master mode, seven cycles in slave mode for the value to be implemented.\\n\\n The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_sda_hold](ic_sda_hold) module"]
pub type IC_SDA_HOLD = crate::Reg<u32, _IC_SDA_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_SDA_HOLD;
#[doc = "`read()` method returns [ic_sda_hold::R](ic_sda_hold::R) reader structure"]
impl crate::Readable for IC_SDA_HOLD {}
#[doc = "`write(|w| ..)` method takes [ic_sda_hold::W](ic_sda_hold::W) writer structure"]
impl crate::Writable for IC_SDA_HOLD {}
#[doc = "I2C SDA Hold Time Length Register\\n\\n The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).\\n\\n The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.\\n\\n Writes to this register succeed only when IC_ENABLE\\[0\\]=0.\\n\\n The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode one cycle in master mode, seven cycles in slave mode for the value to be implemented.\\n\\n The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles."]
pub mod ic_sda_hold;
#[doc = "I2C Transmit Abort Source Register\\n\\n This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).\\n\\n Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_tx_abrt_source](ic_tx_abrt_source) module"]
pub type IC_TX_ABRT_SOURCE = crate::Reg<u32, _IC_TX_ABRT_SOURCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_TX_ABRT_SOURCE;
#[doc = "`read()` method returns [ic_tx_abrt_source::R](ic_tx_abrt_source::R) reader structure"]
impl crate::Readable for IC_TX_ABRT_SOURCE {}
#[doc = "I2C Transmit Abort Source Register\\n\\n This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).\\n\\n Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
pub mod ic_tx_abrt_source;
#[doc = "Generate Slave Data NACK Register\\n\\n The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.\\n\\n A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_slv_data_nack_only](ic_slv_data_nack_only) module"]
pub type IC_SLV_DATA_NACK_ONLY = crate::Reg<u32, _IC_SLV_DATA_NACK_ONLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_SLV_DATA_NACK_ONLY;
#[doc = "`read()` method returns [ic_slv_data_nack_only::R](ic_slv_data_nack_only::R) reader structure"]
impl crate::Readable for IC_SLV_DATA_NACK_ONLY {}
#[doc = "`write(|w| ..)` method takes [ic_slv_data_nack_only::W](ic_slv_data_nack_only::W) writer structure"]
impl crate::Writable for IC_SLV_DATA_NACK_ONLY {}
#[doc = "Generate Slave Data NACK Register\\n\\n The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.\\n\\n A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
pub mod ic_slv_data_nack_only;
#[doc = "DMA Control Register\\n\\n The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_dma_cr](ic_dma_cr) module"]
pub type IC_DMA_CR = crate::Reg<u32, _IC_DMA_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_DMA_CR;
#[doc = "`read()` method returns [ic_dma_cr::R](ic_dma_cr::R) reader structure"]
impl crate::Readable for IC_DMA_CR {}
#[doc = "`write(|w| ..)` method takes [ic_dma_cr::W](ic_dma_cr::W) writer structure"]
impl crate::Writable for IC_DMA_CR {}
#[doc = "DMA Control Register\\n\\n The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
pub mod ic_dma_cr;
#[doc = "DMA Transmit Data Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_dma_tdlr](ic_dma_tdlr) module"]
pub type IC_DMA_TDLR = crate::Reg<u32, _IC_DMA_TDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_DMA_TDLR;
#[doc = "`read()` method returns [ic_dma_tdlr::R](ic_dma_tdlr::R) reader structure"]
impl crate::Readable for IC_DMA_TDLR {}
#[doc = "`write(|w| ..)` method takes [ic_dma_tdlr::W](ic_dma_tdlr::W) writer structure"]
impl crate::Writable for IC_DMA_TDLR {}
#[doc = "DMA Transmit Data Level Register"]
pub mod ic_dma_tdlr;
#[doc = "I2C Receive Data Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_dma_rdlr](ic_dma_rdlr) module"]
pub type IC_DMA_RDLR = crate::Reg<u32, _IC_DMA_RDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_DMA_RDLR;
#[doc = "`read()` method returns [ic_dma_rdlr::R](ic_dma_rdlr::R) reader structure"]
impl crate::Readable for IC_DMA_RDLR {}
#[doc = "`write(|w| ..)` method takes [ic_dma_rdlr::W](ic_dma_rdlr::W) writer structure"]
impl crate::Writable for IC_DMA_RDLR {}
#[doc = "I2C Receive Data Level Register"]
pub mod ic_dma_rdlr;
#[doc = "I2C SDA Setup Register\\n\\n This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.\\n\\n Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.\\n\\n Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_sda_setup](ic_sda_setup) module"]
pub type IC_SDA_SETUP = crate::Reg<u32, _IC_SDA_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_SDA_SETUP;
#[doc = "`read()` method returns [ic_sda_setup::R](ic_sda_setup::R) reader structure"]
impl crate::Readable for IC_SDA_SETUP {}
#[doc = "`write(|w| ..)` method takes [ic_sda_setup::W](ic_sda_setup::W) writer structure"]
impl crate::Writable for IC_SDA_SETUP {}
#[doc = "I2C SDA Setup Register\\n\\n This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.\\n\\n Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.\\n\\n Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
pub mod ic_sda_setup;
#[doc = "I2C ACK General Call Register\\n\\n The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.\\n\\n This register is applicable only when the DW_apb_i2c is in slave mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_ack_general_call](ic_ack_general_call) module"]
pub type IC_ACK_GENERAL_CALL = crate::Reg<u32, _IC_ACK_GENERAL_CALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_ACK_GENERAL_CALL;
#[doc = "`read()` method returns [ic_ack_general_call::R](ic_ack_general_call::R) reader structure"]
impl crate::Readable for IC_ACK_GENERAL_CALL {}
#[doc = "`write(|w| ..)` method takes [ic_ack_general_call::W](ic_ack_general_call::W) writer structure"]
impl crate::Writable for IC_ACK_GENERAL_CALL {}
#[doc = "I2C ACK General Call Register\\n\\n The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.\\n\\n This register is applicable only when the DW_apb_i2c is in slave mode."]
pub mod ic_ack_general_call;
#[doc = "I2C Enable Status Register\\n\\n The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE\\[0\\]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled.\\n\\n If IC_ENABLE\\[0\\]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.\\n\\n If IC_ENABLE\\[0\\]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.\\n\\n Note: When IC_ENABLE\\[0\\]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_enable_status](ic_enable_status) module"]
pub type IC_ENABLE_STATUS = crate::Reg<u32, _IC_ENABLE_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_ENABLE_STATUS;
#[doc = "`read()` method returns [ic_enable_status::R](ic_enable_status::R) reader structure"]
impl crate::Readable for IC_ENABLE_STATUS {}
#[doc = "I2C Enable Status Register\\n\\n The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE\\[0\\]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled.\\n\\n If IC_ENABLE\\[0\\]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.\\n\\n If IC_ENABLE\\[0\\]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.\\n\\n Note: When IC_ENABLE\\[0\\]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
pub mod ic_enable_status;
#[doc = "I2C SS, FS or FM+ spike suppression limit\\n\\n This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_fs_spklen](ic_fs_spklen) module"]
pub type IC_FS_SPKLEN = crate::Reg<u32, _IC_FS_SPKLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_FS_SPKLEN;
#[doc = "`read()` method returns [ic_fs_spklen::R](ic_fs_spklen::R) reader structure"]
impl crate::Readable for IC_FS_SPKLEN {}
#[doc = "`write(|w| ..)` method takes [ic_fs_spklen::W](ic_fs_spklen::W) writer structure"]
impl crate::Writable for IC_FS_SPKLEN {}
#[doc = "I2C SS, FS or FM+ spike suppression limit\\n\\n This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
pub mod ic_fs_spklen;
#[doc = "Clear RESTART_DET Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_clr_restart_det](ic_clr_restart_det) module"]
pub type IC_CLR_RESTART_DET = crate::Reg<u32, _IC_CLR_RESTART_DET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_CLR_RESTART_DET;
#[doc = "`read()` method returns [ic_clr_restart_det::R](ic_clr_restart_det::R) reader structure"]
impl crate::Readable for IC_CLR_RESTART_DET {}
#[doc = "Clear RESTART_DET Interrupt Register"]
pub mod ic_clr_restart_det;
#[doc = "Component Parameter Register 1\\n\\n Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_comp_param_1](ic_comp_param_1) module"]
pub type IC_COMP_PARAM_1 = crate::Reg<u32, _IC_COMP_PARAM_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_COMP_PARAM_1;
#[doc = "`read()` method returns [ic_comp_param_1::R](ic_comp_param_1::R) reader structure"]
impl crate::Readable for IC_COMP_PARAM_1 {}
#[doc = "Component Parameter Register 1\\n\\n Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
pub mod ic_comp_param_1;
#[doc = "I2C Component Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_comp_version](ic_comp_version) module"]
pub type IC_COMP_VERSION = crate::Reg<u32, _IC_COMP_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_COMP_VERSION;
#[doc = "`read()` method returns [ic_comp_version::R](ic_comp_version::R) reader structure"]
impl crate::Readable for IC_COMP_VERSION {}
#[doc = "I2C Component Version Register"]
pub mod ic_comp_version;
#[doc = "I2C Component Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_comp_type](ic_comp_type) module"]
pub type IC_COMP_TYPE = crate::Reg<u32, _IC_COMP_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_COMP_TYPE;
#[doc = "`read()` method returns [ic_comp_type::R](ic_comp_type::R) reader structure"]
impl crate::Readable for IC_COMP_TYPE {}
#[doc = "I2C Component Type Register"]
pub mod ic_comp_type;
