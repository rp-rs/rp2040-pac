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
    pub en: EN,
    #[doc = "0xa4 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0xa8 - Interrupt Enable"]
    pub inte: INTE,
    #[doc = "0xac - Interrupt Force"]
    pub intf: INTF,
    #[doc = "0xb0 - Interrupt status after masking & forcing"]
    pub ints: INTS,
}
#[doc = "Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
pub mod ch;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "This register aliases the CSR_EN bits for all channels.  
 Writing to this register allows multiple channels to be enabled  
 or disabled simultaneously, so they can run in perfect sync.  
 For each channel, there is only one physical EN register bit,  
 which can be accessed through here or CHx_CSR."]
pub mod en;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE (rw) register accessor: an alias for `Reg<INTE_SPEC>`"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF (rw) register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS (r) register accessor: an alias for `Reg<INTS_SPEC>`"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing"]
pub mod ints;
