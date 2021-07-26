#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xf0 - Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
    pub gpio: [GPIO; 30],
    #[doc = "0xf0 - Raw Interrupts"]
    pub intr0: crate::Reg<intr0::INTR0_SPEC>,
    #[doc = "0xf4 - Raw Interrupts"]
    pub intr1: crate::Reg<intr1::INTR1_SPEC>,
    #[doc = "0xf8 - Raw Interrupts"]
    pub intr2: crate::Reg<intr2::INTR2_SPEC>,
    #[doc = "0xfc - Raw Interrupts"]
    pub intr3: crate::Reg<intr3::INTR3_SPEC>,
    #[doc = "0x100 - Interrupt Enable for proc0"]
    pub proc0_inte0: crate::Reg<proc0_inte0::PROC0_INTE0_SPEC>,
    #[doc = "0x104 - Interrupt Enable for proc0"]
    pub proc0_inte1: crate::Reg<proc0_inte1::PROC0_INTE1_SPEC>,
    #[doc = "0x108 - Interrupt Enable for proc0"]
    pub proc0_inte2: crate::Reg<proc0_inte2::PROC0_INTE2_SPEC>,
    #[doc = "0x10c - Interrupt Enable for proc0"]
    pub proc0_inte3: crate::Reg<proc0_inte3::PROC0_INTE3_SPEC>,
    #[doc = "0x110 - Interrupt Force for proc0"]
    pub proc0_intf0: crate::Reg<proc0_intf0::PROC0_INTF0_SPEC>,
    #[doc = "0x114 - Interrupt Force for proc0"]
    pub proc0_intf1: crate::Reg<proc0_intf1::PROC0_INTF1_SPEC>,
    #[doc = "0x118 - Interrupt Force for proc0"]
    pub proc0_intf2: crate::Reg<proc0_intf2::PROC0_INTF2_SPEC>,
    #[doc = "0x11c - Interrupt Force for proc0"]
    pub proc0_intf3: crate::Reg<proc0_intf3::PROC0_INTF3_SPEC>,
    #[doc = "0x120 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints0: crate::Reg<proc0_ints0::PROC0_INTS0_SPEC>,
    #[doc = "0x124 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints1: crate::Reg<proc0_ints1::PROC0_INTS1_SPEC>,
    #[doc = "0x128 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints2: crate::Reg<proc0_ints2::PROC0_INTS2_SPEC>,
    #[doc = "0x12c - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints3: crate::Reg<proc0_ints3::PROC0_INTS3_SPEC>,
    #[doc = "0x130 - Interrupt Enable for proc1"]
    pub proc1_inte0: crate::Reg<proc1_inte0::PROC1_INTE0_SPEC>,
    #[doc = "0x134 - Interrupt Enable for proc1"]
    pub proc1_inte1: crate::Reg<proc1_inte1::PROC1_INTE1_SPEC>,
    #[doc = "0x138 - Interrupt Enable for proc1"]
    pub proc1_inte2: crate::Reg<proc1_inte2::PROC1_INTE2_SPEC>,
    #[doc = "0x13c - Interrupt Enable for proc1"]
    pub proc1_inte3: crate::Reg<proc1_inte3::PROC1_INTE3_SPEC>,
    #[doc = "0x140 - Interrupt Force for proc1"]
    pub proc1_intf0: crate::Reg<proc1_intf0::PROC1_INTF0_SPEC>,
    #[doc = "0x144 - Interrupt Force for proc1"]
    pub proc1_intf1: crate::Reg<proc1_intf1::PROC1_INTF1_SPEC>,
    #[doc = "0x148 - Interrupt Force for proc1"]
    pub proc1_intf2: crate::Reg<proc1_intf2::PROC1_INTF2_SPEC>,
    #[doc = "0x14c - Interrupt Force for proc1"]
    pub proc1_intf3: crate::Reg<proc1_intf3::PROC1_INTF3_SPEC>,
    #[doc = "0x150 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints0: crate::Reg<proc1_ints0::PROC1_INTS0_SPEC>,
    #[doc = "0x154 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints1: crate::Reg<proc1_ints1::PROC1_INTS1_SPEC>,
    #[doc = "0x158 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints2: crate::Reg<proc1_ints2::PROC1_INTS2_SPEC>,
    #[doc = "0x15c - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints3: crate::Reg<proc1_ints3::PROC1_INTS3_SPEC>,
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
#[doc = "INTR0 register accessor: an alias for `Reg<INTR0_SPEC>`"]
pub type INTR0 = crate::Reg<intr0::INTR0_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr0;
#[doc = "INTR1 register accessor: an alias for `Reg<INTR1_SPEC>`"]
pub type INTR1 = crate::Reg<intr1::INTR1_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr1;
#[doc = "INTR2 register accessor: an alias for `Reg<INTR2_SPEC>`"]
pub type INTR2 = crate::Reg<intr2::INTR2_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr2;
#[doc = "INTR3 register accessor: an alias for `Reg<INTR3_SPEC>`"]
pub type INTR3 = crate::Reg<intr3::INTR3_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr3;
#[doc = "PROC0_INTE0 register accessor: an alias for `Reg<PROC0_INTE0_SPEC>`"]
pub type PROC0_INTE0 = crate::Reg<proc0_inte0::PROC0_INTE0_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte0;
#[doc = "PROC0_INTE1 register accessor: an alias for `Reg<PROC0_INTE1_SPEC>`"]
pub type PROC0_INTE1 = crate::Reg<proc0_inte1::PROC0_INTE1_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte1;
#[doc = "PROC0_INTE2 register accessor: an alias for `Reg<PROC0_INTE2_SPEC>`"]
pub type PROC0_INTE2 = crate::Reg<proc0_inte2::PROC0_INTE2_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte2;
#[doc = "PROC0_INTE3 register accessor: an alias for `Reg<PROC0_INTE3_SPEC>`"]
pub type PROC0_INTE3 = crate::Reg<proc0_inte3::PROC0_INTE3_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte3;
#[doc = "PROC0_INTF0 register accessor: an alias for `Reg<PROC0_INTF0_SPEC>`"]
pub type PROC0_INTF0 = crate::Reg<proc0_intf0::PROC0_INTF0_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf0;
#[doc = "PROC0_INTF1 register accessor: an alias for `Reg<PROC0_INTF1_SPEC>`"]
pub type PROC0_INTF1 = crate::Reg<proc0_intf1::PROC0_INTF1_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf1;
#[doc = "PROC0_INTF2 register accessor: an alias for `Reg<PROC0_INTF2_SPEC>`"]
pub type PROC0_INTF2 = crate::Reg<proc0_intf2::PROC0_INTF2_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf2;
#[doc = "PROC0_INTF3 register accessor: an alias for `Reg<PROC0_INTF3_SPEC>`"]
pub type PROC0_INTF3 = crate::Reg<proc0_intf3::PROC0_INTF3_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf3;
#[doc = "PROC0_INTS0 register accessor: an alias for `Reg<PROC0_INTS0_SPEC>`"]
pub type PROC0_INTS0 = crate::Reg<proc0_ints0::PROC0_INTS0_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints0;
#[doc = "PROC0_INTS1 register accessor: an alias for `Reg<PROC0_INTS1_SPEC>`"]
pub type PROC0_INTS1 = crate::Reg<proc0_ints1::PROC0_INTS1_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints1;
#[doc = "PROC0_INTS2 register accessor: an alias for `Reg<PROC0_INTS2_SPEC>`"]
pub type PROC0_INTS2 = crate::Reg<proc0_ints2::PROC0_INTS2_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints2;
#[doc = "PROC0_INTS3 register accessor: an alias for `Reg<PROC0_INTS3_SPEC>`"]
pub type PROC0_INTS3 = crate::Reg<proc0_ints3::PROC0_INTS3_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints3;
#[doc = "PROC1_INTE0 register accessor: an alias for `Reg<PROC1_INTE0_SPEC>`"]
pub type PROC1_INTE0 = crate::Reg<proc1_inte0::PROC1_INTE0_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte0;
#[doc = "PROC1_INTE1 register accessor: an alias for `Reg<PROC1_INTE1_SPEC>`"]
pub type PROC1_INTE1 = crate::Reg<proc1_inte1::PROC1_INTE1_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte1;
#[doc = "PROC1_INTE2 register accessor: an alias for `Reg<PROC1_INTE2_SPEC>`"]
pub type PROC1_INTE2 = crate::Reg<proc1_inte2::PROC1_INTE2_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte2;
#[doc = "PROC1_INTE3 register accessor: an alias for `Reg<PROC1_INTE3_SPEC>`"]
pub type PROC1_INTE3 = crate::Reg<proc1_inte3::PROC1_INTE3_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte3;
#[doc = "PROC1_INTF0 register accessor: an alias for `Reg<PROC1_INTF0_SPEC>`"]
pub type PROC1_INTF0 = crate::Reg<proc1_intf0::PROC1_INTF0_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf0;
#[doc = "PROC1_INTF1 register accessor: an alias for `Reg<PROC1_INTF1_SPEC>`"]
pub type PROC1_INTF1 = crate::Reg<proc1_intf1::PROC1_INTF1_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf1;
#[doc = "PROC1_INTF2 register accessor: an alias for `Reg<PROC1_INTF2_SPEC>`"]
pub type PROC1_INTF2 = crate::Reg<proc1_intf2::PROC1_INTF2_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf2;
#[doc = "PROC1_INTF3 register accessor: an alias for `Reg<PROC1_INTF3_SPEC>`"]
pub type PROC1_INTF3 = crate::Reg<proc1_intf3::PROC1_INTF3_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf3;
#[doc = "PROC1_INTS0 register accessor: an alias for `Reg<PROC1_INTS0_SPEC>`"]
pub type PROC1_INTS0 = crate::Reg<proc1_ints0::PROC1_INTS0_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints0;
#[doc = "PROC1_INTS1 register accessor: an alias for `Reg<PROC1_INTS1_SPEC>`"]
pub type PROC1_INTS1 = crate::Reg<proc1_ints1::PROC1_INTS1_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints1;
#[doc = "PROC1_INTS2 register accessor: an alias for `Reg<PROC1_INTS2_SPEC>`"]
pub type PROC1_INTS2 = crate::Reg<proc1_ints2::PROC1_INTS2_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints2;
#[doc = "PROC1_INTS3 register accessor: an alias for `Reg<PROC1_INTS3_SPEC>`"]
pub type PROC1_INTS3 = crate::Reg<proc1_ints3::PROC1_INTS3_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints3;
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
