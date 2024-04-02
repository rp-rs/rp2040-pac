#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ic_con: IC_CON,
    ic_tar: IC_TAR,
    ic_sar: IC_SAR,
    _reserved3: [u8; 0x04],
    ic_data_cmd: IC_DATA_CMD,
    ic_ss_scl_hcnt: IC_SS_SCL_HCNT,
    ic_ss_scl_lcnt: IC_SS_SCL_LCNT,
    ic_fs_scl_hcnt: IC_FS_SCL_HCNT,
    ic_fs_scl_lcnt: IC_FS_SCL_LCNT,
    _reserved8: [u8; 0x08],
    ic_intr_stat: IC_INTR_STAT,
    ic_intr_mask: IC_INTR_MASK,
    ic_raw_intr_stat: IC_RAW_INTR_STAT,
    ic_rx_tl: IC_RX_TL,
    ic_tx_tl: IC_TX_TL,
    ic_clr_intr: IC_CLR_INTR,
    ic_clr_rx_under: IC_CLR_RX_UNDER,
    ic_clr_rx_over: IC_CLR_RX_OVER,
    ic_clr_tx_over: IC_CLR_TX_OVER,
    ic_clr_rd_req: IC_CLR_RD_REQ,
    ic_clr_tx_abrt: IC_CLR_TX_ABRT,
    ic_clr_rx_done: IC_CLR_RX_DONE,
    ic_clr_activity: IC_CLR_ACTIVITY,
    ic_clr_stop_det: IC_CLR_STOP_DET,
    ic_clr_start_det: IC_CLR_START_DET,
    ic_clr_gen_call: IC_CLR_GEN_CALL,
    ic_enable: IC_ENABLE,
    ic_status: IC_STATUS,
    ic_txflr: IC_TXFLR,
    ic_rxflr: IC_RXFLR,
    ic_sda_hold: IC_SDA_HOLD,
    ic_tx_abrt_source: IC_TX_ABRT_SOURCE,
    ic_slv_data_nack_only: IC_SLV_DATA_NACK_ONLY,
    ic_dma_cr: IC_DMA_CR,
    ic_dma_tdlr: IC_DMA_TDLR,
    ic_dma_rdlr: IC_DMA_RDLR,
    ic_sda_setup: IC_SDA_SETUP,
    ic_ack_general_call: IC_ACK_GENERAL_CALL,
    ic_enable_status: IC_ENABLE_STATUS,
    ic_fs_spklen: IC_FS_SPKLEN,
    _reserved38: [u8; 0x04],
    ic_clr_restart_det: IC_CLR_RESTART_DET,
    _reserved39: [u8; 0x48],
    ic_comp_param_1: IC_COMP_PARAM_1,
    ic_comp_version: IC_COMP_VERSION,
    ic_comp_type: IC_COMP_TYPE,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
    #[inline(always)]
    pub const fn ic_con(&self) -> &IC_CON {
        &self.ic_con
    }
    #[doc = "0x04 - I2C Target Address Register  

 This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.  

 Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
    #[inline(always)]
    pub const fn ic_tar(&self) -> &IC_TAR {
        &self.ic_tar
    }
    #[doc = "0x08 - I2C Slave Address Register"]
    #[inline(always)]
    pub const fn ic_sar(&self) -> &IC_SAR {
        &self.ic_sar
    }
    #[doc = "0x10 - I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.  

 The size of the register changes as follows:  

 Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
    #[inline(always)]
    pub const fn ic_data_cmd(&self) -> &IC_DATA_CMD {
        &self.ic_data_cmd
    }
    #[doc = "0x14 - Standard Speed I2C Clock SCL High Count Register"]
    #[inline(always)]
    pub const fn ic_ss_scl_hcnt(&self) -> &IC_SS_SCL_HCNT {
        &self.ic_ss_scl_hcnt
    }
    #[doc = "0x18 - Standard Speed I2C Clock SCL Low Count Register"]
    #[inline(always)]
    pub const fn ic_ss_scl_lcnt(&self) -> &IC_SS_SCL_LCNT {
        &self.ic_ss_scl_lcnt
    }
    #[doc = "0x1c - Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
    #[inline(always)]
    pub const fn ic_fs_scl_hcnt(&self) -> &IC_FS_SCL_HCNT {
        &self.ic_fs_scl_hcnt
    }
    #[doc = "0x20 - Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
    #[inline(always)]
    pub const fn ic_fs_scl_lcnt(&self) -> &IC_FS_SCL_LCNT {
        &self.ic_fs_scl_lcnt
    }
    #[doc = "0x2c - I2C Interrupt Status Register  

 Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub const fn ic_intr_stat(&self) -> &IC_INTR_STAT {
        &self.ic_intr_stat
    }
    #[doc = "0x30 - I2C Interrupt Mask Register.  

 These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
    #[inline(always)]
    pub const fn ic_intr_mask(&self) -> &IC_INTR_MASK {
        &self.ic_intr_mask
    }
    #[doc = "0x34 - I2C Raw Interrupt Status Register  

 Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
    #[inline(always)]
    pub const fn ic_raw_intr_stat(&self) -> &IC_RAW_INTR_STAT {
        &self.ic_raw_intr_stat
    }
    #[doc = "0x38 - I2C Receive FIFO Threshold Register"]
    #[inline(always)]
    pub const fn ic_rx_tl(&self) -> &IC_RX_TL {
        &self.ic_rx_tl
    }
    #[doc = "0x3c - I2C Transmit FIFO Threshold Register"]
    #[inline(always)]
    pub const fn ic_tx_tl(&self) -> &IC_TX_TL {
        &self.ic_tx_tl
    }
    #[doc = "0x40 - Clear Combined and Individual Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_intr(&self) -> &IC_CLR_INTR {
        &self.ic_clr_intr
    }
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_rx_under(&self) -> &IC_CLR_RX_UNDER {
        &self.ic_clr_rx_under
    }
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_rx_over(&self) -> &IC_CLR_RX_OVER {
        &self.ic_clr_rx_over
    }
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_tx_over(&self) -> &IC_CLR_TX_OVER {
        &self.ic_clr_tx_over
    }
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_rd_req(&self) -> &IC_CLR_RD_REQ {
        &self.ic_clr_rd_req
    }
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_tx_abrt(&self) -> &IC_CLR_TX_ABRT {
        &self.ic_clr_tx_abrt
    }
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_rx_done(&self) -> &IC_CLR_RX_DONE {
        &self.ic_clr_rx_done
    }
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_activity(&self) -> &IC_CLR_ACTIVITY {
        &self.ic_clr_activity
    }
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_stop_det(&self) -> &IC_CLR_STOP_DET {
        &self.ic_clr_stop_det
    }
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_start_det(&self) -> &IC_CLR_START_DET {
        &self.ic_clr_start_det
    }
    #[doc = "0x68 - Clear GEN_CALL Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_gen_call(&self) -> &IC_CLR_GEN_CALL {
        &self.ic_clr_gen_call
    }
    #[doc = "0x6c - I2C Enable Register"]
    #[inline(always)]
    pub const fn ic_enable(&self) -> &IC_ENABLE {
        &self.ic_enable
    }
    #[doc = "0x70 - I2C Status Register  

 This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.  

 When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
    #[inline(always)]
    pub const fn ic_status(&self) -> &IC_STATUS {
        &self.ic_status
    }
    #[doc = "0x74 - I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
    #[inline(always)]
    pub const fn ic_txflr(&self) -> &IC_TXFLR {
        &self.ic_txflr
    }
    #[doc = "0x78 - I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
    #[inline(always)]
    pub const fn ic_rxflr(&self) -> &IC_RXFLR {
        &self.ic_rxflr
    }
    #[doc = "0x7c - I2C SDA Hold Time Length Register  

 The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).  

 The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]=0.  

 The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode (one cycle in master mode, seven cycles in slave mode) for the value to be implemented.  

 The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles."]
    #[inline(always)]
    pub const fn ic_sda_hold(&self) -> &IC_SDA_HOLD {
        &self.ic_sda_hold
    }
    #[doc = "0x80 - I2C Transmit Abort Source Register  

 This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).  

 Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
    #[inline(always)]
    pub const fn ic_tx_abrt_source(&self) -> &IC_TX_ABRT_SOURCE {
        &self.ic_tx_abrt_source
    }
    #[doc = "0x84 - Generate Slave Data NACK Register  

 The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.  

 A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
    #[inline(always)]
    pub const fn ic_slv_data_nack_only(&self) -> &IC_SLV_DATA_NACK_ONLY {
        &self.ic_slv_data_nack_only
    }
    #[doc = "0x88 - DMA Control Register  

 The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
    #[inline(always)]
    pub const fn ic_dma_cr(&self) -> &IC_DMA_CR {
        &self.ic_dma_cr
    }
    #[doc = "0x8c - DMA Transmit Data Level Register"]
    #[inline(always)]
    pub const fn ic_dma_tdlr(&self) -> &IC_DMA_TDLR {
        &self.ic_dma_tdlr
    }
    #[doc = "0x90 - I2C Receive Data Level Register"]
    #[inline(always)]
    pub const fn ic_dma_rdlr(&self) -> &IC_DMA_RDLR {
        &self.ic_dma_rdlr
    }
    #[doc = "0x94 - I2C SDA Setup Register  

 This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.  

 Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
    #[inline(always)]
    pub const fn ic_sda_setup(&self) -> &IC_SDA_SETUP {
        &self.ic_sda_setup
    }
    #[doc = "0x98 - I2C ACK General Call Register  

 The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.  

 This register is applicable only when the DW_apb_i2c is in slave mode."]
    #[inline(always)]
    pub const fn ic_ack_general_call(&self) -> &IC_ACK_GENERAL_CALL {
        &self.ic_ack_general_call
    }
    #[doc = "0x9c - I2C Enable Status Register  

 The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE\\[0\\]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled.  

 If IC_ENABLE\\[0\\]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.  

 If IC_ENABLE\\[0\\]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.  

 Note: When IC_ENABLE\\[0\\]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
    #[inline(always)]
    pub const fn ic_enable_status(&self) -> &IC_ENABLE_STATUS {
        &self.ic_enable_status
    }
    #[doc = "0xa0 - I2C SS, FS or FM+ spike suppression limit  

 This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
    #[inline(always)]
    pub const fn ic_fs_spklen(&self) -> &IC_FS_SPKLEN {
        &self.ic_fs_spklen
    }
    #[doc = "0xa8 - Clear RESTART_DET Interrupt Register"]
    #[inline(always)]
    pub const fn ic_clr_restart_det(&self) -> &IC_CLR_RESTART_DET {
        &self.ic_clr_restart_det
    }
    #[doc = "0xf4 - Component Parameter Register 1  

 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
    #[inline(always)]
    pub const fn ic_comp_param_1(&self) -> &IC_COMP_PARAM_1 {
        &self.ic_comp_param_1
    }
    #[doc = "0xf8 - I2C Component Version Register"]
    #[inline(always)]
    pub const fn ic_comp_version(&self) -> &IC_COMP_VERSION {
        &self.ic_comp_version
    }
    #[doc = "0xfc - I2C Component Type Register"]
    #[inline(always)]
    pub const fn ic_comp_type(&self) -> &IC_COMP_TYPE {
        &self.ic_comp_type
    }
}
#[doc = "IC_CON (rw) register accessor: I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only.  

