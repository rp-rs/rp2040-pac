#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0, SSPCR0 on page 3-4"]
    pub sspcr0: crate::Reg<sspcr0::SSPCR0_SPEC>,
    #[doc = "0x04 - Control register 1, SSPCR1 on page 3-5"]
    pub sspcr1: crate::Reg<sspcr1::SSPCR1_SPEC>,
    #[doc = "0x08 - Data register, SSPDR on page 3-6"]
    pub sspdr: crate::Reg<sspdr::SSPDR_SPEC>,
    #[doc = "0x0c - Status register, SSPSR on page 3-7"]
    pub sspsr: crate::Reg<sspsr::SSPSR_SPEC>,
    #[doc = "0x10 - Clock prescale register, SSPCPSR on page 3-8"]
    pub sspcpsr: crate::Reg<sspcpsr::SSPCPSR_SPEC>,
    #[doc = "0x14 - Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    pub sspimsc: crate::Reg<sspimsc::SSPIMSC_SPEC>,
    #[doc = "0x18 - Raw interrupt status register, SSPRIS on page 3-10"]
    pub sspris: crate::Reg<sspris::SSPRIS_SPEC>,
    #[doc = "0x1c - Masked interrupt status register, SSPMIS on page 3-11"]
    pub sspmis: crate::Reg<sspmis::SSPMIS_SPEC>,
    #[doc = "0x20 - Interrupt clear register, SSPICR on page 3-11"]
    pub sspicr: crate::Reg<sspicr::SSPICR_SPEC>,
    #[doc = "0x24 - DMA control register, SSPDMACR on page 3-12"]
    pub sspdmacr: crate::Reg<sspdmacr::SSPDMACR_SPEC>,
    _reserved10: [u8; 0x0fb8],
    #[doc = "0xfe0 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid0: crate::Reg<sspperiphid0::SSPPERIPHID0_SPEC>,
    #[doc = "0xfe4 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid1: crate::Reg<sspperiphid1::SSPPERIPHID1_SPEC>,
    #[doc = "0xfe8 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid2: crate::Reg<sspperiphid2::SSPPERIPHID2_SPEC>,
    #[doc = "0xfec - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid3: crate::Reg<sspperiphid3::SSPPERIPHID3_SPEC>,
    #[doc = "0xff0 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid0: crate::Reg<ssppcellid0::SSPPCELLID0_SPEC>,
    #[doc = "0xff4 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid1: crate::Reg<ssppcellid1::SSPPCELLID1_SPEC>,
    #[doc = "0xff8 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid2: crate::Reg<ssppcellid2::SSPPCELLID2_SPEC>,
    #[doc = "0xffc - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid3: crate::Reg<ssppcellid3::SSPPCELLID3_SPEC>,
}
#[doc = "SSPCR0 register accessor: an alias for `Reg<SSPCR0_SPEC>`"]
pub type SSPCR0 = crate::Reg<sspcr0::SSPCR0_SPEC>;
#[doc = "Control register 0, SSPCR0 on page 3-4"]
pub mod sspcr0;
#[doc = "SSPCR1 register accessor: an alias for `Reg<SSPCR1_SPEC>`"]
pub type SSPCR1 = crate::Reg<sspcr1::SSPCR1_SPEC>;
#[doc = "Control register 1, SSPCR1 on page 3-5"]
pub mod sspcr1;
#[doc = "SSPDR register accessor: an alias for `Reg<SSPDR_SPEC>`"]
pub type SSPDR = crate::Reg<sspdr::SSPDR_SPEC>;
#[doc = "Data register, SSPDR on page 3-6"]
pub mod sspdr;
#[doc = "SSPSR register accessor: an alias for `Reg<SSPSR_SPEC>`"]
pub type SSPSR = crate::Reg<sspsr::SSPSR_SPEC>;
#[doc = "Status register, SSPSR on page 3-7"]
pub mod sspsr;
#[doc = "SSPCPSR register accessor: an alias for `Reg<SSPCPSR_SPEC>`"]
pub type SSPCPSR = crate::Reg<sspcpsr::SSPCPSR_SPEC>;
#[doc = "Clock prescale register, SSPCPSR on page 3-8"]
pub mod sspcpsr;
#[doc = "SSPIMSC register accessor: an alias for `Reg<SSPIMSC_SPEC>`"]
pub type SSPIMSC = crate::Reg<sspimsc::SSPIMSC_SPEC>;
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
pub mod sspimsc;
#[doc = "SSPRIS register accessor: an alias for `Reg<SSPRIS_SPEC>`"]
pub type SSPRIS = crate::Reg<sspris::SSPRIS_SPEC>;
#[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
pub mod sspris;
#[doc = "SSPMIS register accessor: an alias for `Reg<SSPMIS_SPEC>`"]
pub type SSPMIS = crate::Reg<sspmis::SSPMIS_SPEC>;
#[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
pub mod sspmis;
#[doc = "SSPICR register accessor: an alias for `Reg<SSPICR_SPEC>`"]
pub type SSPICR = crate::Reg<sspicr::SSPICR_SPEC>;
#[doc = "Interrupt clear register, SSPICR on page 3-11"]
pub mod sspicr;
#[doc = "SSPDMACR register accessor: an alias for `Reg<SSPDMACR_SPEC>`"]
pub type SSPDMACR = crate::Reg<sspdmacr::SSPDMACR_SPEC>;
#[doc = "DMA control register, SSPDMACR on page 3-12"]
pub mod sspdmacr;
#[doc = "SSPPERIPHID0 register accessor: an alias for `Reg<SSPPERIPHID0_SPEC>`"]
pub type SSPPERIPHID0 = crate::Reg<sspperiphid0::SSPPERIPHID0_SPEC>;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid0;
#[doc = "SSPPERIPHID1 register accessor: an alias for `Reg<SSPPERIPHID1_SPEC>`"]
pub type SSPPERIPHID1 = crate::Reg<sspperiphid1::SSPPERIPHID1_SPEC>;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid1;
#[doc = "SSPPERIPHID2 register accessor: an alias for `Reg<SSPPERIPHID2_SPEC>`"]
pub type SSPPERIPHID2 = crate::Reg<sspperiphid2::SSPPERIPHID2_SPEC>;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid2;
#[doc = "SSPPERIPHID3 register accessor: an alias for `Reg<SSPPERIPHID3_SPEC>`"]
pub type SSPPERIPHID3 = crate::Reg<sspperiphid3::SSPPERIPHID3_SPEC>;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid3;
#[doc = "SSPPCELLID0 register accessor: an alias for `Reg<SSPPCELLID0_SPEC>`"]
pub type SSPPCELLID0 = crate::Reg<ssppcellid0::SSPPCELLID0_SPEC>;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid0;
#[doc = "SSPPCELLID1 register accessor: an alias for `Reg<SSPPCELLID1_SPEC>`"]
pub type SSPPCELLID1 = crate::Reg<ssppcellid1::SSPPCELLID1_SPEC>;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid1;
#[doc = "SSPPCELLID2 register accessor: an alias for `Reg<SSPPCELLID2_SPEC>`"]
pub type SSPPCELLID2 = crate::Reg<ssppcellid2::SSPPCELLID2_SPEC>;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid2;
#[doc = "SSPPCELLID3 register accessor: an alias for `Reg<SSPPCELLID3_SPEC>`"]
pub type SSPPCELLID3 = crate::Reg<ssppcellid3::SSPPCELLID3_SPEC>;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid3;
