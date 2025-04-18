#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    frce_on: FRCE_ON,
    frce_off: FRCE_OFF,
    wdsel: WDSEL,
    done: DONE,
}
impl RegisterBlock {
    #[doc = "0x00 - Force block out of reset (i.e. power it on)"]
    #[inline(always)]
    pub const fn frce_on(&self) -> &FRCE_ON {
        &self.frce_on
    }
    #[doc = "0x04 - Force into reset (i.e. power it off)"]
    #[inline(always)]
    pub const fn frce_off(&self) -> &FRCE_OFF {
        &self.frce_off
    }
    #[doc = "0x08 - Set to 1 if this peripheral should be reset when the watchdog fires."]
    #[inline(always)]
    pub const fn wdsel(&self) -> &WDSEL {
        &self.wdsel
    }
    #[doc = "0x0c - Indicates the peripheral's registers are ready to access."]
    #[inline(always)]
    pub const fn done(&self) -> &DONE {
        &self.done
    }
}
#[doc = "FRCE_ON (rw) register accessor: Force block out of reset (i.e. power it on)  

You can [`read`](crate::generic::Reg::read) this register and get [`frce_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frce_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@frce_on`]
module"]
pub type FRCE_ON = crate::Reg<frce_on::FRCE_ON_SPEC>;
#[doc = "Force block out of reset (i.e. power it on)"]
pub mod frce_on;
#[doc = "FRCE_OFF (rw) register accessor: Force into reset (i.e. power it off)  

You can [`read`](crate::generic::Reg::read) this register and get [`frce_off::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frce_off::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@frce_off`]
module"]
pub type FRCE_OFF = crate::Reg<frce_off::FRCE_OFF_SPEC>;
#[doc = "Force into reset (i.e. power it off)"]
pub mod frce_off;
#[doc = "WDSEL (rw) register accessor: Set to 1 if this peripheral should be reset when the watchdog fires.  

You can [`read`](crate::generic::Reg::read) this register and get [`wdsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@wdsel`]
module"]
pub type WDSEL = crate::Reg<wdsel::WDSEL_SPEC>;
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
pub mod wdsel;
#[doc = "DONE (rw) register accessor: Indicates the peripheral's registers are ready to access.  

You can [`read`](crate::generic::Reg::read) this register and get [`done::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`done::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@done`]
module"]
pub type DONE = crate::Reg<done::DONE_SPEC>;
#[doc = "Indicates the peripheral's registers are ready to access."]
pub mod done;
