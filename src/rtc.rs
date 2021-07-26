#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
    pub clkdiv_m1: crate::Reg<clkdiv_m1::CLKDIV_M1_SPEC>,
    #[doc = "0x04 - RTC setup register 0"]
    pub setup_0: crate::Reg<setup_0::SETUP_0_SPEC>,
    #[doc = "0x08 - RTC setup register 1"]
    pub setup_1: crate::Reg<setup_1::SETUP_1_SPEC>,
    #[doc = "0x0c - RTC Control and status"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x10 - Interrupt setup register 0"]
    pub irq_setup_0: crate::Reg<irq_setup_0::IRQ_SETUP_0_SPEC>,
    #[doc = "0x14 - Interrupt setup register 1"]
    pub irq_setup_1: crate::Reg<irq_setup_1::IRQ_SETUP_1_SPEC>,
    #[doc = "0x18 - RTC register 1."]
    pub rtc_1: crate::Reg<rtc_1::RTC_1_SPEC>,
    #[doc = "0x1c - RTC register 0  
 Read this before RTC 1!"]
    pub rtc_0: crate::Reg<rtc_0::RTC_0_SPEC>,
    #[doc = "0x20 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x24 - Interrupt Enable"]
    pub inte: crate::Reg<inte::INTE_SPEC>,
    #[doc = "0x28 - Interrupt Force"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x2c - Interrupt status after masking & forcing"]
    pub ints: crate::Reg<ints::INTS_SPEC>,
}
#[doc = "CLKDIV_M1 register accessor: an alias for `Reg<CLKDIV_M1_SPEC>`"]
pub type CLKDIV_M1 = crate::Reg<clkdiv_m1::CLKDIV_M1_SPEC>;
#[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
pub mod clkdiv_m1;
#[doc = "SETUP_0 register accessor: an alias for `Reg<SETUP_0_SPEC>`"]
pub type SETUP_0 = crate::Reg<setup_0::SETUP_0_SPEC>;
#[doc = "RTC setup register 0"]
pub mod setup_0;
#[doc = "SETUP_1 register accessor: an alias for `Reg<SETUP_1_SPEC>`"]
pub type SETUP_1 = crate::Reg<setup_1::SETUP_1_SPEC>;
#[doc = "RTC setup register 1"]
pub mod setup_1;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC Control and status"]
pub mod ctrl;
#[doc = "IRQ_SETUP_0 register accessor: an alias for `Reg<IRQ_SETUP_0_SPEC>`"]
pub type IRQ_SETUP_0 = crate::Reg<irq_setup_0::IRQ_SETUP_0_SPEC>;
#[doc = "Interrupt setup register 0"]
pub mod irq_setup_0;
#[doc = "IRQ_SETUP_1 register accessor: an alias for `Reg<IRQ_SETUP_1_SPEC>`"]
pub type IRQ_SETUP_1 = crate::Reg<irq_setup_1::IRQ_SETUP_1_SPEC>;
#[doc = "Interrupt setup register 1"]
pub mod irq_setup_1;
#[doc = "RTC_1 register accessor: an alias for `Reg<RTC_1_SPEC>`"]
pub type RTC_1 = crate::Reg<rtc_1::RTC_1_SPEC>;
#[doc = "RTC register 1."]
pub mod rtc_1;
#[doc = "RTC_0 register accessor: an alias for `Reg<RTC_0_SPEC>`"]
pub type RTC_0 = crate::Reg<rtc_0::RTC_0_SPEC>;
#[doc = "RTC register 0  
 Read this before RTC 1!"]
pub mod rtc_0;
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
