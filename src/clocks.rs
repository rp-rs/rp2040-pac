#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_gpout0_ctrl: crate::Reg<clk_gpout0_ctrl::CLK_GPOUT0_CTRL_SPEC>,
    #[doc = "0x04 - Clock divisor, can be changed on-the-fly"]
    pub clk_gpout0_div: crate::Reg<clk_gpout0_div::CLK_GPOUT0_DIV_SPEC>,
    #[doc = "0x08 - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    pub clk_gpout0_selected: crate::Reg<clk_gpout0_selected::CLK_GPOUT0_SELECTED_SPEC>,
    #[doc = "0x0c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_gpout1_ctrl: crate::Reg<clk_gpout1_ctrl::CLK_GPOUT1_CTRL_SPEC>,
    #[doc = "0x10 - Clock divisor, can be changed on-the-fly"]
    pub clk_gpout1_div: crate::Reg<clk_gpout1_div::CLK_GPOUT1_DIV_SPEC>,
    #[doc = "0x14 - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    pub clk_gpout1_selected: crate::Reg<clk_gpout1_selected::CLK_GPOUT1_SELECTED_SPEC>,
    #[doc = "0x18 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_gpout2_ctrl: crate::Reg<clk_gpout2_ctrl::CLK_GPOUT2_CTRL_SPEC>,
    #[doc = "0x1c - Clock divisor, can be changed on-the-fly"]
    pub clk_gpout2_div: crate::Reg<clk_gpout2_div::CLK_GPOUT2_DIV_SPEC>,
    #[doc = "0x20 - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    pub clk_gpout2_selected: crate::Reg<clk_gpout2_selected::CLK_GPOUT2_SELECTED_SPEC>,
    #[doc = "0x24 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_gpout3_ctrl: crate::Reg<clk_gpout3_ctrl::CLK_GPOUT3_CTRL_SPEC>,
    #[doc = "0x28 - Clock divisor, can be changed on-the-fly"]
    pub clk_gpout3_div: crate::Reg<clk_gpout3_div::CLK_GPOUT3_DIV_SPEC>,
    #[doc = "0x2c - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    pub clk_gpout3_selected: crate::Reg<clk_gpout3_selected::CLK_GPOUT3_SELECTED_SPEC>,
    #[doc = "0x30 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_ref_ctrl: crate::Reg<clk_ref_ctrl::CLK_REF_CTRL_SPEC>,
    #[doc = "0x34 - Clock divisor, can be changed on-the-fly"]
    pub clk_ref_div: crate::Reg<clk_ref_div::CLK_REF_DIV_SPEC>,
    #[doc = "0x38 - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
    pub clk_ref_selected: crate::Reg<clk_ref_selected::CLK_REF_SELECTED_SPEC>,
    #[doc = "0x3c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_sys_ctrl: crate::Reg<clk_sys_ctrl::CLK_SYS_CTRL_SPEC>,
    #[doc = "0x40 - Clock divisor, can be changed on-the-fly"]
    pub clk_sys_div: crate::Reg<clk_sys_div::CLK_SYS_DIV_SPEC>,
    #[doc = "0x44 - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
    pub clk_sys_selected: crate::Reg<clk_sys_selected::CLK_SYS_SELECTED_SPEC>,
    #[doc = "0x48 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_peri_ctrl: crate::Reg<clk_peri_ctrl::CLK_PERI_CTRL_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    pub clk_peri_selected: crate::Reg<clk_peri_selected::CLK_PERI_SELECTED_SPEC>,
    #[doc = "0x54 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_usb_ctrl: crate::Reg<clk_usb_ctrl::CLK_USB_CTRL_SPEC>,
    #[doc = "0x58 - Clock divisor, can be changed on-the-fly"]
    pub clk_usb_div: crate::Reg<clk_usb_div::CLK_USB_DIV_SPEC>,
    #[doc = "0x5c - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    pub clk_usb_selected: crate::Reg<clk_usb_selected::CLK_USB_SELECTED_SPEC>,
    #[doc = "0x60 - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_adc_ctrl: crate::Reg<clk_adc_ctrl::CLK_ADC_CTRL_SPEC>,
    #[doc = "0x64 - Clock divisor, can be changed on-the-fly"]
    pub clk_adc_div: crate::Reg<clk_adc_div::CLK_ADC_DIV_SPEC>,
    #[doc = "0x68 - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    pub clk_adc_selected: crate::Reg<clk_adc_selected::CLK_ADC_SELECTED_SPEC>,
    #[doc = "0x6c - Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub clk_rtc_ctrl: crate::Reg<clk_rtc_ctrl::CLK_RTC_CTRL_SPEC>,
    #[doc = "0x70 - Clock divisor, can be changed on-the-fly"]
    pub clk_rtc_div: crate::Reg<clk_rtc_div::CLK_RTC_DIV_SPEC>,
    #[doc = "0x74 - Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    pub clk_rtc_selected: crate::Reg<clk_rtc_selected::CLK_RTC_SELECTED_SPEC>,
    #[doc = "0x78 - "]
    pub clk_sys_resus_ctrl: crate::Reg<clk_sys_resus_ctrl::CLK_SYS_RESUS_CTRL_SPEC>,
    #[doc = "0x7c - "]
    pub clk_sys_resus_status: crate::Reg<clk_sys_resus_status::CLK_SYS_RESUS_STATUS_SPEC>,
    #[doc = "0x80 - Reference clock frequency in kHz"]
    pub fc0_ref_khz: crate::Reg<fc0_ref_khz::FC0_REF_KHZ_SPEC>,
    #[doc = "0x84 - Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
    pub fc0_min_khz: crate::Reg<fc0_min_khz::FC0_MIN_KHZ_SPEC>,
    #[doc = "0x88 - Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
    pub fc0_max_khz: crate::Reg<fc0_max_khz::FC0_MAX_KHZ_SPEC>,
    #[doc = "0x8c - Delays the start of frequency counting to allow the mux to settle  
 Delay is measured in multiples of the reference clock period"]
    pub fc0_delay: crate::Reg<fc0_delay::FC0_DELAY_SPEC>,
    #[doc = "0x90 - The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval  
 The default gives a test interval of 250us"]
    pub fc0_interval: crate::Reg<fc0_interval::FC0_INTERVAL_SPEC>,
    #[doc = "0x94 - Clock sent to frequency counter, set to 0 when not required  
 Writing to this register initiates the frequency count"]
    pub fc0_src: crate::Reg<fc0_src::FC0_SRC_SPEC>,
    #[doc = "0x98 - Frequency counter status"]
    pub fc0_status: crate::Reg<fc0_status::FC0_STATUS_SPEC>,
    #[doc = "0x9c - Result of frequency measurement, only valid when status_done=1"]
    pub fc0_result: crate::Reg<fc0_result::FC0_RESULT_SPEC>,
    #[doc = "0xa0 - enable clock in wake mode"]
    pub wake_en0: crate::Reg<wake_en0::WAKE_EN0_SPEC>,
    #[doc = "0xa4 - enable clock in wake mode"]
    pub wake_en1: crate::Reg<wake_en1::WAKE_EN1_SPEC>,
    #[doc = "0xa8 - enable clock in sleep mode"]
    pub sleep_en0: crate::Reg<sleep_en0::SLEEP_EN0_SPEC>,
    #[doc = "0xac - enable clock in sleep mode"]
    pub sleep_en1: crate::Reg<sleep_en1::SLEEP_EN1_SPEC>,
    #[doc = "0xb0 - indicates the state of the clock enable"]
    pub enabled0: crate::Reg<enabled0::ENABLED0_SPEC>,
    #[doc = "0xb4 - indicates the state of the clock enable"]
    pub enabled1: crate::Reg<enabled1::ENABLED1_SPEC>,
    #[doc = "0xb8 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0xbc - Interrupt Enable"]
    pub inte: crate::Reg<inte::INTE_SPEC>,
    #[doc = "0xc0 - Interrupt Force"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0xc4 - Interrupt status after masking & forcing"]
    pub ints: crate::Reg<ints::INTS_SPEC>,
}
#[doc = "CLK_GPOUT0_CTRL register accessor: an alias for `Reg<CLK_GPOUT0_CTRL_SPEC>`"]
pub type CLK_GPOUT0_CTRL = crate::Reg<clk_gpout0_ctrl::CLK_GPOUT0_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout0_ctrl;
#[doc = "CLK_GPOUT0_DIV register accessor: an alias for `Reg<CLK_GPOUT0_DIV_SPEC>`"]
pub type CLK_GPOUT0_DIV = crate::Reg<clk_gpout0_div::CLK_GPOUT0_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout0_div;
#[doc = "CLK_GPOUT0_SELECTED register accessor: an alias for `Reg<CLK_GPOUT0_SELECTED_SPEC>`"]
pub type CLK_GPOUT0_SELECTED = crate::Reg<clk_gpout0_selected::CLK_GPOUT0_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub mod clk_gpout0_selected;
#[doc = "CLK_GPOUT1_CTRL register accessor: an alias for `Reg<CLK_GPOUT1_CTRL_SPEC>`"]
pub type CLK_GPOUT1_CTRL = crate::Reg<clk_gpout1_ctrl::CLK_GPOUT1_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout1_ctrl;
#[doc = "CLK_GPOUT1_DIV register accessor: an alias for `Reg<CLK_GPOUT1_DIV_SPEC>`"]
pub type CLK_GPOUT1_DIV = crate::Reg<clk_gpout1_div::CLK_GPOUT1_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout1_div;
#[doc = "CLK_GPOUT1_SELECTED register accessor: an alias for `Reg<CLK_GPOUT1_SELECTED_SPEC>`"]
pub type CLK_GPOUT1_SELECTED = crate::Reg<clk_gpout1_selected::CLK_GPOUT1_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub mod clk_gpout1_selected;
#[doc = "CLK_GPOUT2_CTRL register accessor: an alias for `Reg<CLK_GPOUT2_CTRL_SPEC>`"]
pub type CLK_GPOUT2_CTRL = crate::Reg<clk_gpout2_ctrl::CLK_GPOUT2_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout2_ctrl;
#[doc = "CLK_GPOUT2_DIV register accessor: an alias for `Reg<CLK_GPOUT2_DIV_SPEC>`"]
pub type CLK_GPOUT2_DIV = crate::Reg<clk_gpout2_div::CLK_GPOUT2_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout2_div;
#[doc = "CLK_GPOUT2_SELECTED register accessor: an alias for `Reg<CLK_GPOUT2_SELECTED_SPEC>`"]
pub type CLK_GPOUT2_SELECTED = crate::Reg<clk_gpout2_selected::CLK_GPOUT2_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub mod clk_gpout2_selected;
#[doc = "CLK_GPOUT3_CTRL register accessor: an alias for `Reg<CLK_GPOUT3_CTRL_SPEC>`"]
pub type CLK_GPOUT3_CTRL = crate::Reg<clk_gpout3_ctrl::CLK_GPOUT3_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_gpout3_ctrl;
#[doc = "CLK_GPOUT3_DIV register accessor: an alias for `Reg<CLK_GPOUT3_DIV_SPEC>`"]
pub type CLK_GPOUT3_DIV = crate::Reg<clk_gpout3_div::CLK_GPOUT3_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_gpout3_div;
#[doc = "CLK_GPOUT3_SELECTED register accessor: an alias for `Reg<CLK_GPOUT3_SELECTED_SPEC>`"]
pub type CLK_GPOUT3_SELECTED = crate::Reg<clk_gpout3_selected::CLK_GPOUT3_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub mod clk_gpout3_selected;
#[doc = "CLK_REF_CTRL register accessor: an alias for `Reg<CLK_REF_CTRL_SPEC>`"]
pub type CLK_REF_CTRL = crate::Reg<clk_ref_ctrl::CLK_REF_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_ref_ctrl;
#[doc = "CLK_REF_DIV register accessor: an alias for `Reg<CLK_REF_DIV_SPEC>`"]
pub type CLK_REF_DIV = crate::Reg<clk_ref_div::CLK_REF_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_ref_div;
#[doc = "CLK_REF_SELECTED register accessor: an alias for `Reg<CLK_REF_SELECTED_SPEC>`"]
pub type CLK_REF_SELECTED = crate::Reg<clk_ref_selected::CLK_REF_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
pub mod clk_ref_selected;
#[doc = "CLK_SYS_CTRL register accessor: an alias for `Reg<CLK_SYS_CTRL_SPEC>`"]
pub type CLK_SYS_CTRL = crate::Reg<clk_sys_ctrl::CLK_SYS_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_sys_ctrl;
#[doc = "CLK_SYS_DIV register accessor: an alias for `Reg<CLK_SYS_DIV_SPEC>`"]
pub type CLK_SYS_DIV = crate::Reg<clk_sys_div::CLK_SYS_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_sys_div;
#[doc = "CLK_SYS_SELECTED register accessor: an alias for `Reg<CLK_SYS_SELECTED_SPEC>`"]
pub type CLK_SYS_SELECTED = crate::Reg<clk_sys_selected::CLK_SYS_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
pub mod clk_sys_selected;
#[doc = "CLK_PERI_CTRL register accessor: an alias for `Reg<CLK_PERI_CTRL_SPEC>`"]
pub type CLK_PERI_CTRL = crate::Reg<clk_peri_ctrl::CLK_PERI_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_peri_ctrl;
#[doc = "CLK_PERI_SELECTED register accessor: an alias for `Reg<CLK_PERI_SELECTED_SPEC>`"]
pub type CLK_PERI_SELECTED = crate::Reg<clk_peri_selected::CLK_PERI_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub mod clk_peri_selected;
#[doc = "CLK_USB_CTRL register accessor: an alias for `Reg<CLK_USB_CTRL_SPEC>`"]
pub type CLK_USB_CTRL = crate::Reg<clk_usb_ctrl::CLK_USB_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_usb_ctrl;
#[doc = "CLK_USB_DIV register accessor: an alias for `Reg<CLK_USB_DIV_SPEC>`"]
pub type CLK_USB_DIV = crate::Reg<clk_usb_div::CLK_USB_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_usb_div;
#[doc = "CLK_USB_SELECTED register accessor: an alias for `Reg<CLK_USB_SELECTED_SPEC>`"]
pub type CLK_USB_SELECTED = crate::Reg<clk_usb_selected::CLK_USB_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub mod clk_usb_selected;
#[doc = "CLK_ADC_CTRL register accessor: an alias for `Reg<CLK_ADC_CTRL_SPEC>`"]
pub type CLK_ADC_CTRL = crate::Reg<clk_adc_ctrl::CLK_ADC_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_adc_ctrl;
#[doc = "CLK_ADC_DIV register accessor: an alias for `Reg<CLK_ADC_DIV_SPEC>`"]
pub type CLK_ADC_DIV = crate::Reg<clk_adc_div::CLK_ADC_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_adc_div;
#[doc = "CLK_ADC_SELECTED register accessor: an alias for `Reg<CLK_ADC_SELECTED_SPEC>`"]
pub type CLK_ADC_SELECTED = crate::Reg<clk_adc_selected::CLK_ADC_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub mod clk_adc_selected;
#[doc = "CLK_RTC_CTRL register accessor: an alias for `Reg<CLK_RTC_CTRL_SPEC>`"]
pub type CLK_RTC_CTRL = crate::Reg<clk_rtc_ctrl::CLK_RTC_CTRL_SPEC>;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
pub mod clk_rtc_ctrl;
#[doc = "CLK_RTC_DIV register accessor: an alias for `Reg<CLK_RTC_DIV_SPEC>`"]
pub type CLK_RTC_DIV = crate::Reg<clk_rtc_div::CLK_RTC_DIV_SPEC>;
#[doc = "Clock divisor, can be changed on-the-fly"]
pub mod clk_rtc_div;
#[doc = "CLK_RTC_SELECTED register accessor: an alias for `Reg<CLK_RTC_SELECTED_SPEC>`"]
pub type CLK_RTC_SELECTED = crate::Reg<clk_rtc_selected::CLK_RTC_SELECTED_SPEC>;
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub mod clk_rtc_selected;
#[doc = "CLK_SYS_RESUS_CTRL register accessor: an alias for `Reg<CLK_SYS_RESUS_CTRL_SPEC>`"]
pub type CLK_SYS_RESUS_CTRL = crate::Reg<clk_sys_resus_ctrl::CLK_SYS_RESUS_CTRL_SPEC>;
#[doc = ""]
pub mod clk_sys_resus_ctrl;
#[doc = "CLK_SYS_RESUS_STATUS register accessor: an alias for `Reg<CLK_SYS_RESUS_STATUS_SPEC>`"]
pub type CLK_SYS_RESUS_STATUS = crate::Reg<clk_sys_resus_status::CLK_SYS_RESUS_STATUS_SPEC>;
#[doc = ""]
pub mod clk_sys_resus_status;
#[doc = "FC0_REF_KHZ register accessor: an alias for `Reg<FC0_REF_KHZ_SPEC>`"]
pub type FC0_REF_KHZ = crate::Reg<fc0_ref_khz::FC0_REF_KHZ_SPEC>;
#[doc = "Reference clock frequency in kHz"]
pub mod fc0_ref_khz;
#[doc = "FC0_MIN_KHZ register accessor: an alias for `Reg<FC0_MIN_KHZ_SPEC>`"]
pub type FC0_MIN_KHZ = crate::Reg<fc0_min_khz::FC0_MIN_KHZ_SPEC>;
#[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
pub mod fc0_min_khz;
#[doc = "FC0_MAX_KHZ register accessor: an alias for `Reg<FC0_MAX_KHZ_SPEC>`"]
pub type FC0_MAX_KHZ = crate::Reg<fc0_max_khz::FC0_MAX_KHZ_SPEC>;
#[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
pub mod fc0_max_khz;
#[doc = "FC0_DELAY register accessor: an alias for `Reg<FC0_DELAY_SPEC>`"]
pub type FC0_DELAY = crate::Reg<fc0_delay::FC0_DELAY_SPEC>;
#[doc = "Delays the start of frequency counting to allow the mux to settle  
 Delay is measured in multiples of the reference clock period"]
