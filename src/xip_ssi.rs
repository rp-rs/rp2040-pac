#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrlr0: CTRLR0,
    ctrlr1: CTRLR1,
    ssienr: SSIENR,
    mwcr: MWCR,
    ser: SER,
    baudr: BAUDR,
    txftlr: TXFTLR,
    rxftlr: RXFTLR,
    txflr: TXFLR,
    rxflr: RXFLR,
    sr: SR,
    imr: IMR,
    isr: ISR,
    risr: RISR,
    txoicr: TXOICR,
    rxoicr: RXOICR,
    rxuicr: RXUICR,
    msticr: MSTICR,
    icr: ICR,
    dmacr: DMACR,
    dmatdlr: DMATDLR,
    dmardlr: DMARDLR,
    idr: IDR,
    ssi_version_id: SSI_VERSION_ID,
    dr0: DR0,
    _reserved25: [u8; 0x8c],
    rx_sample_dly: RX_SAMPLE_DLY,
    spi_ctrlr0: SPI_CTRLR0,
    txd_drive_edge: TXD_DRIVE_EDGE,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    #[inline(always)]
    pub const fn ctrlr0(&self) -> &CTRLR0 {
        &self.ctrlr0
    }
    #[doc = "0x04 - Master Control register 1"]
    #[inline(always)]
    pub const fn ctrlr1(&self) -> &CTRLR1 {
        &self.ctrlr1
    }
    #[doc = "0x08 - SSI Enable"]
    #[inline(always)]
    pub const fn ssienr(&self) -> &SSIENR {
        &self.ssienr
    }
    #[doc = "0x0c - Microwire Control"]
    #[inline(always)]
    pub const fn mwcr(&self) -> &MWCR {
        &self.mwcr
    }
    #[doc = "0x10 - Slave enable"]
    #[inline(always)]
    pub const fn ser(&self) -> &SER {
        &self.ser
    }
    #[doc = "0x14 - Baud rate"]
    #[inline(always)]
    pub const fn baudr(&self) -> &BAUDR {
        &self.baudr
    }
    #[doc = "0x18 - TX FIFO threshold level"]
    #[inline(always)]
    pub const fn txftlr(&self) -> &TXFTLR {
        &self.txftlr
    }
    #[doc = "0x1c - RX FIFO threshold level"]
    #[inline(always)]
    pub const fn rxftlr(&self) -> &RXFTLR {
        &self.rxftlr
    }
    #[doc = "0x20 - TX FIFO level"]
    #[inline(always)]
    pub const fn txflr(&self) -> &TXFLR {
        &self.txflr
    }
    #[doc = "0x24 - RX FIFO level"]
    #[inline(always)]
    pub const fn rxflr(&self) -> &RXFLR {
        &self.rxflr
    }
    #[doc = "0x28 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x2c - Interrupt mask"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x30 - Interrupt status"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x34 - Raw interrupt status"]
    #[inline(always)]
    pub const fn risr(&self) -> &RISR {
        &self.risr
    }
    #[doc = "0x38 - TX FIFO overflow interrupt clear"]
    #[inline(always)]
    pub const fn txoicr(&self) -> &TXOICR {
        &self.txoicr
    }
    #[doc = "0x3c - RX FIFO overflow interrupt clear"]
    #[inline(always)]
    pub const fn rxoicr(&self) -> &RXOICR {
        &self.rxoicr
    }
    #[doc = "0x40 - RX FIFO underflow interrupt clear"]
    #[inline(always)]
    pub const fn rxuicr(&self) -> &RXUICR {
        &self.rxuicr
    }
    #[doc = "0x44 - Multi-master interrupt clear"]
    #[inline(always)]
    pub const fn msticr(&self) -> &MSTICR {
        &self.msticr
    }
    #[doc = "0x48 - Interrupt clear"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x4c - DMA control"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    #[doc = "0x50 - DMA TX data level"]
    #[inline(always)]
    pub const fn dmatdlr(&self) -> &DMATDLR {
        &self.dmatdlr
    }
    #[doc = "0x54 - DMA RX data level"]
    #[inline(always)]
    pub const fn dmardlr(&self) -> &DMARDLR {
        &self.dmardlr
    }
    #[doc = "0x58 - Identification register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x5c - Version ID"]
    #[inline(always)]
    pub const fn ssi_version_id(&self) -> &SSI_VERSION_ID {
        &self.ssi_version_id
    }
    #[doc = "0x60 - Data Register 0 (of 36)"]
    #[inline(always)]
    pub const fn dr0(&self) -> &DR0 {
        &self.dr0
    }
    #[doc = "0xf0 - RX sample delay"]
    #[inline(always)]
    pub const fn rx_sample_dly(&self) -> &RX_SAMPLE_DLY {
        &self.rx_sample_dly
    }
    #[doc = "0xf4 - SPI control"]
    #[inline(always)]
    pub const fn spi_ctrlr0(&self) -> &SPI_CTRLR0 {
        &self.spi_ctrlr0
    }
    #[doc = "0xf8 - TX drive edge"]
    #[inline(always)]
    pub const fn txd_drive_edge(&self) -> &TXD_DRIVE_EDGE {
        &self.txd_drive_edge
    }
}
#[doc = "CTRLR0 (rw) register accessor: Control register 0  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrlr0`]
module"]
pub type CTRLR0 = crate::Reg<ctrlr0::CTRLR0_SPEC>;
#[doc = "Control register 0"]
pub mod ctrlr0;
#[doc = "CTRLR1 (rw) register accessor: Master Control register 1  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrlr1`]
module"]
pub type CTRLR1 = crate::Reg<ctrlr1::CTRLR1_SPEC>;
#[doc = "Master Control register 1"]
pub mod ctrlr1;
#[doc = "SSIENR (rw) register accessor: SSI Enable  