You can [`read`](crate::Reg::read) this register and get [`ic_con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_con`]
module"]
pub type IC_CON = crate::Reg<ic_con::IC_CON_SPEC>;
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
pub mod ic_con;
#[doc = "IC_TAR (rw) register accessor: I2C Target Address Register  

 This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.  

 Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only.  

You can [`read`](crate::Reg::read) this register and get [`ic_tar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_tar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_tar`]
module"]
pub type IC_TAR = crate::Reg<ic_tar::IC_TAR_SPEC>;
#[doc = "I2C Target Address Register  

 This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.  

 Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
pub mod ic_tar;
#[doc = "IC_SAR (rw) register accessor: I2C Slave Address Register  

You can [`read`](crate::Reg::read) this register and get [`ic_sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_sar`]
module"]
pub type IC_SAR = crate::Reg<ic_sar::IC_SAR_SPEC>;
#[doc = "I2C Slave Address Register"]
pub mod ic_sar;
#[doc = "IC_DATA_CMD (rw) register accessor: I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.  

 The size of the register changes as follows:  

 Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging.  

You can [`read`](crate::Reg::read) this register and get [`ic_data_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_data_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_data_cmd`]
module"]
pub type IC_DATA_CMD = crate::Reg<ic_data_cmd::IC_DATA_CMD_SPEC>;
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.  

 The size of the register changes as follows:  

 Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
