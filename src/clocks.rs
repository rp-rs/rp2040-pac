#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_gpout0_ctrl: CLK_GPOUT0_CTRL,
    clk_gpout0_div: CLK_GPOUT0_DIV,
    clk_gpout0_selected: CLK_GPOUT0_SELECTED,
    clk_gpout1_ctrl: CLK_GPOUT1_CTRL,
    clk_gpout1_div: CLK_GPOUT1_DIV,
    clk_gpout1_selected: CLK_GPOUT1_SELECTED,
    clk_gpout2_ctrl: CLK_GPOUT2_CTRL,
    clk_gpout2_div: CLK_GPOUT2_DIV,
    clk_gpout2_selected: CLK_GPOUT2_SELECTED,
    clk_gpout3_ctrl: CLK_GPOUT3_CTRL,
    clk_gpout3_div: CLK_GPOUT3_DIV,
    clk_gpout3_selected: CLK_GPOUT3_SELECTED,
    clk_ref_ctrl: CLK_REF_CTRL,
    clk_ref_div: CLK_REF_DIV,
    clk_ref_selected: CLK_REF_SELECTED,
    clk_sys_ctrl: CLK_SYS_CTRL,
    clk_sys_div: CLK_SYS_DIV,
    clk_sys_selected: CLK_SYS_SELECTED,
    clk_peri_ctrl: CLK_PERI_CTRL,
    clk_peri_div: CLK_PERI_DIV,
    clk_peri_selected: CLK_PERI_SELECTED,
    clk_usb_ctrl: CLK_USB_CTRL,
    clk_usb_div: CLK_USB_DIV,
    clk_usb_selected: CLK_USB_SELECTED,
    clk_adc_ctrl: CLK_ADC_CTRL,
    clk_adc_div: CLK_ADC_DIV,
    clk_adc_selected: CLK_ADC_SELECTED,
    clk_rtc_ctrl: CLK_RTC_CTRL,
    clk_rtc_div: CLK_RTC_DIV,
    clk_rtc_selected: CLK_RTC_SELECTED,
    clk_sys_resus_ctrl: CLK_SYS_RESUS_CTRL,
    clk_sys_resus_status: CLK_SYS_RESUS_STATUS,
    fc0_ref_khz: FC0_REF_KHZ,
    fc0_min_khz: FC0_MIN_KHZ,
    fc0_max_khz: FC0_MAX_KHZ,
    fc0_delay: FC0_DELAY,
    fc0_interval: FC0_INTERVAL,
    fc0_src: FC0_SRC,
    fc0_status: FC0_STATUS,
    fc0_result: FC0_RESULT,
    wake_en0: WAKE_EN0,
    wake_en1: WAKE_EN1,
    sleep_en0: SLEEP_EN0,
    sleep_en1: SLEEP_EN1,
    enabled0: ENABLED0,
    enabled1: ENABLED1,
    intr: INTR,
    inte: INTE,
    intf: INTF,
    ints: INTS,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_gpout0_ctrl(&self) -> &CLK_GPOUT0_CTRL {
        &self.clk_gpout0_ctrl
    }
    #[doc = "0x04 - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_gpout0_div(&self) -> &CLK_GPOUT0_DIV {
        &self.clk_gpout0_div
    }
    #[doc = "0x08 - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_gpout0_selected(&self) -> &CLK_GPOUT0_SELECTED {
        &self.clk_gpout0_selected
    }
    #[doc = "0x0c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_gpout1_ctrl(&self) -> &CLK_GPOUT1_CTRL {
        &self.clk_gpout1_ctrl
    }
    #[doc = "0x10 - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_gpout1_div(&self) -> &CLK_GPOUT1_DIV {
        &self.clk_gpout1_div
    }
    #[doc = "0x14 - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_gpout1_selected(&self) -> &CLK_GPOUT1_SELECTED {
        &self.clk_gpout1_selected
    }
    #[doc = "0x18 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_gpout2_ctrl(&self) -> &CLK_GPOUT2_CTRL {
        &self.clk_gpout2_ctrl
    }
    #[doc = "0x1c - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_gpout2_div(&self) -> &CLK_GPOUT2_DIV {
        &self.clk_gpout2_div
    }
    #[doc = "0x20 - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_gpout2_selected(&self) -> &CLK_GPOUT2_SELECTED {
        &self.clk_gpout2_selected
    }
    #[doc = "0x24 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_gpout3_ctrl(&self) -> &CLK_GPOUT3_CTRL {
        &self.clk_gpout3_ctrl
    }
    #[doc = "0x28 - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_gpout3_div(&self) -> &CLK_GPOUT3_DIV {
        &self.clk_gpout3_div
    }
    #[doc = "0x2c - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_gpout3_selected(&self) -> &CLK_GPOUT3_SELECTED {
        &self.clk_gpout3_selected
    }
    #[doc = "0x30 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_ref_ctrl(&self) -> &CLK_REF_CTRL {
        &self.clk_ref_ctrl
    }
    #[doc = "0x34 - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_ref_div(&self) -> &CLK_REF_DIV {
        &self.clk_ref_div
    }
    #[doc = "0x38 - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_ref_selected(&self) -> &CLK_REF_SELECTED {
        &self.clk_ref_selected
    }
    #[doc = "0x3c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_sys_ctrl(&self) -> &CLK_SYS_CTRL {
        &self.clk_sys_ctrl
    }
    #[doc = "0x40 - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_sys_div(&self) -> &CLK_SYS_DIV {
        &self.clk_sys_div
    }
    #[doc = "0x44 - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_sys_selected(&self) -> &CLK_SYS_SELECTED {
        &self.clk_sys_selected
    }
    #[doc = "0x48 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_peri_ctrl(&self) -> &CLK_PERI_CTRL {
        &self.clk_peri_ctrl
    }
    #[doc = "0x4c - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_peri_div(&self) -> &CLK_PERI_DIV {
        &self.clk_peri_div
    }
    #[doc = "0x50 - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_peri_selected(&self) -> &CLK_PERI_SELECTED {
        &self.clk_peri_selected
    }
    #[doc = "0x54 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_usb_ctrl(&self) -> &CLK_USB_CTRL {
        &self.clk_usb_ctrl
    }
    #[doc = "0x58 - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_usb_div(&self) -> &CLK_USB_DIV {
        &self.clk_usb_div
    }
    #[doc = "0x5c - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_usb_selected(&self) -> &CLK_USB_SELECTED {
        &self.clk_usb_selected
    }
    #[doc = "0x60 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_adc_ctrl(&self) -> &CLK_ADC_CTRL {
        &self.clk_adc_ctrl
    }
    #[doc = "0x64 - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_adc_div(&self) -> &CLK_ADC_DIV {
        &self.clk_adc_div
    }
    #[doc = "0x68 - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_adc_selected(&self) -> &CLK_ADC_SELECTED {
        &self.clk_adc_selected
    }
    #[doc = "0x6c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_rtc_ctrl(&self) -> &CLK_RTC_CTRL {
        &self.clk_rtc_ctrl
    }
    #[doc = "0x70 - Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_rtc_div(&self) -> &CLK_RTC_DIV {
        &self.clk_rtc_div
    }
    #[doc = "0x74 - Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_rtc_selected(&self) -> &CLK_RTC_SELECTED {
        &self.clk_rtc_selected
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn clk_sys_resus_ctrl(&self) -> &CLK_SYS_RESUS_CTRL {
        &self.clk_sys_resus_ctrl
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn clk_sys_resus_status(&self) -> &CLK_SYS_RESUS_STATUS {
        &self.clk_sys_resus_status
    }
    #[doc = "0x80 - Reference clock frequency in kHz"]
    #[inline(always)]
    pub const fn fc0_ref_khz(&self) -> &FC0_REF_KHZ {
        &self.fc0_ref_khz
    }
    #[doc = "0x84 - Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
    #[inline(always)]
    pub const fn fc0_min_khz(&self) -> &FC0_MIN_KHZ {
        &self.fc0_min_khz
    }
    #[doc = "0x88 - Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
    #[inline(always)]
    pub const fn fc0_max_khz(&self) -> &FC0_MAX_KHZ {
        &self.fc0_max_khz
    }
    #[doc = "0x8c - Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period"]
    #[inline(always)]
    pub const fn fc0_delay(&self) -> &FC0_DELAY {
        &self.fc0_delay
    }
    #[doc = "0x90 - The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us"]
    #[inline(always)]
    pub const fn fc0_interval(&self) -> &FC0_INTERVAL {
        &self.fc0_interval
    }
    #[doc = "0x94 - Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
    #[inline(always)]
    pub const fn fc0_src(&self) -> &FC0_SRC {
        &self.fc0_src
    }
    #[doc = "0x98 - Frequency counter status"]
    #[inline(always)]
    pub const fn fc0_status(&self) -> &FC0_STATUS {
        &self.fc0_status
    }
    #[doc = "0x9c - Result of frequency measurement, only valid when status_done=1"]
    #[inline(always)]
    pub const fn fc0_result(&self) -> &FC0_RESULT {
        &self.fc0_result
    }
    #[doc = "0xa0 - enable clock in wake mode"]
    #[inline(always)]
    pub const fn wake_en0(&self) -> &WAKE_EN0 {
        &self.wake_en0
    }
    #[doc = "0xa4 - enable clock in wake mode"]
    #[inline(always)]
    pub const fn wake_en1(&self) -> &WAKE_EN1 {
        &self.wake_en1
    }
    #[doc = "0xa8 - enable clock in sleep mode"]
    #[inline(always)]
    pub const fn sleep_en0(&self) -> &SLEEP_EN0 {
        &self.sleep_en0
    }
    #[doc = "0xac - enable clock in sleep mode"]
    #[inline(always)]
    pub const fn sleep_en1(&self) -> &SLEEP_EN1 {
        &self.sleep_en1
    }
    #[doc = "0xb0 - indicates the state of the clock enable"]
    #[inline(always)]
    pub const fn enabled0(&self) -> &ENABLED0 {
        &self.enabled0
    }
    #[doc = "0xb4 - indicates the state of the clock enable"]
    #[inline(always)]
    pub const fn enabled1(&self) -> &ENABLED1 {
        &self.enabled1
    }
    #[doc = "0xb8 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0xbc - Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0xc0 - Interrupt Force"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0xc4 - Interrupt status after masking &amp; forcing"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
}
#[doc = "CLK_GPOUT0_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout0_ctrl`]
module"]
pub type CLK_GPOUT0_CTRL = crate::Reg<clk_gpout0_ctrl::CLK_GPOUT0_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout0_ctrl;
#[doc = "CLK_GPOUT0_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout0_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout0_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout0_div`]
module"]
pub type CLK_GPOUT0_DIV = crate::Reg<clk_gpout0_div::CLK_GPOUT0_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout0_div;
#[doc = "CLK_GPOUT0_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout0_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout0_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout0_selected`]
module"]
pub type CLK_GPOUT0_SELECTED = crate::Reg<clk_gpout0_selected::CLK_GPOUT0_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_gpout0_selected;
#[doc = "CLK_GPOUT1_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout1_ctrl`]
module"]
pub type CLK_GPOUT1_CTRL = crate::Reg<clk_gpout1_ctrl::CLK_GPOUT1_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout1_ctrl;
#[doc = "CLK_GPOUT1_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout1_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout1_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout1_div`]
module"]
pub type CLK_GPOUT1_DIV = crate::Reg<clk_gpout1_div::CLK_GPOUT1_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout1_div;
#[doc = "CLK_GPOUT1_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout1_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout1_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout1_selected`]
module"]
pub type CLK_GPOUT1_SELECTED = crate::Reg<clk_gpout1_selected::CLK_GPOUT1_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_gpout1_selected;
#[doc = "CLK_GPOUT2_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout2_ctrl`]
module"]
pub type CLK_GPOUT2_CTRL = crate::Reg<clk_gpout2_ctrl::CLK_GPOUT2_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout2_ctrl;
#[doc = "CLK_GPOUT2_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout2_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout2_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout2_div`]
module"]
pub type CLK_GPOUT2_DIV = crate::Reg<clk_gpout2_div::CLK_GPOUT2_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout2_div;
#[doc = "CLK_GPOUT2_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout2_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout2_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout2_selected`]
module"]
pub type CLK_GPOUT2_SELECTED = crate::Reg<clk_gpout2_selected::CLK_GPOUT2_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_gpout2_selected;
#[doc = "CLK_GPOUT3_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout3_ctrl`]
module"]
pub type CLK_GPOUT3_CTRL = crate::Reg<clk_gpout3_ctrl::CLK_GPOUT3_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout3_ctrl;
#[doc = "CLK_GPOUT3_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout3_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout3_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout3_div`]
module"]
pub type CLK_GPOUT3_DIV = crate::Reg<clk_gpout3_div::CLK_GPOUT3_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout3_div;
#[doc = "CLK_GPOUT3_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout3_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout3_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_gpout3_selected`]
module"]
pub type CLK_GPOUT3_SELECTED = crate::Reg<clk_gpout3_selected::CLK_GPOUT3_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_gpout3_selected;
#[doc = "CLK_REF_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_ref_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ref_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_ref_ctrl`]
module"]
pub type CLK_REF_CTRL = crate::Reg<clk_ref_ctrl::CLK_REF_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_ref_ctrl;
#[doc = "CLK_REF_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_ref_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ref_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_ref_div`]
module"]
pub type CLK_REF_DIV = crate::Reg<clk_ref_div::CLK_REF_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_ref_div;
#[doc = "CLK_REF_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_ref_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ref_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_ref_selected`]
module"]
pub type CLK_REF_SELECTED = crate::Reg<clk_ref_selected::CLK_REF_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_ref_selected;
#[doc = "CLK_SYS_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_sys_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sys_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_sys_ctrl`]
module"]
pub type CLK_SYS_CTRL = crate::Reg<clk_sys_ctrl::CLK_SYS_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_sys_ctrl;
#[doc = "CLK_SYS_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_sys_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sys_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_sys_div`]
module"]
pub type CLK_SYS_DIV = crate::Reg<clk_sys_div::CLK_SYS_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_sys_div;
#[doc = "CLK_SYS_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_sys_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sys_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_sys_selected`]
module"]
pub type CLK_SYS_SELECTED = crate::Reg<clk_sys_selected::CLK_SYS_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_sys_selected;
#[doc = "CLK_PERI_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_peri_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_peri_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_peri_ctrl`]
module"]
pub type CLK_PERI_CTRL = crate::Reg<clk_peri_ctrl::CLK_PERI_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_peri_ctrl;
#[doc = "CLK_PERI_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_peri_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_peri_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_peri_div`]
module"]
pub type CLK_PERI_DIV = crate::Reg<clk_peri_div::CLK_PERI_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_peri_div;
#[doc = "CLK_PERI_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_peri_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_peri_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_peri_selected`]
module"]
pub type CLK_PERI_SELECTED = crate::Reg<clk_peri_selected::CLK_PERI_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_peri_selected;
#[doc = "CLK_USB_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_usb_ctrl`]
module"]
pub type CLK_USB_CTRL = crate::Reg<clk_usb_ctrl::CLK_USB_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_usb_ctrl;
#[doc = "CLK_USB_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_usb_div`]
module"]
pub type CLK_USB_DIV = crate::Reg<clk_usb_div::CLK_USB_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_usb_div;
#[doc = "CLK_USB_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_usb_selected`]
module"]
pub type CLK_USB_SELECTED = crate::Reg<clk_usb_selected::CLK_USB_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_usb_selected;
#[doc = "CLK_ADC_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_adc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_adc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_adc_ctrl`]
module"]
pub type CLK_ADC_CTRL = crate::Reg<clk_adc_ctrl::CLK_ADC_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_adc_ctrl;
#[doc = "CLK_ADC_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_adc_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_adc_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_adc_div`]
module"]
pub type CLK_ADC_DIV = crate::Reg<clk_adc_div::CLK_ADC_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_adc_div;
#[doc = "CLK_ADC_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_adc_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_adc_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_adc_selected`]
module"]
pub type CLK_ADC_SELECTED = crate::Reg<clk_adc_selected::CLK_ADC_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_adc_selected;
#[doc = "CLK_RTC_CTRL (rw) register accessor: Clock control, can be changed on-the-fly (except for auxsrc)  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_rtc_ctrl`]
module"]
pub type CLK_RTC_CTRL = crate::Reg<clk_rtc_ctrl::CLK_RTC_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_rtc_ctrl;
#[doc = "CLK_RTC_DIV (rw) register accessor: Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_rtc_div`]
module"]
pub type CLK_RTC_DIV = crate::Reg<clk_rtc_div::CLK_RTC_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_rtc_div;
#[doc = "CLK_RTC_SELECTED (rw) register accessor: Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_selected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_selected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_rtc_selected`]
module"]
pub type CLK_RTC_SELECTED = crate::Reg<clk_rtc_selected::CLK_RTC_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
pub mod clk_rtc_selected;
#[doc = "CLK_SYS_RESUS_CTRL (rw) register accessor:   

