#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xf0 - Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
    pub gpio: [GPIO; 30],
    #[doc = "0xf0..0x100 - Raw Interrupts"]
    pub intr: [crate::Reg<intr::INTR_SPEC>; 4],
    #[doc = "0x100..0x110 - Interrupt Enable for proc%s"]
    pub proc0_inte: [crate::Reg<proc0_inte::PROC0_INTE_SPEC>; 4],
    #[doc = "0x110..0x120 - Interrupt Force for proc%s"]
    pub proc0_intf: [crate::Reg<proc0_intf::PROC0_INTF_SPEC>; 4],
    #[doc = "0x120..0x130 - Interrupt status after masking & forcing for proc%s"]
    pub proc0_ints: [crate::Reg<proc0_ints::PROC0_INTS_SPEC>; 4],
    #[doc = "0x130..0x140 - Interrupt Enable for proc1"]
    pub proc1_inte: [crate::Reg<proc1_inte::PROC1_INTE_SPEC>; 4],
    #[doc = "0x140..0x150 - Interrupt Force for proc1"]
    pub proc1_intf: [crate::Reg<proc1_intf::PROC1_INTF_SPEC>; 4],
    #[doc = "0x150..0x160 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints: [crate::Reg<proc1_ints::PROC1_INTS_SPEC>; 4],
    #[doc = "0x160 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte0: crate::Reg<dormant_wake_inte0::DORMANT_WAKE_INTE0_SPEC>,
    #[doc = "0x164 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte1: crate::Reg<dormant_wake_inte1::DORMANT_WAKE_INTE1_SPEC>,
    #[doc = "0x168 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte2: crate::Reg<dormant_wake_inte2::DORMANT_WAKE_INTE2_SPEC>,
    #[doc = "0x16c - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte3: crate::Reg<dormant_wake_inte3::DORMANT_WAKE_INTE3_SPEC>,
    #[doc = "0x170 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf0: crate::Reg<dormant_wake_intf0::DORMANT_WAKE_INTF0_SPEC>,
    #[doc = "0x174 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf1: crate::Reg<dormant_wake_intf1::DORMANT_WAKE_INTF1_SPEC>,
    #[doc = "0x178 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf2: crate::Reg<dormant_wake_intf2::DORMANT_WAKE_INTF2_SPEC>,
    #[doc = "0x17c - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf3: crate::Reg<dormant_wake_intf3::DORMANT_WAKE_INTF3_SPEC>,
    #[doc = "0x180 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints0: crate::Reg<dormant_wake_ints0::DORMANT_WAKE_INTS0_SPEC>,
    #[doc = "0x184 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints1: crate::Reg<dormant_wake_ints1::DORMANT_WAKE_INTS1_SPEC>,
    #[doc = "0x188 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints2: crate::Reg<dormant_wake_ints2::DORMANT_WAKE_INTS2_SPEC>,
    #[doc = "0x18c - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints3: crate::Reg<dormant_wake_ints3::DORMANT_WAKE_INTS3_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO {
    #[doc = "0x00 - GPIO status"]
    pub gpio_status: crate::Reg<self::gpio::gpio_status::GPIO_STATUS_SPEC>,
    #[doc = "0x04 - GPIO control including function select and overrides."]
    pub gpio_ctrl: crate::Reg<self::gpio::gpio_ctrl::GPIO_CTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
