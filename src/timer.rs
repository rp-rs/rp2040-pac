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
#[doc = "TIMEHW (w) register accessor: Write to bits 63:32 of time  
 always write timelw before timehw  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timehw::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timehw`]
module"]
pub type TIMEHW = crate::Reg<timehw::TIMEHW_SPEC>;
#[doc = "Write to bits 63:32 of time  
 always write timelw before timehw"]
pub mod timehw;
#[doc = "TIMELW (w) register accessor: Write to bits 31:0 of time  
 writes do not get copied to time until timehw is written  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timelw::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timelw`]
module"]
pub type TIMELW = crate::Reg<timelw::TIMELW_SPEC>;
#[doc = "Write to bits 31:0 of time  
 writes do not get copied to time until timehw is written"]
pub mod timelw;
#[doc = "TIMEHR (r) register accessor: Read from bits 63:32 of time  
 always read timelr before timehr  

You can [`read`](crate::generic::Reg::read) this register and get [`timehr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timehr`]
module"]
pub type TIMEHR = crate::Reg<timehr::TIMEHR_SPEC>;
#[doc = "Read from bits 63:32 of time  
 always read timelr before timehr"]
pub mod timehr;
#[doc = "TIMELR (r) register accessor: Read from bits 31:0 of time  

You can [`read`](crate::generic::Reg::read) this register and get [`timelr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timelr`]
module"]
pub type TIMELR = crate::Reg<timelr::TIMELR_SPEC>;
#[doc = "Read from bits 31:0 of time"]
pub mod timelr;
#[doc = "ALARM0 (rw) register accessor: Arm alarm 0, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm0`]
module"]
pub type ALARM0 = crate::Reg<alarm0::ALARM0_SPEC>;
#[doc = "Arm alarm 0, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm0;
#[doc = "ALARM1 (rw) register accessor: Arm alarm 1, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm1`]
module"]
pub type ALARM1 = crate::Reg<alarm1::ALARM1_SPEC>;
#[doc = "Arm alarm 1, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm1;
#[doc = "ALARM2 (rw) register accessor: Arm alarm 2, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm2`]
module"]
pub type ALARM2 = crate::Reg<alarm2::ALARM2_SPEC>;
#[doc = "Arm alarm 2, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm2;
#[doc = "ALARM3 (rw) register accessor: Arm alarm 3, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm3`]
module"]
pub type ALARM3 = crate::Reg<alarm3::ALARM3_SPEC>;
#[doc = "Arm alarm 3, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register."]
pub mod alarm3;
#[doc = "ARMED (rw) register accessor: Indicates the armed/disarmed status of each alarm.  
 A write to the corresponding ALARMx register arms the alarm.  
 Alarms automatically disarm upon firing, but writing ones here  
 will disarm immediately without waiting to fire.  

You can [`read`](crate::generic::Reg::read) this register and get [`armed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@armed`]
module"]
pub type ARMED = crate::Reg<armed::ARMED_SPEC>;
#[doc = "Indicates the armed/disarmed status of each alarm.  
 A write to the corresponding ALARMx register arms the alarm.  
 Alarms automatically disarm upon firing, but writing ones here  
 will disarm immediately without waiting to fire."]
pub mod armed;
#[doc = "TIMERAWH (r) register accessor: Raw read from bits 63:32 of time (no side effects)  

You can [`read`](crate::generic::Reg::read) this register and get [`timerawh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timerawh`]
module"]
pub type TIMERAWH = crate::Reg<timerawh::TIMERAWH_SPEC>;
#[doc = "Raw read from bits 63:32 of time (no side effects)"]
pub mod timerawh;
#[doc = "TIMERAWL (r) register accessor: Raw read from bits 31:0 of time (no side effects)  

You can [`read`](crate::generic::Reg::read) this register and get [`timerawl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timerawl`]
module"]
pub type TIMERAWL = crate::Reg<timerawl::TIMERAWL_SPEC>;
#[doc = "Raw read from bits 31:0 of time (no side effects)"]
pub mod timerawl;
#[doc = "DBGPAUSE (rw) register accessor: Set bits high to enable pause when the corresponding debug ports are active  

You can [`read`](crate::generic::Reg::read) this register and get [`dbgpause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgpause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbgpause`]
module"]
pub type DBGPAUSE = crate::Reg<dbgpause::DBGPAUSE_SPEC>;
#[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
pub mod dbgpause;
#[doc = "PAUSE (rw) register accessor: Set high to pause the timer  

You can [`read`](crate::generic::Reg::read) this register and get [`pause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pause`]
module"]
pub type PAUSE = crate::Reg<pause::PAUSE_SPEC>;
#[doc = "Set high to pause the timer"]
pub mod pause;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE (rw) register accessor: Interrupt Enable  

You can [`read`](crate::generic::Reg::read) this register and get [`inte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte`]
module"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF (rw) register accessor: Interrupt Force  

You can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS (r) register accessor: Interrupt status after masking &amp; forcing  

You can [`read`](crate::generic::Reg::read) this register and get [`ints::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints`]
module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing"]
pub mod ints;