You can [`read`](crate::generic::Reg::read) this register and get [`clk_sys_resus_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sys_resus_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_sys_resus_ctrl`]
module"]
pub type CLK_SYS_RESUS_CTRL = crate::Reg<clk_sys_resus_ctrl::CLK_SYS_RESUS_CTRL_SPEC>;
#[doc = ""]
pub mod clk_sys_resus_ctrl;
#[doc = "CLK_SYS_RESUS_STATUS (rw) register accessor:   

You can [`read`](crate::generic::Reg::read) this register and get [`clk_sys_resus_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sys_resus_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clk_sys_resus_status`]
module"]
pub type CLK_SYS_RESUS_STATUS = crate::Reg<clk_sys_resus_status::CLK_SYS_RESUS_STATUS_SPEC>;
#[doc = ""]
pub mod clk_sys_resus_status;
#[doc = "FC0_REF_KHZ (rw) register accessor: Reference clock frequency in kHz  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_ref_khz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_ref_khz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fc0_ref_khz`]
module"]
pub type FC0_REF_KHZ = crate::Reg<fc0_ref_khz::FC0_REF_KHZ_SPEC>;
#[doc = "Reference clock frequency in kHz"]
pub mod fc0_ref_khz;
#[doc = "FC0_MIN_KHZ (rw) register accessor: Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_min_khz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_min_khz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fc0_min_khz`]
module"]
pub type FC0_MIN_KHZ = crate::Reg<fc0_min_khz::FC0_MIN_KHZ_SPEC>;
#[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
pub mod fc0_min_khz;
#[doc = "FC0_MAX_KHZ (rw) register accessor: Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_max_khz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_max_khz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fc0_max_khz`]
module"]
pub type FC0_MAX_KHZ = crate::Reg<fc0_max_khz::FC0_MAX_KHZ_SPEC>;
#[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
pub mod fc0_max_khz;
#[doc = "FC0_DELAY (rw) register accessor: Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fc0_delay`]
module"]
pub type FC0_DELAY = crate::Reg<fc0_delay::FC0_DELAY_SPEC>;
#[doc = "Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period"]
pub mod fc0_delay;
#[doc = "FC0_INTERVAL (rw) register accessor: The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fc0_interval`]
module"]
pub type FC0_INTERVAL = crate::Reg<fc0_interval::FC0_INTERVAL_SPEC>;
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us"]
pub mod fc0_interval;
#[doc = "FC0_SRC (rw) register accessor: Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fc0_src`]
module"]
pub type FC0_SRC = crate::Reg<fc0_src::FC0_SRC_SPEC>;
#[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
pub mod fc0_src;
#[doc = "FC0_STATUS (rw) register accessor: Frequency counter status  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fc0_status`]
module"]
pub type FC0_STATUS = crate::Reg<fc0_status::FC0_STATUS_SPEC>;
#[doc = "Frequency counter status"]
pub mod fc0_status;
#[doc = "FC0_RESULT (rw) register accessor: Result of frequency measurement, only valid when status_done=1  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fc0_result`]
module"]
pub type FC0_RESULT = crate::Reg<fc0_result::FC0_RESULT_SPEC>;
#[doc = "Result of frequency measurement, only valid when status_done=1"]
pub mod fc0_result;
#[doc = "WAKE_EN0 (rw) register accessor: enable clock in wake mode  