pub mod fc0_delay;
#[doc = "FC0_INTERVAL register accessor: an alias for `Reg<FC0_INTERVAL_SPEC>`"]
pub type FC0_INTERVAL = crate::Reg<fc0_interval::FC0_INTERVAL_SPEC>;
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval  
 The default gives a test interval of 250us"]
pub mod fc0_interval;
#[doc = "FC0_SRC register accessor: an alias for `Reg<FC0_SRC_SPEC>`"]
pub type FC0_SRC = crate::Reg<fc0_src::FC0_SRC_SPEC>;
#[doc = "Clock sent to frequency counter, set to 0 when not required  
 Writing to this register initiates the frequency count"]
pub mod fc0_src;
#[doc = "FC0_STATUS register accessor: an alias for `Reg<FC0_STATUS_SPEC>`"]
pub type FC0_STATUS = crate::Reg<fc0_status::FC0_STATUS_SPEC>;
#[doc = "Frequency counter status"]
pub mod fc0_status;
#[doc = "FC0_RESULT register accessor: an alias for `Reg<FC0_RESULT_SPEC>`"]
pub type FC0_RESULT = crate::Reg<fc0_result::FC0_RESULT_SPEC>;
#[doc = "Result of frequency measurement, only valid when status_done=1"]
pub mod fc0_result;
#[doc = "WAKE_EN0 register accessor: an alias for `Reg<WAKE_EN0_SPEC>`"]
pub type WAKE_EN0 = crate::Reg<wake_en0::WAKE_EN0_SPEC>;
#[doc = "enable clock in wake mode"]
pub mod wake_en0;
#[doc = "WAKE_EN1 register accessor: an alias for `Reg<WAKE_EN1_SPEC>`"]
pub type WAKE_EN1 = crate::Reg<wake_en1::WAKE_EN1_SPEC>;
#[doc = "enable clock in wake mode"]
pub mod wake_en1;
#[doc = "SLEEP_EN0 register accessor: an alias for `Reg<SLEEP_EN0_SPEC>`"]
pub type SLEEP_EN0 = crate::Reg<sleep_en0::SLEEP_EN0_SPEC>;
#[doc = "enable clock in sleep mode"]
pub mod sleep_en0;
#[doc = "SLEEP_EN1 register accessor: an alias for `Reg<SLEEP_EN1_SPEC>`"]
pub type SLEEP_EN1 = crate::Reg<sleep_en1::SLEEP_EN1_SPEC>;
#[doc = "enable clock in sleep mode"]
pub mod sleep_en1;
#[doc = "ENABLED0 register accessor: an alias for `Reg<ENABLED0_SPEC>`"]
pub type ENABLED0 = crate::Reg<enabled0::ENABLED0_SPEC>;
#[doc = "indicates the state of the clock enable"]
pub mod enabled0;
#[doc = "ENABLED1 register accessor: an alias for `Reg<ENABLED1_SPEC>`"]
pub type ENABLED1 = crate::Reg<enabled1::ENABLED1_SPEC>;
#[doc = "indicates the state of the clock enable"]
pub mod enabled1;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE register accessor: an alias for `Reg<INTE_SPEC>`"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS register accessor: an alias for `Reg<INTS_SPEC>`"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing"]
pub mod ints;
