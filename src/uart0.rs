#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uartdr: UARTDR,
    uartrsr: UARTRSR,
    _reserved2: [u8; 0x10],
    uartfr: UARTFR,
    _reserved3: [u8; 0x04],
    uartilpr: UARTILPR,
    uartibrd: UARTIBRD,
    uartfbrd: UARTFBRD,
    uartlcr_h: UARTLCR_H,
    uartcr: UARTCR,
    uartifls: UARTIFLS,
    uartimsc: UARTIMSC,
    uartris: UARTRIS,
    uartmis: UARTMIS,
    uarticr: UARTICR,
    uartdmacr: UARTDMACR,
    _reserved14: [u8; 0x0f94],
    uartperiphid0: UARTPERIPHID0,
    uartperiphid1: UARTPERIPHID1,
    uartperiphid2: UARTPERIPHID2,
    uartperiphid3: UARTPERIPHID3,
    uartpcellid0: UARTPCELLID0,
    uartpcellid1: UARTPCELLID1,
    uartpcellid2: UARTPCELLID2,
    uartpcellid3: UARTPCELLID3,
}
impl RegisterBlock {
    #[doc = "0x00 - Data Register, UARTDR"]
    #[inline(always)]
    pub const fn uartdr(&self) -> &UARTDR {
        &self.uartdr
    }
    #[doc = "0x04 - Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
    #[inline(always)]
    pub const fn uartrsr(&self) -> &UARTRSR {
        &self.uartrsr
    }
    #[doc = "0x18 - Flag Register, UARTFR"]
    #[inline(always)]
    pub const fn uartfr(&self) -> &UARTFR {
        &self.uartfr
    }
    #[doc = "0x20 - IrDA Low-Power Counter Register, UARTILPR"]
    #[inline(always)]
    pub const fn uartilpr(&self) -> &UARTILPR {
        &self.uartilpr
    }
    #[doc = "0x24 - Integer Baud Rate Register, UARTIBRD"]
    #[inline(always)]
    pub const fn uartibrd(&self) -> &UARTIBRD {
        &self.uartibrd
    }
    #[doc = "0x28 - Fractional Baud Rate Register, UARTFBRD"]
    #[inline(always)]
    pub const fn uartfbrd(&self) -> &UARTFBRD {
        &self.uartfbrd
    }
    #[doc = "0x2c - Line Control Register, UARTLCR_H"]
    #[inline(always)]
    pub const fn uartlcr_h(&self) -> &UARTLCR_H {
        &self.uartlcr_h
    }
    #[doc = "0x30 - Control Register, UARTCR"]
    #[inline(always)]
    pub const fn uartcr(&self) -> &UARTCR {
        &self.uartcr
    }
    #[doc = "0x34 - Interrupt FIFO Level Select Register, UARTIFLS"]
    #[inline(always)]
    pub const fn uartifls(&self) -> &UARTIFLS {
        &self.uartifls
    }
    #[doc = "0x38 - Interrupt Mask Set/Clear Register, UARTIMSC"]
    #[inline(always)]
    pub const fn uartimsc(&self) -> &UARTIMSC {
        &self.uartimsc
    }
    #[doc = "0x3c - Raw Interrupt Status Register, UARTRIS"]
    #[inline(always)]
    pub const fn uartris(&self) -> &UARTRIS {
        &self.uartris
    }
    #[doc = "0x40 - Masked Interrupt Status Register, UARTMIS"]
    #[inline(always)]
    pub const fn uartmis(&self) -> &UARTMIS {
        &self.uartmis
    }
    #[doc = "0x44 - Interrupt Clear Register, UARTICR"]
    #[inline(always)]
    pub const fn uarticr(&self) -> &UARTICR {
        &self.uarticr
    }
    #[doc = "0x48 - DMA Control Register, UARTDMACR"]
    #[inline(always)]
    pub const fn uartdmacr(&self) -> &UARTDMACR {
        &self.uartdmacr
    }
    #[doc = "0xfe0 - UARTPeriphID0 Register"]
    #[inline(always)]
    pub const fn uartperiphid0(&self) -> &UARTPERIPHID0 {
        &self.uartperiphid0
    }
    #[doc = "0xfe4 - UARTPeriphID1 Register"]
    #[inline(always)]
    pub const fn uartperiphid1(&self) -> &UARTPERIPHID1 {
        &self.uartperiphid1
    }
    #[doc = "0xfe8 - UARTPeriphID2 Register"]
    #[inline(always)]
    pub const fn uartperiphid2(&self) -> &UARTPERIPHID2 {
        &self.uartperiphid2
    }
    #[doc = "0xfec - UARTPeriphID3 Register"]
    #[inline(always)]
    pub const fn uartperiphid3(&self) -> &UARTPERIPHID3 {
        &self.uartperiphid3
    }
    #[doc = "0xff0 - UARTPCellID0 Register"]
    #[inline(always)]
    pub const fn uartpcellid0(&self) -> &UARTPCELLID0 {
        &self.uartpcellid0
    }
    #[doc = "0xff4 - UARTPCellID1 Register"]
    #[inline(always)]
    pub const fn uartpcellid1(&self) -> &UARTPCELLID1 {
        &self.uartpcellid1
    }
    #[doc = "0xff8 - UARTPCellID2 Register"]
    #[inline(always)]
    pub const fn uartpcellid2(&self) -> &UARTPCELLID2 {
        &self.uartpcellid2
    }
    #[doc = "0xffc - UARTPCellID3 Register"]
    #[inline(always)]
    pub const fn uartpcellid3(&self) -> &UARTPCELLID3 {
        &self.uartpcellid3
    }
}
#[doc = "UARTDR (rw) register accessor: Data Register, UARTDR  

