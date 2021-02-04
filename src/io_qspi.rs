#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    pub gpio_qspisclk: GPIO_QSPI,
    #[doc = "0x08 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    pub gpio_qspiss: GPIO_QSPI,
    #[doc = "0x10 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    pub gpio_qspisd0: GPIO_QSPI,
    #[doc = "0x18 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    pub gpio_qspisd1: GPIO_QSPI,
    #[doc = "0x20 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    pub gpio_qspisd2: GPIO_QSPI,
    #[doc = "0x28 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    pub gpio_qspisd3: GPIO_QSPI,
    #[doc = "0x30 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0x34 - Interrupt Enable for proc0"]
    pub proc0_inte: PROC0_INTE,
    #[doc = "0x38 - Interrupt Force for proc0"]
    pub proc0_intf: PROC0_INTF,
    #[doc = "0x3c - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints: PROC0_INTS,
    #[doc = "0x40 - Interrupt Enable for proc1"]
    pub proc1_inte: PROC1_INTE,
    #[doc = "0x44 - Interrupt Force for proc1"]
    pub proc1_intf: PROC1_INTF,
    #[doc = "0x48 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints: PROC1_INTS,
    #[doc = "0x4c - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte: DORMANT_WAKE_INTE,
    #[doc = "0x50 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf: DORMANT_WAKE_INTF,
    #[doc = "0x54 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints: DORMANT_WAKE_INTS,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO_QSPI {
    #[doc = "0x00 - GPIO status"]
    pub gpio_status: self::gpio_qspi::GPIO_STATUS,
    #[doc = "0x04 - GPIO control including function select and overrides."]
    pub gpio_ctrl: self::gpio_qspi::GPIO_CTRL,
}
#[doc = r"Register block"]
#[doc = "Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
pub mod gpio_qspi;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "Interrupt Enable for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_inte](proc0_inte) module"]
pub type PROC0_INTE = crate::Reg<u32, _PROC0_INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTE;
#[doc = "`read()` method returns [proc0_inte::R](proc0_inte::R) reader structure"]
impl crate::Readable for PROC0_INTE {}
#[doc = "`write(|w| ..)` method takes [proc0_inte::W](proc0_inte::W) writer structure"]
impl crate::Writable for PROC0_INTE {}
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte;
#[doc = "Interrupt Force for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_intf](proc0_intf) module"]
pub type PROC0_INTF = crate::Reg<u32, _PROC0_INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTF;
#[doc = "`read()` method returns [proc0_intf::R](proc0_intf::R) reader structure"]
impl crate::Readable for PROC0_INTF {}
#[doc = "`write(|w| ..)` method takes [proc0_intf::W](proc0_intf::W) writer structure"]
impl crate::Writable for PROC0_INTF {}
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf;
#[doc = "Interrupt status after masking & forcing for proc0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_ints](proc0_ints) module"]
pub type PROC0_INTS = crate::Reg<u32, _PROC0_INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTS;
#[doc = "`read()` method returns [proc0_ints::R](proc0_ints::R) reader structure"]
impl crate::Readable for PROC0_INTS {}
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints;
#[doc = "Interrupt Enable for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_inte](proc1_inte) module"]
pub type PROC1_INTE = crate::Reg<u32, _PROC1_INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTE;
#[doc = "`read()` method returns [proc1_inte::R](proc1_inte::R) reader structure"]
impl crate::Readable for PROC1_INTE {}
#[doc = "`write(|w| ..)` method takes [proc1_inte::W](proc1_inte::W) writer structure"]
impl crate::Writable for PROC1_INTE {}
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte;
#[doc = "Interrupt Force for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_intf](proc1_intf) module"]
pub type PROC1_INTF = crate::Reg<u32, _PROC1_INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTF;
#[doc = "`read()` method returns [proc1_intf::R](proc1_intf::R) reader structure"]
impl crate::Readable for PROC1_INTF {}
#[doc = "`write(|w| ..)` method takes [proc1_intf::W](proc1_intf::W) writer structure"]
impl crate::Writable for PROC1_INTF {}
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf;
#[doc = "Interrupt status after masking & forcing for proc1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_ints](proc1_ints) module"]
pub type PROC1_INTS = crate::Reg<u32, _PROC1_INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTS;
#[doc = "`read()` method returns [proc1_ints::R](proc1_ints::R) reader structure"]
impl crate::Readable for PROC1_INTS {}
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints;
#[doc = "Interrupt Enable for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_inte](dormant_wake_inte) module"]
pub type DORMANT_WAKE_INTE = crate::Reg<u32, _DORMANT_WAKE_INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTE;
#[doc = "`read()` method returns [dormant_wake_inte::R](dormant_wake_inte::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTE {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_inte::W](dormant_wake_inte::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTE {}
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte;
#[doc = "Interrupt Force for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_intf](dormant_wake_intf) module"]
pub type DORMANT_WAKE_INTF = crate::Reg<u32, _DORMANT_WAKE_INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTF;
#[doc = "`read()` method returns [dormant_wake_intf::R](dormant_wake_intf::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTF {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_intf::W](dormant_wake_intf::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTF {}
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf;
#[doc = "Interrupt status after masking & forcing for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_ints](dormant_wake_ints) module"]
pub type DORMANT_WAKE_INTS = crate::Reg<u32, _DORMANT_WAKE_INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTS;
#[doc = "`read()` method returns [dormant_wake_ints::R](dormant_wake_ints::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTS {}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints;
