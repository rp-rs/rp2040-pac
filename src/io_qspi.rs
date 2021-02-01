#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO status"]
    pub gpio_qspi_sclk_status: GPIO_QSPI_SCLK_STATUS,
    #[doc = "0x04 - GPIO control including function select and overrides."]
    pub gpio_qspi_sclk_ctrl: GPIO_QSPI_SCLK_CTRL,
    #[doc = "0x08 - GPIO status"]
    pub gpio_qspi_ss_status: GPIO_QSPI_SS_STATUS,
    #[doc = "0x0c - GPIO control including function select and overrides."]
    pub gpio_qspi_ss_ctrl: GPIO_QSPI_SS_CTRL,
    #[doc = "0x10 - GPIO status"]
    pub gpio_qspi_sd0_status: GPIO_QSPI_SD0_STATUS,
    #[doc = "0x14 - GPIO control including function select and overrides."]
    pub gpio_qspi_sd0_ctrl: GPIO_QSPI_SD0_CTRL,
    #[doc = "0x18 - GPIO status"]
    pub gpio_qspi_sd1_status: GPIO_QSPI_SD1_STATUS,
    #[doc = "0x1c - GPIO control including function select and overrides."]
    pub gpio_qspi_sd1_ctrl: GPIO_QSPI_SD1_CTRL,
    #[doc = "0x20 - GPIO status"]
    pub gpio_qspi_sd2_status: GPIO_QSPI_SD2_STATUS,
    #[doc = "0x24 - GPIO control including function select and overrides."]
    pub gpio_qspi_sd2_ctrl: GPIO_QSPI_SD2_CTRL,
    #[doc = "0x28 - GPIO status"]
    pub gpio_qspi_sd3_status: GPIO_QSPI_SD3_STATUS,
    #[doc = "0x2c - GPIO control including function select and overrides."]
    pub gpio_qspi_sd3_ctrl: GPIO_QSPI_SD3_CTRL,
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
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sclk_status](gpio_qspi_sclk_status) module"]
pub type GPIO_QSPI_SCLK_STATUS = crate::Reg<u32, _GPIO_QSPI_SCLK_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SCLK_STATUS;
#[doc = "`read()` method returns [gpio_qspi_sclk_status::R](gpio_qspi_sclk_status::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SCLK_STATUS {}
#[doc = "GPIO status"]
pub mod gpio_qspi_sclk_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sclk_ctrl](gpio_qspi_sclk_ctrl) module"]
pub type GPIO_QSPI_SCLK_CTRL = crate::Reg<u32, _GPIO_QSPI_SCLK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SCLK_CTRL;
#[doc = "`read()` method returns [gpio_qspi_sclk_ctrl::R](gpio_qspi_sclk_ctrl::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SCLK_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sclk_ctrl::W](gpio_qspi_sclk_ctrl::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SCLK_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sclk_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_ss_status](gpio_qspi_ss_status) module"]
pub type GPIO_QSPI_SS_STATUS = crate::Reg<u32, _GPIO_QSPI_SS_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SS_STATUS;
#[doc = "`read()` method returns [gpio_qspi_ss_status::R](gpio_qspi_ss_status::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SS_STATUS {}
#[doc = "GPIO status"]
pub mod gpio_qspi_ss_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_ss_ctrl](gpio_qspi_ss_ctrl) module"]
pub type GPIO_QSPI_SS_CTRL = crate::Reg<u32, _GPIO_QSPI_SS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SS_CTRL;
#[doc = "`read()` method returns [gpio_qspi_ss_ctrl::R](gpio_qspi_ss_ctrl::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SS_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_ss_ctrl::W](gpio_qspi_ss_ctrl::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SS_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_ss_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd0_status](gpio_qspi_sd0_status) module"]
pub type GPIO_QSPI_SD0_STATUS = crate::Reg<u32, _GPIO_QSPI_SD0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD0_STATUS;
#[doc = "`read()` method returns [gpio_qspi_sd0_status::R](gpio_qspi_sd0_status::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD0_STATUS {}
#[doc = "GPIO status"]
pub mod gpio_qspi_sd0_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd0_ctrl](gpio_qspi_sd0_ctrl) module"]
pub type GPIO_QSPI_SD0_CTRL = crate::Reg<u32, _GPIO_QSPI_SD0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD0_CTRL;
#[doc = "`read()` method returns [gpio_qspi_sd0_ctrl::R](gpio_qspi_sd0_ctrl::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD0_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd0_ctrl::W](gpio_qspi_sd0_ctrl::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD0_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sd0_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd1_status](gpio_qspi_sd1_status) module"]
pub type GPIO_QSPI_SD1_STATUS = crate::Reg<u32, _GPIO_QSPI_SD1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD1_STATUS;
#[doc = "`read()` method returns [gpio_qspi_sd1_status::R](gpio_qspi_sd1_status::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD1_STATUS {}
#[doc = "GPIO status"]
pub mod gpio_qspi_sd1_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd1_ctrl](gpio_qspi_sd1_ctrl) module"]
pub type GPIO_QSPI_SD1_CTRL = crate::Reg<u32, _GPIO_QSPI_SD1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD1_CTRL;
#[doc = "`read()` method returns [gpio_qspi_sd1_ctrl::R](gpio_qspi_sd1_ctrl::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD1_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd1_ctrl::W](gpio_qspi_sd1_ctrl::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD1_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sd1_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd2_status](gpio_qspi_sd2_status) module"]
pub type GPIO_QSPI_SD2_STATUS = crate::Reg<u32, _GPIO_QSPI_SD2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD2_STATUS;
#[doc = "`read()` method returns [gpio_qspi_sd2_status::R](gpio_qspi_sd2_status::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD2_STATUS {}
#[doc = "GPIO status"]
pub mod gpio_qspi_sd2_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd2_ctrl](gpio_qspi_sd2_ctrl) module"]
pub type GPIO_QSPI_SD2_CTRL = crate::Reg<u32, _GPIO_QSPI_SD2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD2_CTRL;
#[doc = "`read()` method returns [gpio_qspi_sd2_ctrl::R](gpio_qspi_sd2_ctrl::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD2_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd2_ctrl::W](gpio_qspi_sd2_ctrl::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD2_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sd2_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd3_status](gpio_qspi_sd3_status) module"]
pub type GPIO_QSPI_SD3_STATUS = crate::Reg<u32, _GPIO_QSPI_SD3_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD3_STATUS;
#[doc = "`read()` method returns [gpio_qspi_sd3_status::R](gpio_qspi_sd3_status::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD3_STATUS {}
#[doc = "GPIO status"]
pub mod gpio_qspi_sd3_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_sd3_ctrl](gpio_qspi_sd3_ctrl) module"]
pub type GPIO_QSPI_SD3_CTRL = crate::Reg<u32, _GPIO_QSPI_SD3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_SD3_CTRL;
#[doc = "`read()` method returns [gpio_qspi_sd3_ctrl::R](gpio_qspi_sd3_ctrl::R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD3_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd3_ctrl::W](gpio_qspi_sd3_ctrl::W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD3_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_sd3_ctrl;
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
