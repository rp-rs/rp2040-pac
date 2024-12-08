#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    addr_endp: ADDR_ENDP,
    host_addr_endp: [HOST_ADDR_ENDP; 15],
    main_ctrl: MAIN_CTRL,
    sof_wr: SOF_WR,
    sof_rd: SOF_RD,
    sie_ctrl: SIE_CTRL,
    sie_status: SIE_STATUS,
    int_ep_ctrl: INT_EP_CTRL,
    buff_status: BUFF_STATUS,
    buff_cpu_should_handle: BUFF_CPU_SHOULD_HANDLE,
    ep_abort: EP_ABORT,
    ep_abort_done: EP_ABORT_DONE,
    ep_stall_arm: EP_STALL_ARM,
    nak_poll: NAK_POLL,
    ep_status_stall_nak: EP_STATUS_STALL_NAK,
    usb_muxing: USB_MUXING,
    usb_pwr: USB_PWR,
    usbphy_direct: USBPHY_DIRECT,
    usbphy_direct_override: USBPHY_DIRECT_OVERRIDE,
    usbphy_trim: USBPHY_TRIM,
    _reserved20: [u8; 0x04],
    intr: INTR,
    inte: INTE,
    intf: INTF,
    ints: INTS,
}
impl RegisterBlock {
    #[doc = "0x00 - Device address and endpoint control"]
    #[inline(always)]
    pub const fn addr_endp(&self) -> &ADDR_ENDP {
        &self.addr_endp
    }
    #[doc = "0x04..0x40 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp(&self, n: usize) -> &HOST_ADDR_ENDP {
        &self.host_addr_endp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x40 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub fn host_addr_endp_iter(&self) -> impl Iterator<Item = &HOST_ADDR_ENDP> {
        self.host_addr_endp.iter()
    }
    #[doc = "0x04 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp1(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(0)
    }
    #[doc = "0x08 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp2(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(1)
    }
    #[doc = "0x0c - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp3(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(2)
    }
    #[doc = "0x10 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp4(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(3)
    }
    #[doc = "0x14 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp5(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(4)
    }
    #[doc = "0x18 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp6(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(5)
    }
    #[doc = "0x1c - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp7(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(6)
    }
    #[doc = "0x20 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp8(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(7)
    }
    #[doc = "0x24 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp9(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(8)
    }
    #[doc = "0x28 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp10(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(9)
    }
    #[doc = "0x2c - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp11(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(10)
    }
    #[doc = "0x30 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp12(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(11)
    }
    #[doc = "0x34 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp13(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(12)
    }
    #[doc = "0x38 - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp14(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(13)
    }
    #[doc = "0x3c - Interrupt endpoints. Only valid in HOST mode."]
    #[inline(always)]
    pub const fn host_addr_endp15(&self) -> &HOST_ADDR_ENDP {
        self.host_addr_endp(14)
    }
    #[doc = "0x40 - Main control register"]
    #[inline(always)]
    pub const fn main_ctrl(&self) -> &MAIN_CTRL {
        &self.main_ctrl
    }
    #[doc = "0x44 - Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
    #[inline(always)]
    pub const fn sof_wr(&self) -> &SOF_WR {
        &self.sof_wr
    }
    #[doc = "0x48 - Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
    #[inline(always)]
    pub const fn sof_rd(&self) -> &SOF_RD {
        &self.sof_rd
    }
    #[doc = "0x4c - SIE control register"]
    #[inline(always)]
    pub const fn sie_ctrl(&self) -> &SIE_CTRL {
        &self.sie_ctrl
    }
    #[doc = "0x50 - SIE status register"]
    #[inline(always)]
    pub const fn sie_status(&self) -> &SIE_STATUS {
        &self.sie_status
    }
    #[doc = "0x54 - interrupt endpoint control register"]
    #[inline(always)]
    pub const fn int_ep_ctrl(&self) -> &INT_EP_CTRL {
        &self.int_ep_ctrl
    }
    #[doc = "0x58 - Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
    #[inline(always)]
    pub const fn buff_status(&self) -> &BUFF_STATUS {
        &self.buff_status
    }
    #[doc = "0x5c - Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
    #[inline(always)]
    pub const fn buff_cpu_should_handle(&self) -> &BUFF_CPU_SHOULD_HANDLE {
        &self.buff_cpu_should_handle
    }
    #[doc = "0x60 - Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
    #[inline(always)]
    pub const fn ep_abort(&self) -> &EP_ABORT {
        &self.ep_abort
    }
    #[doc = "0x64 - Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
    #[inline(always)]
    pub const fn ep_abort_done(&self) -> &EP_ABORT_DONE {
        &self.ep_abort_done
    }
    #[doc = "0x68 - Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
    #[inline(always)]
    pub const fn ep_stall_arm(&self) -> &EP_STALL_ARM {
        &self.ep_stall_arm
    }
    #[doc = "0x6c - Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
    #[inline(always)]
    pub const fn nak_poll(&self) -> &NAK_POLL {
        &self.nak_poll
    }
    #[doc = "0x70 - Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
    #[inline(always)]
    pub const fn ep_status_stall_nak(&self) -> &EP_STATUS_STALL_NAK {
        &self.ep_status_stall_nak
    }
    #[doc = "0x74 - Where to connect the USB controller. Should be to_phy by default."]
    #[inline(always)]
    pub const fn usb_muxing(&self) -> &USB_MUXING {
        &self.usb_muxing
    }
    #[doc = "0x78 - Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable so switch over to the override value."]
    #[inline(always)]
    pub const fn usb_pwr(&self) -> &USB_PWR {
        &self.usb_pwr
    }
    #[doc = "0x7c - Note that most functions are driven directly from usb_fsls controller. This register allows more detailed control/status from the USB PHY. Useful for debug but not expected to be used in normal operation Use in conjunction with usbphy_direct_override register"]
    #[inline(always)]
    pub const fn usbphy_direct(&self) -> &USBPHY_DIRECT {
        &self.usbphy_direct
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn usbphy_direct_override(&self) -> &USBPHY_DIRECT_OVERRIDE {
        &self.usbphy_direct_override
    }
    #[doc = "0x84 - Note that most functions are driven directly from usb_fsls controller. This register allows more detailed control/status from the USB PHY. Useful for debug but not expected to be used in normal operation"]
    #[inline(always)]
    pub const fn usbphy_trim(&self) -> &USBPHY_TRIM {
        &self.usbphy_trim
    }
    #[doc = "0x8c - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x90 - Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0x94 - Interrupt Force"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0x98 - Interrupt status after masking &amp; forcing"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
}
#[doc = "ADDR_ENDP (rw) register accessor: Device address and endpoint control  

