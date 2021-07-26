#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
    pub ic_con: crate::Reg<ic_con::IC_CON_SPEC>,
    #[doc = "0x04 - I2C Target Address Register  

 This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.  

 Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
    pub ic_tar: crate::Reg<ic_tar::IC_TAR_SPEC>,
    #[doc = "0x08 - I2C Slave Address Register"]
    pub ic_sar: crate::Reg<ic_sar::IC_SAR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.  

 The size of the register changes as follows:  

 Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
    pub ic_data_cmd: crate::Reg<ic_data_cmd::IC_DATA_CMD_SPEC>,
    #[doc = "0x14 - Standard Speed I2C Clock SCL High Count Register"]
    pub ic_ss_scl_hcnt: crate::Reg<ic_ss_scl_hcnt::IC_SS_SCL_HCNT_SPEC>,
    #[doc = "0x18 - Standard Speed I2C Clock SCL Low Count Register"]
    pub ic_ss_scl_lcnt: crate::Reg<ic_ss_scl_lcnt::IC_SS_SCL_LCNT_SPEC>,
    #[doc = "0x1c - Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
    pub ic_fs_scl_hcnt: crate::Reg<ic_fs_scl_hcnt::IC_FS_SCL_HCNT_SPEC>,
    #[doc = "0x20 - Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
    pub ic_fs_scl_lcnt: crate::Reg<ic_fs_scl_lcnt::IC_FS_SCL_LCNT_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x2c - I2C Interrupt Status Register  

 Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
    pub ic_intr_stat: crate::Reg<ic_intr_stat::IC_INTR_STAT_SPEC>,
    #[doc = "0x30 - I2C Interrupt Mask Register.  

 These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
    pub ic_intr_mask: crate::Reg<ic_intr_mask::IC_INTR_MASK_SPEC>,
    #[doc = "0x34 - I2C Raw Interrupt Status Register  

 Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
    pub ic_raw_intr_stat: crate::Reg<ic_raw_intr_stat::IC_RAW_INTR_STAT_SPEC>,
    #[doc = "0x38 - I2C Receive FIFO Threshold Register"]
    pub ic_rx_tl: crate::Reg<ic_rx_tl::IC_RX_TL_SPEC>,
    #[doc = "0x3c - I2C Transmit FIFO Threshold Register"]
    pub ic_tx_tl: crate::Reg<ic_tx_tl::IC_TX_TL_SPEC>,
    #[doc = "0x40 - Clear Combined and Individual Interrupt Register"]
    pub ic_clr_intr: crate::Reg<ic_clr_intr::IC_CLR_INTR_SPEC>,
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    pub ic_clr_rx_under: crate::Reg<ic_clr_rx_under::IC_CLR_RX_UNDER_SPEC>,
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    pub ic_clr_rx_over: crate::Reg<ic_clr_rx_over::IC_CLR_RX_OVER_SPEC>,
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    pub ic_clr_tx_over: crate::Reg<ic_clr_tx_over::IC_CLR_TX_OVER_SPEC>,
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    pub ic_clr_rd_req: crate::Reg<ic_clr_rd_req::IC_CLR_RD_REQ_SPEC>,
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    pub ic_clr_tx_abrt: crate::Reg<ic_clr_tx_abrt::IC_CLR_TX_ABRT_SPEC>,
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    pub ic_clr_rx_done: crate::Reg<ic_clr_rx_done::IC_CLR_RX_DONE_SPEC>,
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    pub ic_clr_activity: crate::Reg<ic_clr_activity::IC_CLR_ACTIVITY_SPEC>,
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    pub ic_clr_stop_det: crate::Reg<ic_clr_stop_det::IC_CLR_STOP_DET_SPEC>,
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    pub ic_clr_start_det: crate::Reg<ic_clr_start_det::IC_CLR_START_DET_SPEC>,
    #[doc = "0x68 - Clear GEN_CALL Interrupt Register"]
    pub ic_clr_gen_call: crate::Reg<ic_clr_gen_call::IC_CLR_GEN_CALL_SPEC>,
    #[doc = "0x6c - I2C Enable Register"]
    pub ic_enable: crate::Reg<ic_enable::IC_ENABLE_SPEC>,
    #[doc = "0x70 - I2C Status Register  

 This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.  

 When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
    pub ic_status: crate::Reg<ic_status::IC_STATUS_SPEC>,
    #[doc = "0x74 - I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
    pub ic_txflr: crate::Reg<ic_txflr::IC_TXFLR_SPEC>,
    #[doc = "0x78 - I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
    pub ic_rxflr: crate::Reg<ic_rxflr::IC_RXFLR_SPEC>,
    #[doc = "0x7c - I2C SDA Hold Time Length Register  

 The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).  

 The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]=0.  

 The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode (one cycle in master mode, seven cycles in slave mode) for the value to be implemented.  

 The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles."]
    pub ic_sda_hold: crate::Reg<ic_sda_hold::IC_SDA_HOLD_SPEC>,
    #[doc = "0x80 - I2C Transmit Abort Source Register  

 This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).  

 Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
    pub ic_tx_abrt_source: crate::Reg<ic_tx_abrt_source::IC_TX_ABRT_SOURCE_SPEC>,
    #[doc = "0x84 - Generate Slave Data NACK Register  

 The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.  

 A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
    pub ic_slv_data_nack_only: crate::Reg<ic_slv_data_nack_only::IC_SLV_DATA_NACK_ONLY_SPEC>,
    #[doc = "0x88 - DMA Control Register  

 The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
    pub ic_dma_cr: crate::Reg<ic_dma_cr::IC_DMA_CR_SPEC>,
    #[doc = "0x8c - DMA Transmit Data Level Register"]
    pub ic_dma_tdlr: crate::Reg<ic_dma_tdlr::IC_DMA_TDLR_SPEC>,
    #[doc = "0x90 - I2C Receive Data Level Register"]
    pub ic_dma_rdlr: crate::Reg<ic_dma_rdlr::IC_DMA_RDLR_SPEC>,
    #[doc = "0x94 - I2C SDA Setup Register  

 This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.  

 Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
    pub ic_sda_setup: crate::Reg<ic_sda_setup::IC_SDA_SETUP_SPEC>,
    #[doc = "0x98 - I2C ACK General Call Register  

 The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.  

 This register is applicable only when the DW_apb_i2c is in slave mode."]
    pub ic_ack_general_call: crate::Reg<ic_ack_general_call::IC_ACK_GENERAL_CALL_SPEC>,
    #[doc = "0x9c - I2C Enable Status Register  

 The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE\\[0\\]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled.  

 If IC_ENABLE\\[0\\]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.  

 If IC_ENABLE\\[0\\]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.  

 Note: When IC_ENABLE\\[0\\]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
    pub ic_enable_status: crate::Reg<ic_enable_status::IC_ENABLE_STATUS_SPEC>,
    #[doc = "0xa0 - I2C SS, FS or FM+ spike suppression limit  

 This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
    pub ic_fs_spklen: crate::Reg<ic_fs_spklen::IC_FS_SPKLEN_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0xa8 - Clear RESTART_DET Interrupt Register"]
    pub ic_clr_restart_det: crate::Reg<ic_clr_restart_det::IC_CLR_RESTART_DET_SPEC>,
    _reserved39: [u8; 0x48],
    #[doc = "0xf4 - Component Parameter Register 1  

 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
    pub ic_comp_param_1: crate::Reg<ic_comp_param_1::IC_COMP_PARAM_1_SPEC>,
    #[doc = "0xf8 - I2C Component Version Register"]
    pub ic_comp_version: crate::Reg<ic_comp_version::IC_COMP_VERSION_SPEC>,
    #[doc = "0xfc - I2C Component Type Register"]
    pub ic_comp_type: crate::Reg<ic_comp_type::IC_COMP_TYPE_SPEC>,
}
#[doc = "IC_CON register accessor: an alias for `Reg<IC_CON_SPEC>`"]
pub type IC_CON = crate::Reg<ic_con::IC_CON_SPEC>;
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
pub mod ic_con;
#[doc = "IC_TAR register accessor: an alias for `Reg<IC_TAR_SPEC>`"]
pub type IC_TAR = crate::Reg<ic_tar::IC_TAR_SPEC>;
#[doc = "I2C Target Address Register  

 This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.  

 Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
