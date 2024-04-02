#[repr(C)]
#[doc = "Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
pub struct SM_IRQ {
    irq_inte: IRQ_INTE,
    irq_intf: IRQ_INTF,
    irq_ints: IRQ_INTS,
}
impl SM_IRQ {
    #[doc = "0x00 - Interrupt Enable for irq0"]
    #[inline(always)]
    pub const fn irq_inte(&self) -> &IRQ_INTE {
        &self.irq_inte
    }
    #[doc = "0x04 - Interrupt Force for irq0"]
    #[inline(always)]
    pub const fn irq_intf(&self) -> &IRQ_INTF {
        &self.irq_intf
    }
    #[doc = "0x08 - Interrupt status after masking &amp; forcing for irq0"]
    #[inline(always)]
    pub const fn irq_ints(&self) -> &IRQ_INTS {
        &self.irq_ints
    }
}
#[doc = "IRQ_INTE (rw) register accessor: Interrupt Enable for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq_inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq_inte`]
module"]
pub type IRQ_INTE = crate::Reg<irq_inte::IRQ_INTE_SPEC>;
#[doc = "Interrupt Enable for irq0"]
pub mod irq_inte;
#[doc = "IRQ_INTF (rw) register accessor: Interrupt Force for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq_intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq_intf`]
module"]
pub type IRQ_INTF = crate::Reg<irq_intf::IRQ_INTF_SPEC>;
#[doc = "Interrupt Force for irq0"]
pub mod irq_intf;
#[doc = "IRQ_INTS (r) register accessor: Interrupt status after masking &amp; forcing for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq_ints::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq_ints`]
module"]
pub type IRQ_INTS = crate::Reg<irq_ints::IRQ_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for irq0"]
pub mod irq_ints;
