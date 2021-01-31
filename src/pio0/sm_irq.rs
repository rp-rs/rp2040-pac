#[doc = "Interrupt Enable for irq0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_inte](irq_inte) module"]
pub type IRQ_INTE = crate::Reg<u32, _IRQ_INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_INTE;
#[doc = "`read()` method returns [irq_inte::R](irq_inte::R) reader structure"]
impl crate::Readable for IRQ_INTE {}
#[doc = "`write(|w| ..)` method takes [irq_inte::W](irq_inte::W) writer structure"]
impl crate::Writable for IRQ_INTE {}
#[doc = "Interrupt Enable for irq0"]
pub mod irq_inte;
#[doc = "Interrupt Force for irq0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_intf](irq_intf) module"]
pub type IRQ_INTF = crate::Reg<u32, _IRQ_INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_INTF;
#[doc = "`read()` method returns [irq_intf::R](irq_intf::R) reader structure"]
impl crate::Readable for IRQ_INTF {}
#[doc = "`write(|w| ..)` method takes [irq_intf::W](irq_intf::W) writer structure"]
impl crate::Writable for IRQ_INTF {}
#[doc = "Interrupt Force for irq0"]
pub mod irq_intf;
#[doc = "Interrupt status after masking & forcing for irq0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_ints](irq_ints) module"]
pub type IRQ_INTS = crate::Reg<u32, _IRQ_INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_INTS;
#[doc = "`read()` method returns [irq_ints::R](irq_ints::R) reader structure"]
impl crate::Readable for IRQ_INTS {}
#[doc = "Interrupt status after masking & forcing for irq0"]
pub mod irq_ints;