pub mod ic_tar;
#[doc = "IC_SAR register accessor: an alias for `Reg<IC_SAR_SPEC>`"]
pub type IC_SAR = crate::Reg<ic_sar::IC_SAR_SPEC>;
#[doc = "I2C Slave Address Register"]
pub mod ic_sar;
#[doc = "IC_DATA_CMD register accessor: an alias for `Reg<IC_DATA_CMD_SPEC>`"]
pub type IC_DATA_CMD = crate::Reg<ic_data_cmd::IC_DATA_CMD_SPEC>;
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.  

 The size of the register changes as follows:  

 Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
pub mod ic_data_cmd;
#[doc = "IC_SS_SCL_HCNT register accessor: an alias for `Reg<IC_SS_SCL_HCNT_SPEC>`"]
pub type IC_SS_SCL_HCNT = crate::Reg<ic_ss_scl_hcnt::IC_SS_SCL_HCNT_SPEC>;
#[doc = "Standard Speed I2C Clock SCL High Count Register"]
pub mod ic_ss_scl_hcnt;
#[doc = "IC_SS_SCL_LCNT register accessor: an alias for `Reg<IC_SS_SCL_LCNT_SPEC>`"]
pub type IC_SS_SCL_LCNT = crate::Reg<ic_ss_scl_lcnt::IC_SS_SCL_LCNT_SPEC>;
#[doc = "Standard Speed I2C Clock SCL Low Count Register"]
pub mod ic_ss_scl_lcnt;
#[doc = "IC_FS_SCL_HCNT register accessor: an alias for `Reg<IC_FS_SCL_HCNT_SPEC>`"]
pub type IC_FS_SCL_HCNT = crate::Reg<ic_fs_scl_hcnt::IC_FS_SCL_HCNT_SPEC>;
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
pub mod ic_fs_scl_hcnt;
#[doc = "IC_FS_SCL_LCNT register accessor: an alias for `Reg<IC_FS_SCL_LCNT_SPEC>`"]
pub type IC_FS_SCL_LCNT = crate::Reg<ic_fs_scl_lcnt::IC_FS_SCL_LCNT_SPEC>;
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
pub mod ic_fs_scl_lcnt;
#[doc = "IC_INTR_STAT register accessor: an alias for `Reg<IC_INTR_STAT_SPEC>`"]
pub type IC_INTR_STAT = crate::Reg<ic_intr_stat::IC_INTR_STAT_SPEC>;
#[doc = "I2C Interrupt Status Register  

 Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
