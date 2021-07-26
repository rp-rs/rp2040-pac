#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write to bits 63:32 of time  
 always write timelw before timehw"]
    pub timehw: crate::Reg<timehw::TIMEHW_SPEC>,
    #[doc = "0x04 - Write to bits 31:0 of time  
 writes do not get copied to time until timehw is written"]
    pub timelw: crate::Reg<timelw::TIMELW_SPEC>,
    #[doc = "0x08 - Read from bits 63:32 of time  
 always read timelr before timehr"]
    pub timehr: crate::Reg<timehr::TIMEHR_SPEC>,
    #[doc = "0x0c - Read from bits 31:0 of time"]
    pub timelr: crate::Reg<timelr::TIMELR_SPEC>,
    #[doc = "0x10 - Arm alarm 0, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
    pub alarm0: crate::Reg<alarm0::ALARM0_SPEC>,
    #[doc = "0x14 - Arm alarm 1, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
    pub alarm1: crate::Reg<alarm1::ALARM1_SPEC>,
    #[doc = "0x18 - Arm alarm 2, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
    pub alarm2: crate::Reg<alarm2::ALARM2_SPEC>,
    #[doc = "0x1c - Arm alarm 3, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
    pub alarm3: crate::Reg<alarm3::ALARM3_SPEC>,
    #[doc = "0x20 - Indicates the armed/disarmed status of each alarm.  
 A write to the corresponding ALARMx register arms the alarm.  
 Alarms automatically disarm upon firing, but writing ones here  
 will disarm immediately without waiting to fire."]
    pub armed: crate::Reg<armed::ARMED_SPEC>,
    #[doc = "0x24 - Raw read from bits 63:32 of time (no side effects)"]
    pub timerawh: crate::Reg<timerawh::TIMERAWH_SPEC>,
    #[doc = "0x28 - Raw read from bits 31:0 of time (no side effects)"]
    pub timerawl: crate::Reg<timerawl::TIMERAWL_SPEC>,
    #[doc = "0x2c - Set bits high to enable pause when the corresponding debug ports are active"]
    pub dbgpause: crate::Reg<dbgpause::DBGPAUSE_SPEC>,
    #[doc = "0x30 - Set high to pause the timer"]
    pub pause: crate::Reg<pause::PAUSE_SPEC>,
    #[doc = "0x34 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x38 - Interrupt Enable"]
    pub inte: crate::Reg<inte::INTE_SPEC>,
    #[doc = "0x3c - Interrupt Force"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x40 - Interrupt status after masking & forcing"]
    pub ints: crate::Reg<ints::INTS_SPEC>,
}
#[doc = "TIMEHW register accessor: an alias for `Reg<TIMEHW_SPEC>`"]
pub type TIMEHW = crate::Reg<timehw::TIMEHW_SPEC>;
#[doc = "Write to bits 63:32 of time  
 always write timelw before timehw"]
pub mod timehw;
#[doc = "TIMELW register accessor: an alias for `Reg<TIMELW_SPEC>`"]
pub type TIMELW = crate::Reg<timelw::TIMELW_SPEC>;
#[doc = "Write to bits 31:0 of time  
 writes do not get copied to time until timehw is written"]
pub mod timelw;
#[doc = "TIMEHR register accessor: an alias for `Reg<TIMEHR_SPEC>`"]
pub type TIMEHR = crate::Reg<timehr::TIMEHR_SPEC>;
#[doc = "Read from bits 63:32 of time  
 always read timelr before timehr"]
pub mod timehr;
#[doc = "TIMELR register accessor: an alias for `Reg<TIMELR_SPEC>`"]
pub type TIMELR = crate::Reg<timelr::TIMELR_SPEC>;
#[doc = "Read from bits 31:0 of time"]
pub mod timelr;
#[doc = "ALARM0 register accessor: an alias for `Reg<ALARM0_SPEC>`"]
pub type ALARM0 = crate::Reg<alarm0::ALARM0_SPEC>;
#[doc = "Arm alarm 0, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm0;
#[doc = "ALARM1 register accessor: an alias for `Reg<ALARM1_SPEC>`"]
pub type ALARM1 = crate::Reg<alarm1::ALARM1_SPEC>;
#[doc = "Arm alarm 1, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm1;
#[doc = "ALARM2 register accessor: an alias for `Reg<ALARM2_SPEC>`"]
pub type ALARM2 = crate::Reg<alarm2::ALARM2_SPEC>;
#[doc = "Arm alarm 2, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm2;
#[doc = "ALARM3 register accessor: an alias for `Reg<ALARM3_SPEC>`"]
pub type ALARM3 = crate::Reg<alarm3::ALARM3_SPEC>;
#[doc = "Arm alarm 3, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm3;
#[doc = "ARMED register accessor: an alias for `Reg<ARMED_SPEC>`"]
pub type ARMED = crate::Reg<armed::ARMED_SPEC>;
#[doc = "Indicates the armed/disarmed status of each alarm.  
 A write to the corresponding ALARMx register arms the alarm.  
 Alarms automatically disarm upon firing, but writing ones here  
 will disarm immediately without waiting to fire."]
pub mod armed;
#[doc = "TIMERAWH register accessor: an alias for `Reg<TIMERAWH_SPEC>`"]
pub type TIMERAWH = crate::Reg<timerawh::TIMERAWH_SPEC>;
#[doc = "Raw read from bits 63:32 of time (no side effects)"]
pub mod timerawh;
#[doc = "TIMERAWL register accessor: an alias for `Reg<TIMERAWL_SPEC>`"]
pub type TIMERAWL = crate::Reg<timerawl::TIMERAWL_SPEC>;
#[doc = "Raw read from bits 31:0 of time (no side effects)"]
pub mod timerawl;
#[doc = "DBGPAUSE register accessor: an alias for `Reg<DBGPAUSE_SPEC>`"]
pub type DBGPAUSE = crate::Reg<dbgpause::DBGPAUSE_SPEC>;
#[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
pub mod dbgpause;
#[doc = "PAUSE register accessor: an alias for `Reg<PAUSE_SPEC>`"]
pub type PAUSE = crate::Reg<pause::PAUSE_SPEC>;
#[doc = "Set high to pause the timer"]
pub mod pause;
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
