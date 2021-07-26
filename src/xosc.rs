#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crystal Oscillator Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Crystal Oscillator Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Crystal Oscillator pause control  
 This is used to save power by pausing the XOSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 WARNING: stop the PLLs before selecting dormant mode  
 WARNING: setup the irq before selecting dormant mode"]
    pub dormant: crate::Reg<dormant::DORMANT_SPEC>,
    #[doc = "0x0c - Controls the startup delay"]
    pub startup: crate::Reg<startup::STARTUP_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Crystal Oscillator Control"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Crystal Oscillator Status"]
pub mod status;
#[doc = "DORMANT register accessor: an alias for `Reg<DORMANT_SPEC>`"]
pub type DORMANT = crate::Reg<dormant::DORMANT_SPEC>;
#[doc = "Crystal Oscillator pause control  
 This is used to save power by pausing the XOSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 WARNING: stop the PLLs before selecting dormant mode  
 WARNING: setup the irq before selecting dormant mode"]
pub mod dormant;
#[doc = "STARTUP register accessor: an alias for `Reg<STARTUP_SPEC>`"]
pub type STARTUP = crate::Reg<startup::STARTUP_SPEC>;
#[doc = "Controls the startup delay"]
pub mod startup;
