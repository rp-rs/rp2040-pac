#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
    pub gpio: [GPIO; 30],
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
#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO {
    #[doc = "0x00 - GPIO status"]
    pub gpio_status: self::gpio::GPIO_STATUS,
    #[doc = "0x04 - GPIO control including function select and overrides."]
    pub gpio_ctrl: self::gpio::GPIO_CTRL,
}
#[doc = r"Register block"]
#[doc = "Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
pub mod gpio;
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
