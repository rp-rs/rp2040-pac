#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted."]
    pub reset: crate::Reg<reset::RESET_SPEC>,
    #[doc = "0x04 - Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
    pub wdsel: crate::Reg<wdsel::WDSEL_SPEC>,
    #[doc = "0x08 - Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
    pub reset_done: crate::Reg<reset_done::RESET_DONE_SPEC>,
}
#[doc = "RESET register accessor: an alias for `Reg<RESET_SPEC>`"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted."]
pub mod reset;
#[doc = "WDSEL register accessor: an alias for `Reg<WDSEL_SPEC>`"]
pub type WDSEL = crate::Reg<wdsel::WDSEL_SPEC>;
#[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
pub mod wdsel;
#[doc = "RESET_DONE register accessor: an alias for `Reg<RESET_DONE_SPEC>`"]
pub type RESET_DONE = crate::Reg<reset_done::RESET_DONE_SPEC>;
#[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
pub mod reset_done;