pub mod ic_data_cmd;
#[doc = "IC_SS_SCL_HCNT (rw) register accessor: Standard Speed I2C Clock SCL High Count Register  

You can [`read`](crate::Reg::read) this register and get [`ic_ss_scl_hcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ss_scl_hcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_ss_scl_hcnt`]
module"]
pub type IC_SS_SCL_HCNT = crate::Reg<ic_ss_scl_hcnt::IC_SS_SCL_HCNT_SPEC>;
#[doc = "Standard Speed I2C Clock SCL High Count Register"]
pub mod ic_ss_scl_hcnt;
#[doc = "IC_SS_SCL_LCNT (rw) register accessor: Standard Speed I2C Clock SCL Low Count Register  

You can [`read`](crate::Reg::read) this register and get [`ic_ss_scl_lcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ss_scl_lcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_ss_scl_lcnt`]
module"]
pub type IC_SS_SCL_LCNT = crate::Reg<ic_ss_scl_lcnt::IC_SS_SCL_LCNT_SPEC>;
#[doc = "Standard Speed I2C Clock SCL Low Count Register"]
pub mod ic_ss_scl_lcnt;
#[doc = "IC_FS_SCL_HCNT (rw) register accessor: Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register  

You can [`read`](crate::Reg::read) this register and get [`ic_fs_scl_hcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_scl_hcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_fs_scl_hcnt`]
module"]
pub type IC_FS_SCL_HCNT = crate::Reg<ic_fs_scl_hcnt::IC_FS_SCL_HCNT_SPEC>;
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
pub mod ic_fs_scl_hcnt;
#[doc = "IC_FS_SCL_LCNT (rw) register accessor: Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register  

