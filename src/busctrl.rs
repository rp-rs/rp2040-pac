#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Set the priority of each master for bus arbitration."]
    pub bus_priority: crate::Reg<bus_priority::BUS_PRIORITY_SPEC>,
    #[doc = "0x04 - Bus priority acknowledge"]
    pub bus_priority_ack: crate::Reg<bus_priority_ack::BUS_PRIORITY_ACK_SPEC>,
    #[doc = "0x08 - Bus fabric performance counter 0"]
    pub perfctr0: crate::Reg<perfctr0::PERFCTR0_SPEC>,
    #[doc = "0x0c - Bus fabric performance event select for PERFCTR0"]
    pub perfsel0: crate::Reg<perfsel0::PERFSEL0_SPEC>,
    #[doc = "0x10 - Bus fabric performance counter 1"]
    pub perfctr1: crate::Reg<perfctr1::PERFCTR1_SPEC>,
    #[doc = "0x14 - Bus fabric performance event select for PERFCTR1"]
    pub perfsel1: crate::Reg<perfsel1::PERFSEL1_SPEC>,
    #[doc = "0x18 - Bus fabric performance counter 2"]
    pub perfctr2: crate::Reg<perfctr2::PERFCTR2_SPEC>,
    #[doc = "0x1c - Bus fabric performance event select for PERFCTR2"]
    pub perfsel2: crate::Reg<perfsel2::PERFSEL2_SPEC>,
    #[doc = "0x20 - Bus fabric performance counter 3"]
    pub perfctr3: crate::Reg<perfctr3::PERFCTR3_SPEC>,
    #[doc = "0x24 - Bus fabric performance event select for PERFCTR3"]
    pub perfsel3: crate::Reg<perfsel3::PERFSEL3_SPEC>,
}
#[doc = "BUS_PRIORITY register accessor: an alias for `Reg<BUS_PRIORITY_SPEC>`"]
pub type BUS_PRIORITY = crate::Reg<bus_priority::BUS_PRIORITY_SPEC>;
#[doc = "Set the priority of each master for bus arbitration."]
pub mod bus_priority;
#[doc = "BUS_PRIORITY_ACK register accessor: an alias for `Reg<BUS_PRIORITY_ACK_SPEC>`"]
pub type BUS_PRIORITY_ACK = crate::Reg<bus_priority_ack::BUS_PRIORITY_ACK_SPEC>;
#[doc = "Bus priority acknowledge"]
pub mod bus_priority_ack;
#[doc = "PERFCTR0 register accessor: an alias for `Reg<PERFCTR0_SPEC>`"]
pub type PERFCTR0 = crate::Reg<perfctr0::PERFCTR0_SPEC>;
#[doc = "Bus fabric performance counter 0"]
pub mod perfctr0;
#[doc = "PERFSEL0 register accessor: an alias for `Reg<PERFSEL0_SPEC>`"]
pub type PERFSEL0 = crate::Reg<perfsel0::PERFSEL0_SPEC>;
#[doc = "Bus fabric performance event select for PERFCTR0"]
pub mod perfsel0;
#[doc = "PERFCTR1 register accessor: an alias for `Reg<PERFCTR1_SPEC>`"]
pub type PERFCTR1 = crate::Reg<perfctr1::PERFCTR1_SPEC>;
#[doc = "Bus fabric performance counter 1"]
pub mod perfctr1;
#[doc = "PERFSEL1 register accessor: an alias for `Reg<PERFSEL1_SPEC>`"]
pub type PERFSEL1 = crate::Reg<perfsel1::PERFSEL1_SPEC>;
#[doc = "Bus fabric performance event select for PERFCTR1"]
pub mod perfsel1;
#[doc = "PERFCTR2 register accessor: an alias for `Reg<PERFCTR2_SPEC>`"]
pub type PERFCTR2 = crate::Reg<perfctr2::PERFCTR2_SPEC>;
#[doc = "Bus fabric performance counter 2"]
pub mod perfctr2;
#[doc = "PERFSEL2 register accessor: an alias for `Reg<PERFSEL2_SPEC>`"]
pub type PERFSEL2 = crate::Reg<perfsel2::PERFSEL2_SPEC>;
#[doc = "Bus fabric performance event select for PERFCTR2"]
pub mod perfsel2;
#[doc = "PERFCTR3 register accessor: an alias for `Reg<PERFCTR3_SPEC>`"]
pub type PERFCTR3 = crate::Reg<perfctr3::PERFCTR3_SPEC>;
#[doc = "Bus fabric performance counter 3"]
pub mod perfctr3;
#[doc = "PERFSEL3 register accessor: an alias for `Reg<PERFSEL3_SPEC>`"]
pub type PERFSEL3 = crate::Reg<perfsel3::PERFSEL3_SPEC>;
#[doc = "Bus fabric performance event select for PERFCTR3"]
pub mod perfsel3;
