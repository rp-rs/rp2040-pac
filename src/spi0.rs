#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0, SSPCR0 on page 3-4"]
    pub sspcr0: SSPCR0,
    #[doc = "0x04 - Control register 1, SSPCR1 on page 3-5"]
    pub sspcr1: SSPCR1,
    #[doc = "0x08 - Data register, SSPDR on page 3-6"]
    pub sspdr: SSPDR,
    #[doc = "0x0c - Status register, SSPSR on page 3-7"]
    pub sspsr: SSPSR,
    #[doc = "0x10 - Clock prescale register, SSPCPSR on page 3-8"]
    pub sspcpsr: SSPCPSR,
    #[doc = "0x14 - Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    pub sspimsc: SSPIMSC,
    #[doc = "0x18 - Raw interrupt status register, SSPRIS on page 3-10"]
    pub sspris: SSPRIS,
    #[doc = "0x1c - Masked interrupt status register, SSPMIS on page 3-11"]
    pub sspmis: SSPMIS,
    #[doc = "0x20 - Interrupt clear register, SSPICR on page 3-11"]
    pub sspicr: SSPICR,
    #[doc = "0x24 - DMA control register, SSPDMACR on page 3-12"]
    pub sspdmacr: SSPDMACR,
    _reserved10: [u8; 4024usize],
    #[doc = "0xfe0 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid0: SSPPERIPHID0,
    #[doc = "0xfe4 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid1: SSPPERIPHID1,
    #[doc = "0xfe8 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid2: SSPPERIPHID2,
    #[doc = "0xfec - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid3: SSPPERIPHID3,
    #[doc = "0xff0 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid0: SSPPCELLID0,
    #[doc = "0xff4 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid1: SSPPCELLID1,
    #[doc = "0xff8 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid2: SSPPCELLID2,
    #[doc = "0xffc - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid3: SSPPCELLID3,
}
#[doc = "Control register 0, SSPCR0 on page 3-4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspcr0](sspcr0) module"]
pub type SSPCR0 = crate::Reg<u32, _SSPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPCR0;
#[doc = "`read()` method returns [sspcr0::R](sspcr0::R) reader structure"]
impl crate::Readable for SSPCR0 {}
#[doc = "`write(|w| ..)` method takes [sspcr0::W](sspcr0::W) writer structure"]
impl crate::Writable for SSPCR0 {}
#[doc = "Control register 0, SSPCR0 on page 3-4"]
pub mod sspcr0;
#[doc = "Control register 1, SSPCR1 on page 3-5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspcr1](sspcr1) module"]
pub type SSPCR1 = crate::Reg<u32, _SSPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPCR1;
#[doc = "`read()` method returns [sspcr1::R](sspcr1::R) reader structure"]
impl crate::Readable for SSPCR1 {}
#[doc = "`write(|w| ..)` method takes [sspcr1::W](sspcr1::W) writer structure"]
impl crate::Writable for SSPCR1 {}
#[doc = "Control register 1, SSPCR1 on page 3-5"]
pub mod sspcr1;
#[doc = "Data register, SSPDR on page 3-6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspdr](sspdr) module"]
pub type SSPDR = crate::Reg<u32, _SSPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPDR;
#[doc = "`read()` method returns [sspdr::R](sspdr::R) reader structure"]
impl crate::Readable for SSPDR {}
#[doc = "`write(|w| ..)` method takes [sspdr::W](sspdr::W) writer structure"]
impl crate::Writable for SSPDR {}
#[doc = "Data register, SSPDR on page 3-6"]
pub mod sspdr;
#[doc = "Status register, SSPSR on page 3-7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspsr](sspsr) module"]
pub type SSPSR = crate::Reg<u32, _SSPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPSR;
#[doc = "`read()` method returns [sspsr::R](sspsr::R) reader structure"]
impl crate::Readable for SSPSR {}
#[doc = "Status register, SSPSR on page 3-7"]
pub mod sspsr;
#[doc = "Clock prescale register, SSPCPSR on page 3-8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspcpsr](sspcpsr) module"]
pub type SSPCPSR = crate::Reg<u32, _SSPCPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPCPSR;
#[doc = "`read()` method returns [sspcpsr::R](sspcpsr::R) reader structure"]
impl crate::Readable for SSPCPSR {}
#[doc = "`write(|w| ..)` method takes [sspcpsr::W](sspcpsr::W) writer structure"]
impl crate::Writable for SSPCPSR {}
#[doc = "Clock prescale register, SSPCPSR on page 3-8"]
pub mod sspcpsr;
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspimsc](sspimsc) module"]
pub type SSPIMSC = crate::Reg<u32, _SSPIMSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPIMSC;
#[doc = "`read()` method returns [sspimsc::R](sspimsc::R) reader structure"]
impl crate::Readable for SSPIMSC {}
#[doc = "`write(|w| ..)` method takes [sspimsc::W](sspimsc::W) writer structure"]
impl crate::Writable for SSPIMSC {}
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
pub mod sspimsc;
#[doc = "Raw interrupt status register, SSPRIS on page 3-10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspris](sspris) module"]
pub type SSPRIS = crate::Reg<u32, _SSPRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPRIS;
#[doc = "`read()` method returns [sspris::R](sspris::R) reader structure"]
impl crate::Readable for SSPRIS {}
#[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
pub mod sspris;
#[doc = "Masked interrupt status register, SSPMIS on page 3-11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspmis](sspmis) module"]
pub type SSPMIS = crate::Reg<u32, _SSPMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPMIS;
#[doc = "`read()` method returns [sspmis::R](sspmis::R) reader structure"]
impl crate::Readable for SSPMIS {}
#[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
pub mod sspmis;
#[doc = "Interrupt clear register, SSPICR on page 3-11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspicr](sspicr) module"]
pub type SSPICR = crate::Reg<u32, _SSPICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPICR;
#[doc = "`read()` method returns [sspicr::R](sspicr::R) reader structure"]
impl crate::Readable for SSPICR {}
#[doc = "`write(|w| ..)` method takes [sspicr::W](sspicr::W) writer structure"]
impl crate::Writable for SSPICR {}
#[doc = "Interrupt clear register, SSPICR on page 3-11"]
pub mod sspicr;
#[doc = "DMA control register, SSPDMACR on page 3-12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspdmacr](sspdmacr) module"]
pub type SSPDMACR = crate::Reg<u32, _SSPDMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPDMACR;
#[doc = "`read()` method returns [sspdmacr::R](sspdmacr::R) reader structure"]
impl crate::Readable for SSPDMACR {}
#[doc = "`write(|w| ..)` method takes [sspdmacr::W](sspdmacr::W) writer structure"]
impl crate::Writable for SSPDMACR {}
#[doc = "DMA control register, SSPDMACR on page 3-12"]
pub mod sspdmacr;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspperiphid0](sspperiphid0) module"]
pub type SSPPERIPHID0 = crate::Reg<u32, _SSPPERIPHID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPPERIPHID0;
#[doc = "`read()` method returns [sspperiphid0::R](sspperiphid0::R) reader structure"]
impl crate::Readable for SSPPERIPHID0 {}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid0;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspperiphid1](sspperiphid1) module"]
pub type SSPPERIPHID1 = crate::Reg<u32, _SSPPERIPHID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPPERIPHID1;
#[doc = "`read()` method returns [sspperiphid1::R](sspperiphid1::R) reader structure"]
impl crate::Readable for SSPPERIPHID1 {}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid1;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspperiphid2](sspperiphid2) module"]
pub type SSPPERIPHID2 = crate::Reg<u32, _SSPPERIPHID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPPERIPHID2;
#[doc = "`read()` method returns [sspperiphid2::R](sspperiphid2::R) reader structure"]
impl crate::Readable for SSPPERIPHID2 {}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid2;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspperiphid3](sspperiphid3) module"]
pub type SSPPERIPHID3 = crate::Reg<u32, _SSPPERIPHID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPPERIPHID3;
#[doc = "`read()` method returns [sspperiphid3::R](sspperiphid3::R) reader structure"]
impl crate::Readable for SSPPERIPHID3 {}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid3;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssppcellid0](ssppcellid0) module"]
pub type SSPPCELLID0 = crate::Reg<u32, _SSPPCELLID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPPCELLID0;
#[doc = "`read()` method returns [ssppcellid0::R](ssppcellid0::R) reader structure"]
impl crate::Readable for SSPPCELLID0 {}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid0;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssppcellid1](ssppcellid1) module"]
pub type SSPPCELLID1 = crate::Reg<u32, _SSPPCELLID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPPCELLID1;
#[doc = "`read()` method returns [ssppcellid1::R](ssppcellid1::R) reader structure"]
impl crate::Readable for SSPPCELLID1 {}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid1;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssppcellid2](ssppcellid2) module"]
pub type SSPPCELLID2 = crate::Reg<u32, _SSPPCELLID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPPCELLID2;
#[doc = "`read()` method returns [ssppcellid2::R](ssppcellid2::R) reader structure"]
impl crate::Readable for SSPPCELLID2 {}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid2;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssppcellid3](ssppcellid3) module"]
pub type SSPPCELLID3 = crate::Reg<u32, _SSPPCELLID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPPCELLID3;
#[doc = "`read()` method returns [ssppcellid3::R](ssppcellid3::R) reader structure"]
impl crate::Readable for SSPPCELLID3 {}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid3;
