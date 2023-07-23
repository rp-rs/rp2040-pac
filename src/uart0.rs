#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register, UARTDR"]
    pub uartdr: UARTDR,
    #[doc = "0x04 - Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
    pub uartrsr: UARTRSR,
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - Flag Register, UARTFR"]
    pub uartfr: UARTFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - IrDA Low-Power Counter Register, UARTILPR"]
    pub uartilpr: UARTILPR,
    #[doc = "0x24 - Integer Baud Rate Register, UARTIBRD"]
    pub uartibrd: UARTIBRD,
    #[doc = "0x28 - Fractional Baud Rate Register, UARTFBRD"]
    pub uartfbrd: UARTFBRD,
    #[doc = "0x2c - Line Control Register, UARTLCR_H"]
    pub uartlcr_h: UARTLCR_H,
    #[doc = "0x30 - Control Register, UARTCR"]
    pub uartcr: UARTCR,
    #[doc = "0x34 - Interrupt FIFO Level Select Register, UARTIFLS"]
    pub uartifls: UARTIFLS,
    #[doc = "0x38 - Interrupt Mask Set/Clear Register, UARTIMSC"]
    pub uartimsc: UARTIMSC,
    #[doc = "0x3c - Raw Interrupt Status Register, UARTRIS"]
    pub uartris: UARTRIS,
    #[doc = "0x40 - Masked Interrupt Status Register, UARTMIS"]
    pub uartmis: UARTMIS,
    #[doc = "0x44 - Interrupt Clear Register, UARTICR"]
    pub uarticr: UARTICR,
    #[doc = "0x48 - DMA Control Register, UARTDMACR"]
    pub uartdmacr: UARTDMACR,
    _reserved14: [u8; 0x0f94],
    #[doc = "0xfe0 - UARTPeriphID0 Register"]
    pub uartperiphid0: UARTPERIPHID0,
    #[doc = "0xfe4 - UARTPeriphID1 Register"]
    pub uartperiphid1: UARTPERIPHID1,
    #[doc = "0xfe8 - UARTPeriphID2 Register"]
    pub uartperiphid2: UARTPERIPHID2,
    #[doc = "0xfec - UARTPeriphID3 Register"]
    pub uartperiphid3: UARTPERIPHID3,
    #[doc = "0xff0 - UARTPCellID0 Register"]
    pub uartpcellid0: UARTPCELLID0,
    #[doc = "0xff4 - UARTPCellID1 Register"]
    pub uartpcellid1: UARTPCELLID1,
    #[doc = "0xff8 - UARTPCellID2 Register"]
    pub uartpcellid2: UARTPCELLID2,
    #[doc = "0xffc - UARTPCellID3 Register"]
    pub uartpcellid3: UARTPCELLID3,
}
#[doc = "UARTDR (rw) register accessor: an alias for `Reg<UARTDR_SPEC>`"]
pub type UARTDR = crate::Reg<uartdr::UARTDR_SPEC>;
#[doc = "Data Register, UARTDR"]
pub mod uartdr;
#[doc = "UARTRSR (rw) register accessor: an alias for `Reg<UARTRSR_SPEC>`"]
pub type UARTRSR = crate::Reg<uartrsr::UARTRSR_SPEC>;
#[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
pub mod uartrsr;
#[doc = "UARTFR (r) register accessor: an alias for `Reg<UARTFR_SPEC>`"]
pub type UARTFR = crate::Reg<uartfr::UARTFR_SPEC>;
#[doc = "Flag Register, UARTFR"]
pub mod uartfr;
#[doc = "UARTILPR (rw) register accessor: an alias for `Reg<UARTILPR_SPEC>`"]
pub type UARTILPR = crate::Reg<uartilpr::UARTILPR_SPEC>;
#[doc = "IrDA Low-Power Counter Register, UARTILPR"]
pub mod uartilpr;
#[doc = "UARTIBRD (rw) register accessor: an alias for `Reg<UARTIBRD_SPEC>`"]
pub type UARTIBRD = crate::Reg<uartibrd::UARTIBRD_SPEC>;
#[doc = "Integer Baud Rate Register, UARTIBRD"]
pub mod uartibrd;
#[doc = "UARTFBRD (rw) register accessor: an alias for `Reg<UARTFBRD_SPEC>`"]
pub type UARTFBRD = crate::Reg<uartfbrd::UARTFBRD_SPEC>;
#[doc = "Fractional Baud Rate Register, UARTFBRD"]
pub mod uartfbrd;
#[doc = "UARTLCR_H (rw) register accessor: an alias for `Reg<UARTLCR_H_SPEC>`"]
pub type UARTLCR_H = crate::Reg<uartlcr_h::UARTLCR_H_SPEC>;
#[doc = "Line Control Register, UARTLCR_H"]
pub mod uartlcr_h;
#[doc = "UARTCR (rw) register accessor: an alias for `Reg<UARTCR_SPEC>`"]
pub type UARTCR = crate::Reg<uartcr::UARTCR_SPEC>;
#[doc = "Control Register, UARTCR"]
pub mod uartcr;
#[doc = "UARTIFLS (rw) register accessor: an alias for `Reg<UARTIFLS_SPEC>`"]
pub type UARTIFLS = crate::Reg<uartifls::UARTIFLS_SPEC>;
#[doc = "Interrupt FIFO Level Select Register, UARTIFLS"]
pub mod uartifls;
#[doc = "UARTIMSC (rw) register accessor: an alias for `Reg<UARTIMSC_SPEC>`"]
pub type UARTIMSC = crate::Reg<uartimsc::UARTIMSC_SPEC>;
#[doc = "Interrupt Mask Set/Clear Register, UARTIMSC"]
pub mod uartimsc;
#[doc = "UARTRIS (r) register accessor: an alias for `Reg<UARTRIS_SPEC>`"]
pub type UARTRIS = crate::Reg<uartris::UARTRIS_SPEC>;
#[doc = "Raw Interrupt Status Register, UARTRIS"]
pub mod uartris;
#[doc = "UARTMIS (r) register accessor: an alias for `Reg<UARTMIS_SPEC>`"]
pub type UARTMIS = crate::Reg<uartmis::UARTMIS_SPEC>;
#[doc = "Masked Interrupt Status Register, UARTMIS"]
pub mod uartmis;
#[doc = "UARTICR (rw) register accessor: an alias for `Reg<UARTICR_SPEC>`"]
pub type UARTICR = crate::Reg<uarticr::UARTICR_SPEC>;
#[doc = "Interrupt Clear Register, UARTICR"]
pub mod uarticr;
#[doc = "UARTDMACR (rw) register accessor: an alias for `Reg<UARTDMACR_SPEC>`"]
pub type UARTDMACR = crate::Reg<uartdmacr::UARTDMACR_SPEC>;
#[doc = "DMA Control Register, UARTDMACR"]
pub mod uartdmacr;
#[doc = "UARTPERIPHID0 (r) register accessor: an alias for `Reg<UARTPERIPHID0_SPEC>`"]
pub type UARTPERIPHID0 = crate::Reg<uartperiphid0::UARTPERIPHID0_SPEC>;
#[doc = "UARTPeriphID0 Register"]
pub mod uartperiphid0;
#[doc = "UARTPERIPHID1 (r) register accessor: an alias for `Reg<UARTPERIPHID1_SPEC>`"]
pub type UARTPERIPHID1 = crate::Reg<uartperiphid1::UARTPERIPHID1_SPEC>;
#[doc = "UARTPeriphID1 Register"]
pub mod uartperiphid1;
#[doc = "UARTPERIPHID2 (r) register accessor: an alias for `Reg<UARTPERIPHID2_SPEC>`"]
pub type UARTPERIPHID2 = crate::Reg<uartperiphid2::UARTPERIPHID2_SPEC>;
#[doc = "UARTPeriphID2 Register"]
pub mod uartperiphid2;
#[doc = "UARTPERIPHID3 (r) register accessor: an alias for `Reg<UARTPERIPHID3_SPEC>`"]
pub type UARTPERIPHID3 = crate::Reg<uartperiphid3::UARTPERIPHID3_SPEC>;
#[doc = "UARTPeriphID3 Register"]
pub mod uartperiphid3;
#[doc = "UARTPCELLID0 (r) register accessor: an alias for `Reg<UARTPCELLID0_SPEC>`"]
pub type UARTPCELLID0 = crate::Reg<uartpcellid0::UARTPCELLID0_SPEC>;
#[doc = "UARTPCellID0 Register"]
pub mod uartpcellid0;
#[doc = "UARTPCELLID1 (r) register accessor: an alias for `Reg<UARTPCELLID1_SPEC>`"]
pub type UARTPCELLID1 = crate::Reg<uartpcellid1::UARTPCELLID1_SPEC>;
#[doc = "UARTPCellID1 Register"]
pub mod uartpcellid1;
#[doc = "UARTPCELLID2 (r) register accessor: an alias for `Reg<UARTPCELLID2_SPEC>`"]
pub type UARTPCELLID2 = crate::Reg<uartpcellid2::UARTPCELLID2_SPEC>;
#[doc = "UARTPCellID2 Register"]
pub mod uartpcellid2;
#[doc = "UARTPCELLID3 (r) register accessor: an alias for `Reg<UARTPCELLID3_SPEC>`"]
pub type UARTPCELLID3 = crate::Reg<uartpcellid3::UARTPCELLID3_SPEC>;
#[doc = "UARTPCellID3 Register"]
pub mod uartpcellid3;