pub mod gpio;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "PROC0_INTE register accessor: an alias for `Reg<PROC0_INTE_SPEC>`"]
pub type PROC0_INTE = crate::Reg<proc0_inte::PROC0_INTE_SPEC>;
#[doc = "Interrupt Enable for proc%s"]
pub mod proc0_inte;
#[doc = "PROC0_INTF register accessor: an alias for `Reg<PROC0_INTF_SPEC>`"]
pub type PROC0_INTF = crate::Reg<proc0_intf::PROC0_INTF_SPEC>;
#[doc = "Interrupt Force for proc%s"]
pub mod proc0_intf;
#[doc = "PROC0_INTS register accessor: an alias for `Reg<PROC0_INTS_SPEC>`"]
pub type PROC0_INTS = crate::Reg<proc0_ints::PROC0_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc%s"]
pub mod proc0_ints;
#[doc = "PROC1_INTE register accessor: an alias for `Reg<PROC1_INTE_SPEC>`"]
pub type PROC1_INTE = crate::Reg<proc1_inte::PROC1_INTE_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte;
#[doc = "PROC1_INTF register accessor: an alias for `Reg<PROC1_INTF_SPEC>`"]
pub type PROC1_INTF = crate::Reg<proc1_intf::PROC1_INTF_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf;
#[doc = "PROC1_INTS register accessor: an alias for `Reg<PROC1_INTS_SPEC>`"]
pub type PROC1_INTS = crate::Reg<proc1_ints::PROC1_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints;
#[doc = "DORMANT_WAKE_INTE0 register accessor: an alias for `Reg<DORMANT_WAKE_INTE0_SPEC>`"]
pub type DORMANT_WAKE_INTE0 = crate::Reg<dormant_wake_inte0::DORMANT_WAKE_INTE0_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte0;
#[doc = "DORMANT_WAKE_INTE1 register accessor: an alias for `Reg<DORMANT_WAKE_INTE1_SPEC>`"]
pub type DORMANT_WAKE_INTE1 = crate::Reg<dormant_wake_inte1::DORMANT_WAKE_INTE1_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte1;
#[doc = "DORMANT_WAKE_INTE2 register accessor: an alias for `Reg<DORMANT_WAKE_INTE2_SPEC>`"]
pub type DORMANT_WAKE_INTE2 = crate::Reg<dormant_wake_inte2::DORMANT_WAKE_INTE2_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte2;
#[doc = "DORMANT_WAKE_INTE3 register accessor: an alias for `Reg<DORMANT_WAKE_INTE3_SPEC>`"]
pub type DORMANT_WAKE_INTE3 = crate::Reg<dormant_wake_inte3::DORMANT_WAKE_INTE3_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte3;
#[doc = "DORMANT_WAKE_INTF0 register accessor: an alias for `Reg<DORMANT_WAKE_INTF0_SPEC>`"]
pub type DORMANT_WAKE_INTF0 = crate::Reg<dormant_wake_intf0::DORMANT_WAKE_INTF0_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf0;
#[doc = "DORMANT_WAKE_INTF1 register accessor: an alias for `Reg<DORMANT_WAKE_INTF1_SPEC>`"]
pub type DORMANT_WAKE_INTF1 = crate::Reg<dormant_wake_intf1::DORMANT_WAKE_INTF1_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf1;
#[doc = "DORMANT_WAKE_INTF2 register accessor: an alias for `Reg<DORMANT_WAKE_INTF2_SPEC>`"]
pub type DORMANT_WAKE_INTF2 = crate::Reg<dormant_wake_intf2::DORMANT_WAKE_INTF2_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf2;
#[doc = "DORMANT_WAKE_INTF3 register accessor: an alias for `Reg<DORMANT_WAKE_INTF3_SPEC>`"]
pub type DORMANT_WAKE_INTF3 = crate::Reg<dormant_wake_intf3::DORMANT_WAKE_INTF3_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf3;
#[doc = "DORMANT_WAKE_INTS0 register accessor: an alias for `Reg<DORMANT_WAKE_INTS0_SPEC>`"]
pub type DORMANT_WAKE_INTS0 = crate::Reg<dormant_wake_ints0::DORMANT_WAKE_INTS0_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints0;
#[doc = "DORMANT_WAKE_INTS1 register accessor: an alias for `Reg<DORMANT_WAKE_INTS1_SPEC>`"]
pub type DORMANT_WAKE_INTS1 = crate::Reg<dormant_wake_ints1::DORMANT_WAKE_INTS1_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints1;
#[doc = "DORMANT_WAKE_INTS2 register accessor: an alias for `Reg<DORMANT_WAKE_INTS2_SPEC>`"]
pub type DORMANT_WAKE_INTS2 = crate::Reg<dormant_wake_ints2::DORMANT_WAKE_INTS2_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints2;
#[doc = "DORMANT_WAKE_INTS3 register accessor: an alias for `Reg<DORMANT_WAKE_INTS3_SPEC>`"]
pub type DORMANT_WAKE_INTS3 = crate::Reg<dormant_wake_ints3::DORMANT_WAKE_INTS3_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints3;
