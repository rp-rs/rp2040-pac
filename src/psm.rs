#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Force block out of reset (i.e. power it on)"]
    pub frce_on: FRCE_ON,
    #[doc = "0x04 - Force into reset (i.e. power it off)"]
    pub frce_off: FRCE_OFF,
    #[doc = "0x08 - Set to 1 if this peripheral should be reset when the watchdog fires."]
    pub wdsel: WDSEL,
    #[doc = "0x0c - Indicates the peripheral's registers are ready to access."]
    pub done: DONE,
}
#[doc = "FRCE_ON (rw) register accessor: an alias for `Reg<FRCE_ON_SPEC>`"]
pub type FRCE_ON = crate::Reg<frce_on::FRCE_ON_SPEC>;
#[doc = "Force block out of reset (i.e. power it on)"]
pub mod frce_on;
#[doc = "FRCE_OFF (rw) register accessor: an alias for `Reg<FRCE_OFF_SPEC>`"]
pub type FRCE_OFF = crate::Reg<frce_off::FRCE_OFF_SPEC>;
#[doc = "Force into reset (i.e. power it off)"]
pub mod frce_off;
#[doc = "WDSEL (rw) register accessor: an alias for `Reg<WDSEL_SPEC>`"]
pub type WDSEL = crate::Reg<wdsel::WDSEL_SPEC>;
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
pub mod wdsel;
#[doc = "DONE (r) register accessor: an alias for `Reg<DONE_SPEC>`"]
pub type DONE = crate::Reg<done::DONE_SPEC>;
#[doc = "Indicates the peripheral's registers are ready to access."]
pub mod done;
