#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctrlr0: CTRLR0,
    #[doc = "0x04 - Master Control register 1"]
    pub ctrlr1: CTRLR1,
    #[doc = "0x08 - SSI Enable"]
    pub ssienr: SSIENR,
    #[doc = "0x0c - Microwire Control"]
    pub mwcr: MWCR,
    #[doc = "0x10 - Slave enable"]
    pub ser: SER,
    #[doc = "0x14 - Baud rate"]
    pub baudr: BAUDR,
    #[doc = "0x18 - TX FIFO threshold level"]
    pub txftlr: TXFTLR,
    #[doc = "0x1c - RX FIFO threshold level"]
    pub rxftlr: RXFTLR,
    #[doc = "0x20 - TX FIFO level"]
    pub txflr: TXFLR,
    #[doc = "0x24 - RX FIFO level"]
    pub rxflr: RXFLR,
    #[doc = "0x28 - Status register"]
    pub sr: SR,
    #[doc = "0x2c - Interrupt mask"]
    pub imr: IMR,
    #[doc = "0x30 - Interrupt status"]
    pub isr: ISR,
    #[doc = "0x34 - Raw interrupt status"]
    pub risr: RISR,
    #[doc = "0x38 - TX FIFO overflow interrupt clear"]
    pub txoicr: TXOICR,
    #[doc = "0x3c - RX FIFO overflow interrupt clear"]
    pub rxoicr: RXOICR,
    #[doc = "0x40 - RX FIFO underflow interrupt clear"]
    pub rxuicr: RXUICR,
    #[doc = "0x44 - Multi-master interrupt clear"]
    pub msticr: MSTICR,
    #[doc = "0x48 - Interrupt clear"]
    pub icr: ICR,
    #[doc = "0x4c - DMA control"]
    pub dmacr: DMACR,
    #[doc = "0x50 - DMA TX data level"]
    pub dmatdlr: DMATDLR,
    #[doc = "0x54 - DMA RX data level"]
    pub dmardlr: DMARDLR,
    #[doc = "0x58 - Identification register"]
    pub idr: IDR,
    #[doc = "0x5c - Version ID"]
    pub ssi_version_id: SSI_VERSION_ID,
    #[doc = "0x60 - Data Register 0 (of 36)"]
    pub dr0: DR0,
    _reserved25: [u8; 140usize],
    #[doc = "0xf0 - RX sample delay"]
    pub rx_sample_dly: RX_SAMPLE_DLY,
    #[doc = "0xf4 - SPI control"]
    pub spi_ctrlr0: SPI_CTRLR0,
    #[doc = "0xf8 - TX drive edge"]
    pub txd_drive_edge: TXD_DRIVE_EDGE,
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlr0](ctrlr0) module"]
pub type CTRLR0 = crate::Reg<u32, _CTRLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLR0;
#[doc = "`read()` method returns [ctrlr0::R](ctrlr0::R) reader structure"]
impl crate::Readable for CTRLR0 {}
#[doc = "`write(|w| ..)` method takes [ctrlr0::W](ctrlr0::W) writer structure"]
impl crate::Writable for CTRLR0 {}
#[doc = "Control register 0"]
pub mod ctrlr0;
#[doc = "Master Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlr1](ctrlr1) module"]
pub type CTRLR1 = crate::Reg<u32, _CTRLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLR1;
#[doc = "`read()` method returns [ctrlr1::R](ctrlr1::R) reader structure"]
impl crate::Readable for CTRLR1 {}
#[doc = "`write(|w| ..)` method takes [ctrlr1::W](ctrlr1::W) writer structure"]
impl crate::Writable for CTRLR1 {}
#[doc = "Master Control register 1"]
pub mod ctrlr1;
#[doc = "SSI Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssienr](ssienr) module"]
pub type SSIENR = crate::Reg<u32, _SSIENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIENR;
#[doc = "`read()` method returns [ssienr::R](ssienr::R) reader structure"]
impl crate::Readable for SSIENR {}
#[doc = "`write(|w| ..)` method takes [ssienr::W](ssienr::W) writer structure"]
impl crate::Writable for SSIENR {}
#[doc = "SSI Enable"]
pub mod ssienr;
#[doc = "Microwire Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwcr](mwcr) module"]
pub type MWCR = crate::Reg<u32, _MWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MWCR;
#[doc = "`read()` method returns [mwcr::R](mwcr::R) reader structure"]
impl crate::Readable for MWCR {}
#[doc = "`write(|w| ..)` method takes [mwcr::W](mwcr::W) writer structure"]
impl crate::Writable for MWCR {}
#[doc = "Microwire Control"]
pub mod mwcr;
#[doc = "Slave enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ser](ser) module"]
pub type SER = crate::Reg<u32, _SER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SER;
#[doc = "`read()` method returns [ser::R](ser::R) reader structure"]
impl crate::Readable for SER {}
#[doc = "`write(|w| ..)` method takes [ser::W](ser::W) writer structure"]
impl crate::Writable for SER {}
#[doc = "Slave enable"]
pub mod ser;
#[doc = "Baud rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baudr](baudr) module"]
pub type BAUDR = crate::Reg<u32, _BAUDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUDR;
#[doc = "`read()` method returns [baudr::R](baudr::R) reader structure"]
impl crate::Readable for BAUDR {}
#[doc = "`write(|w| ..)` method takes [baudr::W](baudr::W) writer structure"]
impl crate::Writable for BAUDR {}
#[doc = "Baud rate"]
pub mod baudr;
#[doc = "TX FIFO threshold level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txftlr](txftlr) module"]
pub type TXFTLR = crate::Reg<u32, _TXFTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFTLR;
#[doc = "`read()` method returns [txftlr::R](txftlr::R) reader structure"]
impl crate::Readable for TXFTLR {}
#[doc = "`write(|w| ..)` method takes [txftlr::W](txftlr::W) writer structure"]
impl crate::Writable for TXFTLR {}
#[doc = "TX FIFO threshold level"]
pub mod txftlr;
#[doc = "RX FIFO threshold level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftlr](rxftlr) module"]
pub type RXFTLR = crate::Reg<u32, _RXFTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFTLR;
#[doc = "`read()` method returns [rxftlr::R](rxftlr::R) reader structure"]
impl crate::Readable for RXFTLR {}
#[doc = "`write(|w| ..)` method takes [rxftlr::W](rxftlr::W) writer structure"]
impl crate::Writable for RXFTLR {}
#[doc = "RX FIFO threshold level"]
pub mod rxftlr;
#[doc = "TX FIFO level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txflr](txflr) module"]
pub type TXFLR = crate::Reg<u32, _TXFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFLR;
#[doc = "`read()` method returns [txflr::R](txflr::R) reader structure"]
impl crate::Readable for TXFLR {}
#[doc = "TX FIFO level"]
pub mod txflr;
#[doc = "RX FIFO level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxflr](rxflr) module"]
pub type RXFLR = crate::Reg<u32, _RXFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFLR;
#[doc = "`read()` method returns [rxflr::R](rxflr::R) reader structure"]
impl crate::Readable for RXFLR {}
#[doc = "RX FIFO level"]
pub mod rxflr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status register"]
pub mod sr;
#[doc = "Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "Interrupt mask"]
pub mod imr;
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt status"]
pub mod isr;
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [risr](risr) module"]
pub type RISR = crate::Reg<u32, _RISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISR;
#[doc = "`read()` method returns [risr::R](risr::R) reader structure"]
impl crate::Readable for RISR {}
#[doc = "Raw interrupt status"]
pub mod risr;
#[doc = "TX FIFO overflow interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txoicr](txoicr) module"]
pub type TXOICR = crate::Reg<u32, _TXOICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXOICR;
#[doc = "`read()` method returns [txoicr::R](txoicr::R) reader structure"]
impl crate::Readable for TXOICR {}
#[doc = "TX FIFO overflow interrupt clear"]
pub mod txoicr;
#[doc = "RX FIFO overflow interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxoicr](rxoicr) module"]
pub type RXOICR = crate::Reg<u32, _RXOICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXOICR;
#[doc = "`read()` method returns [rxoicr::R](rxoicr::R) reader structure"]
impl crate::Readable for RXOICR {}
#[doc = "RX FIFO overflow interrupt clear"]
pub mod rxoicr;
#[doc = "RX FIFO underflow interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxuicr](rxuicr) module"]
pub type RXUICR = crate::Reg<u32, _RXUICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXUICR;
#[doc = "`read()` method returns [rxuicr::R](rxuicr::R) reader structure"]
impl crate::Readable for RXUICR {}
#[doc = "RX FIFO underflow interrupt clear"]
pub mod rxuicr;
#[doc = "Multi-master interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msticr](msticr) module"]
pub type MSTICR = crate::Reg<u32, _MSTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTICR;
#[doc = "`read()` method returns [msticr::R](msticr::R) reader structure"]
impl crate::Readable for MSTICR {}
#[doc = "Multi-master interrupt clear"]
pub mod msticr;
#[doc = "Interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "Interrupt clear"]
pub mod icr;
#[doc = "DMA control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacr](dmacr) module"]
pub type DMACR = crate::Reg<u32, _DMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACR;
#[doc = "`read()` method returns [dmacr::R](dmacr::R) reader structure"]
impl crate::Readable for DMACR {}
#[doc = "`write(|w| ..)` method takes [dmacr::W](dmacr::W) writer structure"]
impl crate::Writable for DMACR {}
#[doc = "DMA control"]
pub mod dmacr;
#[doc = "DMA TX data level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatdlr](dmatdlr) module"]
pub type DMATDLR = crate::Reg<u32, _DMATDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATDLR;
#[doc = "`read()` method returns [dmatdlr::R](dmatdlr::R) reader structure"]
impl crate::Readable for DMATDLR {}
#[doc = "`write(|w| ..)` method takes [dmatdlr::W](dmatdlr::W) writer structure"]
impl crate::Writable for DMATDLR {}
#[doc = "DMA TX data level"]
pub mod dmatdlr;
#[doc = "DMA RX data level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmardlr](dmardlr) module"]
pub type DMARDLR = crate::Reg<u32, _DMARDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARDLR;
#[doc = "`read()` method returns [dmardlr::R](dmardlr::R) reader structure"]
impl crate::Readable for DMARDLR {}
#[doc = "`write(|w| ..)` method takes [dmardlr::W](dmardlr::W) writer structure"]
impl crate::Writable for DMARDLR {}
#[doc = "DMA RX data level"]
pub mod dmardlr;
#[doc = "Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "Identification register"]
pub mod idr;
#[doc = "Version ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssi_version_id](ssi_version_id) module"]
pub type SSI_VERSION_ID = crate::Reg<u32, _SSI_VERSION_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSI_VERSION_ID;
#[doc = "`read()` method returns [ssi_version_id::R](ssi_version_id::R) reader structure"]
impl crate::Readable for SSI_VERSION_ID {}
#[doc = "Version ID"]
pub mod ssi_version_id;
#[doc = "Data Register 0 (of 36)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr0](dr0) module"]
pub type DR0 = crate::Reg<u32, _DR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR0;
#[doc = "`read()` method returns [dr0::R](dr0::R) reader structure"]
impl crate::Readable for DR0 {}
#[doc = "`write(|w| ..)` method takes [dr0::W](dr0::W) writer structure"]
impl crate::Writable for DR0 {}
#[doc = "Data Register 0 (of 36)"]
pub mod dr0;
#[doc = "RX sample delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_sample_dly](rx_sample_dly) module"]
pub type RX_SAMPLE_DLY = crate::Reg<u32, _RX_SAMPLE_DLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_SAMPLE_DLY;
#[doc = "`read()` method returns [rx_sample_dly::R](rx_sample_dly::R) reader structure"]
impl crate::Readable for RX_SAMPLE_DLY {}
#[doc = "`write(|w| ..)` method takes [rx_sample_dly::W](rx_sample_dly::W) writer structure"]
impl crate::Writable for RX_SAMPLE_DLY {}
#[doc = "RX sample delay"]
pub mod rx_sample_dly;
#[doc = "SPI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrlr0](spi_ctrlr0) module"]
pub type SPI_CTRLR0 = crate::Reg<u32, _SPI_CTRLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CTRLR0;
#[doc = "`read()` method returns [spi_ctrlr0::R](spi_ctrlr0::R) reader structure"]
impl crate::Readable for SPI_CTRLR0 {}
#[doc = "`write(|w| ..)` method takes [spi_ctrlr0::W](spi_ctrlr0::W) writer structure"]
impl crate::Writable for SPI_CTRLR0 {}
#[doc = "SPI control"]
pub mod spi_ctrlr0;
#[doc = "TX drive edge\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txd_drive_edge](txd_drive_edge) module"]
pub type TXD_DRIVE_EDGE = crate::Reg<u32, _TXD_DRIVE_EDGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXD_DRIVE_EDGE;
#[doc = "`read()` method returns [txd_drive_edge::R](txd_drive_edge::R) reader structure"]
impl crate::Readable for TXD_DRIVE_EDGE {}
#[doc = "`write(|w| ..)` method takes [txd_drive_edge::W](txd_drive_edge::W) writer structure"]
impl crate::Writable for TXD_DRIVE_EDGE {}
#[doc = "TX drive edge"]
pub mod txd_drive_edge;
