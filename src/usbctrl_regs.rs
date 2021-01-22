#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device address and endpoint control"]
    pub addr_endp: ADDR_ENDP,
    #[doc = "0x04 - Interrupt endpoint 1. Only valid for HOST mode."]
    pub addr_endp1: ADDR_ENDP1,
    #[doc = "0x08 - Interrupt endpoint 2. Only valid for HOST mode."]
    pub addr_endp2: ADDR_ENDP2,
    #[doc = "0x0c - Interrupt endpoint 3. Only valid for HOST mode."]
    pub addr_endp3: ADDR_ENDP3,
    #[doc = "0x10 - Interrupt endpoint 4. Only valid for HOST mode."]
    pub addr_endp4: ADDR_ENDP4,
    #[doc = "0x14 - Interrupt endpoint 5. Only valid for HOST mode."]
    pub addr_endp5: ADDR_ENDP5,
    #[doc = "0x18 - Interrupt endpoint 6. Only valid for HOST mode."]
    pub addr_endp6: ADDR_ENDP6,
    #[doc = "0x1c - Interrupt endpoint 7. Only valid for HOST mode."]
    pub addr_endp7: ADDR_ENDP7,
    #[doc = "0x20 - Interrupt endpoint 8. Only valid for HOST mode."]
    pub addr_endp8: ADDR_ENDP8,
    #[doc = "0x24 - Interrupt endpoint 9. Only valid for HOST mode."]
    pub addr_endp9: ADDR_ENDP9,
    #[doc = "0x28 - Interrupt endpoint 10. Only valid for HOST mode."]
    pub addr_endp10: ADDR_ENDP10,
    #[doc = "0x2c - Interrupt endpoint 11. Only valid for HOST mode."]
    pub addr_endp11: ADDR_ENDP11,
    #[doc = "0x30 - Interrupt endpoint 12. Only valid for HOST mode."]
    pub addr_endp12: ADDR_ENDP12,
    #[doc = "0x34 - Interrupt endpoint 13. Only valid for HOST mode."]
    pub addr_endp13: ADDR_ENDP13,
    #[doc = "0x38 - Interrupt endpoint 14. Only valid for HOST mode."]
    pub addr_endp14: ADDR_ENDP14,
    #[doc = "0x3c - Interrupt endpoint 15. Only valid for HOST mode."]
    pub addr_endp15: ADDR_ENDP15,
    #[doc = "0x40 - Main control register"]
    pub main_ctrl: MAIN_CTRL,
    #[doc = "0x44 - Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
    pub sof_wr: SOF_WR,
    #[doc = "0x48 - Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
    pub sof_rd: SOF_RD,
    #[doc = "0x4c - SIE control register"]
    pub sie_ctrl: SIE_CTRL,
    #[doc = "0x50 - SIE status register"]
    pub sie_status: SIE_STATUS,
    #[doc = "0x54 - interrupt endpoint control register"]
    pub int_ep_ctrl: INT_EP_CTRL,
    #[doc = "0x58 - Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
    pub buff_status: BUFF_STATUS,
    #[doc = "0x5c - Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
    pub buff_cpu_should_handle: BUFF_CPU_SHOULD_HANDLE,
    #[doc = "0x60 - Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
    pub ep_abort: EP_ABORT,
    #[doc = "0x64 - Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
    pub ep_abort_done: EP_ABORT_DONE,
    #[doc = "0x68 - Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
    pub ep_stall_arm: EP_STALL_ARM,
    #[doc = "0x6c - Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
    pub nak_poll: NAK_POLL,
    #[doc = "0x70 - Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
    pub ep_status_stall_nak: EP_STATUS_STALL_NAK,
    #[doc = "0x74 - Where to connect the USB controller. Should be to_phy by default."]
    pub usb_muxing: USB_MUXING,
    #[doc = "0x78 - Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
    pub usb_pwr: USB_PWR,
    #[doc = "0x7c - This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
    pub usbphy_direct: USBPHY_DIRECT,
    #[doc = "0x80 - Override enable for each control in usbphy_direct"]
    pub usbphy_direct_override: USBPHY_DIRECT_OVERRIDE,
    #[doc = "0x84 - Used to adjust trim values of USB phy pull down resistors."]
    pub usbphy_trim: USBPHY_TRIM,
    _reserved34: [u8; 4usize],
    #[doc = "0x8c - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0x90 - Interrupt Enable"]
    pub inte: INTE,
    #[doc = "0x94 - Interrupt Force"]
    pub intf: INTF,
    #[doc = "0x98 - Interrupt status after masking & forcing"]
    pub ints: INTS,
}
#[doc = "Device address and endpoint control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp](addr_endp) module"]
pub type ADDR_ENDP = crate::Reg<u32, _ADDR_ENDP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP;
#[doc = "`read()` method returns [addr_endp::R](addr_endp::R) reader structure"]
impl crate::Readable for ADDR_ENDP {}
#[doc = "`write(|w| ..)` method takes [addr_endp::W](addr_endp::W) writer structure"]
impl crate::Writable for ADDR_ENDP {}
#[doc = "Device address and endpoint control"]
pub mod addr_endp;
#[doc = "Interrupt endpoint 1. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp1](addr_endp1) module"]
pub type ADDR_ENDP1 = crate::Reg<u32, _ADDR_ENDP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP1;
#[doc = "`read()` method returns [addr_endp1::R](addr_endp1::R) reader structure"]
impl crate::Readable for ADDR_ENDP1 {}
#[doc = "`write(|w| ..)` method takes [addr_endp1::W](addr_endp1::W) writer structure"]
impl crate::Writable for ADDR_ENDP1 {}
#[doc = "Interrupt endpoint 1. Only valid for HOST mode."]
pub mod addr_endp1;
#[doc = "Interrupt endpoint 2. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp2](addr_endp2) module"]
pub type ADDR_ENDP2 = crate::Reg<u32, _ADDR_ENDP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP2;
#[doc = "`read()` method returns [addr_endp2::R](addr_endp2::R) reader structure"]
impl crate::Readable for ADDR_ENDP2 {}
#[doc = "`write(|w| ..)` method takes [addr_endp2::W](addr_endp2::W) writer structure"]
impl crate::Writable for ADDR_ENDP2 {}
#[doc = "Interrupt endpoint 2. Only valid for HOST mode."]
pub mod addr_endp2;
#[doc = "Interrupt endpoint 3. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp3](addr_endp3) module"]
pub type ADDR_ENDP3 = crate::Reg<u32, _ADDR_ENDP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP3;
#[doc = "`read()` method returns [addr_endp3::R](addr_endp3::R) reader structure"]
impl crate::Readable for ADDR_ENDP3 {}
#[doc = "`write(|w| ..)` method takes [addr_endp3::W](addr_endp3::W) writer structure"]
impl crate::Writable for ADDR_ENDP3 {}
#[doc = "Interrupt endpoint 3. Only valid for HOST mode."]
pub mod addr_endp3;
#[doc = "Interrupt endpoint 4. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp4](addr_endp4) module"]
pub type ADDR_ENDP4 = crate::Reg<u32, _ADDR_ENDP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP4;
#[doc = "`read()` method returns [addr_endp4::R](addr_endp4::R) reader structure"]
impl crate::Readable for ADDR_ENDP4 {}
#[doc = "`write(|w| ..)` method takes [addr_endp4::W](addr_endp4::W) writer structure"]
impl crate::Writable for ADDR_ENDP4 {}
#[doc = "Interrupt endpoint 4. Only valid for HOST mode."]
pub mod addr_endp4;
#[doc = "Interrupt endpoint 5. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp5](addr_endp5) module"]
pub type ADDR_ENDP5 = crate::Reg<u32, _ADDR_ENDP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP5;
#[doc = "`read()` method returns [addr_endp5::R](addr_endp5::R) reader structure"]
impl crate::Readable for ADDR_ENDP5 {}
#[doc = "`write(|w| ..)` method takes [addr_endp5::W](addr_endp5::W) writer structure"]
impl crate::Writable for ADDR_ENDP5 {}
#[doc = "Interrupt endpoint 5. Only valid for HOST mode."]
pub mod addr_endp5;
#[doc = "Interrupt endpoint 6. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp6](addr_endp6) module"]
pub type ADDR_ENDP6 = crate::Reg<u32, _ADDR_ENDP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP6;
#[doc = "`read()` method returns [addr_endp6::R](addr_endp6::R) reader structure"]
impl crate::Readable for ADDR_ENDP6 {}
#[doc = "`write(|w| ..)` method takes [addr_endp6::W](addr_endp6::W) writer structure"]
impl crate::Writable for ADDR_ENDP6 {}
#[doc = "Interrupt endpoint 6. Only valid for HOST mode."]
pub mod addr_endp6;
#[doc = "Interrupt endpoint 7. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp7](addr_endp7) module"]
pub type ADDR_ENDP7 = crate::Reg<u32, _ADDR_ENDP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP7;
#[doc = "`read()` method returns [addr_endp7::R](addr_endp7::R) reader structure"]
impl crate::Readable for ADDR_ENDP7 {}
#[doc = "`write(|w| ..)` method takes [addr_endp7::W](addr_endp7::W) writer structure"]
impl crate::Writable for ADDR_ENDP7 {}
#[doc = "Interrupt endpoint 7. Only valid for HOST mode."]
pub mod addr_endp7;
#[doc = "Interrupt endpoint 8. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp8](addr_endp8) module"]
pub type ADDR_ENDP8 = crate::Reg<u32, _ADDR_ENDP8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP8;
#[doc = "`read()` method returns [addr_endp8::R](addr_endp8::R) reader structure"]
impl crate::Readable for ADDR_ENDP8 {}
#[doc = "`write(|w| ..)` method takes [addr_endp8::W](addr_endp8::W) writer structure"]
impl crate::Writable for ADDR_ENDP8 {}
#[doc = "Interrupt endpoint 8. Only valid for HOST mode."]
pub mod addr_endp8;
#[doc = "Interrupt endpoint 9. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp9](addr_endp9) module"]
pub type ADDR_ENDP9 = crate::Reg<u32, _ADDR_ENDP9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP9;
#[doc = "`read()` method returns [addr_endp9::R](addr_endp9::R) reader structure"]
impl crate::Readable for ADDR_ENDP9 {}
#[doc = "`write(|w| ..)` method takes [addr_endp9::W](addr_endp9::W) writer structure"]
impl crate::Writable for ADDR_ENDP9 {}
#[doc = "Interrupt endpoint 9. Only valid for HOST mode."]
pub mod addr_endp9;
#[doc = "Interrupt endpoint 10. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp10](addr_endp10) module"]
pub type ADDR_ENDP10 = crate::Reg<u32, _ADDR_ENDP10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP10;
#[doc = "`read()` method returns [addr_endp10::R](addr_endp10::R) reader structure"]
impl crate::Readable for ADDR_ENDP10 {}
#[doc = "`write(|w| ..)` method takes [addr_endp10::W](addr_endp10::W) writer structure"]
impl crate::Writable for ADDR_ENDP10 {}
#[doc = "Interrupt endpoint 10. Only valid for HOST mode."]
pub mod addr_endp10;
#[doc = "Interrupt endpoint 11. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp11](addr_endp11) module"]
pub type ADDR_ENDP11 = crate::Reg<u32, _ADDR_ENDP11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP11;
#[doc = "`read()` method returns [addr_endp11::R](addr_endp11::R) reader structure"]
impl crate::Readable for ADDR_ENDP11 {}
#[doc = "`write(|w| ..)` method takes [addr_endp11::W](addr_endp11::W) writer structure"]
impl crate::Writable for ADDR_ENDP11 {}
#[doc = "Interrupt endpoint 11. Only valid for HOST mode."]
pub mod addr_endp11;
#[doc = "Interrupt endpoint 12. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp12](addr_endp12) module"]
pub type ADDR_ENDP12 = crate::Reg<u32, _ADDR_ENDP12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP12;
#[doc = "`read()` method returns [addr_endp12::R](addr_endp12::R) reader structure"]
impl crate::Readable for ADDR_ENDP12 {}
#[doc = "`write(|w| ..)` method takes [addr_endp12::W](addr_endp12::W) writer structure"]
impl crate::Writable for ADDR_ENDP12 {}
#[doc = "Interrupt endpoint 12. Only valid for HOST mode."]
pub mod addr_endp12;
#[doc = "Interrupt endpoint 13. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp13](addr_endp13) module"]
pub type ADDR_ENDP13 = crate::Reg<u32, _ADDR_ENDP13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP13;
#[doc = "`read()` method returns [addr_endp13::R](addr_endp13::R) reader structure"]
impl crate::Readable for ADDR_ENDP13 {}
#[doc = "`write(|w| ..)` method takes [addr_endp13::W](addr_endp13::W) writer structure"]
impl crate::Writable for ADDR_ENDP13 {}
#[doc = "Interrupt endpoint 13. Only valid for HOST mode."]
pub mod addr_endp13;
#[doc = "Interrupt endpoint 14. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp14](addr_endp14) module"]
pub type ADDR_ENDP14 = crate::Reg<u32, _ADDR_ENDP14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP14;
#[doc = "`read()` method returns [addr_endp14::R](addr_endp14::R) reader structure"]
impl crate::Readable for ADDR_ENDP14 {}
#[doc = "`write(|w| ..)` method takes [addr_endp14::W](addr_endp14::W) writer structure"]
impl crate::Writable for ADDR_ENDP14 {}
#[doc = "Interrupt endpoint 14. Only valid for HOST mode."]
pub mod addr_endp14;
#[doc = "Interrupt endpoint 15. Only valid for HOST mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_endp15](addr_endp15) module"]
pub type ADDR_ENDP15 = crate::Reg<u32, _ADDR_ENDP15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_ENDP15;
#[doc = "`read()` method returns [addr_endp15::R](addr_endp15::R) reader structure"]
impl crate::Readable for ADDR_ENDP15 {}
#[doc = "`write(|w| ..)` method takes [addr_endp15::W](addr_endp15::W) writer structure"]
impl crate::Writable for ADDR_ENDP15 {}
#[doc = "Interrupt endpoint 15. Only valid for HOST mode."]
pub mod addr_endp15;
#[doc = "Main control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [main_ctrl](main_ctrl) module"]
pub type MAIN_CTRL = crate::Reg<u32, _MAIN_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAIN_CTRL;
#[doc = "`read()` method returns [main_ctrl::R](main_ctrl::R) reader structure"]
impl crate::Readable for MAIN_CTRL {}
#[doc = "`write(|w| ..)` method takes [main_ctrl::W](main_ctrl::W) writer structure"]
impl crate::Writable for MAIN_CTRL {}
#[doc = "Main control register"]
pub mod main_ctrl;
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof_wr](sof_wr) module"]
pub type SOF_WR = crate::Reg<u32, _SOF_WR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOF_WR;
#[doc = "`write(|w| ..)` method takes [sof_wr::W](sof_wr::W) writer structure"]
impl crate::Writable for SOF_WR {}
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
pub mod sof_wr;
#[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof_rd](sof_rd) module"]
pub type SOF_RD = crate::Reg<u32, _SOF_RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOF_RD;
#[doc = "`read()` method returns [sof_rd::R](sof_rd::R) reader structure"]
impl crate::Readable for SOF_RD {}
#[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
pub mod sof_rd;
#[doc = "SIE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ctrl](sie_ctrl) module"]
pub type SIE_CTRL = crate::Reg<u32, _SIE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_CTRL;
#[doc = "`read()` method returns [sie_ctrl::R](sie_ctrl::R) reader structure"]
impl crate::Readable for SIE_CTRL {}
#[doc = "`write(|w| ..)` method takes [sie_ctrl::W](sie_ctrl::W) writer structure"]
impl crate::Writable for SIE_CTRL {}
#[doc = "SIE control register"]
pub mod sie_ctrl;
#[doc = "SIE status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_status](sie_status) module"]
pub type SIE_STATUS = crate::Reg<u32, _SIE_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_STATUS;
#[doc = "`read()` method returns [sie_status::R](sie_status::R) reader structure"]
impl crate::Readable for SIE_STATUS {}
#[doc = "`write(|w| ..)` method takes [sie_status::W](sie_status::W) writer structure"]
impl crate::Writable for SIE_STATUS {}
#[doc = "SIE status register"]
pub mod sie_status;
#[doc = "interrupt endpoint control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ep_ctrl](int_ep_ctrl) module"]
pub type INT_EP_CTRL = crate::Reg<u32, _INT_EP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EP_CTRL;
#[doc = "`read()` method returns [int_ep_ctrl::R](int_ep_ctrl::R) reader structure"]
impl crate::Readable for INT_EP_CTRL {}
#[doc = "`write(|w| ..)` method takes [int_ep_ctrl::W](int_ep_ctrl::W) writer structure"]
impl crate::Writable for INT_EP_CTRL {}
#[doc = "interrupt endpoint control register"]
pub mod int_ep_ctrl;
#[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buff_status](buff_status) module"]
pub type BUFF_STATUS = crate::Reg<u32, _BUFF_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFF_STATUS;
#[doc = "`read()` method returns [buff_status::R](buff_status::R) reader structure"]
impl crate::Readable for BUFF_STATUS {}
#[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
pub mod buff_status;
#[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buff_cpu_should_handle](buff_cpu_should_handle) module"]
pub type BUFF_CPU_SHOULD_HANDLE = crate::Reg<u32, _BUFF_CPU_SHOULD_HANDLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFF_CPU_SHOULD_HANDLE;
#[doc = "`read()` method returns [buff_cpu_should_handle::R](buff_cpu_should_handle::R) reader structure"]
impl crate::Readable for BUFF_CPU_SHOULD_HANDLE {}
#[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
pub mod buff_cpu_should_handle;
#[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_abort](ep_abort) module"]
pub type EP_ABORT = crate::Reg<u32, _EP_ABORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP_ABORT;
#[doc = "`read()` method returns [ep_abort::R](ep_abort::R) reader structure"]
impl crate::Readable for EP_ABORT {}
#[doc = "`write(|w| ..)` method takes [ep_abort::W](ep_abort::W) writer structure"]
impl crate::Writable for EP_ABORT {}
#[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
pub mod ep_abort;
#[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_abort_done](ep_abort_done) module"]
pub type EP_ABORT_DONE = crate::Reg<u32, _EP_ABORT_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP_ABORT_DONE;
#[doc = "`read()` method returns [ep_abort_done::R](ep_abort_done::R) reader structure"]
impl crate::Readable for EP_ABORT_DONE {}
#[doc = "`write(|w| ..)` method takes [ep_abort_done::W](ep_abort_done::W) writer structure"]
impl crate::Writable for EP_ABORT_DONE {}
#[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
pub mod ep_abort_done;
#[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_stall_arm](ep_stall_arm) module"]
pub type EP_STALL_ARM = crate::Reg<u32, _EP_STALL_ARM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP_STALL_ARM;
#[doc = "`read()` method returns [ep_stall_arm::R](ep_stall_arm::R) reader structure"]
impl crate::Readable for EP_STALL_ARM {}
#[doc = "`write(|w| ..)` method takes [ep_stall_arm::W](ep_stall_arm::W) writer structure"]
impl crate::Writable for EP_STALL_ARM {}
#[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
pub mod ep_stall_arm;
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nak_poll](nak_poll) module"]
pub type NAK_POLL = crate::Reg<u32, _NAK_POLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NAK_POLL;
#[doc = "`read()` method returns [nak_poll::R](nak_poll::R) reader structure"]
impl crate::Readable for NAK_POLL {}
#[doc = "`write(|w| ..)` method takes [nak_poll::W](nak_poll::W) writer structure"]
impl crate::Writable for NAK_POLL {}
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
pub mod nak_poll;
#[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_status_stall_nak](ep_status_stall_nak) module"]
pub type EP_STATUS_STALL_NAK = crate::Reg<u32, _EP_STATUS_STALL_NAK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP_STATUS_STALL_NAK;
#[doc = "`read()` method returns [ep_status_stall_nak::R](ep_status_stall_nak::R) reader structure"]
impl crate::Readable for EP_STATUS_STALL_NAK {}
#[doc = "`write(|w| ..)` method takes [ep_status_stall_nak::W](ep_status_stall_nak::W) writer structure"]
impl crate::Writable for EP_STATUS_STALL_NAK {}
#[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
pub mod ep_status_stall_nak;
#[doc = "Where to connect the USB controller. Should be to_phy by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_muxing](usb_muxing) module"]
pub type USB_MUXING = crate::Reg<u32, _USB_MUXING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_MUXING;
#[doc = "`read()` method returns [usb_muxing::R](usb_muxing::R) reader structure"]
impl crate::Readable for USB_MUXING {}
#[doc = "`write(|w| ..)` method takes [usb_muxing::W](usb_muxing::W) writer structure"]
impl crate::Writable for USB_MUXING {}
#[doc = "Where to connect the USB controller. Should be to_phy by default."]
pub mod usb_muxing;
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_pwr](usb_pwr) module"]
pub type USB_PWR = crate::Reg<u32, _USB_PWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_PWR;
#[doc = "`read()` method returns [usb_pwr::R](usb_pwr::R) reader structure"]
impl crate::Readable for USB_PWR {}
#[doc = "`write(|w| ..)` method takes [usb_pwr::W](usb_pwr::W) writer structure"]
impl crate::Writable for USB_PWR {}
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
pub mod usb_pwr;
#[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphy_direct](usbphy_direct) module"]
pub type USBPHY_DIRECT = crate::Reg<u32, _USBPHY_DIRECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHY_DIRECT;
#[doc = "`read()` method returns [usbphy_direct::R](usbphy_direct::R) reader structure"]
impl crate::Readable for USBPHY_DIRECT {}
#[doc = "`write(|w| ..)` method takes [usbphy_direct::W](usbphy_direct::W) writer structure"]
impl crate::Writable for USBPHY_DIRECT {}
#[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
pub mod usbphy_direct;
#[doc = "Override enable for each control in usbphy_direct\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphy_direct_override](usbphy_direct_override) module"]
pub type USBPHY_DIRECT_OVERRIDE = crate::Reg<u32, _USBPHY_DIRECT_OVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHY_DIRECT_OVERRIDE;
#[doc = "`read()` method returns [usbphy_direct_override::R](usbphy_direct_override::R) reader structure"]
impl crate::Readable for USBPHY_DIRECT_OVERRIDE {}
#[doc = "`write(|w| ..)` method takes [usbphy_direct_override::W](usbphy_direct_override::W) writer structure"]
impl crate::Writable for USBPHY_DIRECT_OVERRIDE {}
#[doc = "Override enable for each control in usbphy_direct"]
pub mod usbphy_direct_override;
#[doc = "Used to adjust trim values of USB phy pull down resistors.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphy_trim](usbphy_trim) module"]
pub type USBPHY_TRIM = crate::Reg<u32, _USBPHY_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHY_TRIM;
#[doc = "`read()` method returns [usbphy_trim::R](usbphy_trim::R) reader structure"]
impl crate::Readable for USBPHY_TRIM {}
#[doc = "`write(|w| ..)` method takes [usbphy_trim::W](usbphy_trim::W) writer structure"]
impl crate::Writable for USBPHY_TRIM {}
#[doc = "Used to adjust trim values of USB phy pull down resistors."]
pub mod usbphy_trim;
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
