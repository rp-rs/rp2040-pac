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
#[doc = "Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](reset) module"]
pub type RESET = crate::Reg<u32, _RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET;
#[doc = "`read()` method returns [reset::R](reset::R) reader structure"]
impl crate::Readable for RESET {}
#[doc = "`write(|w| ..)` method takes [reset::W](reset::W) writer structure"]
impl crate::Writable for RESET {}
#[doc = "Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted."]
pub mod reset;
#[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdsel](wdsel) module"]
pub type WDSEL = crate::Reg<u32, _WDSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDSEL;
#[doc = "`read()` method returns [wdsel::R](wdsel::R) reader structure"]
impl crate::Readable for WDSEL {}
#[doc = "`write(|w| ..)` method takes [wdsel::W](wdsel::W) writer structure"]
impl crate::Writable for WDSEL {}
#[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
pub mod wdsel;
#[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_done](reset_done) module"]
pub type RESET_DONE = crate::Reg<u32, _RESET_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_DONE;
#[doc = "`read()` method returns [reset_done::R](reset_done::R) reader structure"]
impl crate::Readable for RESET_DONE {}
#[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
pub mod reset_done;
