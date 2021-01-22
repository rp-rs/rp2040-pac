#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write to bits 63:32 of time\\n always write timelw before timehw"]
    pub timehw: TIMEHW,
    #[doc = "0x04 - Write to bits 31:0 of time\\n writes do not get copied to time until timehw is written"]
    pub timelw: TIMELW,
    #[doc = "0x08 - Read from bits 63:32 of time\\n always read timelr before timehr"]
    pub timehr: TIMEHR,
    #[doc = "0x0c - Read from bits 31:0 of time"]
    pub timelr: TIMELR,
    #[doc = "0x10 - Arm alarm 0, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register."]
    pub alarm0: ALARM0,
    #[doc = "0x14 - Arm alarm 1, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register."]
    pub alarm1: ALARM1,
    #[doc = "0x18 - Arm alarm 2, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register."]
    pub alarm2: ALARM2,
    #[doc = "0x1c - Arm alarm 3, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register."]
    pub alarm3: ALARM3,
    #[doc = "0x20 - Indicates the armed/disarmed status of each alarm.\\n A write to the corresponding ALARMx register arms the alarm.\\n Alarms automatically disarm upon firing, but writing ones here\\n will disarm immediately without waiting to fire."]
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
    #[doc = "0x40 - Interrupt status after masking & forcing"]
    pub ints: INTS,
}
#[doc = "Write to bits 63:32 of time\\n always write timelw before timehw\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timehw](timehw) module"]
pub type TIMEHW = crate::Reg<u32, _TIMEHW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEHW;
#[doc = "`write(|w| ..)` method takes [timehw::W](timehw::W) writer structure"]
impl crate::Writable for TIMEHW {}
#[doc = "Write to bits 63:32 of time\\n always write timelw before timehw"]
pub mod timehw;
#[doc = "Write to bits 31:0 of time\\n writes do not get copied to time until timehw is written\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timelw](timelw) module"]
pub type TIMELW = crate::Reg<u32, _TIMELW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMELW;
#[doc = "`write(|w| ..)` method takes [timelw::W](timelw::W) writer structure"]
impl crate::Writable for TIMELW {}
#[doc = "Write to bits 31:0 of time\\n writes do not get copied to time until timehw is written"]
pub mod timelw;
#[doc = "Read from bits 63:32 of time\\n always read timelr before timehr\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timehr](timehr) module"]
pub type TIMEHR = crate::Reg<u32, _TIMEHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEHR;
#[doc = "`read()` method returns [timehr::R](timehr::R) reader structure"]
impl crate::Readable for TIMEHR {}
#[doc = "Read from bits 63:32 of time\\n always read timelr before timehr"]
pub mod timehr;
#[doc = "Read from bits 31:0 of time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timelr](timelr) module"]
pub type TIMELR = crate::Reg<u32, _TIMELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMELR;
#[doc = "`read()` method returns [timelr::R](timelr::R) reader structure"]
impl crate::Readable for TIMELR {}
#[doc = "Read from bits 31:0 of time"]
pub mod timelr;
#[doc = "Arm alarm 0, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm0](alarm0) module"]
pub type ALARM0 = crate::Reg<u32, _ALARM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARM0;
#[doc = "`read()` method returns [alarm0::R](alarm0::R) reader structure"]
impl crate::Readable for ALARM0 {}
#[doc = "`write(|w| ..)` method takes [alarm0::W](alarm0::W) writer structure"]
impl crate::Writable for ALARM0 {}
#[doc = "Arm alarm 0, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register."]
pub mod alarm0;
#[doc = "Arm alarm 1, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm1](alarm1) module"]
pub type ALARM1 = crate::Reg<u32, _ALARM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARM1;
#[doc = "`read()` method returns [alarm1::R](alarm1::R) reader structure"]
impl crate::Readable for ALARM1 {}
#[doc = "`write(|w| ..)` method takes [alarm1::W](alarm1::W) writer structure"]
impl crate::Writable for ALARM1 {}
#[doc = "Arm alarm 1, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register."]
pub mod alarm1;
#[doc = "Arm alarm 2, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm2](alarm2) module"]
pub type ALARM2 = crate::Reg<u32, _ALARM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARM2;
#[doc = "`read()` method returns [alarm2::R](alarm2::R) reader structure"]
impl crate::Readable for ALARM2 {}
#[doc = "`write(|w| ..)` method takes [alarm2::W](alarm2::W) writer structure"]
impl crate::Writable for ALARM2 {}
#[doc = "Arm alarm 2, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register."]
pub mod alarm2;
#[doc = "Arm alarm 3, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm3](alarm3) module"]
pub type ALARM3 = crate::Reg<u32, _ALARM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARM3;
#[doc = "`read()` method returns [alarm3::R](alarm3::R) reader structure"]
impl crate::Readable for ALARM3 {}
#[doc = "`write(|w| ..)` method takes [alarm3::W](alarm3::W) writer structure"]
impl crate::Writable for ALARM3 {}
#[doc = "Arm alarm 3, and configure the time it will fire.\\n Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.\\n The alarm will disarm itself once it fires, and can\\n be disarmed early using the ARMED status register."]
pub mod alarm3;
#[doc = "Indicates the armed/disarmed status of each alarm.\\n A write to the corresponding ALARMx register arms the alarm.\\n Alarms automatically disarm upon firing, but writing ones here\\n will disarm immediately without waiting to fire.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [armed](armed) module"]
pub type ARMED = crate::Reg<u32, _ARMED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARMED;
#[doc = "`read()` method returns [armed::R](armed::R) reader structure"]
impl crate::Readable for ARMED {}
#[doc = "`write(|w| ..)` method takes [armed::W](armed::W) writer structure"]
impl crate::Writable for ARMED {}
#[doc = "Indicates the armed/disarmed status of each alarm.\\n A write to the corresponding ALARMx register arms the alarm.\\n Alarms automatically disarm upon firing, but writing ones here\\n will disarm immediately without waiting to fire."]
pub mod armed;
#[doc = "Raw read from bits 63:32 of time (no side effects)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timerawh](timerawh) module"]
pub type TIMERAWH = crate::Reg<u32, _TIMERAWH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMERAWH;
#[doc = "`read()` method returns [timerawh::R](timerawh::R) reader structure"]
impl crate::Readable for TIMERAWH {}
#[doc = "Raw read from bits 63:32 of time (no side effects)"]
pub mod timerawh;
#[doc = "Raw read from bits 31:0 of time (no side effects)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timerawl](timerawl) module"]
pub type TIMERAWL = crate::Reg<u32, _TIMERAWL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMERAWL;
#[doc = "`read()` method returns [timerawl::R](timerawl::R) reader structure"]
impl crate::Readable for TIMERAWL {}
#[doc = "Raw read from bits 31:0 of time (no side effects)"]
pub mod timerawl;
#[doc = "Set bits high to enable pause when the corresponding debug ports are active\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgpause](dbgpause) module"]
pub type DBGPAUSE = crate::Reg<u32, _DBGPAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGPAUSE;
#[doc = "`read()` method returns [dbgpause::R](dbgpause::R) reader structure"]
impl crate::Readable for DBGPAUSE {}
#[doc = "`write(|w| ..)` method takes [dbgpause::W](dbgpause::W) writer structure"]
impl crate::Writable for DBGPAUSE {}
#[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
pub mod dbgpause;
#[doc = "Set high to pause the timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pause](pause) module"]
pub type PAUSE = crate::Reg<u32, _PAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAUSE;
#[doc = "`read()` method returns [pause::R](pause::R) reader structure"]
impl crate::Readable for PAUSE {}
#[doc = "`write(|w| ..)` method takes [pause::W](pause::W) writer structure"]
impl crate::Writable for PAUSE {}
#[doc = "Set high to pause the timer"]
pub mod pause;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inte](inte) module"]
pub type INTE = crate::Reg<u32, _INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTE;
#[doc = "`read()` method returns [inte::R](inte::R) reader structure"]
impl crate::Readable for INTE {}
#[doc = "`write(|w| ..)` method takes [inte::W](inte::W) writer structure"]
impl crate::Writable for INTE {}
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "Interrupt Force\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u32, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "`write(|w| ..)` method takes [intf::W](intf::W) writer structure"]
impl crate::Writable for INTF {}
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "Interrupt status after masking & forcing\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ints](ints) module"]
pub type INTS = crate::Reg<u32, _INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTS;
#[doc = "`read()` method returns [ints::R](ints::R) reader structure"]
impl crate::Readable for INTS {}
#[doc = "Interrupt status after masking & forcing"]
pub mod ints;
