#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage regulator control and status"]
    pub vreg: crate::Reg<vreg::VREG_SPEC>,
    #[doc = "0x04 - brown-out detection control"]
    pub bod: crate::Reg<bod::BOD_SPEC>,
    #[doc = "0x08 - Chip reset control and status"]
    pub chip_reset: crate::Reg<chip_reset::CHIP_RESET_SPEC>,
}
#[doc = "VREG register accessor: an alias for `Reg<VREG_SPEC>`"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "Voltage regulator control and status"]
pub mod vreg;
#[doc = "BOD register accessor: an alias for `Reg<BOD_SPEC>`"]
pub type BOD = crate::Reg<bod::BOD_SPEC>;
#[doc = "brown-out detection control"]
pub mod bod;
#[doc = "CHIP_RESET register accessor: an alias for `Reg<CHIP_RESET_SPEC>`"]
pub type CHIP_RESET = crate::Reg<chip_reset::CHIP_RESET_SPEC>;
#[doc = "Chip reset control and status"]
pub mod chip_reset;
