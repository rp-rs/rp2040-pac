#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device address and endpoint control"]
    pub addr_endp: crate::Reg<addr_endp::ADDR_ENDP_SPEC>,
    #[doc = "0x04 - Interrupt endpoint 1. Only valid for HOST mode."]
    pub addr_endp1: crate::Reg<addr_endp1::ADDR_ENDP1_SPEC>,
    #[doc = "0x08 - Interrupt endpoint 2. Only valid for HOST mode."]
    pub addr_endp2: crate::Reg<addr_endp2::ADDR_ENDP2_SPEC>,
    #[doc = "0x0c - Interrupt endpoint 3. Only valid for HOST mode."]
    pub addr_endp3: crate::Reg<addr_endp3::ADDR_ENDP3_SPEC>,
    #[doc = "0x10 - Interrupt endpoint 4. Only valid for HOST mode."]
    pub addr_endp4: crate::Reg<addr_endp4::ADDR_ENDP4_SPEC>,
    #[doc = "0x14 - Interrupt endpoint 5. Only valid for HOST mode."]
    pub addr_endp5: crate::Reg<addr_endp5::ADDR_ENDP5_SPEC>,
    #[doc = "0x18 - Interrupt endpoint 6. Only valid for HOST mode."]
    pub addr_endp6: crate::Reg<addr_endp6::ADDR_ENDP6_SPEC>,
    #[doc = "0x1c - Interrupt endpoint 7. Only valid for HOST mode."]
    pub addr_endp7: crate::Reg<addr_endp7::ADDR_ENDP7_SPEC>,
    #[doc = "0x20 - Interrupt endpoint 8. Only valid for HOST mode."]
    pub addr_endp8: crate::Reg<addr_endp8::ADDR_ENDP8_SPEC>,
    #[doc = "0x24 - Interrupt endpoint 9. Only valid for HOST mode."]
    pub addr_endp9: crate::Reg<addr_endp9::ADDR_ENDP9_SPEC>,
    #[doc = "0x28 - Interrupt endpoint 10. Only valid for HOST mode."]
    pub addr_endp10: crate::Reg<addr_endp10::ADDR_ENDP10_SPEC>,
    #[doc = "0x2c - Interrupt endpoint 11. Only valid for HOST mode."]
    pub addr_endp11: crate::Reg<addr_endp11::ADDR_ENDP11_SPEC>,
    #[doc = "0x30 - Interrupt endpoint 12. Only valid for HOST mode."]
    pub addr_endp12: crate::Reg<addr_endp12::ADDR_ENDP12_SPEC>,
    #[doc = "0x34 - Interrupt endpoint 13. Only valid for HOST mode."]
    pub addr_endp13: crate::Reg<addr_endp13::ADDR_ENDP13_SPEC>,
    #[doc = "0x38 - Interrupt endpoint 14. Only valid for HOST mode."]
    pub addr_endp14: crate::Reg<addr_endp14::ADDR_ENDP14_SPEC>,
    #[doc = "0x3c - Interrupt endpoint 15. Only valid for HOST mode."]
    pub addr_endp15: crate::Reg<addr_endp15::ADDR_ENDP15_SPEC>,
    #[doc = "0x40 - Main control register"]
    pub main_ctrl: crate::Reg<main_ctrl::MAIN_CTRL_SPEC>,
    #[doc = "0x44 - Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
    pub sof_wr: crate::Reg<sof_wr::SOF_WR_SPEC>,
    #[doc = "0x48 - Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
    pub sof_rd: crate::Reg<sof_rd::SOF_RD_SPEC>,
    #[doc = "0x4c - SIE control register"]
    pub sie_ctrl: crate::Reg<sie_ctrl::SIE_CTRL_SPEC>,
    #[doc = "0x50 - SIE status register"]
    pub sie_status: crate::Reg<sie_status::SIE_STATUS_SPEC>,
    #[doc = "0x54 - interrupt endpoint control register"]
    pub int_ep_ctrl: crate::Reg<int_ep_ctrl::INT_EP_CTRL_SPEC>,
    #[doc = "0x58 - Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
    pub buff_status: crate::Reg<buff_status::BUFF_STATUS_SPEC>,
    #[doc = "0x5c - Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
    pub buff_cpu_should_handle: crate::Reg<buff_cpu_should_handle::BUFF_CPU_SHOULD_HANDLE_SPEC>,
    #[doc = "0x60 - Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
    pub ep_abort: crate::Reg<ep_abort::EP_ABORT_SPEC>,
    #[doc = "0x64 - Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
    pub ep_abort_done: crate::Reg<ep_abort_done::EP_ABORT_DONE_SPEC>,
    #[doc = "0x68 - Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
    pub ep_stall_arm: crate::Reg<ep_stall_arm::EP_STALL_ARM_SPEC>,
    #[doc = "0x6c - Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
    pub nak_poll: crate::Reg<nak_poll::NAK_POLL_SPEC>,
    #[doc = "0x70 - Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
    pub ep_status_stall_nak: crate::Reg<ep_status_stall_nak::EP_STATUS_STALL_NAK_SPEC>,
    #[doc = "0x74 - Where to connect the USB controller. Should be to_phy by default."]
    pub usb_muxing: crate::Reg<usb_muxing::USB_MUXING_SPEC>,
    #[doc = "0x78 - Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
    pub usb_pwr: crate::Reg<usb_pwr::USB_PWR_SPEC>,
    #[doc = "0x7c - This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
    pub usbphy_direct: crate::Reg<usbphy_direct::USBPHY_DIRECT_SPEC>,
    #[doc = "0x80 - Override enable for each control in usbphy_direct"]
    pub usbphy_direct_override: crate::Reg<usbphy_direct_override::USBPHY_DIRECT_OVERRIDE_SPEC>,
    #[doc = "0x84 - Used to adjust trim values of USB phy pull down resistors."]
    pub usbphy_trim: crate::Reg<usbphy_trim::USBPHY_TRIM_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0x8c - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x90 - Interrupt Enable"]
    pub inte: crate::Reg<inte::INTE_SPEC>,
    #[doc = "0x94 - Interrupt Force"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x98 - Interrupt status after masking & forcing"]
    pub ints: crate::Reg<ints::INTS_SPEC>,
}
#[doc = "ADDR_ENDP register accessor: an alias for `Reg<ADDR_ENDP_SPEC>`"]
pub type ADDR_ENDP = crate::Reg<addr_endp::ADDR_ENDP_SPEC>;
#[doc = "Device address and endpoint control"]
pub mod addr_endp;
#[doc = "ADDR_ENDP1 register accessor: an alias for `Reg<ADDR_ENDP1_SPEC>`"]
pub type ADDR_ENDP1 = crate::Reg<addr_endp1::ADDR_ENDP1_SPEC>;
#[doc = "Interrupt endpoint 1. Only valid for HOST mode."]
pub mod addr_endp1;
#[doc = "ADDR_ENDP2 register accessor: an alias for `Reg<ADDR_ENDP2_SPEC>`"]
pub type ADDR_ENDP2 = crate::Reg<addr_endp2::ADDR_ENDP2_SPEC>;
#[doc = "Interrupt endpoint 2. Only valid for HOST mode."]
pub mod addr_endp2;
#[doc = "ADDR_ENDP3 register accessor: an alias for `Reg<ADDR_ENDP3_SPEC>`"]
pub type ADDR_ENDP3 = crate::Reg<addr_endp3::ADDR_ENDP3_SPEC>;
#[doc = "Interrupt endpoint 3. Only valid for HOST mode."]
pub mod addr_endp3;
#[doc = "ADDR_ENDP4 register accessor: an alias for `Reg<ADDR_ENDP4_SPEC>`"]
pub type ADDR_ENDP4 = crate::Reg<addr_endp4::ADDR_ENDP4_SPEC>;
#[doc = "Interrupt endpoint 4. Only valid for HOST mode."]
pub mod addr_endp4;
#[doc = "ADDR_ENDP5 register accessor: an alias for `Reg<ADDR_ENDP5_SPEC>`"]
pub type ADDR_ENDP5 = crate::Reg<addr_endp5::ADDR_ENDP5_SPEC>;
#[doc = "Interrupt endpoint 5. Only valid for HOST mode."]
pub mod addr_endp5;
#[doc = "ADDR_ENDP6 register accessor: an alias for `Reg<ADDR_ENDP6_SPEC>`"]
pub type ADDR_ENDP6 = crate::Reg<addr_endp6::ADDR_ENDP6_SPEC>;
#[doc = "Interrupt endpoint 6. Only valid for HOST mode."]
pub mod addr_endp6;
#[doc = "ADDR_ENDP7 register accessor: an alias for `Reg<ADDR_ENDP7_SPEC>`"]
pub type ADDR_ENDP7 = crate::Reg<addr_endp7::ADDR_ENDP7_SPEC>;
#[doc = "Interrupt endpoint 7. Only valid for HOST mode."]
pub mod addr_endp7;
#[doc = "ADDR_ENDP8 register accessor: an alias for `Reg<ADDR_ENDP8_SPEC>`"]
pub type ADDR_ENDP8 = crate::Reg<addr_endp8::ADDR_ENDP8_SPEC>;
#[doc = "Interrupt endpoint 8. Only valid for HOST mode."]
pub mod addr_endp8;
#[doc = "ADDR_ENDP9 register accessor: an alias for `Reg<ADDR_ENDP9_SPEC>`"]
pub type ADDR_ENDP9 = crate::Reg<addr_endp9::ADDR_ENDP9_SPEC>;
#[doc = "Interrupt endpoint 9. Only valid for HOST mode."]
pub mod addr_endp9;
#[doc = "ADDR_ENDP10 register accessor: an alias for `Reg<ADDR_ENDP10_SPEC>`"]
pub type ADDR_ENDP10 = crate::Reg<addr_endp10::ADDR_ENDP10_SPEC>;
#[doc = "Interrupt endpoint 10. Only valid for HOST mode."]
pub mod addr_endp10;
#[doc = "ADDR_ENDP11 register accessor: an alias for `Reg<ADDR_ENDP11_SPEC>`"]
pub type ADDR_ENDP11 = crate::Reg<addr_endp11::ADDR_ENDP11_SPEC>;
#[doc = "Interrupt endpoint 11. Only valid for HOST mode."]
pub mod addr_endp11;
#[doc = "ADDR_ENDP12 register accessor: an alias for `Reg<ADDR_ENDP12_SPEC>`"]
pub type ADDR_ENDP12 = crate::Reg<addr_endp12::ADDR_ENDP12_SPEC>;
#[doc = "Interrupt endpoint 12. Only valid for HOST mode."]
pub mod addr_endp12;
#[doc = "ADDR_ENDP13 register accessor: an alias for `Reg<ADDR_ENDP13_SPEC>`"]
pub type ADDR_ENDP13 = crate::Reg<addr_endp13::ADDR_ENDP13_SPEC>;
#[doc = "Interrupt endpoint 13. Only valid for HOST mode."]
pub mod addr_endp13;
#[doc = "ADDR_ENDP14 register accessor: an alias for `Reg<ADDR_ENDP14_SPEC>`"]
pub type ADDR_ENDP14 = crate::Reg<addr_endp14::ADDR_ENDP14_SPEC>;
#[doc = "Interrupt endpoint 14. Only valid for HOST mode."]
pub mod addr_endp14;
#[doc = "ADDR_ENDP15 register accessor: an alias for `Reg<ADDR_ENDP15_SPEC>`"]
pub type ADDR_ENDP15 = crate::Reg<addr_endp15::ADDR_ENDP15_SPEC>;
#[doc = "Interrupt endpoint 15. Only valid for HOST mode."]
pub mod addr_endp15;
#[doc = "MAIN_CTRL register accessor: an alias for `Reg<MAIN_CTRL_SPEC>`"]
pub type MAIN_CTRL = crate::Reg<main_ctrl::MAIN_CTRL_SPEC>;
#[doc = "Main control register"]
pub mod main_ctrl;
#[doc = "SOF_WR register accessor: an alias for `Reg<SOF_WR_SPEC>`"]
pub type SOF_WR = crate::Reg<sof_wr::SOF_WR_SPEC>;
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
pub mod sof_wr;
#[doc = "SOF_RD register accessor: an alias for `Reg<SOF_RD_SPEC>`"]
pub type SOF_RD = crate::Reg<sof_rd::SOF_RD_SPEC>;
#[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
pub mod sof_rd;
#[doc = "SIE_CTRL register accessor: an alias for `Reg<SIE_CTRL_SPEC>`"]
pub type SIE_CTRL = crate::Reg<sie_ctrl::SIE_CTRL_SPEC>;
#[doc = "SIE control register"]
pub mod sie_ctrl;
#[doc = "SIE_STATUS register accessor: an alias for `Reg<SIE_STATUS_SPEC>`"]
pub type SIE_STATUS = crate::Reg<sie_status::SIE_STATUS_SPEC>;
#[doc = "SIE status register"]
pub mod sie_status;
#[doc = "INT_EP_CTRL register accessor: an alias for `Reg<INT_EP_CTRL_SPEC>`"]
pub type INT_EP_CTRL = crate::Reg<int_ep_ctrl::INT_EP_CTRL_SPEC>;
#[doc = "interrupt endpoint control register"]
pub mod int_ep_ctrl;
#[doc = "BUFF_STATUS register accessor: an alias for `Reg<BUFF_STATUS_SPEC>`"]
pub type BUFF_STATUS = crate::Reg<buff_status::BUFF_STATUS_SPEC>;
#[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
pub mod buff_status;
#[doc = "BUFF_CPU_SHOULD_HANDLE register accessor: an alias for `Reg<BUFF_CPU_SHOULD_HANDLE_SPEC>`"]
pub type BUFF_CPU_SHOULD_HANDLE = crate::Reg<buff_cpu_should_handle::BUFF_CPU_SHOULD_HANDLE_SPEC>;
#[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
pub mod buff_cpu_should_handle;
#[doc = "EP_ABORT register accessor: an alias for `Reg<EP_ABORT_SPEC>`"]
pub type EP_ABORT = crate::Reg<ep_abort::EP_ABORT_SPEC>;
#[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
pub mod ep_abort;
#[doc = "EP_ABORT_DONE register accessor: an alias for `Reg<EP_ABORT_DONE_SPEC>`"]
pub type EP_ABORT_DONE = crate::Reg<ep_abort_done::EP_ABORT_DONE_SPEC>;
#[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
pub mod ep_abort_done;
#[doc = "EP_STALL_ARM register accessor: an alias for `Reg<EP_STALL_ARM_SPEC>`"]
pub type EP_STALL_ARM = crate::Reg<ep_stall_arm::EP_STALL_ARM_SPEC>;
#[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
pub mod ep_stall_arm;
#[doc = "NAK_POLL register accessor: an alias for `Reg<NAK_POLL_SPEC>`"]
pub type NAK_POLL = crate::Reg<nak_poll::NAK_POLL_SPEC>;
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
pub mod nak_poll;
#[doc = "EP_STATUS_STALL_NAK register accessor: an alias for `Reg<EP_STATUS_STALL_NAK_SPEC>`"]
pub type EP_STATUS_STALL_NAK = crate::Reg<ep_status_stall_nak::EP_STATUS_STALL_NAK_SPEC>;
#[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
pub mod ep_status_stall_nak;
#[doc = "USB_MUXING register accessor: an alias for `Reg<USB_MUXING_SPEC>`"]
pub type USB_MUXING = crate::Reg<usb_muxing::USB_MUXING_SPEC>;
#[doc = "Where to connect the USB controller. Should be to_phy by default."]
pub mod usb_muxing;
#[doc = "USB_PWR register accessor: an alias for `Reg<USB_PWR_SPEC>`"]
pub type USB_PWR = crate::Reg<usb_pwr::USB_PWR_SPEC>;
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
pub mod usb_pwr;
#[doc = "USBPHY_DIRECT register accessor: an alias for `Reg<USBPHY_DIRECT_SPEC>`"]
pub type USBPHY_DIRECT = crate::Reg<usbphy_direct::USBPHY_DIRECT_SPEC>;
#[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
pub mod usbphy_direct;
#[doc = "USBPHY_DIRECT_OVERRIDE register accessor: an alias for `Reg<USBPHY_DIRECT_OVERRIDE_SPEC>`"]
pub type USBPHY_DIRECT_OVERRIDE = crate::Reg<usbphy_direct_override::USBPHY_DIRECT_OVERRIDE_SPEC>;
#[doc = "Override enable for each control in usbphy_direct"]
pub mod usbphy_direct_override;
#[doc = "USBPHY_TRIM register accessor: an alias for `Reg<USBPHY_TRIM_SPEC>`"]
pub type USBPHY_TRIM = crate::Reg<usbphy_trim::USBPHY_TRIM_SPEC>;
#[doc = "Used to adjust trim values of USB phy pull down resistors."]
pub mod usbphy_trim;
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
