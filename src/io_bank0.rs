#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio: [GPIO; 30],
    intr: [INTR; 4],
    proc0_inte: [PROC0_INTE; 4],
    proc0_intf: [PROC0_INTF; 4],
    proc0_ints: [PROC0_INTS; 4],
    proc1_inte: [PROC1_INTE; 4],
    proc1_intf: [PROC1_INTF; 4],
    proc1_ints: [PROC1_INTS; 4],
    dormant_wake_inte: [DORMANT_WAKE_INTE; 4],
    dormant_wake_intf: [DORMANT_WAKE_INTF; 4],
    dormant_wake_ints: [DORMANT_WAKE_INTS; 4],
}
impl RegisterBlock {
    #[doc = "0x00..0xf0 - Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xf0 - Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    #[doc = "0xf0..0x100 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self, n: usize) -> &INTR {
        &self.intr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf0..0x100 - Raw Interrupts"]
    #[inline(always)]
    pub fn intr_iter(&self) -> impl Iterator<Item = &INTR> {
        self.intr.iter()
    }
    #[doc = "0x100..0x110 - Interrupt Enable for proc0"]
    #[inline(always)]
    pub const fn proc0_inte(&self, n: usize) -> &PROC0_INTE {
        &self.proc0_inte[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - Interrupt Enable for proc0"]
    #[inline(always)]
    pub fn proc0_inte_iter(&self) -> impl Iterator<Item = &PROC0_INTE> {
        self.proc0_inte.iter()
    }
    #[doc = "0x110..0x120 - Interrupt Force for proc0"]
    #[inline(always)]
    pub const fn proc0_intf(&self, n: usize) -> &PROC0_INTF {
        &self.proc0_intf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - Interrupt Force for proc0"]
    #[inline(always)]
    pub fn proc0_intf_iter(&self) -> impl Iterator<Item = &PROC0_INTF> {
        self.proc0_intf.iter()
    }
    #[doc = "0x120..0x130 - Interrupt status after masking &amp; forcing for proc0"]
    #[inline(always)]
    pub const fn proc0_ints(&self, n: usize) -> &PROC0_INTS {
        &self.proc0_ints[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x130 - Interrupt status after masking &amp; forcing for proc0"]
    #[inline(always)]
    pub fn proc0_ints_iter(&self) -> impl Iterator<Item = &PROC0_INTS> {
        self.proc0_ints.iter()
    }
    #[doc = "0x130..0x140 - Interrupt Enable for proc1"]
    #[inline(always)]
    pub const fn proc1_inte(&self, n: usize) -> &PROC1_INTE {
        &self.proc1_inte[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x140 - Interrupt Enable for proc1"]
    #[inline(always)]
    pub fn proc1_inte_iter(&self) -> impl Iterator<Item = &PROC1_INTE> {
        self.proc1_inte.iter()
    }
    #[doc = "0x140..0x150 - Interrupt Force for proc1"]
    #[inline(always)]
    pub const fn proc1_intf(&self, n: usize) -> &PROC1_INTF {
        &self.proc1_intf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - Interrupt Force for proc1"]
    #[inline(always)]
    pub fn proc1_intf_iter(&self) -> impl Iterator<Item = &PROC1_INTF> {
        self.proc1_intf.iter()
    }
    #[doc = "0x150..0x160 - Interrupt status after masking &amp; forcing for proc1"]
    #[inline(always)]
    pub const fn proc1_ints(&self, n: usize) -> &PROC1_INTS {
        &self.proc1_ints[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - Interrupt status after masking &amp; forcing for proc1"]
    #[inline(always)]
    pub fn proc1_ints_iter(&self) -> impl Iterator<Item = &PROC1_INTS> {
        self.proc1_ints.iter()
    }
    #[doc = "0x160..0x170 - Interrupt Enable for dormant_wake"]
    #[inline(always)]
    pub const fn dormant_wake_inte(&self, n: usize) -> &DORMANT_WAKE_INTE {
        &self.dormant_wake_inte[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x170 - Interrupt Enable for dormant_wake"]
    #[inline(always)]
    pub fn dormant_wake_inte_iter(&self) -> impl Iterator<Item = &DORMANT_WAKE_INTE> {
        self.dormant_wake_inte.iter()
    }
    #[doc = "0x170..0x180 - Interrupt Force for dormant_wake"]
    #[inline(always)]
    pub const fn dormant_wake_intf(&self, n: usize) -> &DORMANT_WAKE_INTF {
        &self.dormant_wake_intf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x170..0x180 - Interrupt Force for dormant_wake"]
    #[inline(always)]
    pub fn dormant_wake_intf_iter(&self) -> impl Iterator<Item = &DORMANT_WAKE_INTF> {
        self.dormant_wake_intf.iter()
    }
    #[doc = "0x180..0x190 - Interrupt status after masking &amp; forcing for dormant_wake"]
    #[inline(always)]
    pub const fn dormant_wake_ints(&self, n: usize) -> &DORMANT_WAKE_INTS {
        &self.dormant_wake_ints[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x190 - Interrupt status after masking &amp; forcing for dormant_wake"]
    #[inline(always)]
    pub fn dormant_wake_ints_iter(&self) -> impl Iterator<Item = &DORMANT_WAKE_INTS> {
        self.dormant_wake_ints.iter()
    }
}
#[doc = "Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
pub use self::gpio::GPIO;
#[doc = r"Cluster"]
#[doc = "Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
pub mod gpio;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "PROC0_INTE (rw) register accessor: Interrupt Enable for proc0  

You can [`read`](crate::generic::Reg::read) this register and get [`proc0_inte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc0_inte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc0_inte`]
module"]
pub type PROC0_INTE = crate::Reg<proc0_inte::PROC0_INTE_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte;
#[doc = "PROC0_INTF (rw) register accessor: Interrupt Force for proc0  

You can [`read`](crate::generic::Reg::read) this register and get [`proc0_intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc0_intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc0_intf`]
module"]
pub type PROC0_INTF = crate::Reg<proc0_intf::PROC0_INTF_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf;
#[doc = "PROC0_INTS (rw) register accessor: Interrupt status after masking &amp; forcing for proc0  

You can [`read`](crate::generic::Reg::read) this register and get [`proc0_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc0_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc0_ints`]
module"]
pub type PROC0_INTS = crate::Reg<proc0_ints::PROC0_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for proc0"]
pub mod proc0_ints;
#[doc = "PROC1_INTE (rw) register accessor: Interrupt Enable for proc1  

You can [`read`](crate::generic::Reg::read) this register and get [`proc1_inte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc1_inte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc1_inte`]
module"]
pub type PROC1_INTE = crate::Reg<proc1_inte::PROC1_INTE_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte;
#[doc = "PROC1_INTF (rw) register accessor: Interrupt Force for proc1  

You can [`read`](crate::generic::Reg::read) this register and get [`proc1_intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc1_intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc1_intf`]
module"]
pub type PROC1_INTF = crate::Reg<proc1_intf::PROC1_INTF_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf;
#[doc = "PROC1_INTS (rw) register accessor: Interrupt status after masking &amp; forcing for proc1  

You can [`read`](crate::generic::Reg::read) this register and get [`proc1_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc1_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc1_ints`]
module"]
pub type PROC1_INTS = crate::Reg<proc1_ints::PROC1_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for proc1"]
pub mod proc1_ints;
#[doc = "DORMANT_WAKE_INTE (rw) register accessor: Interrupt Enable for dormant_wake  

You can [`read`](crate::generic::Reg::read) this register and get [`dormant_wake_inte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dormant_wake_inte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dormant_wake_inte`]
module"]
pub type DORMANT_WAKE_INTE = crate::Reg<dormant_wake_inte::DORMANT_WAKE_INTE_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte;
#[doc = "DORMANT_WAKE_INTF (rw) register accessor: Interrupt Force for dormant_wake  

You can [`read`](crate::generic::Reg::read) this register and get [`dormant_wake_intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dormant_wake_intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dormant_wake_intf`]
module"]
pub type DORMANT_WAKE_INTF = crate::Reg<dormant_wake_intf::DORMANT_WAKE_INTF_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf;
#[doc = "DORMANT_WAKE_INTS (rw) register accessor: Interrupt status after masking &amp; forcing for dormant_wake  

You can [`read`](crate::generic::Reg::read) this register and get [`dormant_wake_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dormant_wake_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dormant_wake_ints`]
module"]
pub type DORMANT_WAKE_INTS = crate::Reg<dormant_wake_ints::DORMANT_WAKE_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for dormant_wake"]
pub mod dormant_wake_ints;