You can [`read`](crate::generic::Reg::read) this register and get [`addr_endp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_endp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@addr_endp`]
module"]
pub type ADDR_ENDP = crate::Reg<addr_endp::ADDR_ENDP_SPEC>;
#[doc = "Device address and endpoint control"]
pub mod addr_endp;
#[doc = "HOST_ADDR_ENDP (rw) register accessor: Interrupt endpoints. Only valid in HOST mode.  

You can [`read`](crate::generic::Reg::read) this register and get [`host_addr_endp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_addr_endp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@host_addr_endp`]
module"]
pub type HOST_ADDR_ENDP = crate::Reg<host_addr_endp::HOST_ADDR_ENDP_SPEC>;
#[doc = "Interrupt endpoints. Only valid in HOST mode."]
pub mod host_addr_endp;
#[doc = "MAIN_CTRL (rw) register accessor: Main control register  

You can [`read`](crate::generic::Reg::read) this register and get [`main_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@main_ctrl`]
module"]
pub type MAIN_CTRL = crate::Reg<main_ctrl::MAIN_CTRL_SPEC>;
#[doc = "Main control register"]
pub mod main_ctrl;
#[doc = "SOF_WR (rw) register accessor: Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time.  

You can [`read`](crate::generic::Reg::read) this register and get [`sof_wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sof_wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sof_wr`]
module"]
pub type SOF_WR = crate::Reg<sof_wr::SOF_WR_SPEC>;
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
pub mod sof_wr;
#[doc = "SOF_RD (rw) register accessor: Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host.  

You can [`read`](crate::generic::Reg::read) this register and get [`sof_rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sof_rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sof_rd`]
module"]
pub type SOF_RD = crate::Reg<sof_rd::SOF_RD_SPEC>;
#[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
pub mod sof_rd;
#[doc = "SIE_CTRL (rw) register accessor: SIE control register  

You can [`read`](crate::generic::Reg::read) this register and get [`sie_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sie_ctrl`]
module"]
pub type SIE_CTRL = crate::Reg<sie_ctrl::SIE_CTRL_SPEC>;
#[doc = "SIE control register"]
pub mod sie_ctrl;
#[doc = "SIE_STATUS (rw) register accessor: SIE status register  

You can [`read`](crate::generic::Reg::read) this register and get [`sie_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sie_status`]
module"]
pub type SIE_STATUS = crate::Reg<sie_status::SIE_STATUS_SPEC>;
#[doc = "SIE status register"]
pub mod sie_status;
#[doc = "INT_EP_CTRL (rw) register accessor: interrupt endpoint control register  

You can [`read`](crate::generic::Reg::read) this register and get [`int_ep_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ep_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@int_ep_ctrl`]
module"]
pub type INT_EP_CTRL = crate::Reg<int_ep_ctrl::INT_EP_CTRL_SPEC>;
#[doc = "interrupt endpoint control register"]
pub mod int_ep_ctrl;
#[doc = "BUFF_STATUS (rw) register accessor: Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle.  