pub mod ic_intr_stat;
#[doc = "IC_INTR_MASK register accessor: an alias for `Reg<IC_INTR_MASK_SPEC>`"]
pub type IC_INTR_MASK = crate::Reg<ic_intr_mask::IC_INTR_MASK_SPEC>;
#[doc = "I2C Interrupt Mask Register.  

 These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
pub mod ic_intr_mask;
#[doc = "IC_RAW_INTR_STAT register accessor: an alias for `Reg<IC_RAW_INTR_STAT_SPEC>`"]
pub type IC_RAW_INTR_STAT = crate::Reg<ic_raw_intr_stat::IC_RAW_INTR_STAT_SPEC>;
#[doc = "I2C Raw Interrupt Status Register  

 Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
pub mod ic_raw_intr_stat;
#[doc = "IC_RX_TL register accessor: an alias for `Reg<IC_RX_TL_SPEC>`"]
pub type IC_RX_TL = crate::Reg<ic_rx_tl::IC_RX_TL_SPEC>;
#[doc = "I2C Receive FIFO Threshold Register"]
pub mod ic_rx_tl;
#[doc = "IC_TX_TL register accessor: an alias for `Reg<IC_TX_TL_SPEC>`"]
pub type IC_TX_TL = crate::Reg<ic_tx_tl::IC_TX_TL_SPEC>;
#[doc = "I2C Transmit FIFO Threshold Register"]
pub mod ic_tx_tl;
#[doc = "IC_CLR_INTR register accessor: an alias for `Reg<IC_CLR_INTR_SPEC>`"]
pub type IC_CLR_INTR = crate::Reg<ic_clr_intr::IC_CLR_INTR_SPEC>;
#[doc = "Clear Combined and Individual Interrupt Register"]
pub mod ic_clr_intr;
#[doc = "IC_CLR_RX_UNDER register accessor: an alias for `Reg<IC_CLR_RX_UNDER_SPEC>`"]
pub type IC_CLR_RX_UNDER = crate::Reg<ic_clr_rx_under::IC_CLR_RX_UNDER_SPEC>;
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod ic_clr_rx_under;
#[doc = "IC_CLR_RX_OVER register accessor: an alias for `Reg<IC_CLR_RX_OVER_SPEC>`"]
pub type IC_CLR_RX_OVER = crate::Reg<ic_clr_rx_over::IC_CLR_RX_OVER_SPEC>;
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod ic_clr_rx_over;
#[doc = "IC_CLR_TX_OVER register accessor: an alias for `Reg<IC_CLR_TX_OVER_SPEC>`"]
pub type IC_CLR_TX_OVER = crate::Reg<ic_clr_tx_over::IC_CLR_TX_OVER_SPEC>;
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod ic_clr_tx_over;
#[doc = "IC_CLR_RD_REQ register accessor: an alias for `Reg<IC_CLR_RD_REQ_SPEC>`"]
pub type IC_CLR_RD_REQ = crate::Reg<ic_clr_rd_req::IC_CLR_RD_REQ_SPEC>;
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod ic_clr_rd_req;
#[doc = "IC_CLR_TX_ABRT register accessor: an alias for `Reg<IC_CLR_TX_ABRT_SPEC>`"]
pub type IC_CLR_TX_ABRT = crate::Reg<ic_clr_tx_abrt::IC_CLR_TX_ABRT_SPEC>;
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod ic_clr_tx_abrt;
#[doc = "IC_CLR_RX_DONE register accessor: an alias for `Reg<IC_CLR_RX_DONE_SPEC>`"]
pub type IC_CLR_RX_DONE = crate::Reg<ic_clr_rx_done::IC_CLR_RX_DONE_SPEC>;
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod ic_clr_rx_done;
#[doc = "IC_CLR_ACTIVITY register accessor: an alias for `Reg<IC_CLR_ACTIVITY_SPEC>`"]
pub type IC_CLR_ACTIVITY = crate::Reg<ic_clr_activity::IC_CLR_ACTIVITY_SPEC>;
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod ic_clr_activity;
#[doc = "IC_CLR_STOP_DET register accessor: an alias for `Reg<IC_CLR_STOP_DET_SPEC>`"]
pub type IC_CLR_STOP_DET = crate::Reg<ic_clr_stop_det::IC_CLR_STOP_DET_SPEC>;
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod ic_clr_stop_det;
#[doc = "IC_CLR_START_DET register accessor: an alias for `Reg<IC_CLR_START_DET_SPEC>`"]
pub type IC_CLR_START_DET = crate::Reg<ic_clr_start_det::IC_CLR_START_DET_SPEC>;
#[doc = "Clear START_DET Interrupt Register"]
pub mod ic_clr_start_det;
#[doc = "IC_CLR_GEN_CALL register accessor: an alias for `Reg<IC_CLR_GEN_CALL_SPEC>`"]
pub type IC_CLR_GEN_CALL = crate::Reg<ic_clr_gen_call::IC_CLR_GEN_CALL_SPEC>;
#[doc = "Clear GEN_CALL Interrupt Register"]
pub mod ic_clr_gen_call;
#[doc = "IC_ENABLE register accessor: an alias for `Reg<IC_ENABLE_SPEC>`"]
pub type IC_ENABLE = crate::Reg<ic_enable::IC_ENABLE_SPEC>;
#[doc = "I2C Enable Register"]
pub mod ic_enable;
#[doc = "IC_STATUS register accessor: an alias for `Reg<IC_STATUS_SPEC>`"]
pub type IC_STATUS = crate::Reg<ic_status::IC_STATUS_SPEC>;
#[doc = "I2C Status Register  

 This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.  

 When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