You can [`read`](crate::Reg::read) this register and get [`ic_fs_scl_lcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_scl_lcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_fs_scl_lcnt`]
module"]
pub type IC_FS_SCL_LCNT = crate::Reg<ic_fs_scl_lcnt::IC_FS_SCL_LCNT_SPEC>;
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
pub mod ic_fs_scl_lcnt;
#[doc = "IC_INTR_STAT (r) register accessor: I2C Interrupt Status Register  

 Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register.  

You can [`read`](crate::Reg::read) this register and get [`ic_intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_intr_stat`]
module"]
pub type IC_INTR_STAT = crate::Reg<ic_intr_stat::IC_INTR_STAT_SPEC>;
#[doc = "I2C Interrupt Status Register  

 Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
pub mod ic_intr_stat;
#[doc = "IC_INTR_MASK (rw) register accessor: I2C Interrupt Mask Register.  

 These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt.  

You can [`read`](crate::Reg::read) this register and get [`ic_intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_intr_mask`]
module"]
pub type IC_INTR_MASK = crate::Reg<ic_intr_mask::IC_INTR_MASK_SPEC>;
#[doc = "I2C Interrupt Mask Register.  

 These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
pub mod ic_intr_mask;
#[doc = "IC_RAW_INTR_STAT (r) register accessor: I2C Raw Interrupt Status Register  

 Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c.  

You can [`read`](crate::Reg::read) this register and get [`ic_raw_intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_raw_intr_stat`]
module"]
pub type IC_RAW_INTR_STAT = crate::Reg<ic_raw_intr_stat::IC_RAW_INTR_STAT_SPEC>;
#[doc = "I2C Raw Interrupt Status Register  

 Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
pub mod ic_raw_intr_stat;
#[doc = "IC_RX_TL (rw) register accessor: I2C Receive FIFO Threshold Register  

