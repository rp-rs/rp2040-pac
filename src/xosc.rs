#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    dormant: DORMANT,
    startup: STARTUP,
}
impl RegisterBlock {
    #[doc = "0x00 - Crystal Oscillator Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Crystal Oscillator Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Crystal Oscillator pause control"]
    #[inline(always)]
    pub const fn dormant(&self) -> &DORMANT {
        &self.dormant
    }
    #[doc = "0x0c - Controls the startup delay"]
    #[inline(always)]
    pub const fn startup(&self) -> &STARTUP {
        &self.startup
    }
}
#[doc = "CTRL (rw) register accessor: Crystal Oscillator Control  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Crystal Oscillator Control"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Crystal Oscillator Status  

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Crystal Oscillator Status"]
pub mod status;
#[doc = "DORMANT (rw) register accessor: Crystal Oscillator pause control  

You can [`read`](crate::generic::Reg::read) this register and get [`dormant::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dormant::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dormant`]
module"]
pub type DORMANT = crate::Reg<dormant::DORMANT_SPEC>;
#[doc = "Crystal Oscillator pause control"]
pub mod dormant;
#[doc = "STARTUP (rw) register accessor: Controls the startup delay  

You can [`read`](crate::generic::Reg::read) this register and get [`startup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@startup`]
module"]
pub type STARTUP = crate::Reg<startup::STARTUP_SPEC>;
#[doc = "Controls the startup delay"]
pub mod startup;