pub mod ic_status;
#[doc = "IC_TXFLR register accessor: an alias for `Reg<IC_TXFLR_SPEC>`"]
pub type IC_TXFLR = crate::Reg<ic_txflr::IC_TXFLR_SPEC>;
#[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
pub mod ic_txflr;
#[doc = "IC_RXFLR register accessor: an alias for `Reg<IC_RXFLR_SPEC>`"]
pub type IC_RXFLR = crate::Reg<ic_rxflr::IC_RXFLR_SPEC>;
#[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
pub mod ic_rxflr;
#[doc = "IC_SDA_HOLD register accessor: an alias for `Reg<IC_SDA_HOLD_SPEC>`"]
pub type IC_SDA_HOLD = crate::Reg<ic_sda_hold::IC_SDA_HOLD_SPEC>;
#[doc = "I2C SDA Hold Time Length Register  

 The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).  

 The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]=0.  

 The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode (one cycle in master mode, seven cycles in slave mode) for the value to be implemented.  

 The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles."]
pub mod ic_sda_hold;
#[doc = "IC_TX_ABRT_SOURCE register accessor: an alias for `Reg<IC_TX_ABRT_SOURCE_SPEC>`"]
pub type IC_TX_ABRT_SOURCE = crate::Reg<ic_tx_abrt_source::IC_TX_ABRT_SOURCE_SPEC>;
#[doc = "I2C Transmit Abort Source Register  

 This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).  

 Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
