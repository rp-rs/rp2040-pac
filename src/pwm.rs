#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xa0 - Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
    pub ch: [CH; 8],
    #[doc = "0xa0 - This register aliases the CSR_EN bits for all channels.  
 Writing to this register allows multiple channels to be enabled  
 or disabled simultaneously, so they can run in perfect sync.  
 For each channel, there is only one physical EN register bit,  
 which can be accessed through here or CHx_CSR."]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0xa4 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0xa8 - Interrupt Enable"]
    pub inte: crate::Reg<inte::INTE_SPEC>,
    #[doc = "0xac - Interrupt Force"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0xb0 - Interrupt status after masking & forcing"]
    pub ints: crate::Reg<ints::INTS_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Control and status register"]
    pub csr: crate::Reg<self::ch::csr::CSR_SPEC>,
    #[doc = "0x04 - INT and FRAC form a fixed-point fractional number.  
 Counting rate is system clock frequency divided by this number.  
 Fractional division uses simple 1st-order sigma-delta."]
    pub div: crate::Reg<self::ch::div::DIV_SPEC>,
    #[doc = "0x08 - Direct access to the PWM counter"]
    pub ctr: crate::Reg<self::ch::ctr::CTR_SPEC>,
    #[doc = "0x0c - Counter compare values"]
    pub cc: crate::Reg<self::ch::cc::CC_SPEC>,
    #[doc = "0x10 - Counter wrap value"]
    pub top: crate::Reg<self::ch::top::TOP_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
pub mod ch;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "This register aliases the CSR_EN bits for all channels.  
 Writing to this register allows multiple channels to be enabled  
 or disabled simultaneously, so they can run in perfect sync.  
 For each channel, there is only one physical EN register bit,  
 which can be accessed through here or CHx_CSR."]
pub mod en;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE register accessor: an alias for `Reg<INTE_SPEC>`"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS register accessor: an alias for `Reg<INTS_SPEC>`"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing"]
pub mod ints;
