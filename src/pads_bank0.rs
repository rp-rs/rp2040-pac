#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    pub voltage_select: VOLTAGE_SELECT,
    #[doc = "0x04 - Pad control register"]
    pub gpio0: GPIO0,
    #[doc = "0x08 - Pad control register"]
    pub gpio1: GPIO1,
    #[doc = "0x0c - Pad control register"]
    pub gpio2: GPIO2,
    #[doc = "0x10 - Pad control register"]
    pub gpio3: GPIO3,
    #[doc = "0x14 - Pad control register"]
    pub gpio4: GPIO4,
    #[doc = "0x18 - Pad control register"]
    pub gpio5: GPIO5,
    #[doc = "0x1c - Pad control register"]
    pub gpio6: GPIO6,
    #[doc = "0x20 - Pad control register"]
    pub gpio7: GPIO7,
    #[doc = "0x24 - Pad control register"]
    pub gpio8: GPIO8,
    #[doc = "0x28 - Pad control register"]
    pub gpio9: GPIO9,
    #[doc = "0x2c - Pad control register"]
    pub gpio10: GPIO10,
    #[doc = "0x30 - Pad control register"]
    pub gpio11: GPIO11,
    #[doc = "0x34 - Pad control register"]
    pub gpio12: GPIO12,
    #[doc = "0x38 - Pad control register"]
    pub gpio13: GPIO13,
    #[doc = "0x3c - Pad control register"]
    pub gpio14: GPIO14,
    #[doc = "0x40 - Pad control register"]
    pub gpio15: GPIO15,
    #[doc = "0x44 - Pad control register"]
    pub gpio16: GPIO16,
    #[doc = "0x48 - Pad control register"]
    pub gpio17: GPIO17,
    #[doc = "0x4c - Pad control register"]
    pub gpio18: GPIO18,
    #[doc = "0x50 - Pad control register"]
    pub gpio19: GPIO19,
    #[doc = "0x54 - Pad control register"]
    pub gpio20: GPIO20,
    #[doc = "0x58 - Pad control register"]
    pub gpio21: GPIO21,
    #[doc = "0x5c - Pad control register"]
    pub gpio22: GPIO22,
    #[doc = "0x60 - Pad control register"]
    pub gpio23: GPIO23,
    #[doc = "0x64 - Pad control register"]
    pub gpio24: GPIO24,
    #[doc = "0x68 - Pad control register"]
    pub gpio25: GPIO25,
    #[doc = "0x6c - Pad control register"]
    pub gpio26: GPIO26,
    #[doc = "0x70 - Pad control register"]
    pub gpio27: GPIO27,
    #[doc = "0x74 - Pad control register"]
    pub gpio28: GPIO28,
    #[doc = "0x78 - Pad control register"]
    pub gpio29: GPIO29,
    #[doc = "0x7c - Pad control register"]
    pub swclk: SWCLK,
    #[doc = "0x80 - Pad control register"]
    pub swd: SWD,
}
#[doc = "Voltage select. Per bank control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voltage_select](voltage_select) module"]
pub type VOLTAGE_SELECT = crate::Reg<u32, _VOLTAGE_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VOLTAGE_SELECT;
#[doc = "`read()` method returns [voltage_select::R](voltage_select::R) reader structure"]
impl crate::Readable for VOLTAGE_SELECT {}
#[doc = "`write(|w| ..)` method takes [voltage_select::W](voltage_select::W) writer structure"]
impl crate::Writable for VOLTAGE_SELECT {}
#[doc = "Voltage select. Per bank control"]
pub mod voltage_select;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio0](gpio0) module"]
pub type GPIO0 = crate::Reg<u32, _GPIO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO0;
#[doc = "`read()` method returns [gpio0::R](gpio0::R) reader structure"]
impl crate::Readable for GPIO0 {}
#[doc = "`write(|w| ..)` method takes [gpio0::W](gpio0::W) writer structure"]
impl crate::Writable for GPIO0 {}
#[doc = "Pad control register"]
pub mod gpio0;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio1](gpio1) module"]
pub type GPIO1 = crate::Reg<u32, _GPIO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO1;
#[doc = "`read()` method returns [gpio1::R](gpio1::R) reader structure"]
impl crate::Readable for GPIO1 {}
#[doc = "`write(|w| ..)` method takes [gpio1::W](gpio1::W) writer structure"]
impl crate::Writable for GPIO1 {}
#[doc = "Pad control register"]
pub mod gpio1;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio2](gpio2) module"]
pub type GPIO2 = crate::Reg<u32, _GPIO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO2;
#[doc = "`read()` method returns [gpio2::R](gpio2::R) reader structure"]
impl crate::Readable for GPIO2 {}
#[doc = "`write(|w| ..)` method takes [gpio2::W](gpio2::W) writer structure"]
impl crate::Writable for GPIO2 {}
#[doc = "Pad control register"]
pub mod gpio2;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio3](gpio3) module"]
pub type GPIO3 = crate::Reg<u32, _GPIO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO3;
#[doc = "`read()` method returns [gpio3::R](gpio3::R) reader structure"]
impl crate::Readable for GPIO3 {}
#[doc = "`write(|w| ..)` method takes [gpio3::W](gpio3::W) writer structure"]
impl crate::Writable for GPIO3 {}
#[doc = "Pad control register"]
pub mod gpio3;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio4](gpio4) module"]
pub type GPIO4 = crate::Reg<u32, _GPIO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO4;
#[doc = "`read()` method returns [gpio4::R](gpio4::R) reader structure"]
impl crate::Readable for GPIO4 {}
#[doc = "`write(|w| ..)` method takes [gpio4::W](gpio4::W) writer structure"]
impl crate::Writable for GPIO4 {}
#[doc = "Pad control register"]
pub mod gpio4;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio5](gpio5) module"]
pub type GPIO5 = crate::Reg<u32, _GPIO5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO5;
#[doc = "`read()` method returns [gpio5::R](gpio5::R) reader structure"]
impl crate::Readable for GPIO5 {}
#[doc = "`write(|w| ..)` method takes [gpio5::W](gpio5::W) writer structure"]
impl crate::Writable for GPIO5 {}
#[doc = "Pad control register"]
pub mod gpio5;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio6](gpio6) module"]
pub type GPIO6 = crate::Reg<u32, _GPIO6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO6;
#[doc = "`read()` method returns [gpio6::R](gpio6::R) reader structure"]
impl crate::Readable for GPIO6 {}
#[doc = "`write(|w| ..)` method takes [gpio6::W](gpio6::W) writer structure"]
impl crate::Writable for GPIO6 {}
#[doc = "Pad control register"]
pub mod gpio6;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio7](gpio7) module"]
pub type GPIO7 = crate::Reg<u32, _GPIO7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO7;
#[doc = "`read()` method returns [gpio7::R](gpio7::R) reader structure"]
impl crate::Readable for GPIO7 {}
#[doc = "`write(|w| ..)` method takes [gpio7::W](gpio7::W) writer structure"]
impl crate::Writable for GPIO7 {}
#[doc = "Pad control register"]
pub mod gpio7;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio8](gpio8) module"]
pub type GPIO8 = crate::Reg<u32, _GPIO8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO8;
#[doc = "`read()` method returns [gpio8::R](gpio8::R) reader structure"]
impl crate::Readable for GPIO8 {}
#[doc = "`write(|w| ..)` method takes [gpio8::W](gpio8::W) writer structure"]
impl crate::Writable for GPIO8 {}
#[doc = "Pad control register"]
pub mod gpio8;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio9](gpio9) module"]
pub type GPIO9 = crate::Reg<u32, _GPIO9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO9;
#[doc = "`read()` method returns [gpio9::R](gpio9::R) reader structure"]
impl crate::Readable for GPIO9 {}
#[doc = "`write(|w| ..)` method takes [gpio9::W](gpio9::W) writer structure"]
impl crate::Writable for GPIO9 {}
#[doc = "Pad control register"]
pub mod gpio9;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio10](gpio10) module"]
pub type GPIO10 = crate::Reg<u32, _GPIO10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO10;
#[doc = "`read()` method returns [gpio10::R](gpio10::R) reader structure"]
impl crate::Readable for GPIO10 {}
#[doc = "`write(|w| ..)` method takes [gpio10::W](gpio10::W) writer structure"]
impl crate::Writable for GPIO10 {}
#[doc = "Pad control register"]
pub mod gpio10;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio11](gpio11) module"]
pub type GPIO11 = crate::Reg<u32, _GPIO11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO11;
#[doc = "`read()` method returns [gpio11::R](gpio11::R) reader structure"]
impl crate::Readable for GPIO11 {}
#[doc = "`write(|w| ..)` method takes [gpio11::W](gpio11::W) writer structure"]
impl crate::Writable for GPIO11 {}
#[doc = "Pad control register"]
pub mod gpio11;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio12](gpio12) module"]
pub type GPIO12 = crate::Reg<u32, _GPIO12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO12;
#[doc = "`read()` method returns [gpio12::R](gpio12::R) reader structure"]
impl crate::Readable for GPIO12 {}
#[doc = "`write(|w| ..)` method takes [gpio12::W](gpio12::W) writer structure"]
impl crate::Writable for GPIO12 {}
#[doc = "Pad control register"]
pub mod gpio12;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio13](gpio13) module"]
pub type GPIO13 = crate::Reg<u32, _GPIO13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO13;
#[doc = "`read()` method returns [gpio13::R](gpio13::R) reader structure"]
impl crate::Readable for GPIO13 {}
#[doc = "`write(|w| ..)` method takes [gpio13::W](gpio13::W) writer structure"]
impl crate::Writable for GPIO13 {}
#[doc = "Pad control register"]
pub mod gpio13;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio14](gpio14) module"]
pub type GPIO14 = crate::Reg<u32, _GPIO14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO14;
#[doc = "`read()` method returns [gpio14::R](gpio14::R) reader structure"]
impl crate::Readable for GPIO14 {}
#[doc = "`write(|w| ..)` method takes [gpio14::W](gpio14::W) writer structure"]
impl crate::Writable for GPIO14 {}
#[doc = "Pad control register"]
pub mod gpio14;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio15](gpio15) module"]
pub type GPIO15 = crate::Reg<u32, _GPIO15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO15;
#[doc = "`read()` method returns [gpio15::R](gpio15::R) reader structure"]
impl crate::Readable for GPIO15 {}
#[doc = "`write(|w| ..)` method takes [gpio15::W](gpio15::W) writer structure"]
impl crate::Writable for GPIO15 {}
#[doc = "Pad control register"]
pub mod gpio15;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio16](gpio16) module"]
pub type GPIO16 = crate::Reg<u32, _GPIO16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO16;
#[doc = "`read()` method returns [gpio16::R](gpio16::R) reader structure"]
impl crate::Readable for GPIO16 {}
#[doc = "`write(|w| ..)` method takes [gpio16::W](gpio16::W) writer structure"]
impl crate::Writable for GPIO16 {}
#[doc = "Pad control register"]
pub mod gpio16;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio17](gpio17) module"]
pub type GPIO17 = crate::Reg<u32, _GPIO17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO17;
#[doc = "`read()` method returns [gpio17::R](gpio17::R) reader structure"]
impl crate::Readable for GPIO17 {}
#[doc = "`write(|w| ..)` method takes [gpio17::W](gpio17::W) writer structure"]
impl crate::Writable for GPIO17 {}
#[doc = "Pad control register"]
pub mod gpio17;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio18](gpio18) module"]
pub type GPIO18 = crate::Reg<u32, _GPIO18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO18;
#[doc = "`read()` method returns [gpio18::R](gpio18::R) reader structure"]
impl crate::Readable for GPIO18 {}
#[doc = "`write(|w| ..)` method takes [gpio18::W](gpio18::W) writer structure"]
impl crate::Writable for GPIO18 {}
#[doc = "Pad control register"]
pub mod gpio18;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio19](gpio19) module"]
pub type GPIO19 = crate::Reg<u32, _GPIO19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO19;
#[doc = "`read()` method returns [gpio19::R](gpio19::R) reader structure"]
impl crate::Readable for GPIO19 {}
#[doc = "`write(|w| ..)` method takes [gpio19::W](gpio19::W) writer structure"]
impl crate::Writable for GPIO19 {}
#[doc = "Pad control register"]
pub mod gpio19;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio20](gpio20) module"]
pub type GPIO20 = crate::Reg<u32, _GPIO20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO20;
#[doc = "`read()` method returns [gpio20::R](gpio20::R) reader structure"]
impl crate::Readable for GPIO20 {}
#[doc = "`write(|w| ..)` method takes [gpio20::W](gpio20::W) writer structure"]
impl crate::Writable for GPIO20 {}
#[doc = "Pad control register"]
pub mod gpio20;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio21](gpio21) module"]
pub type GPIO21 = crate::Reg<u32, _GPIO21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO21;
#[doc = "`read()` method returns [gpio21::R](gpio21::R) reader structure"]
impl crate::Readable for GPIO21 {}
#[doc = "`write(|w| ..)` method takes [gpio21::W](gpio21::W) writer structure"]
impl crate::Writable for GPIO21 {}
#[doc = "Pad control register"]
pub mod gpio21;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio22](gpio22) module"]
pub type GPIO22 = crate::Reg<u32, _GPIO22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO22;
#[doc = "`read()` method returns [gpio22::R](gpio22::R) reader structure"]
impl crate::Readable for GPIO22 {}
#[doc = "`write(|w| ..)` method takes [gpio22::W](gpio22::W) writer structure"]
impl crate::Writable for GPIO22 {}
#[doc = "Pad control register"]
pub mod gpio22;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio23](gpio23) module"]
pub type GPIO23 = crate::Reg<u32, _GPIO23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO23;
#[doc = "`read()` method returns [gpio23::R](gpio23::R) reader structure"]
impl crate::Readable for GPIO23 {}
#[doc = "`write(|w| ..)` method takes [gpio23::W](gpio23::W) writer structure"]
impl crate::Writable for GPIO23 {}
#[doc = "Pad control register"]
pub mod gpio23;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio24](gpio24) module"]
pub type GPIO24 = crate::Reg<u32, _GPIO24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO24;
#[doc = "`read()` method returns [gpio24::R](gpio24::R) reader structure"]
impl crate::Readable for GPIO24 {}
#[doc = "`write(|w| ..)` method takes [gpio24::W](gpio24::W) writer structure"]
impl crate::Writable for GPIO24 {}
#[doc = "Pad control register"]
pub mod gpio24;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio25](gpio25) module"]
pub type GPIO25 = crate::Reg<u32, _GPIO25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO25;
#[doc = "`read()` method returns [gpio25::R](gpio25::R) reader structure"]
impl crate::Readable for GPIO25 {}
#[doc = "`write(|w| ..)` method takes [gpio25::W](gpio25::W) writer structure"]
impl crate::Writable for GPIO25 {}
#[doc = "Pad control register"]
pub mod gpio25;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio26](gpio26) module"]
pub type GPIO26 = crate::Reg<u32, _GPIO26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO26;
#[doc = "`read()` method returns [gpio26::R](gpio26::R) reader structure"]
impl crate::Readable for GPIO26 {}
#[doc = "`write(|w| ..)` method takes [gpio26::W](gpio26::W) writer structure"]
impl crate::Writable for GPIO26 {}
#[doc = "Pad control register"]
pub mod gpio26;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio27](gpio27) module"]
pub type GPIO27 = crate::Reg<u32, _GPIO27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO27;
#[doc = "`read()` method returns [gpio27::R](gpio27::R) reader structure"]
impl crate::Readable for GPIO27 {}
#[doc = "`write(|w| ..)` method takes [gpio27::W](gpio27::W) writer structure"]
impl crate::Writable for GPIO27 {}
#[doc = "Pad control register"]
pub mod gpio27;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio28](gpio28) module"]
pub type GPIO28 = crate::Reg<u32, _GPIO28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO28;
#[doc = "`read()` method returns [gpio28::R](gpio28::R) reader structure"]
impl crate::Readable for GPIO28 {}
#[doc = "`write(|w| ..)` method takes [gpio28::W](gpio28::W) writer structure"]
impl crate::Writable for GPIO28 {}
#[doc = "Pad control register"]
pub mod gpio28;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio29](gpio29) module"]
pub type GPIO29 = crate::Reg<u32, _GPIO29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO29;
#[doc = "`read()` method returns [gpio29::R](gpio29::R) reader structure"]
impl crate::Readable for GPIO29 {}
#[doc = "`write(|w| ..)` method takes [gpio29::W](gpio29::W) writer structure"]
impl crate::Writable for GPIO29 {}
#[doc = "Pad control register"]
pub mod gpio29;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swclk](swclk) module"]
pub type SWCLK = crate::Reg<u32, _SWCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWCLK;
#[doc = "`read()` method returns [swclk::R](swclk::R) reader structure"]
impl crate::Readable for SWCLK {}
#[doc = "`write(|w| ..)` method takes [swclk::W](swclk::W) writer structure"]
impl crate::Writable for SWCLK {}
#[doc = "Pad control register"]
pub mod swclk;
#[doc = "Pad control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd](swd) module"]
pub type SWD = crate::Reg<u32, _SWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWD;
#[doc = "`read()` method returns [swd::R](swd::R) reader structure"]
impl crate::Readable for SWD {}
#[doc = "`write(|w| ..)` method takes [swd::W](swd::W) writer structure"]
impl crate::Writable for SWD {}
#[doc = "Pad control register"]
pub mod swd;
