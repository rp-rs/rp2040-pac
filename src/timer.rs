#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write to bits 63:32 of time  
 always write timelw before timehw"]
    pub timehw: TIMEHW,
    #[doc = "0x04 - Write to bits 31:0 of time  
 writes do not get copied to time until timehw is written"]
    pub timelw: TIMELW,
    #[doc = "0x08 - Read from bits 63:32 of time  
 always read timelr before timehr"]
    pub timehr: TIMEHR,
    #[doc = "0x0c - Read from bits 31:0 of time"]
    pub timelr: TIMELR,
    #[doc = "0x10 - Arm alarm 0, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
    pub alarm0: ALARM0,
    #[doc = "0x14 - Arm alarm 1, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
    pub alarm1: ALARM1,
    #[doc = "0x18 - Arm alarm 2, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
    pub alarm2: ALARM2,
    #[doc = "0x1c - Arm alarm 3, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
    pub alarm3: ALARM3,
    #[doc = "0x20 - Indicates the armed/disarmed status of each alarm.  
 A write to the corresponding ALARMx register arms the alarm.  
 Alarms automatically disarm upon firing, but writing ones here  
 will disarm immediately without waiting to fire."]
    pub armed: ARMED,
    #[doc = "0x24 - Raw read from bits 63:32 of time (no side effects)"]
    pub timerawh: TIMERAWH,
    #[doc = "0x28 - Raw read from bits 31:0 of time (no side effects)"]
    pub timerawl: TIMERAWL,
    #[doc = "0x2c - Set bits high to enable pause when the corresponding debug ports are active"]
    pub dbgpause: DBGPAUSE,
    #[doc = "0x30 - Set high to pause the timer"]
    pub pause: PAUSE,
    #[doc = "0x34 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0x38 - Interrupt Enable"]
    pub inte: INTE,
    #[doc = "0x3c - Interrupt Force"]
    pub intf: INTF,
    #[doc = "0x40 - Interrupt status after masking &amp; forcing"]
    pub ints: INTS,
}
#[doc = "TIMEHW (w) register accessor: an alias for `Reg<TIMEHW_SPEC>`"]
pub type TIMEHW = crate::Reg<timehw::TIMEHW_SPEC>;
#[doc = "Write to bits 63:32 of time  
 always write timelw before timehw"]
pub mod timehw;
#[doc = "TIMELW (w) register accessor: an alias for `Reg<TIMELW_SPEC>`"]
pub type TIMELW = crate::Reg<timelw::TIMELW_SPEC>;
#[doc = "Write to bits 31:0 of time  
 writes do not get copied to time until timehw is written"]
pub mod timelw;
#[doc = "TIMEHR (r) register accessor: an alias for `Reg<TIMEHR_SPEC>`"]
pub type TIMEHR = crate::Reg<timehr::TIMEHR_SPEC>;
#[doc = "Read from bits 63:32 of time  
 always read timelr before timehr"]
pub mod timehr;
#[doc = "TIMELR (r) register accessor: an alias for `Reg<TIMELR_SPEC>`"]
pub type TIMELR = crate::Reg<timelr::TIMELR_SPEC>;
#[doc = "Read from bits 31:0 of time"]
pub mod timelr;
#[doc = "ALARM0 (rw) register accessor: an alias for `Reg<ALARM0_SPEC>`"]
pub type ALARM0 = crate::Reg<alarm0::ALARM0_SPEC>;
#[doc = "Arm alarm 0, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm0;
#[doc = "ALARM1 (rw) register accessor: an alias for `Reg<ALARM1_SPEC>`"]
pub type ALARM1 = crate::Reg<alarm1::ALARM1_SPEC>;
#[doc = "Arm alarm 1, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm1;
#[doc = "ALARM2 (rw) register accessor: an alias for `Reg<ALARM2_SPEC>`"]
pub type ALARM2 = crate::Reg<alarm2::ALARM2_SPEC>;
#[doc = "Arm alarm 2, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm2;
#[doc = "ALARM3 (rw) register accessor: an alias for `Reg<ALARM3_SPEC>`"]
pub type ALARM3 = crate::Reg<alarm3::ALARM3_SPEC>;
#[doc = "Arm alarm 3, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm3;
#[doc = "ARMED (rw) register accessor: an alias for `Reg<ARMED_SPEC>`"]
pub type ARMED = crate::Reg<armed::ARMED_SPEC>;
#[doc = "Indicates the armed/disarmed status of each alarm.  
 A write to the corresponding ALARMx register arms the alarm.  
 Alarms automatically disarm upon firing, but writing ones here  
 will disarm immediately without waiting to fire."]
pub mod armed;
#[doc = "TIMERAWH (r) register accessor: an alias for `Reg<TIMERAWH_SPEC>`"]
pub type TIMERAWH = crate::Reg<timerawh::TIMERAWH_SPEC>;
#[doc = "Raw read from bits 63:32 of time (no side effects)"]
pub mod timerawh;
#[doc = "TIMERAWL (r) register accessor: an alias for `Reg<TIMERAWL_SPEC>`"]
pub type TIMERAWL = crate::Reg<timerawl::TIMERAWL_SPEC>;
#[doc = "Raw read from bits 31:0 of time (no side effects)"]
pub mod timerawl;
#[doc = "DBGPAUSE (rw) register accessor: an alias for `Reg<DBGPAUSE_SPEC>`"]
pub type DBGPAUSE = crate::Reg<dbgpause::DBGPAUSE_SPEC>;
#[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
pub mod dbgpause;
#[doc = "PAUSE (rw) register accessor: an alias for `Reg<PAUSE_SPEC>`"]
pub type PAUSE = crate::Reg<pause::PAUSE_SPEC>;
#[doc = "Set high to pause the timer"]
pub mod pause;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE (rw) register accessor: an alias for `Reg<INTE_SPEC>`"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF (rw) register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS (r) register accessor: an alias for `Reg<INTS_SPEC>`"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing"]
pub mod ints;
