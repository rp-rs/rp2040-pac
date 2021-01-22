#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Force block out of reset (i.e. power it on)"]
    pub frce_on: FRCE_ON,
    #[doc = "0x04 - Force into reset (i.e. power it off)"]
    pub frce_off: FRCE_OFF,
    #[doc = "0x08 - Set to 1 if this peripheral should be reset when the watchdog fires."]
    pub wdsel: WDSEL,
    #[doc = "0x0c - Indicates the peripheral's registers are ready to access."]
    pub done: DONE,
}
#[doc = "Force block out of reset (i.e. power it on)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frce_on](frce_on) module"]
pub type FRCE_ON = crate::Reg<u32, _FRCE_ON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRCE_ON;
#[doc = "`read()` method returns [frce_on::R](frce_on::R) reader structure"]
impl crate::Readable for FRCE_ON {}
#[doc = "`write(|w| ..)` method takes [frce_on::W](frce_on::W) writer structure"]
impl crate::Writable for FRCE_ON {}
#[doc = "Force block out of reset (i.e. power it on)"]
pub mod frce_on;
#[doc = "Force into reset (i.e. power it off)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frce_off](frce_off) module"]
pub type FRCE_OFF = crate::Reg<u32, _FRCE_OFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRCE_OFF;
#[doc = "`read()` method returns [frce_off::R](frce_off::R) reader structure"]
impl crate::Readable for FRCE_OFF {}
#[doc = "`write(|w| ..)` method takes [frce_off::W](frce_off::W) writer structure"]
impl crate::Writable for FRCE_OFF {}
#[doc = "Force into reset (i.e. power it off)"]
pub mod frce_off;
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdsel](wdsel) module"]
pub type WDSEL = crate::Reg<u32, _WDSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDSEL;
#[doc = "`read()` method returns [wdsel::R](wdsel::R) reader structure"]
impl crate::Readable for WDSEL {}
#[doc = "`write(|w| ..)` method takes [wdsel::W](wdsel::W) writer structure"]
impl crate::Writable for WDSEL {}
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
pub mod wdsel;
#[doc = "Indicates the peripheral's registers are ready to access.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [done](done) module"]
pub type DONE = crate::Reg<u32, _DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DONE;
#[doc = "`read()` method returns [done::R](done::R) reader structure"]
impl crate::Readable for DONE {}
#[doc = "Indicates the peripheral's registers are ready to access."]
pub mod done;