You can [`read`](crate::generic::Reg::read) this register and get [`ssienr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssienr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ssienr`]
module"]
pub type SSIENR = crate::Reg<ssienr::SSIENR_SPEC>;
#[doc = "SSI Enable"]
pub mod ssienr;
#[doc = "MWCR (rw) register accessor: Microwire Control  

You can [`read`](crate::generic::Reg::read) this register and get [`mwcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mwcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mwcr`]
module"]
pub type MWCR = crate::Reg<mwcr::MWCR_SPEC>;
#[doc = "Microwire Control"]
pub mod mwcr;
#[doc = "SER (rw) register accessor: Slave enable  

You can [`read`](crate::generic::Reg::read) this register and get [`ser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ser`]
module"]
pub type SER = crate::Reg<ser::SER_SPEC>;
#[doc = "Slave enable"]
pub mod ser;
#[doc = "BAUDR (rw) register accessor: Baud rate  

You can [`read`](crate::generic::Reg::read) this register and get [`baudr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@baudr`]
module"]
pub type BAUDR = crate::Reg<baudr::BAUDR_SPEC>;
#[doc = "Baud rate"]
pub mod baudr;
#[doc = "TXFTLR (rw) register accessor: TX FIFO threshold level  

You can [`read`](crate::generic::Reg::read) this register and get [`txftlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txftlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@txftlr`]
module"]
pub type TXFTLR = crate::Reg<txftlr::TXFTLR_SPEC>;
#[doc = "TX FIFO threshold level"]
pub mod txftlr;
#[doc = "RXFTLR (rw) register accessor: RX FIFO threshold level  

You can [`read`](crate::generic::Reg::read) this register and get [`rxftlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxftlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rxftlr`]
module"]
pub type RXFTLR = crate::Reg<rxftlr::RXFTLR_SPEC>;
#[doc = "RX FIFO threshold level"]
pub mod rxftlr;
#[doc = "TXFLR (rw) register accessor: TX FIFO level  

You can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@txflr`]
module"]
pub type TXFLR = crate::Reg<txflr::TXFLR_SPEC>;
#[doc = "TX FIFO level"]
pub mod txflr;
#[doc = "RXFLR (rw) register accessor: RX FIFO level  

You can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rxflr`]
module"]
pub type RXFLR = crate::Reg<rxflr::RXFLR_SPEC>;
#[doc = "RX FIFO level"]
pub mod rxflr;
#[doc = "SR (rw) register accessor: Status register  

You can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IMR (rw) register accessor: Interrupt mask  

You can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask"]
pub mod imr;
#[doc = "ISR (rw) register accessor: Interrupt status  

You can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt status"]
pub mod isr;
#[doc = "RISR (rw) register accessor: Raw interrupt status  

