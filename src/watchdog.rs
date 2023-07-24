#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog control  
 The rst_wdsel register determines which subsystems are reset when the watchdog is triggered.  
 The watchdog can be triggered in software."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
    pub load: LOAD,
    #[doc = "0x08 - Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
    pub reason: REASON,
    #[doc = "0x0c - Scratch register. Information persists through soft reset of the chip."]
    pub scratch0: SCRATCH0,
    #[doc = "0x10 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch1: SCRATCH1,
    #[doc = "0x14 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch2: SCRATCH2,
    #[doc = "0x18 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch3: SCRATCH3,
    #[doc = "0x1c - Scratch register. Information persists through soft reset of the chip."]
    pub scratch4: SCRATCH4,
    #[doc = "0x20 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch5: SCRATCH5,
    #[doc = "0x24 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch6: SCRATCH6,
    #[doc = "0x28 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch7: SCRATCH7,
    #[doc = "0x2c - Controls the tick generator"]
    pub tick: TICK,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Watchdog control  
 The rst_wdsel register determines which subsystems are reset when the watchdog is triggered.  
 The watchdog can be triggered in software."]
pub mod ctrl;
#[doc = "LOAD (w) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
pub mod load;
#[doc = "REASON (r) register accessor: an alias for `Reg<REASON_SPEC>`"]
pub type REASON = crate::Reg<reason::REASON_SPEC>;
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
pub mod reason;
#[doc = "SCRATCH0 (rw) register accessor: an alias for `Reg<SCRATCH0_SPEC>`"]
pub type SCRATCH0 = crate::Reg<scratch0::SCRATCH0_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch0;
#[doc = "SCRATCH1 (rw) register accessor: an alias for `Reg<SCRATCH1_SPEC>`"]
pub type SCRATCH1 = crate::Reg<scratch1::SCRATCH1_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch1;
#[doc = "SCRATCH2 (rw) register accessor: an alias for `Reg<SCRATCH2_SPEC>`"]
pub type SCRATCH2 = crate::Reg<scratch2::SCRATCH2_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch2;
#[doc = "SCRATCH3 (rw) register accessor: an alias for `Reg<SCRATCH3_SPEC>`"]
pub type SCRATCH3 = crate::Reg<scratch3::SCRATCH3_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch3;
#[doc = "SCRATCH4 (rw) register accessor: an alias for `Reg<SCRATCH4_SPEC>`"]
pub type SCRATCH4 = crate::Reg<scratch4::SCRATCH4_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch4;
#[doc = "SCRATCH5 (rw) register accessor: an alias for `Reg<SCRATCH5_SPEC>`"]
pub type SCRATCH5 = crate::Reg<scratch5::SCRATCH5_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch5;
#[doc = "SCRATCH6 (rw) register accessor: an alias for `Reg<SCRATCH6_SPEC>`"]
pub type SCRATCH6 = crate::Reg<scratch6::SCRATCH6_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch6;
#[doc = "SCRATCH7 (rw) register accessor: an alias for `Reg<SCRATCH7_SPEC>`"]
pub type SCRATCH7 = crate::Reg<scratch7::SCRATCH7_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch7;
#[doc = "TICK (rw) register accessor: an alias for `Reg<TICK_SPEC>`"]
pub type TICK = crate::Reg<tick::TICK_SPEC>;
#[doc = "Controls the tick generator"]
pub mod tick;
