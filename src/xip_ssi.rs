#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctrlr0: crate::Reg<ctrlr0::CTRLR0_SPEC>,
    #[doc = "0x04 - Master Control register 1"]
    pub ctrlr1: crate::Reg<ctrlr1::CTRLR1_SPEC>,
    #[doc = "0x08 - SSI Enable"]
    pub ssienr: crate::Reg<ssienr::SSIENR_SPEC>,
    #[doc = "0x0c - Microwire Control"]
    pub mwcr: crate::Reg<mwcr::MWCR_SPEC>,
    #[doc = "0x10 - Slave enable"]
    pub ser: crate::Reg<ser::SER_SPEC>,
    #[doc = "0x14 - Baud rate"]
    pub baudr: crate::Reg<baudr::BAUDR_SPEC>,
    #[doc = "0x18 - TX FIFO threshold level"]
    pub txftlr: crate::Reg<txftlr::TXFTLR_SPEC>,
    #[doc = "0x1c - RX FIFO threshold level"]
    pub rxftlr: crate::Reg<rxftlr::RXFTLR_SPEC>,
    #[doc = "0x20 - TX FIFO level"]
    pub txflr: crate::Reg<txflr::TXFLR_SPEC>,
    #[doc = "0x24 - RX FIFO level"]
    pub rxflr: crate::Reg<rxflr::RXFLR_SPEC>,
    #[doc = "0x28 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x2c - Interrupt mask"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x30 - Interrupt status"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x34 - Raw interrupt status"]
    pub risr: crate::Reg<risr::RISR_SPEC>,
    #[doc = "0x38 - TX FIFO overflow interrupt clear"]
    pub txoicr: crate::Reg<txoicr::TXOICR_SPEC>,
    #[doc = "0x3c - RX FIFO overflow interrupt clear"]
    pub rxoicr: crate::Reg<rxoicr::RXOICR_SPEC>,
    #[doc = "0x40 - RX FIFO underflow interrupt clear"]
    pub rxuicr: crate::Reg<rxuicr::RXUICR_SPEC>,
    #[doc = "0x44 - Multi-master interrupt clear"]
    pub msticr: crate::Reg<msticr::MSTICR_SPEC>,
    #[doc = "0x48 - Interrupt clear"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x4c - DMA control"]
    pub dmacr: crate::Reg<dmacr::DMACR_SPEC>,
    #[doc = "0x50 - DMA TX data level"]
    pub dmatdlr: crate::Reg<dmatdlr::DMATDLR_SPEC>,
    #[doc = "0x54 - DMA RX data level"]
    pub dmardlr: crate::Reg<dmardlr::DMARDLR_SPEC>,
    #[doc = "0x58 - Identification register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x5c - Version ID"]
    pub ssi_version_id: crate::Reg<ssi_version_id::SSI_VERSION_ID_SPEC>,
    #[doc = "0x60 - Data Register 0 (of 36)"]
    pub dr0: crate::Reg<dr0::DR0_SPEC>,
    _reserved25: [u8; 0x8c],
    #[doc = "0xf0 - RX sample delay"]
    pub rx_sample_dly: crate::Reg<rx_sample_dly::RX_SAMPLE_DLY_SPEC>,
    #[doc = "0xf4 - SPI control"]
    pub spi_ctrlr0: crate::Reg<spi_ctrlr0::SPI_CTRLR0_SPEC>,
    #[doc = "0xf8 - TX drive edge"]
    pub txd_drive_edge: crate::Reg<txd_drive_edge::TXD_DRIVE_EDGE_SPEC>,
}
#[doc = "CTRLR0 register accessor: an alias for `Reg<CTRLR0_SPEC>`"]
pub type CTRLR0 = crate::Reg<ctrlr0::CTRLR0_SPEC>;
#[doc = "Control register 0"]
pub mod ctrlr0;
#[doc = "CTRLR1 register accessor: an alias for `Reg<CTRLR1_SPEC>`"]
pub type CTRLR1 = crate::Reg<ctrlr1::CTRLR1_SPEC>;
#[doc = "Master Control register 1"]
pub mod ctrlr1;
#[doc = "SSIENR register accessor: an alias for `Reg<SSIENR_SPEC>`"]
pub type SSIENR = crate::Reg<ssienr::SSIENR_SPEC>;
#[doc = "SSI Enable"]
pub mod ssienr;
#[doc = "MWCR register accessor: an alias for `Reg<MWCR_SPEC>`"]
pub type MWCR = crate::Reg<mwcr::MWCR_SPEC>;
#[doc = "Microwire Control"]
pub mod mwcr;
#[doc = "SER register accessor: an alias for `Reg<SER_SPEC>`"]
pub type SER = crate::Reg<ser::SER_SPEC>;
#[doc = "Slave enable"]
pub mod ser;
#[doc = "BAUDR register accessor: an alias for `Reg<BAUDR_SPEC>`"]
pub type BAUDR = crate::Reg<baudr::BAUDR_SPEC>;
#[doc = "Baud rate"]
pub mod baudr;
#[doc = "TXFTLR register accessor: an alias for `Reg<TXFTLR_SPEC>`"]
pub type TXFTLR = crate::Reg<txftlr::TXFTLR_SPEC>;
#[doc = "TX FIFO threshold level"]
pub mod txftlr;
#[doc = "RXFTLR register accessor: an alias for `Reg<RXFTLR_SPEC>`"]
pub type RXFTLR = crate::Reg<rxftlr::RXFTLR_SPEC>;
#[doc = "RX FIFO threshold level"]
pub mod rxftlr;
#[doc = "TXFLR register accessor: an alias for `Reg<TXFLR_SPEC>`"]
pub type TXFLR = crate::Reg<txflr::TXFLR_SPEC>;
#[doc = "TX FIFO level"]
pub mod txflr;
#[doc = "RXFLR register accessor: an alias for `Reg<RXFLR_SPEC>`"]
pub type RXFLR = crate::Reg<rxflr::RXFLR_SPEC>;
#[doc = "RX FIFO level"]
pub mod rxflr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt status"]
pub mod isr;
#[doc = "RISR register accessor: an alias for `Reg<RISR_SPEC>`"]
pub type RISR = crate::Reg<risr::RISR_SPEC>;
#[doc = "Raw interrupt status"]
pub mod risr;
#[doc = "TXOICR register accessor: an alias for `Reg<TXOICR_SPEC>`"]
pub type TXOICR = crate::Reg<txoicr::TXOICR_SPEC>;
#[doc = "TX FIFO overflow interrupt clear"]
pub mod txoicr;
#[doc = "RXOICR register accessor: an alias for `Reg<RXOICR_SPEC>`"]
pub type RXOICR = crate::Reg<rxoicr::RXOICR_SPEC>;
#[doc = "RX FIFO overflow interrupt clear"]
pub mod rxoicr;
#[doc = "RXUICR register accessor: an alias for `Reg<RXUICR_SPEC>`"]
pub type RXUICR = crate::Reg<rxuicr::RXUICR_SPEC>;
#[doc = "RX FIFO underflow interrupt clear"]
pub mod rxuicr;
#[doc = "MSTICR register accessor: an alias for `Reg<MSTICR_SPEC>`"]
pub type MSTICR = crate::Reg<msticr::MSTICR_SPEC>;
#[doc = "Multi-master interrupt clear"]
pub mod msticr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear"]
pub mod icr;
#[doc = "DMACR register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA control"]
pub mod dmacr;
#[doc = "DMATDLR register accessor: an alias for `Reg<DMATDLR_SPEC>`"]
pub type DMATDLR = crate::Reg<dmatdlr::DMATDLR_SPEC>;
#[doc = "DMA TX data level"]
pub mod dmatdlr;
#[doc = "DMARDLR register accessor: an alias for `Reg<DMARDLR_SPEC>`"]
pub type DMARDLR = crate::Reg<dmardlr::DMARDLR_SPEC>;
#[doc = "DMA RX data level"]
pub mod dmardlr;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Identification register"]
pub mod idr;
#[doc = "SSI_VERSION_ID register accessor: an alias for `Reg<SSI_VERSION_ID_SPEC>`"]
pub type SSI_VERSION_ID = crate::Reg<ssi_version_id::SSI_VERSION_ID_SPEC>;
#[doc = "Version ID"]
pub mod ssi_version_id;
#[doc = "DR0 register accessor: an alias for `Reg<DR0_SPEC>`"]
pub type DR0 = crate::Reg<dr0::DR0_SPEC>;
#[doc = "Data Register 0 (of 36)"]
pub mod dr0;
#[doc = "RX_SAMPLE_DLY register accessor: an alias for `Reg<RX_SAMPLE_DLY_SPEC>`"]
pub type RX_SAMPLE_DLY = crate::Reg<rx_sample_dly::RX_SAMPLE_DLY_SPEC>;
#[doc = "RX sample delay"]
pub mod rx_sample_dly;
#[doc = "SPI_CTRLR0 register accessor: an alias for `Reg<SPI_CTRLR0_SPEC>`"]
pub type SPI_CTRLR0 = crate::Reg<spi_ctrlr0::SPI_CTRLR0_SPEC>;
#[doc = "SPI control"]
pub mod spi_ctrlr0;
#[doc = "TXD_DRIVE_EDGE register accessor: an alias for `Reg<TXD_DRIVE_EDGE_SPEC>`"]
pub type TXD_DRIVE_EDGE = crate::Reg<txd_drive_edge::TXD_DRIVE_EDGE_SPEC>;
#[doc = "TX drive edge"]
pub mod txd_drive_edge;