You can [`read`](crate::generic::Reg::read) this register and get [`risr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`risr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@risr`]
module"]
pub type RISR = crate::Reg<risr::RISR_SPEC>;
#[doc = "Raw interrupt status"]
pub mod risr;
#[doc = "TXOICR (rw) register accessor: TX FIFO overflow interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`txoicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txoicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@txoicr`]
module"]
pub type TXOICR = crate::Reg<txoicr::TXOICR_SPEC>;
#[doc = "TX FIFO overflow interrupt clear"]
pub mod txoicr;
#[doc = "RXOICR (rw) register accessor: RX FIFO overflow interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`rxoicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxoicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rxoicr`]
module"]
pub type RXOICR = crate::Reg<rxoicr::RXOICR_SPEC>;
#[doc = "RX FIFO overflow interrupt clear"]
pub mod rxoicr;
#[doc = "RXUICR (rw) register accessor: RX FIFO underflow interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`rxuicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxuicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rxuicr`]
module"]
pub type RXUICR = crate::Reg<rxuicr::RXUICR_SPEC>;
#[doc = "RX FIFO underflow interrupt clear"]
pub mod rxuicr;
#[doc = "MSTICR (rw) register accessor: Multi-master interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`msticr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msticr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@msticr`]
module"]
pub type MSTICR = crate::Reg<msticr::MSTICR_SPEC>;
#[doc = "Multi-master interrupt clear"]
pub mod msticr;
#[doc = "ICR (rw) register accessor: Interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear"]
pub mod icr;
#[doc = "DMACR (rw) register accessor: DMA control  

You can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dmacr`]
module"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA control"]
pub mod dmacr;
#[doc = "DMATDLR (rw) register accessor: DMA TX data level  

You can [`read`](crate::generic::Reg::read) this register and get [`dmatdlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dmatdlr`]
module"]
pub type DMATDLR = crate::Reg<dmatdlr::DMATDLR_SPEC>;
#[doc = "DMA TX data level"]
pub mod dmatdlr;
#[doc = "DMARDLR (rw) register accessor: DMA RX data level  

You can [`read`](crate::generic::Reg::read) this register and get [`dmardlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dmardlr`]
module"]
pub type DMARDLR = crate::Reg<dmardlr::DMARDLR_SPEC>;
#[doc = "DMA RX data level"]
pub mod dmardlr;
#[doc = "IDR (rw) register accessor: Identification register  

You can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Identification register"]
pub mod idr;
#[doc = "SSI_VERSION_ID (rw) register accessor: Version ID  

You can [`read`](crate::generic::Reg::read) this register and get [`ssi_version_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssi_version_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ssi_version_id`]
module"]
pub type SSI_VERSION_ID = crate::Reg<ssi_version_id::SSI_VERSION_ID_SPEC>;
#[doc = "Version ID"]
pub mod ssi_version_id;
#[doc = "DR0 (rw) register accessor: Data Register 0 (of 36)  

You can [`read`](crate::generic::Reg::read) this register and get [`dr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dr0`]
module"]
pub type DR0 = crate::Reg<dr0::DR0_SPEC>;
#[doc = "Data Register 0 (of 36)"]
pub mod dr0;
#[doc = "RX_SAMPLE_DLY (rw) register accessor: RX sample delay  

You can [`read`](crate::generic::Reg::read) this register and get [`rx_sample_dly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_sample_dly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rx_sample_dly`]
module"]
pub type RX_SAMPLE_DLY = crate::Reg<rx_sample_dly::RX_SAMPLE_DLY_SPEC>;
#[doc = "RX sample delay"]
pub mod rx_sample_dly;
#[doc = "SPI_CTRLR0 (rw) register accessor: SPI control  

You can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@spi_ctrlr0`]
module"]
pub type SPI_CTRLR0 = crate::Reg<spi_ctrlr0::SPI_CTRLR0_SPEC>;
#[doc = "SPI control"]
pub mod spi_ctrlr0;
#[doc = "TXD_DRIVE_EDGE (rw) register accessor: TX drive edge  

You can [`read`](crate::generic::Reg::read) this register and get [`txd_drive_edge::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txd_drive_edge::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@txd_drive_edge`]
module"]
pub type TXD_DRIVE_EDGE = crate::Reg<txd_drive_edge::TXD_DRIVE_EDGE_SPEC>;
#[doc = "TX drive edge"]
pub mod txd_drive_edge;