You can [`read`](crate::Reg::read) this register and get [`ic_rx_tl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_rx_tl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_rx_tl`]
module"]
pub type IC_RX_TL = crate::Reg<ic_rx_tl::IC_RX_TL_SPEC>;
#[doc = "I2C Receive FIFO Threshold Register"]
pub mod ic_rx_tl;
#[doc = "IC_TX_TL (rw) register accessor: I2C Transmit FIFO Threshold Register  

You can [`read`](crate::Reg::read) this register and get [`ic_tx_tl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_tx_tl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_tx_tl`]
module"]
pub type IC_TX_TL = crate::Reg<ic_tx_tl::IC_TX_TL_SPEC>;
#[doc = "I2C Transmit FIFO Threshold Register"]
pub mod ic_tx_tl;
#[doc = "IC_CLR_INTR (r) register accessor: Clear Combined and Individual Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_intr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_intr`]
module"]
pub type IC_CLR_INTR = crate::Reg<ic_clr_intr::IC_CLR_INTR_SPEC>;
#[doc = "Clear Combined and Individual Interrupt Register"]
pub mod ic_clr_intr;
#[doc = "IC_CLR_RX_UNDER (r) register accessor: Clear RX_UNDER Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_under::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_rx_under`]
module"]
pub type IC_CLR_RX_UNDER = crate::Reg<ic_clr_rx_under::IC_CLR_RX_UNDER_SPEC>;
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod ic_clr_rx_under;
#[doc = "IC_CLR_RX_OVER (r) register accessor: Clear RX_OVER Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_rx_over`]
module"]
pub type IC_CLR_RX_OVER = crate::Reg<ic_clr_rx_over::IC_CLR_RX_OVER_SPEC>;
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod ic_clr_rx_over;
#[doc = "IC_CLR_TX_OVER (r) register accessor: Clear TX_OVER Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_tx_over`]
module"]
pub type IC_CLR_TX_OVER = crate::Reg<ic_clr_tx_over::IC_CLR_TX_OVER_SPEC>;
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod ic_clr_tx_over;
#[doc = "IC_CLR_RD_REQ (r) register accessor: Clear RD_REQ Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rd_req::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_rd_req`]
module"]
pub type IC_CLR_RD_REQ = crate::Reg<ic_clr_rd_req::IC_CLR_RD_REQ_SPEC>;
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod ic_clr_rd_req;
#[doc = "IC_CLR_TX_ABRT (r) register accessor: Clear TX_ABRT Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_abrt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_tx_abrt`]
module"]
pub type IC_CLR_TX_ABRT = crate::Reg<ic_clr_tx_abrt::IC_CLR_TX_ABRT_SPEC>;
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod ic_clr_tx_abrt;
#[doc = "IC_CLR_RX_DONE (r) register accessor: Clear RX_DONE Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_done::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_rx_done`]
module"]
pub type IC_CLR_RX_DONE = crate::Reg<ic_clr_rx_done::IC_CLR_RX_DONE_SPEC>;
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod ic_clr_rx_done;
#[doc = "IC_CLR_ACTIVITY (r) register accessor: Clear ACTIVITY Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_activity::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_activity`]
module"]
pub type IC_CLR_ACTIVITY = crate::Reg<ic_clr_activity::IC_CLR_ACTIVITY_SPEC>;
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod ic_clr_activity;
#[doc = "IC_CLR_STOP_DET (r) register accessor: Clear STOP_DET Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_stop_det::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_stop_det`]
module"]
pub type IC_CLR_STOP_DET = crate::Reg<ic_clr_stop_det::IC_CLR_STOP_DET_SPEC>;
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod ic_clr_stop_det;
#[doc = "IC_CLR_START_DET (r) register accessor: Clear START_DET Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_start_det::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_start_det`]
module"]
pub type IC_CLR_START_DET = crate::Reg<ic_clr_start_det::IC_CLR_START_DET_SPEC>;
#[doc = "Clear START_DET Interrupt Register"]
pub mod ic_clr_start_det;
#[doc = "IC_CLR_GEN_CALL (r) register accessor: Clear GEN_CALL Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_gen_call::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_gen_call`]
module"]
pub type IC_CLR_GEN_CALL = crate::Reg<ic_clr_gen_call::IC_CLR_GEN_CALL_SPEC>;
#[doc = "Clear GEN_CALL Interrupt Register"]
pub mod ic_clr_gen_call;
#[doc = "IC_ENABLE (rw) register accessor: I2C Enable Register  