You can [`read`](crate::Reg::read) this register and get [`uartdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartdr`]
module"]
pub type UARTDR = crate::Reg<uartdr::UARTDR_SPEC>;
#[doc = "Data Register, UARTDR"]
pub mod uartdr;
#[doc = "UARTRSR (rw) register accessor: Receive Status Register/Error Clear Register, UARTRSR/UARTECR  

You can [`read`](crate::Reg::read) this register and get [`uartrsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartrsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartrsr`]
module"]
pub type UARTRSR = crate::Reg<uartrsr::UARTRSR_SPEC>;
#[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
pub mod uartrsr;
#[doc = "UARTFR (r) register accessor: Flag Register, UARTFR  

You can [`read`](crate::Reg::read) this register and get [`uartfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartfr`]
module"]
pub type UARTFR = crate::Reg<uartfr::UARTFR_SPEC>;
#[doc = "Flag Register, UARTFR"]
pub mod uartfr;
#[doc = "UARTILPR (rw) register accessor: IrDA Low-Power Counter Register, UARTILPR  

You can [`read`](crate::Reg::read) this register and get [`uartilpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartilpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartilpr`]
module"]
pub type UARTILPR = crate::Reg<uartilpr::UARTILPR_SPEC>;
#[doc = "IrDA Low-Power Counter Register, UARTILPR"]
pub mod uartilpr;
#[doc = "UARTIBRD (rw) register accessor: Integer Baud Rate Register, UARTIBRD  

You can [`read`](crate::Reg::read) this register and get [`uartibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartibrd`]
module"]
pub type UARTIBRD = crate::Reg<uartibrd::UARTIBRD_SPEC>;
#[doc = "Integer Baud Rate Register, UARTIBRD"]
pub mod uartibrd;
#[doc = "UARTFBRD (rw) register accessor: Fractional Baud Rate Register, UARTFBRD  

You can [`read`](crate::Reg::read) this register and get [`uartfbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartfbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartfbrd`]
module"]
pub type UARTFBRD = crate::Reg<uartfbrd::UARTFBRD_SPEC>;
#[doc = "Fractional Baud Rate Register, UARTFBRD"]
pub mod uartfbrd;
#[doc = "UARTLCR_H (rw) register accessor: Line Control Register, UARTLCR_H  

You can [`read`](crate::Reg::read) this register and get [`uartlcr_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlcr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartlcr_h`]
module"]
pub type UARTLCR_H = crate::Reg<uartlcr_h::UARTLCR_H_SPEC>;
#[doc = "Line Control Register, UARTLCR_H"]
pub mod uartlcr_h;
#[doc = "UARTCR (rw) register accessor: Control Register, UARTCR  

You can [`read`](crate::Reg::read) this register and get [`uartcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartcr`]
module"]
pub type UARTCR = crate::Reg<uartcr::UARTCR_SPEC>;
#[doc = "Control Register, UARTCR"]
pub mod uartcr;
#[doc = "UARTIFLS (rw) register accessor: Interrupt FIFO Level Select Register, UARTIFLS  

You can [`read`](crate::Reg::read) this register and get [`uartifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartifls`]
module"]
pub type UARTIFLS = crate::Reg<uartifls::UARTIFLS_SPEC>;
#[doc = "Interrupt FIFO Level Select Register, UARTIFLS"]
pub mod uartifls;
#[doc = "UARTIMSC (rw) register accessor: Interrupt Mask Set/Clear Register, UARTIMSC  

