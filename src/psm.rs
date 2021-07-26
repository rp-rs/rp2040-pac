#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Force block out of reset (i.e. power it on)"]
    pub frce_on: crate::Reg<frce_on::FRCE_ON_SPEC>,
    #[doc = "0x04 - Force into reset (i.e. power it off)"]
    pub frce_off: crate::Reg<frce_off::FRCE_OFF_SPEC>,
    #[doc = "0x08 - Set to 1 if this peripheral should be reset when the watchdog fires."]
    pub wdsel: crate::Reg<wdsel::WDSEL_SPEC>,
    #[doc = "0x0c - Indicates the peripheral's registers are ready to access."]
    pub done: crate::Reg<done::DONE_SPEC>,
}
#[doc = "FRCE_ON register accessor: an alias for `Reg<FRCE_ON_SPEC>`"]
pub type FRCE_ON = crate::Reg<frce_on::FRCE_ON_SPEC>;
#[doc = "Force block out of reset (i.e. power it on)"]
pub mod frce_on;
#[doc = "FRCE_OFF register accessor: an alias for `Reg<FRCE_OFF_SPEC>`"]
pub type FRCE_OFF = crate::Reg<frce_off::FRCE_OFF_SPEC>;
#[doc = "Force into reset (i.e. power it off)"]
pub mod frce_off;
#[doc = "WDSEL register accessor: an alias for `Reg<WDSEL_SPEC>`"]
pub type WDSEL = crate::Reg<wdsel::WDSEL_SPEC>;
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
pub mod wdsel;
#[doc = "DONE register accessor: an alias for `Reg<DONE_SPEC>`"]
pub type DONE = crate::Reg<done::DONE_SPEC>;
#[doc = "Indicates the peripheral's registers are ready to access."]
pub mod done;