pub mod ic_tx_abrt_source;
#[doc = "IC_SLV_DATA_NACK_ONLY register accessor: an alias for `Reg<IC_SLV_DATA_NACK_ONLY_SPEC>`"]
pub type IC_SLV_DATA_NACK_ONLY = crate::Reg<ic_slv_data_nack_only::IC_SLV_DATA_NACK_ONLY_SPEC>;
#[doc = "Generate Slave Data NACK Register  

 The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.  

 A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
pub mod ic_slv_data_nack_only;
#[doc = "IC_DMA_CR register accessor: an alias for `Reg<IC_DMA_CR_SPEC>`"]
pub type IC_DMA_CR = crate::Reg<ic_dma_cr::IC_DMA_CR_SPEC>;
#[doc = "DMA Control Register  

 The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
pub mod ic_dma_cr;
#[doc = "IC_DMA_TDLR register accessor: an alias for `Reg<IC_DMA_TDLR_SPEC>`"]
pub type IC_DMA_TDLR = crate::Reg<ic_dma_tdlr::IC_DMA_TDLR_SPEC>;
#[doc = "DMA Transmit Data Level Register"]
pub mod ic_dma_tdlr;
#[doc = "IC_DMA_RDLR register accessor: an alias for `Reg<IC_DMA_RDLR_SPEC>`"]
pub type IC_DMA_RDLR = crate::Reg<ic_dma_rdlr::IC_DMA_RDLR_SPEC>;
#[doc = "I2C Receive Data Level Register"]
pub mod ic_dma_rdlr;
#[doc = "IC_SDA_SETUP register accessor: an alias for `Reg<IC_SDA_SETUP_SPEC>`"]
pub type IC_SDA_SETUP = crate::Reg<ic_sda_setup::IC_SDA_SETUP_SPEC>;
#[doc = "I2C SDA Setup Register  

 This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.  

 Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
pub mod ic_sda_setup;
#[doc = "IC_ACK_GENERAL_CALL register accessor: an alias for `Reg<IC_ACK_GENERAL_CALL_SPEC>`"]
pub type IC_ACK_GENERAL_CALL = crate::Reg<ic_ack_general_call::IC_ACK_GENERAL_CALL_SPEC>;
#[doc = "I2C ACK General Call Register  

 The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.  

 This register is applicable only when the DW_apb_i2c is in slave mode."]
pub mod ic_ack_general_call;
#[doc = "IC_ENABLE_STATUS register accessor: an alias for `Reg<IC_ENABLE_STATUS_SPEC>`"]
pub type IC_ENABLE_STATUS = crate::Reg<ic_enable_status::IC_ENABLE_STATUS_SPEC>;
#[doc = "I2C Enable Status Register  

 The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE\\[0\\]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled.  

 If IC_ENABLE\\[0\\]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.  

 If IC_ENABLE\\[0\\]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.  

 Note: When IC_ENABLE\\[0\\]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
pub mod ic_enable_status;
#[doc = "IC_FS_SPKLEN register accessor: an alias for `Reg<IC_FS_SPKLEN_SPEC>`"]
pub type IC_FS_SPKLEN = crate::Reg<ic_fs_spklen::IC_FS_SPKLEN_SPEC>;
#[doc = "I2C SS, FS or FM+ spike suppression limit  

 This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
pub mod ic_fs_spklen;
#[doc = "IC_CLR_RESTART_DET register accessor: an alias for `Reg<IC_CLR_RESTART_DET_SPEC>`"]
pub type IC_CLR_RESTART_DET = crate::Reg<ic_clr_restart_det::IC_CLR_RESTART_DET_SPEC>;
#[doc = "Clear RESTART_DET Interrupt Register"]
pub mod ic_clr_restart_det;
#[doc = "IC_COMP_PARAM_1 register accessor: an alias for `Reg<IC_COMP_PARAM_1_SPEC>`"]
pub type IC_COMP_PARAM_1 = crate::Reg<ic_comp_param_1::IC_COMP_PARAM_1_SPEC>;
#[doc = "Component Parameter Register 1  

 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
pub mod ic_comp_param_1;
#[doc = "IC_COMP_VERSION register accessor: an alias for `Reg<IC_COMP_VERSION_SPEC>`"]
pub type IC_COMP_VERSION = crate::Reg<ic_comp_version::IC_COMP_VERSION_SPEC>;
#[doc = "I2C Component Version Register"]
pub mod ic_comp_version;
#[doc = "IC_COMP_TYPE register accessor: an alias for `Reg<IC_COMP_TYPE_SPEC>`"]
pub type IC_COMP_TYPE = crate::Reg<ic_comp_type::IC_COMP_TYPE_SPEC>;
#[doc = "I2C Component Type Register"]
pub mod ic_comp_type;