You can [`read`](crate::generic::Reg::read) this register and get [`buff_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buff_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@buff_status`]
module"]
pub type BUFF_STATUS = crate::Reg<buff_status::BUFF_STATUS_SPEC>;
#[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
pub mod buff_status;
#[doc = "BUFF_CPU_SHOULD_HANDLE (rw) register accessor: Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered.  

You can [`read`](crate::generic::Reg::read) this register and get [`buff_cpu_should_handle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buff_cpu_should_handle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@buff_cpu_should_handle`]
module"]
pub type BUFF_CPU_SHOULD_HANDLE = crate::Reg<buff_cpu_should_handle::BUFF_CPU_SHOULD_HANDLE_SPEC>;
#[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
pub mod buff_cpu_should_handle;
#[doc = "EP_ABORT (rw) register accessor: Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register.  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_abort::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_abort::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep_abort`]
module"]
pub type EP_ABORT = crate::Reg<ep_abort::EP_ABORT_SPEC>;
#[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
pub mod ep_abort;
#[doc = "EP_ABORT_DONE (rw) register accessor: Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register.  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_abort_done::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_abort_done::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep_abort_done`]
module"]
pub type EP_ABORT_DONE = crate::Reg<ep_abort_done::EP_ABORT_DONE_SPEC>;
#[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
pub mod ep_abort_done;
#[doc = "EP_STALL_ARM (rw) register accessor: Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received.  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_stall_arm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_stall_arm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep_stall_arm`]
module"]
pub type EP_STALL_ARM = crate::Reg<ep_stall_arm::EP_STALL_ARM_SPEC>;
#[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
pub mod ep_stall_arm;
#[doc = "NAK_POLL (rw) register accessor: Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK.  

You can [`read`](crate::generic::Reg::read) this register and get [`nak_poll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nak_poll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nak_poll`]
module"]
pub type NAK_POLL = crate::Reg<nak_poll::NAK_POLL_SPEC>;
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
pub mod nak_poll;
#[doc = "EP_STATUS_STALL_NAK (rw) register accessor: Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register.  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_status_stall_nak::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_status_stall_nak::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep_status_stall_nak`]
module"]
pub type EP_STATUS_STALL_NAK = crate::Reg<ep_status_stall_nak::EP_STATUS_STALL_NAK_SPEC>;
#[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
pub mod ep_status_stall_nak;
#[doc = "USB_MUXING (rw) register accessor: Where to connect the USB controller. Should be to_phy by default.  

You can [`read`](crate::generic::Reg::read) this register and get [`usb_muxing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_muxing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usb_muxing`]
module"]
pub type USB_MUXING = crate::Reg<usb_muxing::USB_MUXING_SPEC>;
#[doc = "Where to connect the USB controller. Should be to_phy by default."]
pub mod usb_muxing;
#[doc = "USB_PWR (rw) register accessor: Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable so switch over to the override value.  

You can [`read`](crate::generic::Reg::read) this register and get [`usb_pwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_pwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usb_pwr`]
module"]
pub type USB_PWR = crate::Reg<usb_pwr::USB_PWR_SPEC>;
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable so switch over to the override value."]
pub mod usb_pwr;
#[doc = "USBPHY_DIRECT (rw) register accessor: Note that most functions are driven directly from usb_fsls controller. This register allows more detailed control/status from the USB PHY. Useful for debug but not expected to be used in normal operation Use in conjunction with usbphy_direct_override register  

You can [`read`](crate::generic::Reg::read) this register and get [`usbphy_direct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy_direct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usbphy_direct`]
module"]
pub type USBPHY_DIRECT = crate::Reg<usbphy_direct::USBPHY_DIRECT_SPEC>;
#[doc = "Note that most functions are driven directly from usb_fsls controller. This register allows more detailed control/status from the USB PHY. Useful for debug but not expected to be used in normal operation Use in conjunction with usbphy_direct_override register"]
pub mod usbphy_direct;
#[doc = "USBPHY_DIRECT_OVERRIDE (rw) register accessor:   

You can [`read`](crate::generic::Reg::read) this register and get [`usbphy_direct_override::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy_direct_override::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usbphy_direct_override`]
module"]
pub type USBPHY_DIRECT_OVERRIDE = crate::Reg<usbphy_direct_override::USBPHY_DIRECT_OVERRIDE_SPEC>;
#[doc = ""]
pub mod usbphy_direct_override;
#[doc = "USBPHY_TRIM (rw) register accessor: Note that most functions are driven directly from usb_fsls controller. This register allows more detailed control/status from the USB PHY. Useful for debug but not expected to be used in normal operation  

You can [`read`](crate::generic::Reg::read) this register and get [`usbphy_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usbphy_trim`]
module"]
pub type USBPHY_TRIM = crate::Reg<usbphy_trim::USBPHY_TRIM_SPEC>;
#[doc = "Note that most functions are driven directly from usb_fsls controller. This register allows more detailed control/status from the USB PHY. Useful for debug but not expected to be used in normal operation"]
pub mod usbphy_trim;
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