You can [`read`](crate::generic::Reg::read) this register and get [`wake_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@wake_en0`]
module"]
pub type WAKE_EN0 = crate::Reg<wake_en0::WAKE_EN0_SPEC>;
#[doc = "enable clock in wake mode"]
pub mod wake_en0;
#[doc = "WAKE_EN1 (rw) register accessor: enable clock in wake mode  

You can [`read`](crate::generic::Reg::read) this register and get [`wake_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@wake_en1`]
module"]
pub type WAKE_EN1 = crate::Reg<wake_en1::WAKE_EN1_SPEC>;
#[doc = "enable clock in wake mode"]
pub mod wake_en1;
#[doc = "SLEEP_EN0 (rw) register accessor: enable clock in sleep mode  

You can [`read`](crate::generic::Reg::read) this register and get [`sleep_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sleep_en0`]
module"]
pub type SLEEP_EN0 = crate::Reg<sleep_en0::SLEEP_EN0_SPEC>;
#[doc = "enable clock in sleep mode"]
pub mod sleep_en0;
#[doc = "SLEEP_EN1 (rw) register accessor: enable clock in sleep mode  

You can [`read`](crate::generic::Reg::read) this register and get [`sleep_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sleep_en1`]
module"]
pub type SLEEP_EN1 = crate::Reg<sleep_en1::SLEEP_EN1_SPEC>;
#[doc = "enable clock in sleep mode"]
pub mod sleep_en1;
#[doc = "ENABLED0 (rw) register accessor: indicates the state of the clock enable  

You can [`read`](crate::generic::Reg::read) this register and get [`enabled0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enabled0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@enabled0`]
module"]
pub type ENABLED0 = crate::Reg<enabled0::ENABLED0_SPEC>;
#[doc = "indicates the state of the clock enable"]
pub mod enabled0;
#[doc = "ENABLED1 (rw) register accessor: indicates the state of the clock enable  

You can [`read`](crate::generic::Reg::read) this register and get [`enabled1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enabled1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@enabled1`]
module"]
pub type ENABLED1 = crate::Reg<enabled1::ENABLED1_SPEC>;
#[doc = "indicates the state of the clock enable"]
pub mod enabled1;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE (rw) register accessor: Interrupt Enable  

You can [`read`](crate::generic::Reg::read) this register and get [`inte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte`]
module"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF (rw) register accessor: Interrupt Force  

You can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS (rw) register accessor: Interrupt status after masking &amp; forcing  

You can [`read`](crate::generic::Reg::read) this register and get [`ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints`]
module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing"]
pub mod ints;
