#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO status"]
    pub gpio0_status: GPIO0_STATUS,
    #[doc = "0x04 - GPIO control including function select and overrides."]
    pub gpio0_ctrl: GPIO0_CTRL,
    #[doc = "0x08 - GPIO status"]
    pub gpio1_status: GPIO1_STATUS,
    #[doc = "0x0c - GPIO control including function select and overrides."]
    pub gpio1_ctrl: GPIO1_CTRL,
    #[doc = "0x10 - GPIO status"]
    pub gpio2_status: GPIO2_STATUS,
    #[doc = "0x14 - GPIO control including function select and overrides."]
    pub gpio2_ctrl: GPIO2_CTRL,
    #[doc = "0x18 - GPIO status"]
    pub gpio3_status: GPIO3_STATUS,
    #[doc = "0x1c - GPIO control including function select and overrides."]
    pub gpio3_ctrl: GPIO3_CTRL,
    #[doc = "0x20 - GPIO status"]
    pub gpio4_status: GPIO4_STATUS,
    #[doc = "0x24 - GPIO control including function select and overrides."]
    pub gpio4_ctrl: GPIO4_CTRL,
    #[doc = "0x28 - GPIO status"]
    pub gpio5_status: GPIO5_STATUS,
    #[doc = "0x2c - GPIO control including function select and overrides."]
    pub gpio5_ctrl: GPIO5_CTRL,
    #[doc = "0x30 - GPIO status"]
    pub gpio6_status: GPIO6_STATUS,
    #[doc = "0x34 - GPIO control including function select and overrides."]
    pub gpio6_ctrl: GPIO6_CTRL,
    #[doc = "0x38 - GPIO status"]
    pub gpio7_status: GPIO7_STATUS,
    #[doc = "0x3c - GPIO control including function select and overrides."]
    pub gpio7_ctrl: GPIO7_CTRL,
    #[doc = "0x40 - GPIO status"]
    pub gpio8_status: GPIO8_STATUS,
    #[doc = "0x44 - GPIO control including function select and overrides."]
    pub gpio8_ctrl: GPIO8_CTRL,
    #[doc = "0x48 - GPIO status"]
    pub gpio9_status: GPIO9_STATUS,
    #[doc = "0x4c - GPIO control including function select and overrides."]
    pub gpio9_ctrl: GPIO9_CTRL,
    #[doc = "0x50 - GPIO status"]
    pub gpio10_status: GPIO10_STATUS,
    #[doc = "0x54 - GPIO control including function select and overrides."]
    pub gpio10_ctrl: GPIO10_CTRL,
    #[doc = "0x58 - GPIO status"]
    pub gpio11_status: GPIO11_STATUS,
    #[doc = "0x5c - GPIO control including function select and overrides."]
    pub gpio11_ctrl: GPIO11_CTRL,
    #[doc = "0x60 - GPIO status"]
    pub gpio12_status: GPIO12_STATUS,
    #[doc = "0x64 - GPIO control including function select and overrides."]
    pub gpio12_ctrl: GPIO12_CTRL,
    #[doc = "0x68 - GPIO status"]
    pub gpio13_status: GPIO13_STATUS,
    #[doc = "0x6c - GPIO control including function select and overrides."]
    pub gpio13_ctrl: GPIO13_CTRL,
    #[doc = "0x70 - GPIO status"]
    pub gpio14_status: GPIO14_STATUS,
    #[doc = "0x74 - GPIO control including function select and overrides."]
    pub gpio14_ctrl: GPIO14_CTRL,
    #[doc = "0x78 - GPIO status"]
    pub gpio15_status: GPIO15_STATUS,
    #[doc = "0x7c - GPIO control including function select and overrides."]
    pub gpio15_ctrl: GPIO15_CTRL,
    #[doc = "0x80 - GPIO status"]
    pub gpio16_status: GPIO16_STATUS,
    #[doc = "0x84 - GPIO control including function select and overrides."]
    pub gpio16_ctrl: GPIO16_CTRL,
    #[doc = "0x88 - GPIO status"]
    pub gpio17_status: GPIO17_STATUS,
    #[doc = "0x8c - GPIO control including function select and overrides."]
    pub gpio17_ctrl: GPIO17_CTRL,
    #[doc = "0x90 - GPIO status"]
    pub gpio18_status: GPIO18_STATUS,
    #[doc = "0x94 - GPIO control including function select and overrides."]
    pub gpio18_ctrl: GPIO18_CTRL,
    #[doc = "0x98 - GPIO status"]
    pub gpio19_status: GPIO19_STATUS,
    #[doc = "0x9c - GPIO control including function select and overrides."]
    pub gpio19_ctrl: GPIO19_CTRL,
    #[doc = "0xa0 - GPIO status"]
    pub gpio20_status: GPIO20_STATUS,
    #[doc = "0xa4 - GPIO control including function select and overrides."]
    pub gpio20_ctrl: GPIO20_CTRL,
    #[doc = "0xa8 - GPIO status"]
    pub gpio21_status: GPIO21_STATUS,
    #[doc = "0xac - GPIO control including function select and overrides."]
    pub gpio21_ctrl: GPIO21_CTRL,
    #[doc = "0xb0 - GPIO status"]
    pub gpio22_status: GPIO22_STATUS,
    #[doc = "0xb4 - GPIO control including function select and overrides."]
    pub gpio22_ctrl: GPIO22_CTRL,
    #[doc = "0xb8 - GPIO status"]
    pub gpio23_status: GPIO23_STATUS,
    #[doc = "0xbc - GPIO control including function select and overrides."]
    pub gpio23_ctrl: GPIO23_CTRL,
    #[doc = "0xc0 - GPIO status"]
    pub gpio24_status: GPIO24_STATUS,
    #[doc = "0xc4 - GPIO control including function select and overrides."]
    pub gpio24_ctrl: GPIO24_CTRL,
    #[doc = "0xc8 - GPIO status"]
    pub gpio25_status: GPIO25_STATUS,
    #[doc = "0xcc - GPIO control including function select and overrides."]
    pub gpio25_ctrl: GPIO25_CTRL,
    #[doc = "0xd0 - GPIO status"]
    pub gpio26_status: GPIO26_STATUS,
    #[doc = "0xd4 - GPIO control including function select and overrides."]
    pub gpio26_ctrl: GPIO26_CTRL,
    #[doc = "0xd8 - GPIO status"]
    pub gpio27_status: GPIO27_STATUS,
    #[doc = "0xdc - GPIO control including function select and overrides."]
    pub gpio27_ctrl: GPIO27_CTRL,
    #[doc = "0xe0 - GPIO status"]
    pub gpio28_status: GPIO28_STATUS,
    #[doc = "0xe4 - GPIO control including function select and overrides."]
    pub gpio28_ctrl: GPIO28_CTRL,
    #[doc = "0xe8 - GPIO status"]
    pub gpio29_status: GPIO29_STATUS,
    #[doc = "0xec - GPIO control including function select and overrides."]
    pub gpio29_ctrl: GPIO29_CTRL,
    #[doc = "0xf0 - Raw Interrupts"]
    pub intr0: INTR0,
    #[doc = "0xf4 - Raw Interrupts"]
    pub intr1: INTR1,
    #[doc = "0xf8 - Raw Interrupts"]
    pub intr2: INTR2,
    #[doc = "0xfc - Raw Interrupts"]
    pub intr3: INTR3,
    #[doc = "0x100 - Interrupt Enable for proc0"]
    pub proc0_inte0: PROC0_INTE0,
    #[doc = "0x104 - Interrupt Enable for proc0"]
    pub proc0_inte1: PROC0_INTE1,
    #[doc = "0x108 - Interrupt Enable for proc0"]
    pub proc0_inte2: PROC0_INTE2,
    #[doc = "0x10c - Interrupt Enable for proc0"]
    pub proc0_inte3: PROC0_INTE3,
    #[doc = "0x110 - Interrupt Force for proc0"]
    pub proc0_intf0: PROC0_INTF0,
    #[doc = "0x114 - Interrupt Force for proc0"]
    pub proc0_intf1: PROC0_INTF1,
    #[doc = "0x118 - Interrupt Force for proc0"]
    pub proc0_intf2: PROC0_INTF2,
    #[doc = "0x11c - Interrupt Force for proc0"]
    pub proc0_intf3: PROC0_INTF3,
    #[doc = "0x120 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints0: PROC0_INTS0,
    #[doc = "0x124 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints1: PROC0_INTS1,
    #[doc = "0x128 - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints2: PROC0_INTS2,
    #[doc = "0x12c - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints3: PROC0_INTS3,
    #[doc = "0x130 - Interrupt Enable for proc1"]
    pub proc1_inte0: PROC1_INTE0,
    #[doc = "0x134 - Interrupt Enable for proc1"]
    pub proc1_inte1: PROC1_INTE1,
    #[doc = "0x138 - Interrupt Enable for proc1"]
    pub proc1_inte2: PROC1_INTE2,
    #[doc = "0x13c - Interrupt Enable for proc1"]
    pub proc1_inte3: PROC1_INTE3,
    #[doc = "0x140 - Interrupt Force for proc1"]
    pub proc1_intf0: PROC1_INTF0,
    #[doc = "0x144 - Interrupt Force for proc1"]
    pub proc1_intf1: PROC1_INTF1,
    #[doc = "0x148 - Interrupt Force for proc1"]
    pub proc1_intf2: PROC1_INTF2,
    #[doc = "0x14c - Interrupt Force for proc1"]
    pub proc1_intf3: PROC1_INTF3,
    #[doc = "0x150 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints0: PROC1_INTS0,
    #[doc = "0x154 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints1: PROC1_INTS1,
    #[doc = "0x158 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints2: PROC1_INTS2,
    #[doc = "0x15c - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints3: PROC1_INTS3,
    #[doc = "0x160 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte0: DORMANT_WAKE_INTE0,
    #[doc = "0x164 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte1: DORMANT_WAKE_INTE1,
    #[doc = "0x168 - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte2: DORMANT_WAKE_INTE2,
    #[doc = "0x16c - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte3: DORMANT_WAKE_INTE3,
    #[doc = "0x170 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf0: DORMANT_WAKE_INTF0,
    #[doc = "0x174 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf1: DORMANT_WAKE_INTF1,
    #[doc = "0x178 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf2: DORMANT_WAKE_INTF2,
    #[doc = "0x17c - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf3: DORMANT_WAKE_INTF3,
    #[doc = "0x180 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints0: DORMANT_WAKE_INTS0,
    #[doc = "0x184 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints1: DORMANT_WAKE_INTS1,
    #[doc = "0x188 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints2: DORMANT_WAKE_INTS2,
    #[doc = "0x18c - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints3: DORMANT_WAKE_INTS3,
}
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio0_status](gpio0_status) module"]
pub type GPIO0_STATUS = crate::Reg<u32, _GPIO0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO0_STATUS;
#[doc = "`read()` method returns [gpio0_status::R](gpio0_status::R) reader structure"]
impl crate::Readable for GPIO0_STATUS {}
#[doc = "GPIO status"]
pub mod gpio0_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio0_ctrl](gpio0_ctrl) module"]
pub type GPIO0_CTRL = crate::Reg<u32, _GPIO0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO0_CTRL;
#[doc = "`read()` method returns [gpio0_ctrl::R](gpio0_ctrl::R) reader structure"]
impl crate::Readable for GPIO0_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio0_ctrl::W](gpio0_ctrl::W) writer structure"]
impl crate::Writable for GPIO0_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio0_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio1_status](gpio1_status) module"]
pub type GPIO1_STATUS = crate::Reg<u32, _GPIO1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO1_STATUS;
#[doc = "`read()` method returns [gpio1_status::R](gpio1_status::R) reader structure"]
impl crate::Readable for GPIO1_STATUS {}
#[doc = "GPIO status"]
pub mod gpio1_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio1_ctrl](gpio1_ctrl) module"]
pub type GPIO1_CTRL = crate::Reg<u32, _GPIO1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO1_CTRL;
#[doc = "`read()` method returns [gpio1_ctrl::R](gpio1_ctrl::R) reader structure"]
impl crate::Readable for GPIO1_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio1_ctrl::W](gpio1_ctrl::W) writer structure"]
impl crate::Writable for GPIO1_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio1_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio2_status](gpio2_status) module"]
pub type GPIO2_STATUS = crate::Reg<u32, _GPIO2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO2_STATUS;
#[doc = "`read()` method returns [gpio2_status::R](gpio2_status::R) reader structure"]
impl crate::Readable for GPIO2_STATUS {}
#[doc = "GPIO status"]
pub mod gpio2_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio2_ctrl](gpio2_ctrl) module"]
pub type GPIO2_CTRL = crate::Reg<u32, _GPIO2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO2_CTRL;
#[doc = "`read()` method returns [gpio2_ctrl::R](gpio2_ctrl::R) reader structure"]
impl crate::Readable for GPIO2_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio2_ctrl::W](gpio2_ctrl::W) writer structure"]
impl crate::Writable for GPIO2_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio2_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio3_status](gpio3_status) module"]
pub type GPIO3_STATUS = crate::Reg<u32, _GPIO3_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO3_STATUS;
#[doc = "`read()` method returns [gpio3_status::R](gpio3_status::R) reader structure"]
impl crate::Readable for GPIO3_STATUS {}
#[doc = "GPIO status"]
pub mod gpio3_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio3_ctrl](gpio3_ctrl) module"]
pub type GPIO3_CTRL = crate::Reg<u32, _GPIO3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO3_CTRL;
#[doc = "`read()` method returns [gpio3_ctrl::R](gpio3_ctrl::R) reader structure"]
impl crate::Readable for GPIO3_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio3_ctrl::W](gpio3_ctrl::W) writer structure"]
impl crate::Writable for GPIO3_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio3_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio4_status](gpio4_status) module"]
pub type GPIO4_STATUS = crate::Reg<u32, _GPIO4_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO4_STATUS;
#[doc = "`read()` method returns [gpio4_status::R](gpio4_status::R) reader structure"]
impl crate::Readable for GPIO4_STATUS {}
#[doc = "GPIO status"]
pub mod gpio4_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio4_ctrl](gpio4_ctrl) module"]
pub type GPIO4_CTRL = crate::Reg<u32, _GPIO4_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO4_CTRL;
#[doc = "`read()` method returns [gpio4_ctrl::R](gpio4_ctrl::R) reader structure"]
impl crate::Readable for GPIO4_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio4_ctrl::W](gpio4_ctrl::W) writer structure"]
impl crate::Writable for GPIO4_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio4_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio5_status](gpio5_status) module"]
pub type GPIO5_STATUS = crate::Reg<u32, _GPIO5_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO5_STATUS;
#[doc = "`read()` method returns [gpio5_status::R](gpio5_status::R) reader structure"]
impl crate::Readable for GPIO5_STATUS {}
#[doc = "GPIO status"]
pub mod gpio5_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio5_ctrl](gpio5_ctrl) module"]
pub type GPIO5_CTRL = crate::Reg<u32, _GPIO5_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO5_CTRL;
#[doc = "`read()` method returns [gpio5_ctrl::R](gpio5_ctrl::R) reader structure"]
impl crate::Readable for GPIO5_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio5_ctrl::W](gpio5_ctrl::W) writer structure"]
impl crate::Writable for GPIO5_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio5_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio6_status](gpio6_status) module"]
pub type GPIO6_STATUS = crate::Reg<u32, _GPIO6_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO6_STATUS;
#[doc = "`read()` method returns [gpio6_status::R](gpio6_status::R) reader structure"]
impl crate::Readable for GPIO6_STATUS {}
#[doc = "GPIO status"]
pub mod gpio6_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio6_ctrl](gpio6_ctrl) module"]
pub type GPIO6_CTRL = crate::Reg<u32, _GPIO6_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO6_CTRL;
#[doc = "`read()` method returns [gpio6_ctrl::R](gpio6_ctrl::R) reader structure"]
impl crate::Readable for GPIO6_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio6_ctrl::W](gpio6_ctrl::W) writer structure"]
impl crate::Writable for GPIO6_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio6_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio7_status](gpio7_status) module"]
pub type GPIO7_STATUS = crate::Reg<u32, _GPIO7_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO7_STATUS;
#[doc = "`read()` method returns [gpio7_status::R](gpio7_status::R) reader structure"]
impl crate::Readable for GPIO7_STATUS {}
#[doc = "GPIO status"]
pub mod gpio7_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio7_ctrl](gpio7_ctrl) module"]
pub type GPIO7_CTRL = crate::Reg<u32, _GPIO7_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO7_CTRL;
#[doc = "`read()` method returns [gpio7_ctrl::R](gpio7_ctrl::R) reader structure"]
impl crate::Readable for GPIO7_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio7_ctrl::W](gpio7_ctrl::W) writer structure"]
impl crate::Writable for GPIO7_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio7_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio8_status](gpio8_status) module"]
pub type GPIO8_STATUS = crate::Reg<u32, _GPIO8_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO8_STATUS;
#[doc = "`read()` method returns [gpio8_status::R](gpio8_status::R) reader structure"]
impl crate::Readable for GPIO8_STATUS {}
#[doc = "GPIO status"]
pub mod gpio8_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio8_ctrl](gpio8_ctrl) module"]
pub type GPIO8_CTRL = crate::Reg<u32, _GPIO8_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO8_CTRL;
#[doc = "`read()` method returns [gpio8_ctrl::R](gpio8_ctrl::R) reader structure"]
impl crate::Readable for GPIO8_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio8_ctrl::W](gpio8_ctrl::W) writer structure"]
impl crate::Writable for GPIO8_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio8_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio9_status](gpio9_status) module"]
pub type GPIO9_STATUS = crate::Reg<u32, _GPIO9_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO9_STATUS;
#[doc = "`read()` method returns [gpio9_status::R](gpio9_status::R) reader structure"]
impl crate::Readable for GPIO9_STATUS {}
#[doc = "GPIO status"]
pub mod gpio9_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio9_ctrl](gpio9_ctrl) module"]
pub type GPIO9_CTRL = crate::Reg<u32, _GPIO9_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO9_CTRL;
#[doc = "`read()` method returns [gpio9_ctrl::R](gpio9_ctrl::R) reader structure"]
impl crate::Readable for GPIO9_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio9_ctrl::W](gpio9_ctrl::W) writer structure"]
impl crate::Writable for GPIO9_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio9_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio10_status](gpio10_status) module"]
pub type GPIO10_STATUS = crate::Reg<u32, _GPIO10_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO10_STATUS;
#[doc = "`read()` method returns [gpio10_status::R](gpio10_status::R) reader structure"]
impl crate::Readable for GPIO10_STATUS {}
#[doc = "GPIO status"]
pub mod gpio10_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio10_ctrl](gpio10_ctrl) module"]
pub type GPIO10_CTRL = crate::Reg<u32, _GPIO10_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO10_CTRL;
#[doc = "`read()` method returns [gpio10_ctrl::R](gpio10_ctrl::R) reader structure"]
impl crate::Readable for GPIO10_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio10_ctrl::W](gpio10_ctrl::W) writer structure"]
impl crate::Writable for GPIO10_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio10_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio11_status](gpio11_status) module"]
pub type GPIO11_STATUS = crate::Reg<u32, _GPIO11_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO11_STATUS;
#[doc = "`read()` method returns [gpio11_status::R](gpio11_status::R) reader structure"]
impl crate::Readable for GPIO11_STATUS {}
#[doc = "GPIO status"]
pub mod gpio11_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio11_ctrl](gpio11_ctrl) module"]
pub type GPIO11_CTRL = crate::Reg<u32, _GPIO11_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO11_CTRL;
#[doc = "`read()` method returns [gpio11_ctrl::R](gpio11_ctrl::R) reader structure"]
impl crate::Readable for GPIO11_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio11_ctrl::W](gpio11_ctrl::W) writer structure"]
impl crate::Writable for GPIO11_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio11_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio12_status](gpio12_status) module"]
pub type GPIO12_STATUS = crate::Reg<u32, _GPIO12_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO12_STATUS;
#[doc = "`read()` method returns [gpio12_status::R](gpio12_status::R) reader structure"]
impl crate::Readable for GPIO12_STATUS {}
#[doc = "GPIO status"]
pub mod gpio12_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio12_ctrl](gpio12_ctrl) module"]
pub type GPIO12_CTRL = crate::Reg<u32, _GPIO12_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO12_CTRL;
#[doc = "`read()` method returns [gpio12_ctrl::R](gpio12_ctrl::R) reader structure"]
impl crate::Readable for GPIO12_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio12_ctrl::W](gpio12_ctrl::W) writer structure"]
impl crate::Writable for GPIO12_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio12_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio13_status](gpio13_status) module"]
pub type GPIO13_STATUS = crate::Reg<u32, _GPIO13_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO13_STATUS;
#[doc = "`read()` method returns [gpio13_status::R](gpio13_status::R) reader structure"]
impl crate::Readable for GPIO13_STATUS {}
#[doc = "GPIO status"]
pub mod gpio13_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio13_ctrl](gpio13_ctrl) module"]
pub type GPIO13_CTRL = crate::Reg<u32, _GPIO13_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO13_CTRL;
#[doc = "`read()` method returns [gpio13_ctrl::R](gpio13_ctrl::R) reader structure"]
impl crate::Readable for GPIO13_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio13_ctrl::W](gpio13_ctrl::W) writer structure"]
impl crate::Writable for GPIO13_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio13_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio14_status](gpio14_status) module"]
pub type GPIO14_STATUS = crate::Reg<u32, _GPIO14_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO14_STATUS;
#[doc = "`read()` method returns [gpio14_status::R](gpio14_status::R) reader structure"]
impl crate::Readable for GPIO14_STATUS {}
#[doc = "GPIO status"]
pub mod gpio14_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio14_ctrl](gpio14_ctrl) module"]
pub type GPIO14_CTRL = crate::Reg<u32, _GPIO14_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO14_CTRL;
#[doc = "`read()` method returns [gpio14_ctrl::R](gpio14_ctrl::R) reader structure"]
impl crate::Readable for GPIO14_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio14_ctrl::W](gpio14_ctrl::W) writer structure"]
impl crate::Writable for GPIO14_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio14_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio15_status](gpio15_status) module"]
pub type GPIO15_STATUS = crate::Reg<u32, _GPIO15_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO15_STATUS;
#[doc = "`read()` method returns [gpio15_status::R](gpio15_status::R) reader structure"]
impl crate::Readable for GPIO15_STATUS {}
#[doc = "GPIO status"]
pub mod gpio15_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio15_ctrl](gpio15_ctrl) module"]
pub type GPIO15_CTRL = crate::Reg<u32, _GPIO15_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO15_CTRL;
#[doc = "`read()` method returns [gpio15_ctrl::R](gpio15_ctrl::R) reader structure"]
impl crate::Readable for GPIO15_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio15_ctrl::W](gpio15_ctrl::W) writer structure"]
impl crate::Writable for GPIO15_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio15_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio16_status](gpio16_status) module"]
pub type GPIO16_STATUS = crate::Reg<u32, _GPIO16_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO16_STATUS;
#[doc = "`read()` method returns [gpio16_status::R](gpio16_status::R) reader structure"]
impl crate::Readable for GPIO16_STATUS {}
#[doc = "GPIO status"]
pub mod gpio16_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio16_ctrl](gpio16_ctrl) module"]
pub type GPIO16_CTRL = crate::Reg<u32, _GPIO16_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO16_CTRL;
#[doc = "`read()` method returns [gpio16_ctrl::R](gpio16_ctrl::R) reader structure"]
impl crate::Readable for GPIO16_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio16_ctrl::W](gpio16_ctrl::W) writer structure"]
impl crate::Writable for GPIO16_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio16_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio17_status](gpio17_status) module"]
pub type GPIO17_STATUS = crate::Reg<u32, _GPIO17_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO17_STATUS;
#[doc = "`read()` method returns [gpio17_status::R](gpio17_status::R) reader structure"]
impl crate::Readable for GPIO17_STATUS {}
#[doc = "GPIO status"]
pub mod gpio17_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio17_ctrl](gpio17_ctrl) module"]
pub type GPIO17_CTRL = crate::Reg<u32, _GPIO17_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO17_CTRL;
#[doc = "`read()` method returns [gpio17_ctrl::R](gpio17_ctrl::R) reader structure"]
impl crate::Readable for GPIO17_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio17_ctrl::W](gpio17_ctrl::W) writer structure"]
impl crate::Writable for GPIO17_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio17_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio18_status](gpio18_status) module"]
pub type GPIO18_STATUS = crate::Reg<u32, _GPIO18_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO18_STATUS;
#[doc = "`read()` method returns [gpio18_status::R](gpio18_status::R) reader structure"]
impl crate::Readable for GPIO18_STATUS {}
#[doc = "GPIO status"]
pub mod gpio18_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio18_ctrl](gpio18_ctrl) module"]
pub type GPIO18_CTRL = crate::Reg<u32, _GPIO18_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO18_CTRL;
#[doc = "`read()` method returns [gpio18_ctrl::R](gpio18_ctrl::R) reader structure"]
impl crate::Readable for GPIO18_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio18_ctrl::W](gpio18_ctrl::W) writer structure"]
impl crate::Writable for GPIO18_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio18_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio19_status](gpio19_status) module"]
pub type GPIO19_STATUS = crate::Reg<u32, _GPIO19_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO19_STATUS;
#[doc = "`read()` method returns [gpio19_status::R](gpio19_status::R) reader structure"]
impl crate::Readable for GPIO19_STATUS {}
#[doc = "GPIO status"]
pub mod gpio19_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio19_ctrl](gpio19_ctrl) module"]
pub type GPIO19_CTRL = crate::Reg<u32, _GPIO19_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO19_CTRL;
#[doc = "`read()` method returns [gpio19_ctrl::R](gpio19_ctrl::R) reader structure"]
impl crate::Readable for GPIO19_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio19_ctrl::W](gpio19_ctrl::W) writer structure"]
impl crate::Writable for GPIO19_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio19_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio20_status](gpio20_status) module"]
pub type GPIO20_STATUS = crate::Reg<u32, _GPIO20_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO20_STATUS;
#[doc = "`read()` method returns [gpio20_status::R](gpio20_status::R) reader structure"]
impl crate::Readable for GPIO20_STATUS {}
#[doc = "GPIO status"]
pub mod gpio20_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio20_ctrl](gpio20_ctrl) module"]
pub type GPIO20_CTRL = crate::Reg<u32, _GPIO20_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO20_CTRL;
#[doc = "`read()` method returns [gpio20_ctrl::R](gpio20_ctrl::R) reader structure"]
impl crate::Readable for GPIO20_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio20_ctrl::W](gpio20_ctrl::W) writer structure"]
impl crate::Writable for GPIO20_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio20_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio21_status](gpio21_status) module"]
pub type GPIO21_STATUS = crate::Reg<u32, _GPIO21_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO21_STATUS;
#[doc = "`read()` method returns [gpio21_status::R](gpio21_status::R) reader structure"]
impl crate::Readable for GPIO21_STATUS {}
#[doc = "GPIO status"]
pub mod gpio21_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio21_ctrl](gpio21_ctrl) module"]
pub type GPIO21_CTRL = crate::Reg<u32, _GPIO21_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO21_CTRL;
#[doc = "`read()` method returns [gpio21_ctrl::R](gpio21_ctrl::R) reader structure"]
impl crate::Readable for GPIO21_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio21_ctrl::W](gpio21_ctrl::W) writer structure"]
impl crate::Writable for GPIO21_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio21_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio22_status](gpio22_status) module"]
pub type GPIO22_STATUS = crate::Reg<u32, _GPIO22_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO22_STATUS;
#[doc = "`read()` method returns [gpio22_status::R](gpio22_status::R) reader structure"]
impl crate::Readable for GPIO22_STATUS {}
#[doc = "GPIO status"]
pub mod gpio22_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio22_ctrl](gpio22_ctrl) module"]
pub type GPIO22_CTRL = crate::Reg<u32, _GPIO22_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO22_CTRL;
#[doc = "`read()` method returns [gpio22_ctrl::R](gpio22_ctrl::R) reader structure"]
impl crate::Readable for GPIO22_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio22_ctrl::W](gpio22_ctrl::W) writer structure"]
impl crate::Writable for GPIO22_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio22_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio23_status](gpio23_status) module"]
pub type GPIO23_STATUS = crate::Reg<u32, _GPIO23_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO23_STATUS;
#[doc = "`read()` method returns [gpio23_status::R](gpio23_status::R) reader structure"]
impl crate::Readable for GPIO23_STATUS {}
#[doc = "GPIO status"]
pub mod gpio23_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio23_ctrl](gpio23_ctrl) module"]
pub type GPIO23_CTRL = crate::Reg<u32, _GPIO23_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO23_CTRL;
#[doc = "`read()` method returns [gpio23_ctrl::R](gpio23_ctrl::R) reader structure"]
impl crate::Readable for GPIO23_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio23_ctrl::W](gpio23_ctrl::W) writer structure"]
impl crate::Writable for GPIO23_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio23_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio24_status](gpio24_status) module"]
pub type GPIO24_STATUS = crate::Reg<u32, _GPIO24_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO24_STATUS;
#[doc = "`read()` method returns [gpio24_status::R](gpio24_status::R) reader structure"]
impl crate::Readable for GPIO24_STATUS {}
#[doc = "GPIO status"]
pub mod gpio24_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio24_ctrl](gpio24_ctrl) module"]
pub type GPIO24_CTRL = crate::Reg<u32, _GPIO24_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO24_CTRL;
#[doc = "`read()` method returns [gpio24_ctrl::R](gpio24_ctrl::R) reader structure"]
impl crate::Readable for GPIO24_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio24_ctrl::W](gpio24_ctrl::W) writer structure"]
impl crate::Writable for GPIO24_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio24_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio25_status](gpio25_status) module"]
pub type GPIO25_STATUS = crate::Reg<u32, _GPIO25_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO25_STATUS;
#[doc = "`read()` method returns [gpio25_status::R](gpio25_status::R) reader structure"]
impl crate::Readable for GPIO25_STATUS {}
#[doc = "GPIO status"]
pub mod gpio25_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio25_ctrl](gpio25_ctrl) module"]
pub type GPIO25_CTRL = crate::Reg<u32, _GPIO25_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO25_CTRL;
#[doc = "`read()` method returns [gpio25_ctrl::R](gpio25_ctrl::R) reader structure"]
impl crate::Readable for GPIO25_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio25_ctrl::W](gpio25_ctrl::W) writer structure"]
impl crate::Writable for GPIO25_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio25_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio26_status](gpio26_status) module"]
pub type GPIO26_STATUS = crate::Reg<u32, _GPIO26_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO26_STATUS;
#[doc = "`read()` method returns [gpio26_status::R](gpio26_status::R) reader structure"]
impl crate::Readable for GPIO26_STATUS {}
#[doc = "GPIO status"]
pub mod gpio26_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio26_ctrl](gpio26_ctrl) module"]
pub type GPIO26_CTRL = crate::Reg<u32, _GPIO26_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO26_CTRL;
#[doc = "`read()` method returns [gpio26_ctrl::R](gpio26_ctrl::R) reader structure"]
impl crate::Readable for GPIO26_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio26_ctrl::W](gpio26_ctrl::W) writer structure"]
impl crate::Writable for GPIO26_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio26_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio27_status](gpio27_status) module"]
pub type GPIO27_STATUS = crate::Reg<u32, _GPIO27_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO27_STATUS;
#[doc = "`read()` method returns [gpio27_status::R](gpio27_status::R) reader structure"]
impl crate::Readable for GPIO27_STATUS {}
#[doc = "GPIO status"]
pub mod gpio27_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio27_ctrl](gpio27_ctrl) module"]
pub type GPIO27_CTRL = crate::Reg<u32, _GPIO27_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO27_CTRL;
#[doc = "`read()` method returns [gpio27_ctrl::R](gpio27_ctrl::R) reader structure"]
impl crate::Readable for GPIO27_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio27_ctrl::W](gpio27_ctrl::W) writer structure"]
impl crate::Writable for GPIO27_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio27_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio28_status](gpio28_status) module"]
pub type GPIO28_STATUS = crate::Reg<u32, _GPIO28_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO28_STATUS;
#[doc = "`read()` method returns [gpio28_status::R](gpio28_status::R) reader structure"]
impl crate::Readable for GPIO28_STATUS {}
#[doc = "GPIO status"]
pub mod gpio28_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio28_ctrl](gpio28_ctrl) module"]
pub type GPIO28_CTRL = crate::Reg<u32, _GPIO28_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO28_CTRL;
#[doc = "`read()` method returns [gpio28_ctrl::R](gpio28_ctrl::R) reader structure"]
impl crate::Readable for GPIO28_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio28_ctrl::W](gpio28_ctrl::W) writer structure"]
impl crate::Writable for GPIO28_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio28_ctrl;
#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio29_status](gpio29_status) module"]
pub type GPIO29_STATUS = crate::Reg<u32, _GPIO29_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO29_STATUS;
#[doc = "`read()` method returns [gpio29_status::R](gpio29_status::R) reader structure"]
impl crate::Readable for GPIO29_STATUS {}
#[doc = "GPIO status"]
pub mod gpio29_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio29_ctrl](gpio29_ctrl) module"]
pub type GPIO29_CTRL = crate::Reg<u32, _GPIO29_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO29_CTRL;
#[doc = "`read()` method returns [gpio29_ctrl::R](gpio29_ctrl::R) reader structure"]
impl crate::Readable for GPIO29_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio29_ctrl::W](gpio29_ctrl::W) writer structure"]
impl crate::Writable for GPIO29_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio29_ctrl;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr0](intr0) module"]
pub type INTR0 = crate::Reg<u32, _INTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR0;
#[doc = "`read()` method returns [intr0::R](intr0::R) reader structure"]
impl crate::Readable for INTR0 {}
#[doc = "`write(|w| ..)` method takes [intr0::W](intr0::W) writer structure"]
impl crate::Writable for INTR0 {}
#[doc = "Raw Interrupts"]
pub mod intr0;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr1](intr1) module"]
pub type INTR1 = crate::Reg<u32, _INTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR1;
#[doc = "`read()` method returns [intr1::R](intr1::R) reader structure"]
impl crate::Readable for INTR1 {}
#[doc = "`write(|w| ..)` method takes [intr1::W](intr1::W) writer structure"]
impl crate::Writable for INTR1 {}
#[doc = "Raw Interrupts"]
pub mod intr1;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr2](intr2) module"]
pub type INTR2 = crate::Reg<u32, _INTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR2;
#[doc = "`read()` method returns [intr2::R](intr2::R) reader structure"]
impl crate::Readable for INTR2 {}
#[doc = "`write(|w| ..)` method takes [intr2::W](intr2::W) writer structure"]
impl crate::Writable for INTR2 {}
#[doc = "Raw Interrupts"]
pub mod intr2;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr3](intr3) module"]
pub type INTR3 = crate::Reg<u32, _INTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR3;
#[doc = "`read()` method returns [intr3::R](intr3::R) reader structure"]
impl crate::Readable for INTR3 {}
#[doc = "`write(|w| ..)` method takes [intr3::W](intr3::W) writer structure"]
impl crate::Writable for INTR3 {}
#[doc = "Raw Interrupts"]
pub mod intr3;
#[doc = "Interrupt Enable for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_inte0](proc0_inte0) module"]
pub type PROC0_INTE0 = crate::Reg<u32, _PROC0_INTE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTE0;
#[doc = "`read()` method returns [proc0_inte0::R](proc0_inte0::R) reader structure"]
impl crate::Readable for PROC0_INTE0 {}
#[doc = "`write(|w| ..)` method takes [proc0_inte0::W](proc0_inte0::W) writer structure"]
impl crate::Writable for PROC0_INTE0 {}
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte0;
#[doc = "Interrupt Enable for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_inte1](proc0_inte1) module"]
pub type PROC0_INTE1 = crate::Reg<u32, _PROC0_INTE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTE1;
#[doc = "`read()` method returns [proc0_inte1::R](proc0_inte1::R) reader structure"]
impl crate::Readable for PROC0_INTE1 {}
#[doc = "`write(|w| ..)` method takes [proc0_inte1::W](proc0_inte1::W) writer structure"]
impl crate::Writable for PROC0_INTE1 {}
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte1;
#[doc = "Interrupt Enable for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_inte2](proc0_inte2) module"]
pub type PROC0_INTE2 = crate::Reg<u32, _PROC0_INTE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTE2;
#[doc = "`read()` method returns [proc0_inte2::R](proc0_inte2::R) reader structure"]
impl crate::Readable for PROC0_INTE2 {}
#[doc = "`write(|w| ..)` method takes [proc0_inte2::W](proc0_inte2::W) writer structure"]
impl crate::Writable for PROC0_INTE2 {}
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte2;
#[doc = "Interrupt Enable for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_inte3](proc0_inte3) module"]
pub type PROC0_INTE3 = crate::Reg<u32, _PROC0_INTE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTE3;
#[doc = "`read()` method returns [proc0_inte3::R](proc0_inte3::R) reader structure"]
impl crate::Readable for PROC0_INTE3 {}
#[doc = "`write(|w| ..)` method takes [proc0_inte3::W](proc0_inte3::W) writer structure"]
impl crate::Writable for PROC0_INTE3 {}
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte3;
#[doc = "Interrupt Force for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_intf0](proc0_intf0) module"]
pub type PROC0_INTF0 = crate::Reg<u32, _PROC0_INTF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTF0;
#[doc = "`read()` method returns [proc0_intf0::R](proc0_intf0::R) reader structure"]
impl crate::Readable for PROC0_INTF0 {}
#[doc = "`write(|w| ..)` method takes [proc0_intf0::W](proc0_intf0::W) writer structure"]
impl crate::Writable for PROC0_INTF0 {}
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf0;
#[doc = "Interrupt Force for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_intf1](proc0_intf1) module"]
pub type PROC0_INTF1 = crate::Reg<u32, _PROC0_INTF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTF1;
#[doc = "`read()` method returns [proc0_intf1::R](proc0_intf1::R) reader structure"]
impl crate::Readable for PROC0_INTF1 {}
#[doc = "`write(|w| ..)` method takes [proc0_intf1::W](proc0_intf1::W) writer structure"]
impl crate::Writable for PROC0_INTF1 {}
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf1;
#[doc = "Interrupt Force for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_intf2](proc0_intf2) module"]
pub type PROC0_INTF2 = crate::Reg<u32, _PROC0_INTF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTF2;
#[doc = "`read()` method returns [proc0_intf2::R](proc0_intf2::R) reader structure"]
impl crate::Readable for PROC0_INTF2 {}
#[doc = "`write(|w| ..)` method takes [proc0_intf2::W](proc0_intf2::W) writer structure"]
impl crate::Writable for PROC0_INTF2 {}
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf2;
#[doc = "Interrupt Force for proc0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_intf3](proc0_intf3) module"]
pub type PROC0_INTF3 = crate::Reg<u32, _PROC0_INTF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTF3;
#[doc = "`read()` method returns [proc0_intf3::R](proc0_intf3::R) reader structure"]
impl crate::Readable for PROC0_INTF3 {}
#[doc = "`write(|w| ..)` method takes [proc0_intf3::W](proc0_intf3::W) writer structure"]
impl crate::Writable for PROC0_INTF3 {}
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf3;
#[doc = "Interrupt status after masking & forcing for proc0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_ints0](proc0_ints0) module"]
pub type PROC0_INTS0 = crate::Reg<u32, _PROC0_INTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTS0;
#[doc = "`read()` method returns [proc0_ints0::R](proc0_ints0::R) reader structure"]
impl crate::Readable for PROC0_INTS0 {}
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints0;
#[doc = "Interrupt status after masking & forcing for proc0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_ints1](proc0_ints1) module"]
pub type PROC0_INTS1 = crate::Reg<u32, _PROC0_INTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTS1;
#[doc = "`read()` method returns [proc0_ints1::R](proc0_ints1::R) reader structure"]
impl crate::Readable for PROC0_INTS1 {}
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints1;
#[doc = "Interrupt status after masking & forcing for proc0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_ints2](proc0_ints2) module"]
pub type PROC0_INTS2 = crate::Reg<u32, _PROC0_INTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTS2;
#[doc = "`read()` method returns [proc0_ints2::R](proc0_ints2::R) reader structure"]
impl crate::Readable for PROC0_INTS2 {}
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints2;
#[doc = "Interrupt status after masking & forcing for proc0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_ints3](proc0_ints3) module"]
pub type PROC0_INTS3 = crate::Reg<u32, _PROC0_INTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_INTS3;
#[doc = "`read()` method returns [proc0_ints3::R](proc0_ints3::R) reader structure"]
impl crate::Readable for PROC0_INTS3 {}
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints3;
#[doc = "Interrupt Enable for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_inte0](proc1_inte0) module"]
pub type PROC1_INTE0 = crate::Reg<u32, _PROC1_INTE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTE0;
#[doc = "`read()` method returns [proc1_inte0::R](proc1_inte0::R) reader structure"]
impl crate::Readable for PROC1_INTE0 {}
#[doc = "`write(|w| ..)` method takes [proc1_inte0::W](proc1_inte0::W) writer structure"]
impl crate::Writable for PROC1_INTE0 {}
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte0;
#[doc = "Interrupt Enable for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_inte1](proc1_inte1) module"]
pub type PROC1_INTE1 = crate::Reg<u32, _PROC1_INTE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTE1;
#[doc = "`read()` method returns [proc1_inte1::R](proc1_inte1::R) reader structure"]
impl crate::Readable for PROC1_INTE1 {}
#[doc = "`write(|w| ..)` method takes [proc1_inte1::W](proc1_inte1::W) writer structure"]
impl crate::Writable for PROC1_INTE1 {}
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte1;
#[doc = "Interrupt Enable for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_inte2](proc1_inte2) module"]
pub type PROC1_INTE2 = crate::Reg<u32, _PROC1_INTE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTE2;
#[doc = "`read()` method returns [proc1_inte2::R](proc1_inte2::R) reader structure"]
impl crate::Readable for PROC1_INTE2 {}
#[doc = "`write(|w| ..)` method takes [proc1_inte2::W](proc1_inte2::W) writer structure"]
impl crate::Writable for PROC1_INTE2 {}
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte2;
#[doc = "Interrupt Enable for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_inte3](proc1_inte3) module"]
pub type PROC1_INTE3 = crate::Reg<u32, _PROC1_INTE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTE3;
#[doc = "`read()` method returns [proc1_inte3::R](proc1_inte3::R) reader structure"]
impl crate::Readable for PROC1_INTE3 {}
#[doc = "`write(|w| ..)` method takes [proc1_inte3::W](proc1_inte3::W) writer structure"]
impl crate::Writable for PROC1_INTE3 {}
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte3;
#[doc = "Interrupt Force for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_intf0](proc1_intf0) module"]
pub type PROC1_INTF0 = crate::Reg<u32, _PROC1_INTF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTF0;
#[doc = "`read()` method returns [proc1_intf0::R](proc1_intf0::R) reader structure"]
impl crate::Readable for PROC1_INTF0 {}
#[doc = "`write(|w| ..)` method takes [proc1_intf0::W](proc1_intf0::W) writer structure"]
impl crate::Writable for PROC1_INTF0 {}
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf0;
#[doc = "Interrupt Force for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_intf1](proc1_intf1) module"]
pub type PROC1_INTF1 = crate::Reg<u32, _PROC1_INTF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTF1;
#[doc = "`read()` method returns [proc1_intf1::R](proc1_intf1::R) reader structure"]
impl crate::Readable for PROC1_INTF1 {}
#[doc = "`write(|w| ..)` method takes [proc1_intf1::W](proc1_intf1::W) writer structure"]
impl crate::Writable for PROC1_INTF1 {}
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf1;
#[doc = "Interrupt Force for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_intf2](proc1_intf2) module"]
pub type PROC1_INTF2 = crate::Reg<u32, _PROC1_INTF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTF2;
#[doc = "`read()` method returns [proc1_intf2::R](proc1_intf2::R) reader structure"]
impl crate::Readable for PROC1_INTF2 {}
#[doc = "`write(|w| ..)` method takes [proc1_intf2::W](proc1_intf2::W) writer structure"]
impl crate::Writable for PROC1_INTF2 {}
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf2;
#[doc = "Interrupt Force for proc1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_intf3](proc1_intf3) module"]
pub type PROC1_INTF3 = crate::Reg<u32, _PROC1_INTF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTF3;
#[doc = "`read()` method returns [proc1_intf3::R](proc1_intf3::R) reader structure"]
impl crate::Readable for PROC1_INTF3 {}
#[doc = "`write(|w| ..)` method takes [proc1_intf3::W](proc1_intf3::W) writer structure"]
impl crate::Writable for PROC1_INTF3 {}
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf3;
#[doc = "Interrupt status after masking & forcing for proc1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_ints0](proc1_ints0) module"]
pub type PROC1_INTS0 = crate::Reg<u32, _PROC1_INTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTS0;
#[doc = "`read()` method returns [proc1_ints0::R](proc1_ints0::R) reader structure"]
impl crate::Readable for PROC1_INTS0 {}
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints0;
#[doc = "Interrupt status after masking & forcing for proc1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_ints1](proc1_ints1) module"]
pub type PROC1_INTS1 = crate::Reg<u32, _PROC1_INTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTS1;
#[doc = "`read()` method returns [proc1_ints1::R](proc1_ints1::R) reader structure"]
impl crate::Readable for PROC1_INTS1 {}
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints1;
#[doc = "Interrupt status after masking & forcing for proc1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_ints2](proc1_ints2) module"]
pub type PROC1_INTS2 = crate::Reg<u32, _PROC1_INTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTS2;
#[doc = "`read()` method returns [proc1_ints2::R](proc1_ints2::R) reader structure"]
impl crate::Readable for PROC1_INTS2 {}
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints2;
#[doc = "Interrupt status after masking & forcing for proc1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_ints3](proc1_ints3) module"]
pub type PROC1_INTS3 = crate::Reg<u32, _PROC1_INTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_INTS3;
#[doc = "`read()` method returns [proc1_ints3::R](proc1_ints3::R) reader structure"]
impl crate::Readable for PROC1_INTS3 {}
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints3;
#[doc = "Interrupt Enable for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_inte0](dormant_wake_inte0) module"]
pub type DORMANT_WAKE_INTE0 = crate::Reg<u32, _DORMANT_WAKE_INTE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTE0;
#[doc = "`read()` method returns [dormant_wake_inte0::R](dormant_wake_inte0::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTE0 {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_inte0::W](dormant_wake_inte0::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTE0 {}
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte0;
#[doc = "Interrupt Enable for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_inte1](dormant_wake_inte1) module"]
pub type DORMANT_WAKE_INTE1 = crate::Reg<u32, _DORMANT_WAKE_INTE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTE1;
#[doc = "`read()` method returns [dormant_wake_inte1::R](dormant_wake_inte1::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTE1 {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_inte1::W](dormant_wake_inte1::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTE1 {}
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte1;
#[doc = "Interrupt Enable for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_inte2](dormant_wake_inte2) module"]
pub type DORMANT_WAKE_INTE2 = crate::Reg<u32, _DORMANT_WAKE_INTE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTE2;
#[doc = "`read()` method returns [dormant_wake_inte2::R](dormant_wake_inte2::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTE2 {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_inte2::W](dormant_wake_inte2::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTE2 {}
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte2;
#[doc = "Interrupt Enable for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_inte3](dormant_wake_inte3) module"]
pub type DORMANT_WAKE_INTE3 = crate::Reg<u32, _DORMANT_WAKE_INTE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTE3;
#[doc = "`read()` method returns [dormant_wake_inte3::R](dormant_wake_inte3::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTE3 {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_inte3::W](dormant_wake_inte3::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTE3 {}
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte3;
#[doc = "Interrupt Force for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_intf0](dormant_wake_intf0) module"]
pub type DORMANT_WAKE_INTF0 = crate::Reg<u32, _DORMANT_WAKE_INTF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTF0;
#[doc = "`read()` method returns [dormant_wake_intf0::R](dormant_wake_intf0::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTF0 {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_intf0::W](dormant_wake_intf0::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTF0 {}
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf0;
#[doc = "Interrupt Force for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_intf1](dormant_wake_intf1) module"]
pub type DORMANT_WAKE_INTF1 = crate::Reg<u32, _DORMANT_WAKE_INTF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTF1;
#[doc = "`read()` method returns [dormant_wake_intf1::R](dormant_wake_intf1::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTF1 {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_intf1::W](dormant_wake_intf1::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTF1 {}
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf1;
#[doc = "Interrupt Force for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_intf2](dormant_wake_intf2) module"]
pub type DORMANT_WAKE_INTF2 = crate::Reg<u32, _DORMANT_WAKE_INTF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTF2;
#[doc = "`read()` method returns [dormant_wake_intf2::R](dormant_wake_intf2::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTF2 {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_intf2::W](dormant_wake_intf2::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTF2 {}
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf2;
#[doc = "Interrupt Force for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_intf3](dormant_wake_intf3) module"]
pub type DORMANT_WAKE_INTF3 = crate::Reg<u32, _DORMANT_WAKE_INTF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTF3;
#[doc = "`read()` method returns [dormant_wake_intf3::R](dormant_wake_intf3::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTF3 {}
#[doc = "`write(|w| ..)` method takes [dormant_wake_intf3::W](dormant_wake_intf3::W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTF3 {}
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf3;
#[doc = "Interrupt status after masking & forcing for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_ints0](dormant_wake_ints0) module"]
pub type DORMANT_WAKE_INTS0 = crate::Reg<u32, _DORMANT_WAKE_INTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTS0;
#[doc = "`read()` method returns [dormant_wake_ints0::R](dormant_wake_ints0::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTS0 {}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints0;
#[doc = "Interrupt status after masking & forcing for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_ints1](dormant_wake_ints1) module"]
pub type DORMANT_WAKE_INTS1 = crate::Reg<u32, _DORMANT_WAKE_INTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTS1;
#[doc = "`read()` method returns [dormant_wake_ints1::R](dormant_wake_ints1::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTS1 {}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints1;
#[doc = "Interrupt status after masking & forcing for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_ints2](dormant_wake_ints2) module"]
pub type DORMANT_WAKE_INTS2 = crate::Reg<u32, _DORMANT_WAKE_INTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTS2;
#[doc = "`read()` method returns [dormant_wake_ints2::R](dormant_wake_ints2::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTS2 {}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints2;
#[doc = "Interrupt status after masking & forcing for dormant_wake\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dormant_wake_ints3](dormant_wake_ints3) module"]
pub type DORMANT_WAKE_INTS3 = crate::Reg<u32, _DORMANT_WAKE_INTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT_WAKE_INTS3;
#[doc = "`read()` method returns [dormant_wake_ints3::R](dormant_wake_ints3::R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTS3 {}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints3;