You can [`read`](crate::Reg::read) this register and get [`ic_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_enable`]
module"]
pub type IC_ENABLE = crate::Reg<ic_enable::IC_ENABLE_SPEC>;
#[doc = "I2C Enable Register"]
pub mod ic_enable;
#[doc = "IC_STATUS (r) register accessor: I2C Status Register  

 This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.  

 When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0  

You can [`read`](crate::Reg::read) this register and get [`ic_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_status`]
module"]
pub type IC_STATUS = crate::Reg<ic_status::IC_STATUS_SPEC>;
#[doc = "I2C Status Register  

 This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.  

 When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
pub mod ic_status;
#[doc = "IC_TXFLR (r) register accessor: I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO.  

You can [`read`](crate::Reg::read) this register and get [`ic_txflr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_txflr`]
module"]
pub type IC_TXFLR = crate::Reg<ic_txflr::IC_TXFLR_SPEC>;
#[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
pub mod ic_txflr;
#[doc = "IC_RXFLR (r) register accessor: I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO.  

You can [`read`](crate::Reg::read) this register and get [`ic_rxflr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_rxflr`]
module"]
pub type IC_RXFLR = crate::Reg<ic_rxflr::IC_RXFLR_SPEC>;
#[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
pub mod ic_rxflr;
#[doc = "IC_SDA_HOLD (rw) register accessor: I2C SDA Hold Time Length Register  

 The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).  

 The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]=0.  

 The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode (one cycle in master mode, seven cycles in slave mode) for the value to be implemented.  

 The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles.  

You can [`read`](crate::Reg::read) this register and get [`ic_sda_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_sda_hold`]
module"]
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
#[doc = "IC_TX_ABRT_SOURCE (r) register accessor: I2C Transmit Abort Source Register  

 This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).  

 Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted.  

You can [`read`](crate::Reg::read) this register and get [`ic_tx_abrt_source::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_tx_abrt_source`]
module"]
pub type IC_TX_ABRT_SOURCE = crate::Reg<ic_tx_abrt_source::IC_TX_ABRT_SOURCE_SPEC>;
#[doc = "I2C Transmit Abort Source Register  

 This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).  

 Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
pub mod ic_tx_abrt_source;
#[doc = "IC_SLV_DATA_NACK_ONLY (rw) register accessor: Generate Slave Data NACK Register  

 The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.  

 A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit.  

You can [`read`](crate::Reg::read) this register and get [`ic_slv_data_nack_only::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_slv_data_nack_only::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_slv_data_nack_only`]
module"]
pub type IC_SLV_DATA_NACK_ONLY = crate::Reg<ic_slv_data_nack_only::IC_SLV_DATA_NACK_ONLY_SPEC>;
#[doc = "Generate Slave Data NACK Register  

 The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.  

 A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
pub mod ic_slv_data_nack_only;
#[doc = "IC_DMA_CR (rw) register accessor: DMA Control Register  

 The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE.  

You can [`read`](crate::Reg::read) this register and get [`ic_dma_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_dma_cr`]
module"]
pub type IC_DMA_CR = crate::Reg<ic_dma_cr::IC_DMA_CR_SPEC>;
#[doc = "DMA Control Register  

 The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
pub mod ic_dma_cr;
#[doc = "IC_DMA_TDLR (rw) register accessor: DMA Transmit Data Level Register  

You can [`read`](crate::Reg::read) this register and get [`ic_dma_tdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_tdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_dma_tdlr`]
module"]
pub type IC_DMA_TDLR = crate::Reg<ic_dma_tdlr::IC_DMA_TDLR_SPEC>;
#[doc = "DMA Transmit Data Level Register"]
pub mod ic_dma_tdlr;
#[doc = "IC_DMA_RDLR (rw) register accessor: I2C Receive Data Level Register  

