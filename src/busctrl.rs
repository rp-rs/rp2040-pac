#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bus_priority: BUS_PRIORITY,
    bus_priority_ack: BUS_PRIORITY_ACK,
    perfctr0: PERFCTR0,
    perfsel0: PERFSEL0,
    perfctr1: PERFCTR1,
    perfsel1: PERFSEL1,
    perfctr2: PERFCTR2,
    perfsel2: PERFSEL2,
    perfctr3: PERFCTR3,
    perfsel3: PERFSEL3,
}
impl RegisterBlock {
    #[doc = "0x00 - Set the priority of each master for bus arbitration."]
    #[inline(always)]
    pub const fn bus_priority(&self) -> &BUS_PRIORITY {
        &self.bus_priority
    }
    #[doc = "0x04 - Bus priority acknowledge"]
    #[inline(always)]
    pub const fn bus_priority_ack(&self) -> &BUS_PRIORITY_ACK {
        &self.bus_priority_ack
    }
    #[doc = "0x08 - Bus fabric performance counter 0"]
    #[inline(always)]
    pub const fn perfctr0(&self) -> &PERFCTR0 {
        &self.perfctr0
    }
    #[doc = "0x0c - Bus fabric performance event select for PERFCTR0"]
    #[inline(always)]
    pub const fn perfsel0(&self) -> &PERFSEL0 {
        &self.perfsel0
    }
    #[doc = "0x10 - Bus fabric performance counter 1"]
    #[inline(always)]
    pub const fn perfctr1(&self) -> &PERFCTR1 {
        &self.perfctr1
    }
    #[doc = "0x14 - Bus fabric performance event select for PERFCTR1"]
    #[inline(always)]
    pub const fn perfsel1(&self) -> &PERFSEL1 {
        &self.perfsel1
    }
    #[doc = "0x18 - Bus fabric performance counter 2"]
    #[inline(always)]
    pub const fn perfctr2(&self) -> &PERFCTR2 {
        &self.perfctr2
    }
    #[doc = "0x1c - Bus fabric performance event select for PERFCTR2"]
    #[inline(always)]
    pub const fn perfsel2(&self) -> &PERFSEL2 {
        &self.perfsel2
    }
    #[doc = "0x20 - Bus fabric performance counter 3"]
    #[inline(always)]
    pub const fn perfctr3(&self) -> &PERFCTR3 {
        &self.perfctr3
    }
    #[doc = "0x24 - Bus fabric performance event select for PERFCTR3"]
    #[inline(always)]
    pub const fn perfsel3(&self) -> &PERFSEL3 {
        &self.perfsel3
    }
}
#[doc = "BUS_PRIORITY (rw) register accessor: Set the priority of each master for bus arbitration.  

You can [`read`](crate::generic::Reg::read) this register and get [`bus_priority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_priority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bus_priority`]
module"]
pub type BUS_PRIORITY = crate::Reg<bus_priority::BUS_PRIORITY_SPEC>;
#[doc = "Set the priority of each master for bus arbitration."]
pub mod bus_priority;
#[doc = "BUS_PRIORITY_ACK (rw) register accessor: Bus priority acknowledge  

You can [`read`](crate::generic::Reg::read) this register and get [`bus_priority_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_priority_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bus_priority_ack`]
module"]
pub type BUS_PRIORITY_ACK = crate::Reg<bus_priority_ack::BUS_PRIORITY_ACK_SPEC>;
#[doc = "Bus priority acknowledge"]
pub mod bus_priority_ack;
#[doc = "PERFCTR0 (rw) register accessor: Bus fabric performance counter 0  

You can [`read`](crate::generic::Reg::read) this register and get [`perfctr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfctr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@perfctr0`]
module"]
pub type PERFCTR0 = crate::Reg<perfctr0::PERFCTR0_SPEC>;
#[doc = "Bus fabric performance counter 0"]
pub mod perfctr0;
#[doc = "PERFSEL0 (rw) register accessor: Bus fabric performance event select for PERFCTR0  

You can [`read`](crate::generic::Reg::read) this register and get [`perfsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@perfsel0`]
module"]
pub type PERFSEL0 = crate::Reg<perfsel0::PERFSEL0_SPEC>;
#[doc = "Bus fabric performance event select for PERFCTR0"]
pub mod perfsel0;
#[doc = "PERFCTR1 (rw) register accessor: Bus fabric performance counter 1  

You can [`read`](crate::generic::Reg::read) this register and get [`perfctr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfctr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@perfctr1`]
module"]
pub type PERFCTR1 = crate::Reg<perfctr1::PERFCTR1_SPEC>;
#[doc = "Bus fabric performance counter 1"]
pub mod perfctr1;
#[doc = "PERFSEL1 (rw) register accessor: Bus fabric performance event select for PERFCTR1  

You can [`read`](crate::generic::Reg::read) this register and get [`perfsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@perfsel1`]
module"]
pub type PERFSEL1 = crate::Reg<perfsel1::PERFSEL1_SPEC>;
#[doc = "Bus fabric performance event select for PERFCTR1"]
pub mod perfsel1;
#[doc = "PERFCTR2 (rw) register accessor: Bus fabric performance counter 2  

You can [`read`](crate::generic::Reg::read) this register and get [`perfctr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfctr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@perfctr2`]
module"]
pub type PERFCTR2 = crate::Reg<perfctr2::PERFCTR2_SPEC>;
#[doc = "Bus fabric performance counter 2"]
pub mod perfctr2;
#[doc = "PERFSEL2 (rw) register accessor: Bus fabric performance event select for PERFCTR2  

You can [`read`](crate::generic::Reg::read) this register and get [`perfsel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfsel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@perfsel2`]
module"]
pub type PERFSEL2 = crate::Reg<perfsel2::PERFSEL2_SPEC>;
#[doc = "Bus fabric performance event select for PERFCTR2"]
pub mod perfsel2;
#[doc = "PERFCTR3 (rw) register accessor: Bus fabric performance counter 3  

You can [`read`](crate::generic::Reg::read) this register and get [`perfctr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfctr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@perfctr3`]
module"]
pub type PERFCTR3 = crate::Reg<perfctr3::PERFCTR3_SPEC>;
#[doc = "Bus fabric performance counter 3"]
pub mod perfctr3;
#[doc = "PERFSEL3 (rw) register accessor: Bus fabric performance event select for PERFCTR3  

You can [`read`](crate::generic::Reg::read) this register and get [`perfsel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfsel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@perfsel3`]
module"]
pub type PERFSEL3 = crate::Reg<perfsel3::PERFSEL3_SPEC>;
#[doc = "Bus fabric performance event select for PERFCTR3"]
pub mod perfsel3;
