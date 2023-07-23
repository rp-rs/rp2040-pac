#[doc = r"Register block"]
#[repr(C)]
pub struct SM_IRQ {
    #[doc = "0x00 - Interrupt Enable for irq0"]
    pub irq_inte: IRQ_INTE,
    #[doc = "0x04 - Interrupt Force for irq0"]
    pub irq_intf: IRQ_INTF,
    #[doc = "0x08 - Interrupt status after masking & forcing for irq0"]
    pub irq_ints: IRQ_INTS,
}
#[doc = "IRQ_INTE (rw) register accessor: an alias for `Reg<IRQ_INTE_SPEC>`"]
pub type IRQ_INTE = crate::Reg<irq_inte::IRQ_INTE_SPEC>;
#[doc = "Interrupt Enable for irq0"]
pub mod irq_inte;
#[doc = "IRQ_INTF (rw) register accessor: an alias for `Reg<IRQ_INTF_SPEC>`"]
pub type IRQ_INTF = crate::Reg<irq_intf::IRQ_INTF_SPEC>;
#[doc = "Interrupt Force for irq0"]
pub mod irq_intf;
#[doc = "IRQ_INTS (r) register accessor: an alias for `Reg<IRQ_INTS_SPEC>`"]
pub type IRQ_INTS = crate::Reg<irq_ints::IRQ_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for irq0"]
pub mod irq_ints;
