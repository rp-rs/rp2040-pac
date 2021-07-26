#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ring Oscillator control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage  
 The drive strength has 4 levels determined by the number of bits set  
 Increasing the number of bits set increases the drive strength and increases the oscillation frequency  
 0 bits set is the default drive strength  
 1 bit set doubles the drive strength  
 2 bits set triples drive strength  
 3 bits set quadruples drive strength"]
    pub freqa: crate::Reg<freqa::FREQA_SPEC>,
    #[doc = "0x08 - For a detailed description see freqa register"]
    pub freqb: crate::Reg<freqb::FREQB_SPEC>,
    #[doc = "0x0c - Ring Oscillator pause control  
 This is used to save power by pausing the ROSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 Warning: setup the irq before selecting dormant mode"]
    pub dormant: crate::Reg<dormant::DORMANT_SPEC>,
    #[doc = "0x10 - Controls the output divider"]
    pub div: crate::Reg<div::DIV_SPEC>,
    #[doc = "0x14 - Controls the phase shifted output"]
    pub phase: crate::Reg<phase::PHASE_SPEC>,
    #[doc = "0x18 - Ring Oscillator Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1c - This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
    pub randombit: crate::Reg<randombit::RANDOMBIT_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Ring Oscillator control"]
pub mod ctrl;
#[doc = "FREQA register accessor: an alias for `Reg<FREQA_SPEC>`"]
pub type FREQA = crate::Reg<freqa::FREQA_SPEC>;
#[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage  
 The drive strength has 4 levels determined by the number of bits set  
 Increasing the number of bits set increases the drive strength and increases the oscillation frequency  
 0 bits set is the default drive strength  
 1 bit set doubles the drive strength  
 2 bits set triples drive strength  
 3 bits set quadruples drive strength"]
pub mod freqa;
#[doc = "FREQB register accessor: an alias for `Reg<FREQB_SPEC>`"]
pub type FREQB = crate::Reg<freqb::FREQB_SPEC>;
#[doc = "For a detailed description see freqa register"]
pub mod freqb;
#[doc = "DORMANT register accessor: an alias for `Reg<DORMANT_SPEC>`"]
pub type DORMANT = crate::Reg<dormant::DORMANT_SPEC>;
#[doc = "Ring Oscillator pause control  
 This is used to save power by pausing the ROSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 Warning: setup the irq before selecting dormant mode"]
pub mod dormant;
#[doc = "DIV register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Controls the output divider"]
pub mod div;
#[doc = "PHASE register accessor: an alias for `Reg<PHASE_SPEC>`"]
pub type PHASE = crate::Reg<phase::PHASE_SPEC>;
#[doc = "Controls the phase shifted output"]
pub mod phase;
#[doc = "RANDOMBIT register accessor: an alias for `Reg<RANDOMBIT_SPEC>`"]
pub type RANDOMBIT = crate::Reg<randombit::RANDOMBIT_SPEC>;
#[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
pub mod randombit;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Ring Oscillator Status"]
pub mod status;
