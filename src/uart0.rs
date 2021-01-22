#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register, UARTDR"]
    pub uartdr: UARTDR,
    #[doc = "0x04 - Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
    pub uartrsr: UARTRSR,
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - Flag Register, UARTFR"]
    pub uartfr: UARTFR,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - IrDA Low-Power Counter Register, UARTILPR"]
    pub uartilpr: UARTILPR,
    #[doc = "0x24 - Integer Baud Rate Register, UARTIBRD"]
    pub uartibrd: UARTIBRD,
    #[doc = "0x28 - Fractional Baud Rate Register, UARTFBRD"]
    pub uartfbrd: UARTFBRD,
    #[doc = "0x2c - Line Control Register, UARTLCR_H"]
    pub uartlcr_h: UARTLCR_H,
    #[doc = "0x30 - Control Register, UARTCR"]
    pub uartcr: UARTCR,
    #[doc = "0x34 - Interrupt FIFO Level Select Register, UARTIFLS"]
    pub uartifls: UARTIFLS,
    #[doc = "0x38 - Interrupt Mask Set/Clear Register, UARTIMSC"]
    pub uartimsc: UARTIMSC,
    #[doc = "0x3c - Raw Interrupt Status Register, UARTRIS"]
    pub uartris: UARTRIS,
    #[doc = "0x40 - Masked Interrupt Status Register, UARTMIS"]
    pub uartmis: UARTMIS,
    #[doc = "0x44 - Interrupt Clear Register, UARTICR"]
    pub uarticr: UARTICR,
    #[doc = "0x48 - DMA Control Register, UARTDMACR"]
    pub uartdmacr: UARTDMACR,
    _reserved14: [u8; 3988usize],
    #[doc = "0xfe0 - UARTPeriphID0 Register"]
    pub uartperiphid0: UARTPERIPHID0,
    #[doc = "0xfe4 - UARTPeriphID1 Register"]
    pub uartperiphid1: UARTPERIPHID1,
    #[doc = "0xfe8 - UARTPeriphID2 Register"]
    pub uartperiphid2: UARTPERIPHID2,
    #[doc = "0xfec - UARTPeriphID3 Register"]
    pub uartperiphid3: UARTPERIPHID3,
    #[doc = "0xff0 - UARTPCellID0 Register"]
    pub uartpcellid0: UARTPCELLID0,
    #[doc = "0xff4 - UARTPCellID1 Register"]
    pub uartpcellid1: UARTPCELLID1,
    #[doc = "0xff8 - UARTPCellID2 Register"]
    pub uartpcellid2: UARTPCELLID2,
    #[doc = "0xffc - UARTPCellID3 Register"]
    pub uartpcellid3: UARTPCELLID3,
}
#[doc = "Data Register, UARTDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartdr](uartdr) module"]
pub type UARTDR = crate::Reg<u32, _UARTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTDR;
#[doc = "`read()` method returns [uartdr::R](uartdr::R) reader structure"]
impl crate::Readable for UARTDR {}
#[doc = "`write(|w| ..)` method takes [uartdr::W](uartdr::W) writer structure"]
impl crate::Writable for UARTDR {}
#[doc = "Data Register, UARTDR"]
pub mod uartdr;
#[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartrsr](uartrsr) module"]
pub type UARTRSR = crate::Reg<u32, _UARTRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRSR;
#[doc = "`read()` method returns [uartrsr::R](uartrsr::R) reader structure"]
impl crate::Readable for UARTRSR {}
#[doc = "`write(|w| ..)` method takes [uartrsr::W](uartrsr::W) writer structure"]
impl crate::Writable for UARTRSR {}
#[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
pub mod uartrsr;
#[doc = "Flag Register, UARTFR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartfr](uartfr) module"]
pub type UARTFR = crate::Reg<u32, _UARTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTFR;
#[doc = "`read()` method returns [uartfr::R](uartfr::R) reader structure"]
impl crate::Readable for UARTFR {}
#[doc = "Flag Register, UARTFR"]
pub mod uartfr;
#[doc = "IrDA Low-Power Counter Register, UARTILPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartilpr](uartilpr) module"]
pub type UARTILPR = crate::Reg<u32, _UARTILPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTILPR;
#[doc = "`read()` method returns [uartilpr::R](uartilpr::R) reader structure"]
impl crate::Readable for UARTILPR {}
#[doc = "`write(|w| ..)` method takes [uartilpr::W](uartilpr::W) writer structure"]
impl crate::Writable for UARTILPR {}
#[doc = "IrDA Low-Power Counter Register, UARTILPR"]
pub mod uartilpr;
#[doc = "Integer Baud Rate Register, UARTIBRD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartibrd](uartibrd) module"]
pub type UARTIBRD = crate::Reg<u32, _UARTIBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIBRD;
#[doc = "`read()` method returns [uartibrd::R](uartibrd::R) reader structure"]
impl crate::Readable for UARTIBRD {}
#[doc = "`write(|w| ..)` method takes [uartibrd::W](uartibrd::W) writer structure"]
impl crate::Writable for UARTIBRD {}
#[doc = "Integer Baud Rate Register, UARTIBRD"]
pub mod uartibrd;
#[doc = "Fractional Baud Rate Register, UARTFBRD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartfbrd](uartfbrd) module"]
pub type UARTFBRD = crate::Reg<u32, _UARTFBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTFBRD;
#[doc = "`read()` method returns [uartfbrd::R](uartfbrd::R) reader structure"]
impl crate::Readable for UARTFBRD {}
#[doc = "`write(|w| ..)` method takes [uartfbrd::W](uartfbrd::W) writer structure"]
impl crate::Writable for UARTFBRD {}
#[doc = "Fractional Baud Rate Register, UARTFBRD"]
pub mod uartfbrd;
#[doc = "Line Control Register, UARTLCR_H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartlcr_h](uartlcr_h) module"]
pub type UARTLCR_H = crate::Reg<u32, _UARTLCR_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTLCR_H;
#[doc = "`read()` method returns [uartlcr_h::R](uartlcr_h::R) reader structure"]
impl crate::Readable for UARTLCR_H {}
#[doc = "`write(|w| ..)` method takes [uartlcr_h::W](uartlcr_h::W) writer structure"]
impl crate::Writable for UARTLCR_H {}
#[doc = "Line Control Register, UARTLCR_H"]
pub mod uartlcr_h;
#[doc = "Control Register, UARTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartcr](uartcr) module"]
pub type UARTCR = crate::Reg<u32, _UARTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCR;
#[doc = "`read()` method returns [uartcr::R](uartcr::R) reader structure"]
impl crate::Readable for UARTCR {}
#[doc = "`write(|w| ..)` method takes [uartcr::W](uartcr::W) writer structure"]
impl crate::Writable for UARTCR {}
#[doc = "Control Register, UARTCR"]
pub mod uartcr;
#[doc = "Interrupt FIFO Level Select Register, UARTIFLS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartifls](uartifls) module"]
pub type UARTIFLS = crate::Reg<u32, _UARTIFLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIFLS;
#[doc = "`read()` method returns [uartifls::R](uartifls::R) reader structure"]
impl crate::Readable for UARTIFLS {}
#[doc = "`write(|w| ..)` method takes [uartifls::W](uartifls::W) writer structure"]
impl crate::Writable for UARTIFLS {}
#[doc = "Interrupt FIFO Level Select Register, UARTIFLS"]
pub mod uartifls;
#[doc = "Interrupt Mask Set/Clear Register, UARTIMSC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartimsc](uartimsc) module"]
pub type UARTIMSC = crate::Reg<u32, _UARTIMSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIMSC;
#[doc = "`read()` method returns [uartimsc::R](uartimsc::R) reader structure"]
impl crate::Readable for UARTIMSC {}
#[doc = "`write(|w| ..)` method takes [uartimsc::W](uartimsc::W) writer structure"]
impl crate::Writable for UARTIMSC {}
#[doc = "Interrupt Mask Set/Clear Register, UARTIMSC"]
pub mod uartimsc;
#[doc = "Raw Interrupt Status Register, UARTRIS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartris](uartris) module"]
pub type UARTRIS = crate::Reg<u32, _UARTRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRIS;
#[doc = "`read()` method returns [uartris::R](uartris::R) reader structure"]
impl crate::Readable for UARTRIS {}
#[doc = "Raw Interrupt Status Register, UARTRIS"]
pub mod uartris;
#[doc = "Masked Interrupt Status Register, UARTMIS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartmis](uartmis) module"]
pub type UARTMIS = crate::Reg<u32, _UARTMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTMIS;
#[doc = "`read()` method returns [uartmis::R](uartmis::R) reader structure"]
impl crate::Readable for UARTMIS {}
#[doc = "Masked Interrupt Status Register, UARTMIS"]
pub mod uartmis;
#[doc = "Interrupt Clear Register, UARTICR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uarticr](uarticr) module"]
pub type UARTICR = crate::Reg<u32, _UARTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTICR;
#[doc = "`read()` method returns [uarticr::R](uarticr::R) reader structure"]
impl crate::Readable for UARTICR {}
#[doc = "`write(|w| ..)` method takes [uarticr::W](uarticr::W) writer structure"]
impl crate::Writable for UARTICR {}
#[doc = "Interrupt Clear Register, UARTICR"]
pub mod uarticr;
#[doc = "DMA Control Register, UARTDMACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartdmacr](uartdmacr) module"]
pub type UARTDMACR = crate::Reg<u32, _UARTDMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTDMACR;
#[doc = "`read()` method returns [uartdmacr::R](uartdmacr::R) reader structure"]
impl crate::Readable for UARTDMACR {}
#[doc = "`write(|w| ..)` method takes [uartdmacr::W](uartdmacr::W) writer structure"]
impl crate::Writable for UARTDMACR {}
#[doc = "DMA Control Register, UARTDMACR"]
pub mod uartdmacr;
#[doc = "UARTPeriphID0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartperiphid0](uartperiphid0) module"]
pub type UARTPERIPHID0 = crate::Reg<u32, _UARTPERIPHID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPERIPHID0;
#[doc = "`read()` method returns [uartperiphid0::R](uartperiphid0::R) reader structure"]
impl crate::Readable for UARTPERIPHID0 {}
#[doc = "UARTPeriphID0 Register"]
pub mod uartperiphid0;
#[doc = "UARTPeriphID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartperiphid1](uartperiphid1) module"]
pub type UARTPERIPHID1 = crate::Reg<u32, _UARTPERIPHID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPERIPHID1;
#[doc = "`read()` method returns [uartperiphid1::R](uartperiphid1::R) reader structure"]
impl crate::Readable for UARTPERIPHID1 {}
#[doc = "UARTPeriphID1 Register"]
pub mod uartperiphid1;
#[doc = "UARTPeriphID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartperiphid2](uartperiphid2) module"]
pub type UARTPERIPHID2 = crate::Reg<u32, _UARTPERIPHID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPERIPHID2;
#[doc = "`read()` method returns [uartperiphid2::R](uartperiphid2::R) reader structure"]
impl crate::Readable for UARTPERIPHID2 {}
#[doc = "UARTPeriphID2 Register"]
pub mod uartperiphid2;
#[doc = "UARTPeriphID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartperiphid3](uartperiphid3) module"]
pub type UARTPERIPHID3 = crate::Reg<u32, _UARTPERIPHID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPERIPHID3;
#[doc = "`read()` method returns [uartperiphid3::R](uartperiphid3::R) reader structure"]
impl crate::Readable for UARTPERIPHID3 {}
#[doc = "UARTPeriphID3 Register"]
pub mod uartperiphid3;
#[doc = "UARTPCellID0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartpcellid0](uartpcellid0) module"]
pub type UARTPCELLID0 = crate::Reg<u32, _UARTPCELLID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPCELLID0;
#[doc = "`read()` method returns [uartpcellid0::R](uartpcellid0::R) reader structure"]
impl crate::Readable for UARTPCELLID0 {}
#[doc = "UARTPCellID0 Register"]
pub mod uartpcellid0;
#[doc = "UARTPCellID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartpcellid1](uartpcellid1) module"]
pub type UARTPCELLID1 = crate::Reg<u32, _UARTPCELLID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPCELLID1;
#[doc = "`read()` method returns [uartpcellid1::R](uartpcellid1::R) reader structure"]
impl crate::Readable for UARTPCELLID1 {}
#[doc = "UARTPCellID1 Register"]
pub mod uartpcellid1;
#[doc = "UARTPCellID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartpcellid2](uartpcellid2) module"]
pub type UARTPCELLID2 = crate::Reg<u32, _UARTPCELLID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPCELLID2;
#[doc = "`read()` method returns [uartpcellid2::R](uartpcellid2::R) reader structure"]
impl crate::Readable for UARTPCELLID2 {}
#[doc = "UARTPCellID2 Register"]
pub mod uartpcellid2;
#[doc = "UARTPCellID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartpcellid3](uartpcellid3) module"]
pub type UARTPCELLID3 = crate::Reg<u32, _UARTPCELLID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPCELLID3;
#[doc = "`read()` method returns [uartpcellid3::R](uartpcellid3::R) reader structure"]
impl crate::Readable for UARTPCELLID3 {}
#[doc = "UARTPCellID3 Register"]
pub mod uartpcellid3;
