#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clkdiv_m1: CLKDIV_M1,
    setup_0: SETUP_0,
    setup_1: SETUP_1,
    ctrl: CTRL,
    irq_setup_0: IRQ_SETUP_0,
    irq_setup_1: IRQ_SETUP_1,
    rtc_1: RTC_1,
    rtc_0: RTC_0,
    intr: INTR,
    inte: INTE,
    intf: INTF,
    ints: INTS,
}
impl RegisterBlock {
    #[doc = "0x00 - Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
    #[inline(always)]
    pub const fn clkdiv_m1(&self) -> &CLKDIV_M1 {
        &self.clkdiv_m1
    }
    #[doc = "0x04 - RTC setup register 0"]
    #[inline(always)]
    pub const fn setup_0(&self) -> &SETUP_0 {
        &self.setup_0
    }
    #[doc = "0x08 - RTC setup register 1"]
    #[inline(always)]
    pub const fn setup_1(&self) -> &SETUP_1 {
        &self.setup_1
    }
    #[doc = "0x0c - RTC Control and status"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x10 - Interrupt setup register 0"]
    #[inline(always)]
    pub const fn irq_setup_0(&self) -> &IRQ_SETUP_0 {
        &self.irq_setup_0
    }
    #[doc = "0x14 - Interrupt setup register 1"]
    #[inline(always)]
    pub const fn irq_setup_1(&self) -> &IRQ_SETUP_1 {
        &self.irq_setup_1
    }
    #[doc = "0x18 - RTC register 1."]
    #[inline(always)]
    pub const fn rtc_1(&self) -> &RTC_1 {
        &self.rtc_1
    }
    #[doc = "0x1c - RTC register 0 Read this before RTC 1!"]
    #[inline(always)]
    pub const fn rtc_0(&self) -> &RTC_0 {
        &self.rtc_0
    }
    #[doc = "0x20 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x24 - Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0x28 - Interrupt Force"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0x2c - Interrupt status after masking &amp; forcing"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
}
#[doc = "CLKDIV_M1 (rw) register accessor: Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled.  

You can [`read`](crate::generic::Reg::read) this register and get [`clkdiv_m1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv_m1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clkdiv_m1`]
module"]
pub type CLKDIV_M1 = crate::Reg<clkdiv_m1::CLKDIV_M1_SPEC>;
#[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
pub mod clkdiv_m1;
#[doc = "SETUP_0 (rw) register accessor: RTC setup register 0  

You can [`read`](crate::generic::Reg::read) this register and get [`setup_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@setup_0`]
module"]
pub type SETUP_0 = crate::Reg<setup_0::SETUP_0_SPEC>;
#[doc = "RTC setup register 0"]
pub mod setup_0;
#[doc = "SETUP_1 (rw) register accessor: RTC setup register 1  

You can [`read`](crate::generic::Reg::read) this register and get [`setup_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@setup_1`]
module"]
pub type SETUP_1 = crate::Reg<setup_1::SETUP_1_SPEC>;
#[doc = "RTC setup register 1"]
pub mod setup_1;
#[doc = "CTRL (rw) register accessor: RTC Control and status  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC Control and status"]
pub mod ctrl;
#[doc = "IRQ_SETUP_0 (rw) register accessor: Interrupt setup register 0  

You can [`read`](crate::generic::Reg::read) this register and get [`irq_setup_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_setup_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq_setup_0`]
module"]
pub type IRQ_SETUP_0 = crate::Reg<irq_setup_0::IRQ_SETUP_0_SPEC>;
#[doc = "Interrupt setup register 0"]
pub mod irq_setup_0;
#[doc = "IRQ_SETUP_1 (rw) register accessor: Interrupt setup register 1  

You can [`read`](crate::generic::Reg::read) this register and get [`irq_setup_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_setup_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq_setup_1`]
module"]
pub type IRQ_SETUP_1 = crate::Reg<irq_setup_1::IRQ_SETUP_1_SPEC>;
#[doc = "Interrupt setup register 1"]
pub mod irq_setup_1;
#[doc = "RTC_1 (rw) register accessor: RTC register 1.  

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rtc_1`]
module"]
pub type RTC_1 = crate::Reg<rtc_1::RTC_1_SPEC>;
#[doc = "RTC register 1."]
pub mod rtc_1;
#[doc = "RTC_0 (rw) register accessor: RTC register 0 Read this before RTC 1!  

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rtc_0`]
module"]
pub type RTC_0 = crate::Reg<rtc_0::RTC_0_SPEC>;
#[doc = "RTC register 0 Read this before RTC 1!"]
pub mod rtc_0;
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