You can [`read`](crate::Reg::read) this register and get [`uartimsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartimsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartimsc`]
module"]
pub type UARTIMSC = crate::Reg<uartimsc::UARTIMSC_SPEC>;
#[doc = "Interrupt Mask Set/Clear Register, UARTIMSC"]
pub mod uartimsc;
#[doc = "UARTRIS (r) register accessor: Raw Interrupt Status Register, UARTRIS  

You can [`read`](crate::Reg::read) this register and get [`uartris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartris`]
module"]
pub type UARTRIS = crate::Reg<uartris::UARTRIS_SPEC>;
#[doc = "Raw Interrupt Status Register, UARTRIS"]
pub mod uartris;
#[doc = "UARTMIS (r) register accessor: Masked Interrupt Status Register, UARTMIS  

You can [`read`](crate::Reg::read) this register and get [`uartmis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartmis`]
module"]
pub type UARTMIS = crate::Reg<uartmis::UARTMIS_SPEC>;
#[doc = "Masked Interrupt Status Register, UARTMIS"]
pub mod uartmis;
#[doc = "UARTICR (rw) register accessor: Interrupt Clear Register, UARTICR  

You can [`read`](crate::Reg::read) this register and get [`uarticr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uarticr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uarticr`]
module"]
pub type UARTICR = crate::Reg<uarticr::UARTICR_SPEC>;
#[doc = "Interrupt Clear Register, UARTICR"]
pub mod uarticr;
#[doc = "UARTDMACR (rw) register accessor: DMA Control Register, UARTDMACR  

You can [`read`](crate::Reg::read) this register and get [`uartdmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartdmacr`]
module"]
pub type UARTDMACR = crate::Reg<uartdmacr::UARTDMACR_SPEC>;
#[doc = "DMA Control Register, UARTDMACR"]
pub mod uartdmacr;
#[doc = "UARTPERIPHID0 (r) register accessor: UARTPeriphID0 Register  

You can [`read`](crate::Reg::read) this register and get [`uartperiphid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartperiphid0`]
module"]
pub type UARTPERIPHID0 = crate::Reg<uartperiphid0::UARTPERIPHID0_SPEC>;
#[doc = "UARTPeriphID0 Register"]
pub mod uartperiphid0;
#[doc = "UARTPERIPHID1 (r) register accessor: UARTPeriphID1 Register  

You can [`read`](crate::Reg::read) this register and get [`uartperiphid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartperiphid1`]
module"]
pub type UARTPERIPHID1 = crate::Reg<uartperiphid1::UARTPERIPHID1_SPEC>;
#[doc = "UARTPeriphID1 Register"]
pub mod uartperiphid1;
#[doc = "UARTPERIPHID2 (r) register accessor: UARTPeriphID2 Register  

You can [`read`](crate::Reg::read) this register and get [`uartperiphid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartperiphid2`]
module"]
pub type UARTPERIPHID2 = crate::Reg<uartperiphid2::UARTPERIPHID2_SPEC>;
#[doc = "UARTPeriphID2 Register"]
pub mod uartperiphid2;
#[doc = "UARTPERIPHID3 (r) register accessor: UARTPeriphID3 Register  

You can [`read`](crate::Reg::read) this register and get [`uartperiphid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartperiphid3`]
module"]
pub type UARTPERIPHID3 = crate::Reg<uartperiphid3::UARTPERIPHID3_SPEC>;
#[doc = "UARTPeriphID3 Register"]
pub mod uartperiphid3;
#[doc = "UARTPCELLID0 (r) register accessor: UARTPCellID0 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartpcellid0`]
module"]
pub type UARTPCELLID0 = crate::Reg<uartpcellid0::UARTPCELLID0_SPEC>;
#[doc = "UARTPCellID0 Register"]
pub mod uartpcellid0;
#[doc = "UARTPCELLID1 (r) register accessor: UARTPCellID1 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartpcellid1`]
module"]
pub type UARTPCELLID1 = crate::Reg<uartpcellid1::UARTPCELLID1_SPEC>;
#[doc = "UARTPCellID1 Register"]
pub mod uartpcellid1;
#[doc = "UARTPCELLID2 (r) register accessor: UARTPCellID2 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartpcellid2`]
module"]
pub type UARTPCELLID2 = crate::Reg<uartpcellid2::UARTPCELLID2_SPEC>;
#[doc = "UARTPCellID2 Register"]
pub mod uartpcellid2;
#[doc = "UARTPCELLID3 (r) register accessor: UARTPCellID3 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uartpcellid3`]
module"]
pub type UARTPCELLID3 = crate::Reg<uartpcellid3::UARTPCELLID3_SPEC>;
#[doc = "UARTPCellID3 Register"]
pub mod uartpcellid3;
