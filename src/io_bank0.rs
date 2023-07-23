#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xf0 - Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
    pub gpio: [GPIO; 30],
    #[doc = "0xf0..0x100 - Raw Interrupts"]
    pub intr: [INTR; 4],
    #[doc = "0x100..0x110 - Interrupt Enable for proc%s"]
    pub proc0_inte: [PROC0_INTE; 4],
    #[doc = "0x110..0x120 - Interrupt Force for proc%s"]
    pub proc0_intf: [PROC0_INTF; 4],
    #[doc = "0x120..0x130 - Interrupt status after masking & forcing for proc%s"]
    pub proc0_ints: [PROC0_INTS; 4],
    #[doc = "0x130..0x140 - Interrupt Enable for proc1"]
    pub proc1_inte: [PROC1_INTE; 4],
    #[doc = "0x140..0x150 - Interrupt Force for proc1"]
    pub proc1_intf: [PROC1_INTF; 4],
    #[doc = "0x150..0x160 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints: [PROC1_INTS; 4],
    #[doc = "0x160..0x170 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte: [DORMANT_WAKE_INTE; 4],
    #[doc = "0x170..0x180 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf: [DORMANT_WAKE_INTF; 4],
    #[doc = "0x180..0x190 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints: [DORMANT_WAKE_INTS; 4],
}
#[doc = "Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
pub use self::gpio::GPIO;
#[doc = r"Cluster"]
#[doc = "Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
pub mod gpio;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "PROC0_INTE (rw) register accessor: an alias for `Reg<PROC0_INTE_SPEC>`"]
pub type PROC0_INTE = crate::Reg<proc0_inte::PROC0_INTE_SPEC>;
#[doc = "Interrupt Enable for proc%s"]
pub mod proc0_inte;
#[doc = "PROC0_INTF (rw) register accessor: an alias for `Reg<PROC0_INTF_SPEC>`"]
pub type PROC0_INTF = crate::Reg<proc0_intf::PROC0_INTF_SPEC>;
#[doc = "Interrupt Force for proc%s"]
pub mod proc0_intf;
#[doc = "PROC0_INTS (r) register accessor: an alias for `Reg<PROC0_INTS_SPEC>`"]
pub type PROC0_INTS = crate::Reg<proc0_ints::PROC0_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc%s"]
pub mod proc0_ints;
#[doc = "PROC1_INTE (rw) register accessor: an alias for `Reg<PROC1_INTE_SPEC>`"]
pub type PROC1_INTE = crate::Reg<proc1_inte::PROC1_INTE_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte;
#[doc = "PROC1_INTF (rw) register accessor: an alias for `Reg<PROC1_INTF_SPEC>`"]
pub type PROC1_INTF = crate::Reg<proc1_intf::PROC1_INTF_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf;
#[doc = "PROC1_INTS (r) register accessor: an alias for `Reg<PROC1_INTS_SPEC>`"]
pub type PROC1_INTS = crate::Reg<proc1_ints::PROC1_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints;
#[doc = "DORMANT_WAKE_INTE (rw) register accessor: an alias for `Reg<DORMANT_WAKE_INTE_SPEC>`"]
pub type DORMANT_WAKE_INTE = crate::Reg<dormant_wake_inte::DORMANT_WAKE_INTE_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte;
#[doc = "DORMANT_WAKE_INTF (rw) register accessor: an alias for `Reg<DORMANT_WAKE_INTF_SPEC>`"]
pub type DORMANT_WAKE_INTF = crate::Reg<dormant_wake_intf::DORMANT_WAKE_INTF_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf;
#[doc = "DORMANT_WAKE_INTS (r) register accessor: an alias for `Reg<DORMANT_WAKE_INTS_SPEC>`"]
pub type DORMANT_WAKE_INTS = crate::Reg<dormant_wake_ints::DORMANT_WAKE_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints;
