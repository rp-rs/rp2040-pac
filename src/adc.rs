#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control and Status"]
    pub cs: crate::Reg<cs::CS_SPEC>,
    #[doc = "0x04 - Result of most recent ADC conversion"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    #[doc = "0x08 - FIFO control and status"]
    pub fcs: crate::Reg<fcs::FCS_SPEC>,
    #[doc = "0x0c - Conversion result FIFO"]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
    #[doc = "0x10 - Clock divider. If non-zero, CS_START_MANY will start conversions  
 at regular intervals rather than back-to-back.  
 The divider is reset when either of these fields are written.  
 Total period is 1 + INT + FRAC / 256"]
    pub div: crate::Reg<div::DIV_SPEC>,
    #[doc = "0x14 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x18 - Interrupt Enable"]
    pub inte: crate::Reg<inte::INTE_SPEC>,
    #[doc = "0x1c - Interrupt Force"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x20 - Interrupt status after masking & forcing"]
    pub ints: crate::Reg<ints::INTS_SPEC>,
}
#[doc = "CS register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "ADC Control and Status"]
pub mod cs;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result of most recent ADC conversion"]
pub mod result;
#[doc = "FCS register accessor: an alias for `Reg<FCS_SPEC>`"]
pub type FCS = crate::Reg<fcs::FCS_SPEC>;
#[doc = "FIFO control and status"]
pub mod fcs;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Conversion result FIFO"]
pub mod fifo;
#[doc = "DIV register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions  
 at regular intervals rather than back-to-back.  
 The divider is reset when either of these fields are written.  
 Total period is 1 + INT + FRAC / 256"]
pub mod div;
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
