#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bytes 0-3 of the SETUP packet from the host."]
    pub setup_packet_low: SETUP_PACKET_LOW,
    #[doc = "0x04 - Bytes 4-7 of the setup packet from the host."]
    pub setup_packet_high: SETUP_PACKET_HIGH,
    #[doc = "0x08..0x80 - -"]
    pub ep_control: [EP_CONTROL; 30],
    #[doc = "0x80..0x100 - -"]
    pub ep_buffer_control: [EP_BUFFER_CONTROL; 32],
    #[doc = "0x100 - EPx Control (Host-mode only!)"]
    pub epx_control: EPX_CONTROL,
}
#[doc = "SETUP_PACKET_LOW (rw) register accessor: Bytes 0-3 of the SETUP packet from the host.  

You can [`read`](crate::generic::Reg::read) this register and get [`setup_packet_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup_packet_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@setup_packet_low`]
module"]
pub type SETUP_PACKET_LOW = crate::Reg<setup_packet_low::SETUP_PACKET_LOW_SPEC>;
#[doc = "Bytes 0-3 of the SETUP packet from the host."]
pub mod setup_packet_low;
#[doc = "SETUP_PACKET_HIGH (rw) register accessor: Bytes 4-7 of the setup packet from the host.  

You can [`read`](crate::generic::Reg::read) this register and get [`setup_packet_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup_packet_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@setup_packet_high`]
module"]
pub type SETUP_PACKET_HIGH = crate::Reg<setup_packet_high::SETUP_PACKET_HIGH_SPEC>;
#[doc = "Bytes 4-7 of the setup packet from the host."]
pub mod setup_packet_high;
#[doc = "EP_CONTROL (rw) register accessor: -  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep_control`]
module"]
pub type EP_CONTROL = crate::Reg<ep_control::EP_CONTROL_SPEC>;
#[doc = "-"]
pub mod ep_control;
#[doc = "EP_BUFFER_CONTROL (rw) register accessor: -  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_buffer_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_buffer_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep_buffer_control`]
module"]
pub type EP_BUFFER_CONTROL = crate::Reg<ep_buffer_control::EP_BUFFER_CONTROL_SPEC>;
#[doc = "-"]
pub mod ep_buffer_control;
#[doc = "EPX_CONTROL (rw) register accessor: EPx Control (Host-mode only!)  

You can [`read`](crate::generic::Reg::read) this register and get [`epx_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epx_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@epx_control`]
module"]
pub type EPX_CONTROL = crate::Reg<epx_control::EPX_CONTROL_SPEC>;
#[doc = "EPx Control (Host-mode only!)"]
pub mod epx_control;
