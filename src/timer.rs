#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timehw: TIMEHW,
    timelw: TIMELW,
    timehr: TIMEHR,
    timelr: TIMELR,
    alarm0: ALARM0,
    alarm1: ALARM1,
    alarm2: ALARM2,
    alarm3: ALARM3,
    armed: ARMED,
    timerawh: TIMERAWH,
    timerawl: TIMERAWL,
    dbgpause: DBGPAUSE,
    pause: PAUSE,
    intr: INTR,
    inte: INTE,
    intf: INTF,
    ints: INTS,
}
impl RegisterBlock {
    #[doc = "0x00 - Write to bits 63:32 of time always write timelw before timehw"]
    #[inline(always)]
    pub const fn timehw(&self) -> &TIMEHW {
        &self.timehw
    }
    #[doc = "0x04 - Write to bits 31:0 of time writes do not get copied to time until timehw is written"]
    #[inline(always)]
    pub const fn timelw(&self) -> &TIMELW {
        &self.timelw
    }
    #[doc = "0x08 - Read from bits 63:32 of time always read timelr before timehr"]
    #[inline(always)]
    pub const fn timehr(&self) -> &TIMEHR {
        &self.timehr
    }
    #[doc = "0x0c - Read from bits 31:0 of time"]
    #[inline(always)]
    pub const fn timelr(&self) -> &TIMELR {
        &self.timelr
    }
    #[doc = "0x10 - Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    #[inline(always)]
    pub const fn alarm0(&self) -> &ALARM0 {
        &self.alarm0
    }
    #[doc = "0x14 - Arm alarm 1, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM1 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    #[inline(always)]
    pub const fn alarm1(&self) -> &ALARM1 {
        &self.alarm1
    }
    #[doc = "0x18 - Arm alarm 2, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM2 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    #[inline(always)]
    pub const fn alarm2(&self) -> &ALARM2 {
        &self.alarm2
    }
    #[doc = "0x1c - Arm alarm 3, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM3 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    #[inline(always)]
    pub const fn alarm3(&self) -> &ALARM3 {
        &self.alarm3
    }
    #[doc = "0x20 - Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire."]
    #[inline(always)]
    pub const fn armed(&self) -> &ARMED {
        &self.armed
    }
    #[doc = "0x24 - Raw read from bits 63:32 of time (no side effects)"]
    #[inline(always)]
    pub const fn timerawh(&self) -> &TIMERAWH {
        &self.timerawh
    }
    #[doc = "0x28 - Raw read from bits 31:0 of time (no side effects)"]
    #[inline(always)]
    pub const fn timerawl(&self) -> &TIMERAWL {
        &self.timerawl
    }
    #[doc = "0x2c - Set bits high to enable pause when the corresponding debug ports are active"]
    #[inline(always)]
    pub const fn dbgpause(&self) -> &DBGPAUSE {
        &self.dbgpause
    }
    #[doc = "0x30 - Set high to pause the timer"]
    #[inline(always)]
    pub const fn pause(&self) -> &PAUSE {
        &self.pause
    }
    #[doc = "0x34 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x38 - Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0x3c - Interrupt Force"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0x40 - Interrupt status after masking &amp; forcing"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
}
#[doc = "TIMEHW (rw) register accessor: Write to bits 63:32 of time always write timelw before timehw  

You can [`read`](crate::generic::Reg::read) this register and get [`timehw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timehw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timehw`]
module"]
pub type TIMEHW = crate::Reg<timehw::TIMEHW_SPEC>;
#[doc = "Write to bits 63:32 of time always write timelw before timehw"]
pub mod timehw;
#[doc = "TIMELW (rw) register accessor: Write to bits 31:0 of time writes do not get copied to time until timehw is written  

You can [`read`](crate::generic::Reg::read) this register and get [`timelw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timelw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timelw`]
module"]
pub type TIMELW = crate::Reg<timelw::TIMELW_SPEC>;
#[doc = "Write to bits 31:0 of time writes do not get copied to time until timehw is written"]
pub mod timelw;
#[doc = "TIMEHR (rw) register accessor: Read from bits 63:32 of time always read timelr before timehr  

You can [`read`](crate::generic::Reg::read) this register and get [`timehr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timehr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timehr`]
module"]
pub type TIMEHR = crate::Reg<timehr::TIMEHR_SPEC>;
#[doc = "Read from bits 63:32 of time always read timelr before timehr"]
pub mod timehr;
#[doc = "TIMELR (rw) register accessor: Read from bits 31:0 of time  

You can [`read`](crate::generic::Reg::read) this register and get [`timelr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timelr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timelr`]
module"]
pub type TIMELR = crate::Reg<timelr::TIMELR_SPEC>;
#[doc = "Read from bits 31:0 of time"]
pub mod timelr;
#[doc = "ALARM0 (rw) register accessor: Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm0`]
module"]
pub type ALARM0 = crate::Reg<alarm0::ALARM0_SPEC>;
#[doc = "Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
pub mod alarm0;
#[doc = "ALARM1 (rw) register accessor: Arm alarm 1, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM1 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm1`]
module"]
pub type ALARM1 = crate::Reg<alarm1::ALARM1_SPEC>;
#[doc = "Arm alarm 1, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM1 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
pub mod alarm1;
#[doc = "ALARM2 (rw) register accessor: Arm alarm 2, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM2 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm2`]
module"]
pub type ALARM2 = crate::Reg<alarm2::ALARM2_SPEC>;
#[doc = "Arm alarm 2, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM2 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
pub mod alarm2;
#[doc = "ALARM3 (rw) register accessor: Arm alarm 3, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM3 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm3`]
module"]
pub type ALARM3 = crate::Reg<alarm3::ALARM3_SPEC>;
#[doc = "Arm alarm 3, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM3 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
pub mod alarm3;
#[doc = "ARMED (rw) register accessor: Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire.  

You can [`read`](crate::generic::Reg::read) this register and get [`armed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@armed`]
module"]
pub type ARMED = crate::Reg<armed::ARMED_SPEC>;
#[doc = "Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire."]
pub mod armed;
#[doc = "TIMERAWH (rw) register accessor: Raw read from bits 63:32 of time (no side effects)  

You can [`read`](crate::generic::Reg::read) this register and get [`timerawh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timerawh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timerawh`]
module"]
pub type TIMERAWH = crate::Reg<timerawh::TIMERAWH_SPEC>;
#[doc = "Raw read from bits 63:32 of time (no side effects)"]
pub mod timerawh;
#[doc = "TIMERAWL (rw) register accessor: Raw read from bits 31:0 of time (no side effects)  

You can [`read`](crate::generic::Reg::read) this register and get [`timerawl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timerawl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

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
#[doc = "INTS (rw) register accessor: Interrupt status after masking &amp; forcing  

You can [`read`](crate::generic::Reg::read) this register and get [`ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints`]
module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing"]
pub mod ints;
