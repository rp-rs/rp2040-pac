#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted."]
    pub reset: RESET,
    #[doc = "0x04 - Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
    pub wdsel: WDSEL,
    #[doc = "0x08 - Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
    pub reset_done: RESET_DONE,
}
#[doc = "RESET (rw) register accessor: Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted.  

You can [`read`](crate::generic::Reg::read) this register and get [`reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@reset`]
module"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted."]
pub mod reset;
#[doc = "WDSEL (rw) register accessor: Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires.  

You can [`read`](crate::generic::Reg::read) this register and get [`wdsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@wdsel`]
module"]
pub type WDSEL = crate::Reg<wdsel::WDSEL_SPEC>;
#[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
pub mod wdsel;
#[doc = "RESET_DONE (r) register accessor: Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed.  

You can [`read`](crate::generic::Reg::read) this register and get [`reset_done::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@reset_done`]
module"]
pub type RESET_DONE = crate::Reg<reset_done::RESET_DONE_SPEC>;
#[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
pub mod reset_done;
