#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and status register"]
    pub ch0_csr: CH0_CSR,
    #[doc = "0x04 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch0_div: CH0_DIV,
    #[doc = "0x08 - Direct access to the PWM counter"]
    pub ch0_ctr: CH0_CTR,
    #[doc = "0x0c - Counter compare values"]
    pub ch0_cc: CH0_CC,
    #[doc = "0x10 - Counter wrap value"]
    pub ch0_top: CH0_TOP,
    #[doc = "0x14 - Control and status register"]
    pub ch1_csr: CH1_CSR,
    #[doc = "0x18 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch1_div: CH1_DIV,
    #[doc = "0x1c - Direct access to the PWM counter"]
    pub ch1_ctr: CH1_CTR,
    #[doc = "0x20 - Counter compare values"]
    pub ch1_cc: CH1_CC,
    #[doc = "0x24 - Counter wrap value"]
    pub ch1_top: CH1_TOP,
    #[doc = "0x28 - Control and status register"]
    pub ch2_csr: CH2_CSR,
    #[doc = "0x2c - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch2_div: CH2_DIV,
    #[doc = "0x30 - Direct access to the PWM counter"]
    pub ch2_ctr: CH2_CTR,
    #[doc = "0x34 - Counter compare values"]
    pub ch2_cc: CH2_CC,
    #[doc = "0x38 - Counter wrap value"]
    pub ch2_top: CH2_TOP,
    #[doc = "0x3c - Control and status register"]
    pub ch3_csr: CH3_CSR,
    #[doc = "0x40 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch3_div: CH3_DIV,
    #[doc = "0x44 - Direct access to the PWM counter"]
    pub ch3_ctr: CH3_CTR,
    #[doc = "0x48 - Counter compare values"]
    pub ch3_cc: CH3_CC,
    #[doc = "0x4c - Counter wrap value"]
    pub ch3_top: CH3_TOP,
    #[doc = "0x50 - Control and status register"]
    pub ch4_csr: CH4_CSR,
    #[doc = "0x54 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch4_div: CH4_DIV,
    #[doc = "0x58 - Direct access to the PWM counter"]
    pub ch4_ctr: CH4_CTR,
    #[doc = "0x5c - Counter compare values"]
    pub ch4_cc: CH4_CC,
    #[doc = "0x60 - Counter wrap value"]
    pub ch4_top: CH4_TOP,
    #[doc = "0x64 - Control and status register"]
    pub ch5_csr: CH5_CSR,
    #[doc = "0x68 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch5_div: CH5_DIV,
    #[doc = "0x6c - Direct access to the PWM counter"]
    pub ch5_ctr: CH5_CTR,
    #[doc = "0x70 - Counter compare values"]
    pub ch5_cc: CH5_CC,
    #[doc = "0x74 - Counter wrap value"]
    pub ch5_top: CH5_TOP,
    #[doc = "0x78 - Control and status register"]
    pub ch6_csr: CH6_CSR,
    #[doc = "0x7c - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch6_div: CH6_DIV,
    #[doc = "0x80 - Direct access to the PWM counter"]
    pub ch6_ctr: CH6_CTR,
    #[doc = "0x84 - Counter compare values"]
    pub ch6_cc: CH6_CC,
    #[doc = "0x88 - Counter wrap value"]
    pub ch6_top: CH6_TOP,
    #[doc = "0x8c - Control and status register"]
    pub ch7_csr: CH7_CSR,
    #[doc = "0x90 - INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
    pub ch7_div: CH7_DIV,
    #[doc = "0x94 - Direct access to the PWM counter"]
    pub ch7_ctr: CH7_CTR,
    #[doc = "0x98 - Counter compare values"]
    pub ch7_cc: CH7_CC,
    #[doc = "0x9c - Counter wrap value"]
    pub ch7_top: CH7_TOP,
    #[doc = "0xa0 - This register aliases the CSR_EN bits for all channels.\\n Writing to this register allows multiple channels to be enabled\\n or disabled simultaneously, so they can run in perfect sync.\\n For each channel, there is only one physical EN register bit,\\n which can be accessed through here or CHx_CSR."]
    pub en: EN,
    #[doc = "0xa4 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0xa8 - Interrupt Enable"]
    pub inte: INTE,
    #[doc = "0xac - Interrupt Force"]
    pub intf: INTF,
    #[doc = "0xb0 - Interrupt status after masking & forcing"]
    pub ints: INTS,
}
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_csr](ch0_csr) module"]
pub type CH0_CSR = crate::Reg<u32, _CH0_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CSR;
#[doc = "`read()` method returns [ch0_csr::R](ch0_csr::R) reader structure"]
impl crate::Readable for CH0_CSR {}
#[doc = "`write(|w| ..)` method takes [ch0_csr::W](ch0_csr::W) writer structure"]
impl crate::Writable for CH0_CSR {}
#[doc = "Control and status register"]
pub mod ch0_csr;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_div](ch0_div) module"]
pub type CH0_DIV = crate::Reg<u32, _CH0_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_DIV;
#[doc = "`read()` method returns [ch0_div::R](ch0_div::R) reader structure"]
impl crate::Readable for CH0_DIV {}
#[doc = "`write(|w| ..)` method takes [ch0_div::W](ch0_div::W) writer structure"]
impl crate::Writable for CH0_DIV {}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch0_div;
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_ctr](ch0_ctr) module"]
pub type CH0_CTR = crate::Reg<u32, _CH0_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CTR;
#[doc = "`read()` method returns [ch0_ctr::R](ch0_ctr::R) reader structure"]
impl crate::Readable for CH0_CTR {}
#[doc = "`write(|w| ..)` method takes [ch0_ctr::W](ch0_ctr::W) writer structure"]
impl crate::Writable for CH0_CTR {}
#[doc = "Direct access to the PWM counter"]
pub mod ch0_ctr;
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_cc](ch0_cc) module"]
pub type CH0_CC = crate::Reg<u32, _CH0_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CC;
#[doc = "`read()` method returns [ch0_cc::R](ch0_cc::R) reader structure"]
impl crate::Readable for CH0_CC {}
#[doc = "`write(|w| ..)` method takes [ch0_cc::W](ch0_cc::W) writer structure"]
impl crate::Writable for CH0_CC {}
#[doc = "Counter compare values"]
pub mod ch0_cc;
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_top](ch0_top) module"]
pub type CH0_TOP = crate::Reg<u32, _CH0_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_TOP;
#[doc = "`read()` method returns [ch0_top::R](ch0_top::R) reader structure"]
impl crate::Readable for CH0_TOP {}
#[doc = "`write(|w| ..)` method takes [ch0_top::W](ch0_top::W) writer structure"]
impl crate::Writable for CH0_TOP {}
#[doc = "Counter wrap value"]
pub mod ch0_top;
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_csr](ch1_csr) module"]
pub type CH1_CSR = crate::Reg<u32, _CH1_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CSR;
#[doc = "`read()` method returns [ch1_csr::R](ch1_csr::R) reader structure"]
impl crate::Readable for CH1_CSR {}
#[doc = "`write(|w| ..)` method takes [ch1_csr::W](ch1_csr::W) writer structure"]
impl crate::Writable for CH1_CSR {}
#[doc = "Control and status register"]
pub mod ch1_csr;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_div](ch1_div) module"]
pub type CH1_DIV = crate::Reg<u32, _CH1_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_DIV;
#[doc = "`read()` method returns [ch1_div::R](ch1_div::R) reader structure"]
impl crate::Readable for CH1_DIV {}
#[doc = "`write(|w| ..)` method takes [ch1_div::W](ch1_div::W) writer structure"]
impl crate::Writable for CH1_DIV {}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch1_div;
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_ctr](ch1_ctr) module"]
pub type CH1_CTR = crate::Reg<u32, _CH1_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CTR;
#[doc = "`read()` method returns [ch1_ctr::R](ch1_ctr::R) reader structure"]
impl crate::Readable for CH1_CTR {}
#[doc = "`write(|w| ..)` method takes [ch1_ctr::W](ch1_ctr::W) writer structure"]
impl crate::Writable for CH1_CTR {}
#[doc = "Direct access to the PWM counter"]
pub mod ch1_ctr;
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_cc](ch1_cc) module"]
pub type CH1_CC = crate::Reg<u32, _CH1_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CC;
#[doc = "`read()` method returns [ch1_cc::R](ch1_cc::R) reader structure"]
impl crate::Readable for CH1_CC {}
#[doc = "`write(|w| ..)` method takes [ch1_cc::W](ch1_cc::W) writer structure"]
impl crate::Writable for CH1_CC {}
#[doc = "Counter compare values"]
pub mod ch1_cc;
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_top](ch1_top) module"]
pub type CH1_TOP = crate::Reg<u32, _CH1_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_TOP;
#[doc = "`read()` method returns [ch1_top::R](ch1_top::R) reader structure"]
impl crate::Readable for CH1_TOP {}
#[doc = "`write(|w| ..)` method takes [ch1_top::W](ch1_top::W) writer structure"]
impl crate::Writable for CH1_TOP {}
#[doc = "Counter wrap value"]
pub mod ch1_top;
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_csr](ch2_csr) module"]
pub type CH2_CSR = crate::Reg<u32, _CH2_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CSR;
#[doc = "`read()` method returns [ch2_csr::R](ch2_csr::R) reader structure"]
impl crate::Readable for CH2_CSR {}
#[doc = "`write(|w| ..)` method takes [ch2_csr::W](ch2_csr::W) writer structure"]
impl crate::Writable for CH2_CSR {}
#[doc = "Control and status register"]
pub mod ch2_csr;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_div](ch2_div) module"]
pub type CH2_DIV = crate::Reg<u32, _CH2_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_DIV;
#[doc = "`read()` method returns [ch2_div::R](ch2_div::R) reader structure"]
impl crate::Readable for CH2_DIV {}
#[doc = "`write(|w| ..)` method takes [ch2_div::W](ch2_div::W) writer structure"]
impl crate::Writable for CH2_DIV {}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch2_div;
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_ctr](ch2_ctr) module"]
pub type CH2_CTR = crate::Reg<u32, _CH2_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CTR;
#[doc = "`read()` method returns [ch2_ctr::R](ch2_ctr::R) reader structure"]
impl crate::Readable for CH2_CTR {}
#[doc = "`write(|w| ..)` method takes [ch2_ctr::W](ch2_ctr::W) writer structure"]
impl crate::Writable for CH2_CTR {}
#[doc = "Direct access to the PWM counter"]
pub mod ch2_ctr;
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_cc](ch2_cc) module"]
pub type CH2_CC = crate::Reg<u32, _CH2_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CC;
#[doc = "`read()` method returns [ch2_cc::R](ch2_cc::R) reader structure"]
impl crate::Readable for CH2_CC {}
#[doc = "`write(|w| ..)` method takes [ch2_cc::W](ch2_cc::W) writer structure"]
impl crate::Writable for CH2_CC {}
#[doc = "Counter compare values"]
pub mod ch2_cc;
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_top](ch2_top) module"]
pub type CH2_TOP = crate::Reg<u32, _CH2_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_TOP;
#[doc = "`read()` method returns [ch2_top::R](ch2_top::R) reader structure"]
impl crate::Readable for CH2_TOP {}
#[doc = "`write(|w| ..)` method takes [ch2_top::W](ch2_top::W) writer structure"]
impl crate::Writable for CH2_TOP {}
#[doc = "Counter wrap value"]
pub mod ch2_top;
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_csr](ch3_csr) module"]
pub type CH3_CSR = crate::Reg<u32, _CH3_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CSR;
#[doc = "`read()` method returns [ch3_csr::R](ch3_csr::R) reader structure"]
impl crate::Readable for CH3_CSR {}
#[doc = "`write(|w| ..)` method takes [ch3_csr::W](ch3_csr::W) writer structure"]
impl crate::Writable for CH3_CSR {}
#[doc = "Control and status register"]
pub mod ch3_csr;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_div](ch3_div) module"]
pub type CH3_DIV = crate::Reg<u32, _CH3_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_DIV;
#[doc = "`read()` method returns [ch3_div::R](ch3_div::R) reader structure"]
impl crate::Readable for CH3_DIV {}
#[doc = "`write(|w| ..)` method takes [ch3_div::W](ch3_div::W) writer structure"]
impl crate::Writable for CH3_DIV {}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch3_div;
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_ctr](ch3_ctr) module"]
pub type CH3_CTR = crate::Reg<u32, _CH3_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CTR;
#[doc = "`read()` method returns [ch3_ctr::R](ch3_ctr::R) reader structure"]
impl crate::Readable for CH3_CTR {}
#[doc = "`write(|w| ..)` method takes [ch3_ctr::W](ch3_ctr::W) writer structure"]
impl crate::Writable for CH3_CTR {}
#[doc = "Direct access to the PWM counter"]
pub mod ch3_ctr;
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_cc](ch3_cc) module"]
pub type CH3_CC = crate::Reg<u32, _CH3_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CC;
#[doc = "`read()` method returns [ch3_cc::R](ch3_cc::R) reader structure"]
impl crate::Readable for CH3_CC {}
#[doc = "`write(|w| ..)` method takes [ch3_cc::W](ch3_cc::W) writer structure"]
impl crate::Writable for CH3_CC {}
#[doc = "Counter compare values"]
pub mod ch3_cc;
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_top](ch3_top) module"]
pub type CH3_TOP = crate::Reg<u32, _CH3_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_TOP;
#[doc = "`read()` method returns [ch3_top::R](ch3_top::R) reader structure"]
impl crate::Readable for CH3_TOP {}
#[doc = "`write(|w| ..)` method takes [ch3_top::W](ch3_top::W) writer structure"]
impl crate::Writable for CH3_TOP {}
#[doc = "Counter wrap value"]
pub mod ch3_top;
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_csr](ch4_csr) module"]
pub type CH4_CSR = crate::Reg<u32, _CH4_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CSR;
#[doc = "`read()` method returns [ch4_csr::R](ch4_csr::R) reader structure"]
impl crate::Readable for CH4_CSR {}
#[doc = "`write(|w| ..)` method takes [ch4_csr::W](ch4_csr::W) writer structure"]
impl crate::Writable for CH4_CSR {}
#[doc = "Control and status register"]
pub mod ch4_csr;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_div](ch4_div) module"]
pub type CH4_DIV = crate::Reg<u32, _CH4_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_DIV;
#[doc = "`read()` method returns [ch4_div::R](ch4_div::R) reader structure"]
impl crate::Readable for CH4_DIV {}
#[doc = "`write(|w| ..)` method takes [ch4_div::W](ch4_div::W) writer structure"]
impl crate::Writable for CH4_DIV {}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch4_div;
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_ctr](ch4_ctr) module"]
pub type CH4_CTR = crate::Reg<u32, _CH4_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CTR;
#[doc = "`read()` method returns [ch4_ctr::R](ch4_ctr::R) reader structure"]
impl crate::Readable for CH4_CTR {}
#[doc = "`write(|w| ..)` method takes [ch4_ctr::W](ch4_ctr::W) writer structure"]
impl crate::Writable for CH4_CTR {}
#[doc = "Direct access to the PWM counter"]
pub mod ch4_ctr;
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_cc](ch4_cc) module"]
pub type CH4_CC = crate::Reg<u32, _CH4_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CC;
#[doc = "`read()` method returns [ch4_cc::R](ch4_cc::R) reader structure"]
impl crate::Readable for CH4_CC {}
#[doc = "`write(|w| ..)` method takes [ch4_cc::W](ch4_cc::W) writer structure"]
impl crate::Writable for CH4_CC {}
#[doc = "Counter compare values"]
pub mod ch4_cc;
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_top](ch4_top) module"]
pub type CH4_TOP = crate::Reg<u32, _CH4_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_TOP;
#[doc = "`read()` method returns [ch4_top::R](ch4_top::R) reader structure"]
impl crate::Readable for CH4_TOP {}
#[doc = "`write(|w| ..)` method takes [ch4_top::W](ch4_top::W) writer structure"]
impl crate::Writable for CH4_TOP {}
#[doc = "Counter wrap value"]
pub mod ch4_top;
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_csr](ch5_csr) module"]
pub type CH5_CSR = crate::Reg<u32, _CH5_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CSR;
#[doc = "`read()` method returns [ch5_csr::R](ch5_csr::R) reader structure"]
impl crate::Readable for CH5_CSR {}
#[doc = "`write(|w| ..)` method takes [ch5_csr::W](ch5_csr::W) writer structure"]
impl crate::Writable for CH5_CSR {}
#[doc = "Control and status register"]
pub mod ch5_csr;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_div](ch5_div) module"]
pub type CH5_DIV = crate::Reg<u32, _CH5_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_DIV;
#[doc = "`read()` method returns [ch5_div::R](ch5_div::R) reader structure"]
impl crate::Readable for CH5_DIV {}
#[doc = "`write(|w| ..)` method takes [ch5_div::W](ch5_div::W) writer structure"]
impl crate::Writable for CH5_DIV {}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch5_div;
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_ctr](ch5_ctr) module"]
pub type CH5_CTR = crate::Reg<u32, _CH5_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CTR;
#[doc = "`read()` method returns [ch5_ctr::R](ch5_ctr::R) reader structure"]
impl crate::Readable for CH5_CTR {}
#[doc = "`write(|w| ..)` method takes [ch5_ctr::W](ch5_ctr::W) writer structure"]
impl crate::Writable for CH5_CTR {}
#[doc = "Direct access to the PWM counter"]
pub mod ch5_ctr;
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_cc](ch5_cc) module"]
pub type CH5_CC = crate::Reg<u32, _CH5_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CC;
#[doc = "`read()` method returns [ch5_cc::R](ch5_cc::R) reader structure"]
impl crate::Readable for CH5_CC {}
#[doc = "`write(|w| ..)` method takes [ch5_cc::W](ch5_cc::W) writer structure"]
impl crate::Writable for CH5_CC {}
#[doc = "Counter compare values"]
pub mod ch5_cc;
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_top](ch5_top) module"]
pub type CH5_TOP = crate::Reg<u32, _CH5_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_TOP;
#[doc = "`read()` method returns [ch5_top::R](ch5_top::R) reader structure"]
impl crate::Readable for CH5_TOP {}
#[doc = "`write(|w| ..)` method takes [ch5_top::W](ch5_top::W) writer structure"]
impl crate::Writable for CH5_TOP {}
#[doc = "Counter wrap value"]
pub mod ch5_top;
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_csr](ch6_csr) module"]
pub type CH6_CSR = crate::Reg<u32, _CH6_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CSR;
#[doc = "`read()` method returns [ch6_csr::R](ch6_csr::R) reader structure"]
impl crate::Readable for CH6_CSR {}
#[doc = "`write(|w| ..)` method takes [ch6_csr::W](ch6_csr::W) writer structure"]
impl crate::Writable for CH6_CSR {}
#[doc = "Control and status register"]
pub mod ch6_csr;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_div](ch6_div) module"]
pub type CH6_DIV = crate::Reg<u32, _CH6_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_DIV;
#[doc = "`read()` method returns [ch6_div::R](ch6_div::R) reader structure"]
impl crate::Readable for CH6_DIV {}
#[doc = "`write(|w| ..)` method takes [ch6_div::W](ch6_div::W) writer structure"]
impl crate::Writable for CH6_DIV {}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch6_div;
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_ctr](ch6_ctr) module"]
pub type CH6_CTR = crate::Reg<u32, _CH6_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CTR;
#[doc = "`read()` method returns [ch6_ctr::R](ch6_ctr::R) reader structure"]
impl crate::Readable for CH6_CTR {}
#[doc = "`write(|w| ..)` method takes [ch6_ctr::W](ch6_ctr::W) writer structure"]
impl crate::Writable for CH6_CTR {}
#[doc = "Direct access to the PWM counter"]
pub mod ch6_ctr;
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_cc](ch6_cc) module"]
pub type CH6_CC = crate::Reg<u32, _CH6_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CC;
#[doc = "`read()` method returns [ch6_cc::R](ch6_cc::R) reader structure"]
impl crate::Readable for CH6_CC {}
#[doc = "`write(|w| ..)` method takes [ch6_cc::W](ch6_cc::W) writer structure"]
impl crate::Writable for CH6_CC {}
#[doc = "Counter compare values"]
pub mod ch6_cc;
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_top](ch6_top) module"]
pub type CH6_TOP = crate::Reg<u32, _CH6_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_TOP;
#[doc = "`read()` method returns [ch6_top::R](ch6_top::R) reader structure"]
impl crate::Readable for CH6_TOP {}
#[doc = "`write(|w| ..)` method takes [ch6_top::W](ch6_top::W) writer structure"]
impl crate::Writable for CH6_TOP {}
#[doc = "Counter wrap value"]
pub mod ch6_top;
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_csr](ch7_csr) module"]
pub type CH7_CSR = crate::Reg<u32, _CH7_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CSR;
#[doc = "`read()` method returns [ch7_csr::R](ch7_csr::R) reader structure"]
impl crate::Readable for CH7_CSR {}
#[doc = "`write(|w| ..)` method takes [ch7_csr::W](ch7_csr::W) writer structure"]
impl crate::Writable for CH7_CSR {}
#[doc = "Control and status register"]
pub mod ch7_csr;
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_div](ch7_div) module"]
pub type CH7_DIV = crate::Reg<u32, _CH7_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_DIV;
#[doc = "`read()` method returns [ch7_div::R](ch7_div::R) reader structure"]
impl crate::Readable for CH7_DIV {}
#[doc = "`write(|w| ..)` method takes [ch7_div::W](ch7_div::W) writer structure"]
impl crate::Writable for CH7_DIV {}
#[doc = "INT and FRAC form a fixed-point fractional number.\\n Counting rate is system clock frequency divided by this number.\\n Fractional division uses simple 1st-order sigma-delta."]
pub mod ch7_div;
#[doc = "Direct access to the PWM counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_ctr](ch7_ctr) module"]
pub type CH7_CTR = crate::Reg<u32, _CH7_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CTR;
#[doc = "`read()` method returns [ch7_ctr::R](ch7_ctr::R) reader structure"]
impl crate::Readable for CH7_CTR {}
#[doc = "`write(|w| ..)` method takes [ch7_ctr::W](ch7_ctr::W) writer structure"]
impl crate::Writable for CH7_CTR {}
#[doc = "Direct access to the PWM counter"]
pub mod ch7_ctr;
#[doc = "Counter compare values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_cc](ch7_cc) module"]
pub type CH7_CC = crate::Reg<u32, _CH7_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CC;
#[doc = "`read()` method returns [ch7_cc::R](ch7_cc::R) reader structure"]
impl crate::Readable for CH7_CC {}
#[doc = "`write(|w| ..)` method takes [ch7_cc::W](ch7_cc::W) writer structure"]
impl crate::Writable for CH7_CC {}
#[doc = "Counter compare values"]
pub mod ch7_cc;
#[doc = "Counter wrap value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_top](ch7_top) module"]
pub type CH7_TOP = crate::Reg<u32, _CH7_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_TOP;
#[doc = "`read()` method returns [ch7_top::R](ch7_top::R) reader structure"]
impl crate::Readable for CH7_TOP {}
#[doc = "`write(|w| ..)` method takes [ch7_top::W](ch7_top::W) writer structure"]
impl crate::Writable for CH7_TOP {}
#[doc = "Counter wrap value"]
pub mod ch7_top;
#[doc = "This register aliases the CSR_EN bits for all channels.\\n Writing to this register allows multiple channels to be enabled\\n or disabled simultaneously, so they can run in perfect sync.\\n For each channel, there is only one physical EN register bit,\\n which can be accessed through here or CHx_CSR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en](en) module"]
pub type EN = crate::Reg<u32, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`read()` method returns [en::R](en::R) reader structure"]
impl crate::Readable for EN {}
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "This register aliases the CSR_EN bits for all channels.\\n Writing to this register allows multiple channels to be enabled\\n or disabled simultaneously, so they can run in perfect sync.\\n For each channel, there is only one physical EN register bit,\\n which can be accessed through here or CHx_CSR."]
pub mod en;
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
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inte](inte) module"]
pub type INTE = crate::Reg<u32, _INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTE;
#[doc = "`read()` method returns [inte::R](inte::R) reader structure"]
impl crate::Readable for INTE {}
#[doc = "`write(|w| ..)` method takes [inte::W](inte::W) writer structure"]
impl crate::Writable for INTE {}
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "Interrupt Force\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u32, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "`write(|w| ..)` method takes [intf::W](intf::W) writer structure"]
impl crate::Writable for INTF {}
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "Interrupt status after masking & forcing\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ints](ints) module"]
pub type INTS = crate::Reg<u32, _INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTS;
#[doc = "`read()` method returns [ints::R](ints::R) reader structure"]
impl crate::Readable for INTS {}
#[doc = "Interrupt status after masking & forcing"]
pub mod ints;
