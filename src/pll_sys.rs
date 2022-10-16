#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and Status  
 GENERAL CONSTRAINTS:  
 Reference clock frequency min=5MHz, max=800MHz  
 Feedback divider min=16, max=320  
 VCO frequency min=750MHz, max=1600MHz"]
    pub cs: crate::Reg<cs::CS_SPEC>,
    #[doc = "0x04 - Controls the PLL power modes."]
    pub pwr: crate::Reg<pwr::PWR_SPEC>,
    #[doc = "0x08 - Feedback divisor  
 (note: this PLL does not support fractional division)"]
    pub fbdiv_int: crate::Reg<fbdiv_int::FBDIV_INT_SPEC>,
    #[doc = "0x0c - Controls the PLL post dividers for the primary output  
 (note: this PLL does not have a secondary output)  
 the primary output is driven from VCO divided by postdiv1*postdiv2"]
    pub prim: crate::Reg<prim::PRIM_SPEC>,
}
#[doc = "CS register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Control and Status  
 GENERAL CONSTRAINTS:  
 Reference clock frequency min=5MHz, max=800MHz  
 Feedback divider min=16, max=320  
 VCO frequency min=750MHz, max=1600MHz"]
pub mod cs;
#[doc = "PWR register accessor: an alias for `Reg<PWR_SPEC>`"]
pub type PWR = crate::Reg<pwr::PWR_SPEC>;
#[doc = "Controls the PLL power modes."]
pub mod pwr;
#[doc = "FBDIV_INT register accessor: an alias for `Reg<FBDIV_INT_SPEC>`"]
pub type FBDIV_INT = crate::Reg<fbdiv_int::FBDIV_INT_SPEC>;
#[doc = "Feedback divisor  
 (note: this PLL does not support fractional division)"]
pub mod fbdiv_int;
#[doc = "PRIM register accessor: an alias for `Reg<PRIM_SPEC>`"]
pub type PRIM = crate::Reg<prim::PRIM_SPEC>;
#[doc = "Controls the PLL post dividers for the primary output  
 (note: this PLL does not have a secondary output)  
 the primary output is driven from VCO divided by postdiv1*postdiv2"]
pub mod prim;
