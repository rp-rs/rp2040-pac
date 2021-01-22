#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_gpout0_ctrl: CLK_GPOUT0_CTRL,
    #[doc = "0x04 - Clock divisor, can be changed on-the-fly"]
    pub clk_gpout0_div: CLK_GPOUT0_DIV,
    #[doc = "0x08 - Indicates which src is currently selected (one-hot)"]
    pub clk_gpout0_selected: CLK_GPOUT0_SELECTED,
    #[doc = "0x0c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_gpout1_ctrl: CLK_GPOUT1_CTRL,
    #[doc = "0x10 - Clock divisor, can be changed on-the-fly"]
    pub clk_gpout1_div: CLK_GPOUT1_DIV,
    #[doc = "0x14 - Indicates which src is currently selected (one-hot)"]
    pub clk_gpout1_selected: CLK_GPOUT1_SELECTED,
    #[doc = "0x18 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_gpout2_ctrl: CLK_GPOUT2_CTRL,
    #[doc = "0x1c - Clock divisor, can be changed on-the-fly"]
    pub clk_gpout2_div: CLK_GPOUT2_DIV,
    #[doc = "0x20 - Indicates which src is currently selected (one-hot)"]
    pub clk_gpout2_selected: CLK_GPOUT2_SELECTED,
    #[doc = "0x24 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_gpout3_ctrl: CLK_GPOUT3_CTRL,
    #[doc = "0x28 - Clock divisor, can be changed on-the-fly"]
    pub clk_gpout3_div: CLK_GPOUT3_DIV,
    #[doc = "0x2c - Indicates which src is currently selected (one-hot)"]
    pub clk_gpout3_selected: CLK_GPOUT3_SELECTED,
    #[doc = "0x30 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_ref_ctrl: CLK_REF_CTRL,
    #[doc = "0x34 - Clock divisor, can be changed on-the-fly"]
    pub clk_ref_div: CLK_REF_DIV,
    #[doc = "0x38 - Indicates which src is currently selected (one-hot)"]
    pub clk_ref_selected: CLK_REF_SELECTED,
    #[doc = "0x3c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_sys_ctrl: CLK_SYS_CTRL,
    #[doc = "0x40 - Clock divisor, can be changed on-the-fly"]
    pub clk_sys_div: CLK_SYS_DIV,
    #[doc = "0x44 - Indicates which src is currently selected (one-hot)"]
    pub clk_sys_selected: CLK_SYS_SELECTED,
    #[doc = "0x48 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_peri_ctrl: CLK_PERI_CTRL,
    _reserved19: [u8; 4usize],
    #[doc = "0x50 - Indicates which src is currently selected (one-hot)"]
    pub clk_peri_selected: CLK_PERI_SELECTED,
    #[doc = "0x54 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_usb_ctrl: CLK_USB_CTRL,
    #[doc = "0x58 - Clock divisor, can be changed on-the-fly"]
    pub clk_usb_div: CLK_USB_DIV,
    #[doc = "0x5c - Indicates which src is currently selected (one-hot)"]
    pub clk_usb_selected: CLK_USB_SELECTED,
    #[doc = "0x60 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_adc_ctrl: CLK_ADC_CTRL,
    #[doc = "0x64 - Clock divisor, can be changed on-the-fly"]
    pub clk_adc_div: CLK_ADC_DIV,
    #[doc = "0x68 - Indicates which src is currently selected (one-hot)"]
    pub clk_adc_selected: CLK_ADC_SELECTED,
    #[doc = "0x6c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_rtc_ctrl: CLK_RTC_CTRL,
    #[doc = "0x70 - Clock divisor, can be changed on-the-fly"]
    pub clk_rtc_div: CLK_RTC_DIV,
    #[doc = "0x74 - Indicates which src is currently selected (one-hot)"]
    pub clk_rtc_selected: CLK_RTC_SELECTED,
    #[doc = "0x78 - "]
    pub clk_sys_resus_ctrl: CLK_SYS_RESUS_CTRL,
    #[doc = "0x7c - "]
    pub clk_sys_resus_status: CLK_SYS_RESUS_STATUS,
    #[doc = "0x80 - Reference clock frequency in kHz"]
    pub fc0_ref_khz: FC0_REF_KHZ,
    #[doc = "0x84 - Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
    pub fc0_min_khz: FC0_MIN_KHZ,
    #[doc = "0x88 - Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
    pub fc0_max_khz: FC0_MAX_KHZ,
    #[doc = "0x8c - Delays the start of frequency counting to allow the mux to settle\\n Delay is measured in multiples of the reference clock period"]
    pub fc0_delay: FC0_DELAY,
    #[doc = "0x90 - The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval\\n The default gives a test interval of 250us"]
    pub fc0_interval: FC0_INTERVAL,
    #[doc = "0x94 - Clock sent to frequency counter, set to 0 when not required\\n Writing to this register initiates the frequency count"]
    pub fc0_src: FC0_SRC,
    #[doc = "0x98 - Frequency counter status"]
    pub fc0_status: FC0_STATUS,
    #[doc = "0x9c - Result of frequency measurement, only valid when status_done=1"]
    pub fc0_result: FC0_RESULT,
    #[doc = "0xa0 - enable clock in wake mode"]
    pub wake_en0: WAKE_EN0,
    #[doc = "0xa4 - enable clock in wake mode"]
    pub wake_en1: WAKE_EN1,
    #[doc = "0xa8 - enable clock in sleep mode"]
    pub sleep_en0: SLEEP_EN0,
    #[doc = "0xac - enable clock in sleep mode"]
    pub sleep_en1: SLEEP_EN1,
    #[doc = "0xb0 - indicates the state of the clock enable"]
    pub enabled0: ENABLED0,
    #[doc = "0xb4 - indicates the state of the clock enable"]
    pub enabled1: ENABLED1,
    #[doc = "0xb8 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0xbc - Interrupt Enable"]
    pub inte: INTE,
    #[doc = "0xc0 - Interrupt Force"]
    pub intf: INTF,
    #[doc = "0xc4 - Interrupt status after masking & forcing"]
    pub ints: INTS,
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout0_ctrl](clk_gpout0_ctrl) module"]
pub type CLK_GPOUT0_CTRL = crate::Reg<u32, _CLK_GPOUT0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT0_CTRL;
#[doc = "`read()` method returns [clk_gpout0_ctrl::R](clk_gpout0_ctrl::R) reader structure"]
impl crate::Readable for CLK_GPOUT0_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_gpout0_ctrl::W](clk_gpout0_ctrl::W) writer structure"]
impl crate::Writable for CLK_GPOUT0_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout0_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout0_div](clk_gpout0_div) module"]
pub type CLK_GPOUT0_DIV = crate::Reg<u32, _CLK_GPOUT0_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT0_DIV;
#[doc = "`read()` method returns [clk_gpout0_div::R](clk_gpout0_div::R) reader structure"]
impl crate::Readable for CLK_GPOUT0_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_gpout0_div::W](clk_gpout0_div::W) writer structure"]
impl crate::Writable for CLK_GPOUT0_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout0_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout0_selected](clk_gpout0_selected) module"]
pub type CLK_GPOUT0_SELECTED = crate::Reg<u32, _CLK_GPOUT0_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT0_SELECTED;
#[doc = "`read()` method returns [clk_gpout0_selected::R](clk_gpout0_selected::R) reader structure"]
impl crate::Readable for CLK_GPOUT0_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_gpout0_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout1_ctrl](clk_gpout1_ctrl) module"]
pub type CLK_GPOUT1_CTRL = crate::Reg<u32, _CLK_GPOUT1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT1_CTRL;
#[doc = "`read()` method returns [clk_gpout1_ctrl::R](clk_gpout1_ctrl::R) reader structure"]
impl crate::Readable for CLK_GPOUT1_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_gpout1_ctrl::W](clk_gpout1_ctrl::W) writer structure"]
impl crate::Writable for CLK_GPOUT1_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout1_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout1_div](clk_gpout1_div) module"]
pub type CLK_GPOUT1_DIV = crate::Reg<u32, _CLK_GPOUT1_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT1_DIV;
#[doc = "`read()` method returns [clk_gpout1_div::R](clk_gpout1_div::R) reader structure"]
impl crate::Readable for CLK_GPOUT1_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_gpout1_div::W](clk_gpout1_div::W) writer structure"]
impl crate::Writable for CLK_GPOUT1_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout1_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout1_selected](clk_gpout1_selected) module"]
pub type CLK_GPOUT1_SELECTED = crate::Reg<u32, _CLK_GPOUT1_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT1_SELECTED;
#[doc = "`read()` method returns [clk_gpout1_selected::R](clk_gpout1_selected::R) reader structure"]
impl crate::Readable for CLK_GPOUT1_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_gpout1_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout2_ctrl](clk_gpout2_ctrl) module"]
pub type CLK_GPOUT2_CTRL = crate::Reg<u32, _CLK_GPOUT2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT2_CTRL;
#[doc = "`read()` method returns [clk_gpout2_ctrl::R](clk_gpout2_ctrl::R) reader structure"]
impl crate::Readable for CLK_GPOUT2_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_gpout2_ctrl::W](clk_gpout2_ctrl::W) writer structure"]
impl crate::Writable for CLK_GPOUT2_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout2_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout2_div](clk_gpout2_div) module"]
pub type CLK_GPOUT2_DIV = crate::Reg<u32, _CLK_GPOUT2_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT2_DIV;
#[doc = "`read()` method returns [clk_gpout2_div::R](clk_gpout2_div::R) reader structure"]
impl crate::Readable for CLK_GPOUT2_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_gpout2_div::W](clk_gpout2_div::W) writer structure"]
impl crate::Writable for CLK_GPOUT2_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout2_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout2_selected](clk_gpout2_selected) module"]
pub type CLK_GPOUT2_SELECTED = crate::Reg<u32, _CLK_GPOUT2_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT2_SELECTED;
#[doc = "`read()` method returns [clk_gpout2_selected::R](clk_gpout2_selected::R) reader structure"]
impl crate::Readable for CLK_GPOUT2_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_gpout2_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout3_ctrl](clk_gpout3_ctrl) module"]
pub type CLK_GPOUT3_CTRL = crate::Reg<u32, _CLK_GPOUT3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT3_CTRL;
#[doc = "`read()` method returns [clk_gpout3_ctrl::R](clk_gpout3_ctrl::R) reader structure"]
impl crate::Readable for CLK_GPOUT3_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_gpout3_ctrl::W](clk_gpout3_ctrl::W) writer structure"]
impl crate::Writable for CLK_GPOUT3_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout3_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout3_div](clk_gpout3_div) module"]
pub type CLK_GPOUT3_DIV = crate::Reg<u32, _CLK_GPOUT3_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT3_DIV;
#[doc = "`read()` method returns [clk_gpout3_div::R](clk_gpout3_div::R) reader structure"]
impl crate::Readable for CLK_GPOUT3_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_gpout3_div::W](clk_gpout3_div::W) writer structure"]
impl crate::Writable for CLK_GPOUT3_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout3_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gpout3_selected](clk_gpout3_selected) module"]
pub type CLK_GPOUT3_SELECTED = crate::Reg<u32, _CLK_GPOUT3_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GPOUT3_SELECTED;
#[doc = "`read()` method returns [clk_gpout3_selected::R](clk_gpout3_selected::R) reader structure"]
impl crate::Readable for CLK_GPOUT3_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_gpout3_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ref_ctrl](clk_ref_ctrl) module"]
pub type CLK_REF_CTRL = crate::Reg<u32, _CLK_REF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_REF_CTRL;
#[doc = "`read()` method returns [clk_ref_ctrl::R](clk_ref_ctrl::R) reader structure"]
impl crate::Readable for CLK_REF_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_ref_ctrl::W](clk_ref_ctrl::W) writer structure"]
impl crate::Writable for CLK_REF_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_ref_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ref_div](clk_ref_div) module"]
pub type CLK_REF_DIV = crate::Reg<u32, _CLK_REF_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_REF_DIV;
#[doc = "`read()` method returns [clk_ref_div::R](clk_ref_div::R) reader structure"]
impl crate::Readable for CLK_REF_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_ref_div::W](clk_ref_div::W) writer structure"]
impl crate::Writable for CLK_REF_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_ref_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ref_selected](clk_ref_selected) module"]
pub type CLK_REF_SELECTED = crate::Reg<u32, _CLK_REF_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_REF_SELECTED;
#[doc = "`read()` method returns [clk_ref_selected::R](clk_ref_selected::R) reader structure"]
impl crate::Readable for CLK_REF_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_ref_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_sys_ctrl](clk_sys_ctrl) module"]
pub type CLK_SYS_CTRL = crate::Reg<u32, _CLK_SYS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SYS_CTRL;
#[doc = "`read()` method returns [clk_sys_ctrl::R](clk_sys_ctrl::R) reader structure"]
impl crate::Readable for CLK_SYS_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_sys_ctrl::W](clk_sys_ctrl::W) writer structure"]
impl crate::Writable for CLK_SYS_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_sys_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_sys_div](clk_sys_div) module"]
pub type CLK_SYS_DIV = crate::Reg<u32, _CLK_SYS_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SYS_DIV;
#[doc = "`read()` method returns [clk_sys_div::R](clk_sys_div::R) reader structure"]
impl crate::Readable for CLK_SYS_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_sys_div::W](clk_sys_div::W) writer structure"]
impl crate::Writable for CLK_SYS_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_sys_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_sys_selected](clk_sys_selected) module"]
pub type CLK_SYS_SELECTED = crate::Reg<u32, _CLK_SYS_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SYS_SELECTED;
#[doc = "`read()` method returns [clk_sys_selected::R](clk_sys_selected::R) reader structure"]
impl crate::Readable for CLK_SYS_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_sys_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_peri_ctrl](clk_peri_ctrl) module"]
pub type CLK_PERI_CTRL = crate::Reg<u32, _CLK_PERI_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_PERI_CTRL;
#[doc = "`read()` method returns [clk_peri_ctrl::R](clk_peri_ctrl::R) reader structure"]
impl crate::Readable for CLK_PERI_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_peri_ctrl::W](clk_peri_ctrl::W) writer structure"]
impl crate::Writable for CLK_PERI_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_peri_ctrl;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_peri_selected](clk_peri_selected) module"]
pub type CLK_PERI_SELECTED = crate::Reg<u32, _CLK_PERI_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_PERI_SELECTED;
#[doc = "`read()` method returns [clk_peri_selected::R](clk_peri_selected::R) reader structure"]
impl crate::Readable for CLK_PERI_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_peri_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_usb_ctrl](clk_usb_ctrl) module"]
pub type CLK_USB_CTRL = crate::Reg<u32, _CLK_USB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_USB_CTRL;
#[doc = "`read()` method returns [clk_usb_ctrl::R](clk_usb_ctrl::R) reader structure"]
impl crate::Readable for CLK_USB_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_usb_ctrl::W](clk_usb_ctrl::W) writer structure"]
impl crate::Writable for CLK_USB_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_usb_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_usb_div](clk_usb_div) module"]
pub type CLK_USB_DIV = crate::Reg<u32, _CLK_USB_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_USB_DIV;
#[doc = "`read()` method returns [clk_usb_div::R](clk_usb_div::R) reader structure"]
impl crate::Readable for CLK_USB_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_usb_div::W](clk_usb_div::W) writer structure"]
impl crate::Writable for CLK_USB_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_usb_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_usb_selected](clk_usb_selected) module"]
pub type CLK_USB_SELECTED = crate::Reg<u32, _CLK_USB_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_USB_SELECTED;
#[doc = "`read()` method returns [clk_usb_selected::R](clk_usb_selected::R) reader structure"]
impl crate::Readable for CLK_USB_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_usb_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_adc_ctrl](clk_adc_ctrl) module"]
pub type CLK_ADC_CTRL = crate::Reg<u32, _CLK_ADC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_ADC_CTRL;
#[doc = "`read()` method returns [clk_adc_ctrl::R](clk_adc_ctrl::R) reader structure"]
impl crate::Readable for CLK_ADC_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_adc_ctrl::W](clk_adc_ctrl::W) writer structure"]
impl crate::Writable for CLK_ADC_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_adc_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_adc_div](clk_adc_div) module"]
pub type CLK_ADC_DIV = crate::Reg<u32, _CLK_ADC_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_ADC_DIV;
#[doc = "`read()` method returns [clk_adc_div::R](clk_adc_div::R) reader structure"]
impl crate::Readable for CLK_ADC_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_adc_div::W](clk_adc_div::W) writer structure"]
impl crate::Writable for CLK_ADC_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_adc_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_adc_selected](clk_adc_selected) module"]
pub type CLK_ADC_SELECTED = crate::Reg<u32, _CLK_ADC_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_ADC_SELECTED;
#[doc = "`read()` method returns [clk_adc_selected::R](clk_adc_selected::R) reader structure"]
impl crate::Readable for CLK_ADC_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_adc_selected;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_rtc_ctrl](clk_rtc_ctrl) module"]
pub type CLK_RTC_CTRL = crate::Reg<u32, _CLK_RTC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_RTC_CTRL;
#[doc = "`read()` method returns [clk_rtc_ctrl::R](clk_rtc_ctrl::R) reader structure"]
impl crate::Readable for CLK_RTC_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_rtc_ctrl::W](clk_rtc_ctrl::W) writer structure"]
impl crate::Writable for CLK_RTC_CTRL {}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_rtc_ctrl;
#[doc = "Clock divisor, can be changed on-the-fly\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_rtc_div](clk_rtc_div) module"]
pub type CLK_RTC_DIV = crate::Reg<u32, _CLK_RTC_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_RTC_DIV;
#[doc = "`read()` method returns [clk_rtc_div::R](clk_rtc_div::R) reader structure"]
impl crate::Readable for CLK_RTC_DIV {}
#[doc = "`write(|w| ..)` method takes [clk_rtc_div::W](clk_rtc_div::W) writer structure"]
impl crate::Writable for CLK_RTC_DIV {}
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_rtc_div;
#[doc = "Indicates which src is currently selected (one-hot)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_rtc_selected](clk_rtc_selected) module"]
pub type CLK_RTC_SELECTED = crate::Reg<u32, _CLK_RTC_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_RTC_SELECTED;
#[doc = "`read()` method returns [clk_rtc_selected::R](clk_rtc_selected::R) reader structure"]
impl crate::Readable for CLK_RTC_SELECTED {}
#[doc = "Indicates which src is currently selected (one-hot)"]
pub mod clk_rtc_selected;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_sys_resus_ctrl](clk_sys_resus_ctrl) module"]
pub type CLK_SYS_RESUS_CTRL = crate::Reg<u32, _CLK_SYS_RESUS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SYS_RESUS_CTRL;
#[doc = "`read()` method returns [clk_sys_resus_ctrl::R](clk_sys_resus_ctrl::R) reader structure"]
impl crate::Readable for CLK_SYS_RESUS_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_sys_resus_ctrl::W](clk_sys_resus_ctrl::W) writer structure"]
impl crate::Writable for CLK_SYS_RESUS_CTRL {}
#[doc = ""]
pub mod clk_sys_resus_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_sys_resus_status](clk_sys_resus_status) module"]
pub type CLK_SYS_RESUS_STATUS = crate::Reg<u32, _CLK_SYS_RESUS_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SYS_RESUS_STATUS;
#[doc = "`read()` method returns [clk_sys_resus_status::R](clk_sys_resus_status::R) reader structure"]
impl crate::Readable for CLK_SYS_RESUS_STATUS {}
#[doc = ""]
pub mod clk_sys_resus_status;
#[doc = "Reference clock frequency in kHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc0_ref_khz](fc0_ref_khz) module"]
pub type FC0_REF_KHZ = crate::Reg<u32, _FC0_REF_KHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FC0_REF_KHZ;
#[doc = "`read()` method returns [fc0_ref_khz::R](fc0_ref_khz::R) reader structure"]
impl crate::Readable for FC0_REF_KHZ {}
#[doc = "`write(|w| ..)` method takes [fc0_ref_khz::W](fc0_ref_khz::W) writer structure"]
impl crate::Writable for FC0_REF_KHZ {}
#[doc = "Reference clock frequency in kHz"]
pub mod fc0_ref_khz;
#[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc0_min_khz](fc0_min_khz) module"]
pub type FC0_MIN_KHZ = crate::Reg<u32, _FC0_MIN_KHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FC0_MIN_KHZ;
#[doc = "`read()` method returns [fc0_min_khz::R](fc0_min_khz::R) reader structure"]
impl crate::Readable for FC0_MIN_KHZ {}
#[doc = "`write(|w| ..)` method takes [fc0_min_khz::W](fc0_min_khz::W) writer structure"]
impl crate::Writable for FC0_MIN_KHZ {}
#[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
pub mod fc0_min_khz;
#[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc0_max_khz](fc0_max_khz) module"]
pub type FC0_MAX_KHZ = crate::Reg<u32, _FC0_MAX_KHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FC0_MAX_KHZ;
#[doc = "`read()` method returns [fc0_max_khz::R](fc0_max_khz::R) reader structure"]
impl crate::Readable for FC0_MAX_KHZ {}
#[doc = "`write(|w| ..)` method takes [fc0_max_khz::W](fc0_max_khz::W) writer structure"]
impl crate::Writable for FC0_MAX_KHZ {}
#[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
pub mod fc0_max_khz;
#[doc = "Delays the start of frequency counting to allow the mux to settle\\n Delay is measured in multiples of the reference clock period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc0_delay](fc0_delay) module"]
pub type FC0_DELAY = crate::Reg<u32, _FC0_DELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FC0_DELAY;
#[doc = "`read()` method returns [fc0_delay::R](fc0_delay::R) reader structure"]
impl crate::Readable for FC0_DELAY {}
#[doc = "`write(|w| ..)` method takes [fc0_delay::W](fc0_delay::W) writer structure"]
impl crate::Writable for FC0_DELAY {}
#[doc = "Delays the start of frequency counting to allow the mux to settle\\n Delay is measured in multiples of the reference clock period"]
pub mod fc0_delay;
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval\\n The default gives a test interval of 250us\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc0_interval](fc0_interval) module"]
pub type FC0_INTERVAL = crate::Reg<u32, _FC0_INTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FC0_INTERVAL;
#[doc = "`read()` method returns [fc0_interval::R](fc0_interval::R) reader structure"]
impl crate::Readable for FC0_INTERVAL {}
#[doc = "`write(|w| ..)` method takes [fc0_interval::W](fc0_interval::W) writer structure"]
impl crate::Writable for FC0_INTERVAL {}
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval\\n The default gives a test interval of 250us"]
pub mod fc0_interval;
#[doc = "Clock sent to frequency counter, set to 0 when not required\\n Writing to this register initiates the frequency count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc0_src](fc0_src) module"]
pub type FC0_SRC = crate::Reg<u32, _FC0_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FC0_SRC;
#[doc = "`read()` method returns [fc0_src::R](fc0_src::R) reader structure"]
impl crate::Readable for FC0_SRC {}
#[doc = "`write(|w| ..)` method takes [fc0_src::W](fc0_src::W) writer structure"]
impl crate::Writable for FC0_SRC {}
#[doc = "Clock sent to frequency counter, set to 0 when not required\\n Writing to this register initiates the frequency count"]
pub mod fc0_src;
#[doc = "Frequency counter status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc0_status](fc0_status) module"]
pub type FC0_STATUS = crate::Reg<u32, _FC0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FC0_STATUS;
#[doc = "`read()` method returns [fc0_status::R](fc0_status::R) reader structure"]
impl crate::Readable for FC0_STATUS {}
#[doc = "Frequency counter status"]
pub mod fc0_status;
#[doc = "Result of frequency measurement, only valid when status_done=1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc0_result](fc0_result) module"]
pub type FC0_RESULT = crate::Reg<u32, _FC0_RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FC0_RESULT;
#[doc = "`read()` method returns [fc0_result::R](fc0_result::R) reader structure"]
impl crate::Readable for FC0_RESULT {}
#[doc = "Result of frequency measurement, only valid when status_done=1"]
pub mod fc0_result;
#[doc = "enable clock in wake mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en0](wake_en0) module"]
pub type WAKE_EN0 = crate::Reg<u32, _WAKE_EN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE_EN0;
#[doc = "`read()` method returns [wake_en0::R](wake_en0::R) reader structure"]
impl crate::Readable for WAKE_EN0 {}
#[doc = "`write(|w| ..)` method takes [wake_en0::W](wake_en0::W) writer structure"]
impl crate::Writable for WAKE_EN0 {}
#[doc = "enable clock in wake mode"]
pub mod wake_en0;
#[doc = "enable clock in wake mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en1](wake_en1) module"]
pub type WAKE_EN1 = crate::Reg<u32, _WAKE_EN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE_EN1;
#[doc = "`read()` method returns [wake_en1::R](wake_en1::R) reader structure"]
impl crate::Readable for WAKE_EN1 {}
#[doc = "`write(|w| ..)` method takes [wake_en1::W](wake_en1::W) writer structure"]
impl crate::Writable for WAKE_EN1 {}
#[doc = "enable clock in wake mode"]
pub mod wake_en1;
#[doc = "enable clock in sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep_en0](sleep_en0) module"]
pub type SLEEP_EN0 = crate::Reg<u32, _SLEEP_EN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEP_EN0;
#[doc = "`read()` method returns [sleep_en0::R](sleep_en0::R) reader structure"]
impl crate::Readable for SLEEP_EN0 {}
#[doc = "`write(|w| ..)` method takes [sleep_en0::W](sleep_en0::W) writer structure"]
impl crate::Writable for SLEEP_EN0 {}
#[doc = "enable clock in sleep mode"]
pub mod sleep_en0;
#[doc = "enable clock in sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep_en1](sleep_en1) module"]
pub type SLEEP_EN1 = crate::Reg<u32, _SLEEP_EN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEP_EN1;
#[doc = "`read()` method returns [sleep_en1::R](sleep_en1::R) reader structure"]
impl crate::Readable for SLEEP_EN1 {}
#[doc = "`write(|w| ..)` method takes [sleep_en1::W](sleep_en1::W) writer structure"]
impl crate::Writable for SLEEP_EN1 {}
#[doc = "enable clock in sleep mode"]
pub mod sleep_en1;
#[doc = "indicates the state of the clock enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enabled0](enabled0) module"]
pub type ENABLED0 = crate::Reg<u32, _ENABLED0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLED0;
#[doc = "`read()` method returns [enabled0::R](enabled0::R) reader structure"]
impl crate::Readable for ENABLED0 {}
#[doc = "indicates the state of the clock enable"]
pub mod enabled0;
#[doc = "indicates the state of the clock enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enabled1](enabled1) module"]
pub type ENABLED1 = crate::Reg<u32, _ENABLED1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLED1;
#[doc = "`read()` method returns [enabled1::R](enabled1::R) reader structure"]
impl crate::Readable for ENABLED1 {}
#[doc = "indicates the state of the clock enable"]
pub mod enabled1;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
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