You can [`read`](crate::Reg::read) this register and get [`ic_dma_rdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_rdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_dma_rdlr`]
module"]
pub type IC_DMA_RDLR = crate::Reg<ic_dma_rdlr::IC_DMA_RDLR_SPEC>;
#[doc = "I2C Receive Data Level Register"]
pub mod ic_dma_rdlr;
#[doc = "IC_SDA_SETUP (rw) register accessor: I2C SDA Setup Register  

 This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.  

 Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter.  

You can [`read`](crate::Reg::read) this register and get [`ic_sda_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_sda_setup`]
module"]
pub type IC_SDA_SETUP = crate::Reg<ic_sda_setup::IC_SDA_SETUP_SPEC>;
#[doc = "I2C SDA Setup Register  

 This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.  

 Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
pub mod ic_sda_setup;
#[doc = "IC_ACK_GENERAL_CALL (rw) register accessor: I2C ACK General Call Register  

 The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.  

 This register is applicable only when the DW_apb_i2c is in slave mode.  

You can [`read`](crate::Reg::read) this register and get [`ic_ack_general_call::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ack_general_call::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_ack_general_call`]
module"]
pub type IC_ACK_GENERAL_CALL = crate::Reg<ic_ack_general_call::IC_ACK_GENERAL_CALL_SPEC>;
#[doc = "I2C ACK General Call Register  

 The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.  

 This register is applicable only when the DW_apb_i2c is in slave mode."]
pub mod ic_ack_general_call;
#[doc = "IC_ENABLE_STATUS (r) register accessor: I2C Enable Status Register  

 The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE\\[0\\]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled.  

 If IC_ENABLE\\[0\\]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.  

 If IC_ENABLE\\[0\\]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.  

 Note: When IC_ENABLE\\[0\\]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities.  

You can [`read`](crate::Reg::read) this register and get [`ic_enable_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_enable_status`]
module"]
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
#[doc = "IC_FS_SPKLEN (rw) register accessor: I2C SS, FS or FM+ spike suppression limit  

 This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1.  

You can [`read`](crate::Reg::read) this register and get [`ic_fs_spklen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_spklen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_fs_spklen`]
module"]
pub type IC_FS_SPKLEN = crate::Reg<ic_fs_spklen::IC_FS_SPKLEN_SPEC>;
#[doc = "I2C SS, FS or FM+ spike suppression limit  

 This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
pub mod ic_fs_spklen;
#[doc = "IC_CLR_RESTART_DET (r) register accessor: Clear RESTART_DET Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_restart_det::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_clr_restart_det`]
module"]
pub type IC_CLR_RESTART_DET = crate::Reg<ic_clr_restart_det::IC_CLR_RESTART_DET_SPEC>;
#[doc = "Clear RESTART_DET Interrupt Register"]
pub mod ic_clr_restart_det;
#[doc = "IC_COMP_PARAM_1 (r) register accessor: Component Parameter Register 1  

 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters  

You can [`read`](crate::Reg::read) this register and get [`ic_comp_param_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_comp_param_1`]
module"]
pub type IC_COMP_PARAM_1 = crate::Reg<ic_comp_param_1::IC_COMP_PARAM_1_SPEC>;
#[doc = "Component Parameter Register 1  

 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
pub mod ic_comp_param_1;
#[doc = "IC_COMP_VERSION (r) register accessor: I2C Component Version Register  

You can [`read`](crate::Reg::read) this register and get [`ic_comp_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_comp_version`]
module"]
pub type IC_COMP_VERSION = crate::Reg<ic_comp_version::IC_COMP_VERSION_SPEC>;
#[doc = "I2C Component Version Register"]
pub mod ic_comp_version;
#[doc = "IC_COMP_TYPE (r) register accessor: I2C Component Type Register  

You can [`read`](crate::Reg::read) this register and get [`ic_comp_type::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ic_comp_type`]
module"]
pub type IC_COMP_TYPE = crate::Reg<ic_comp_type::IC_COMP_TYPE_SPEC>;
#[doc = "I2C Component Type Register"]
pub mod ic_comp_type;
