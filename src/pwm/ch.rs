#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - INT and FRAC form a fixed-point fractional number.  
 Counting rate is system clock frequency divided by this number.  
 Fractional division uses simple 1st-order sigma-delta."]
    pub div: DIV,
    #[doc = "0x08 - Direct access to the PWM counter"]
    pub ctr: CTR,
    #[doc = "0x0c - Counter compare values"]
    pub cc: CC,
    #[doc = "0x10 - Counter wrap value"]
    pub top: TOP,
}
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Counter compare values"]
pub mod cc;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control and status register"]
pub mod csr;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Direct access to the PWM counter"]
pub mod ctr;
#[doc = "DIV (rw) register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "INT and FRAC form a fixed-point fractional number.  
 Counting rate is system clock frequency divided by this number.  
 Fractional division uses simple 1st-order sigma-delta."]
pub mod div;
#[doc = "TOP (rw) register accessor: an alias for `Reg<TOP_SPEC>`"]
pub type TOP = crate::Reg<top::TOP_SPEC>;
#[doc = "Counter wrap value"]
pub mod top;
