#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    load: LOAD,
    reason: REASON,
    scratch0: SCRATCH0,
    scratch1: SCRATCH1,
    scratch2: SCRATCH2,
    scratch3: SCRATCH3,
    scratch4: SCRATCH4,
    scratch5: SCRATCH5,
    scratch6: SCRATCH6,
    scratch7: SCRATCH7,
    tick: TICK,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
    #[inline(always)]
    pub const fn load(&self) -> &LOAD {
        &self.load
    }
    #[doc = "0x08 - Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
    #[inline(always)]
    pub const fn reason(&self) -> &REASON {
        &self.reason
    }
    #[doc = "0x0c - Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch0(&self) -> &SCRATCH0 {
        &self.scratch0
    }
    #[doc = "0x10 - Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch1(&self) -> &SCRATCH1 {
        &self.scratch1
    }
    #[doc = "0x14 - Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch2(&self) -> &SCRATCH2 {
        &self.scratch2
    }
    #[doc = "0x18 - Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch3(&self) -> &SCRATCH3 {
        &self.scratch3
    }
    #[doc = "0x1c - Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch4(&self) -> &SCRATCH4 {
        &self.scratch4
    }
    #[doc = "0x20 - Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch5(&self) -> &SCRATCH5 {
        &self.scratch5
    }
    #[doc = "0x24 - Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch6(&self) -> &SCRATCH6 {
        &self.scratch6
    }
    #[doc = "0x28 - Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch7(&self) -> &SCRATCH7 {
        &self.scratch7
    }
    #[doc = "0x2c - Controls the tick generator"]
    #[inline(always)]
    pub const fn tick(&self) -> &TICK {
        &self.tick
    }
}
#[doc = "CTRL (rw) register accessor: Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software.  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software."]
pub mod ctrl;
#[doc = "LOAD (rw) register accessor: Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1).  

You can [`read`](crate::generic::Reg::read) this register and get [`load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@load`]
module"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
pub mod load;
#[doc = "REASON (rw) register accessor: Logs the reason for the last reset. Both bits are zero for the case of a hardware reset.  

You can [`read`](crate::generic::Reg::read) this register and get [`reason::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reason::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@reason`]
module"]
pub type REASON = crate::Reg<reason::REASON_SPEC>;
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
pub mod reason;
#[doc = "SCRATCH0 (rw) register accessor: Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch0`]
module"]
pub type SCRATCH0 = crate::Reg<scratch0::SCRATCH0_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch0;
#[doc = "SCRATCH1 (rw) register accessor: Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch1`]
module"]
pub type SCRATCH1 = crate::Reg<scratch1::SCRATCH1_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch1;
#[doc = "SCRATCH2 (rw) register accessor: Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch2`]
module"]
pub type SCRATCH2 = crate::Reg<scratch2::SCRATCH2_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch2;
#[doc = "SCRATCH3 (rw) register accessor: Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch3`]
module"]
pub type SCRATCH3 = crate::Reg<scratch3::SCRATCH3_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch3;
#[doc = "SCRATCH4 (rw) register accessor: Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch4`]
module"]
pub type SCRATCH4 = crate::Reg<scratch4::SCRATCH4_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch4;
#[doc = "SCRATCH5 (rw) register accessor: Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch5`]
module"]
pub type SCRATCH5 = crate::Reg<scratch5::SCRATCH5_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch5;
#[doc = "SCRATCH6 (rw) register accessor: Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch6`]
module"]
pub type SCRATCH6 = crate::Reg<scratch6::SCRATCH6_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch6;
#[doc = "SCRATCH7 (rw) register accessor: Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch7`]
module"]
pub type SCRATCH7 = crate::Reg<scratch7::SCRATCH7_SPEC>;
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch7;
#[doc = "TICK (rw) register accessor: Controls the tick generator  

You can [`read`](crate::generic::Reg::read) this register and get [`tick::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tick::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tick`]
module"]
pub type TICK = crate::Reg<tick::TICK_SPEC>;
#[doc = "Controls the tick generator"]
pub mod tick;
