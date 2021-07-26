#[doc = "IRQ_INTE register accessor: an alias for `Reg<IRQ_INTE_SPEC>`"]
pub type IRQ_INTE = crate::Reg<irq_inte::IRQ_INTE_SPEC>;
#[doc = "Interrupt Enable for irq0"]
pub mod irq_inte;
#[doc = "IRQ_INTF register accessor: an alias for `Reg<IRQ_INTF_SPEC>`"]
pub type IRQ_INTF = crate::Reg<irq_intf::IRQ_INTF_SPEC>;
#[doc = "Interrupt Force for irq0"]
pub mod irq_intf;
#[doc = "IRQ_INTS register accessor: an alias for `Reg<IRQ_INTS_SPEC>`"]
pub type IRQ_INTS = crate::Reg<irq_ints::IRQ_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for irq0"]
pub mod irq_ints;
