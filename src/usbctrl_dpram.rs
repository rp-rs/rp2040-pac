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
#[doc = "SETUP_PACKET_LOW (rw) register accessor: an alias for `Reg<SETUP_PACKET_LOW_SPEC>`"]
pub type SETUP_PACKET_LOW = crate::Reg<setup_packet_low::SETUP_PACKET_LOW_SPEC>;
#[doc = "Bytes 0-3 of the SETUP packet from the host."]
pub mod setup_packet_low;
#[doc = "SETUP_PACKET_HIGH (rw) register accessor: an alias for `Reg<SETUP_PACKET_HIGH_SPEC>`"]
pub type SETUP_PACKET_HIGH = crate::Reg<setup_packet_high::SETUP_PACKET_HIGH_SPEC>;
#[doc = "Bytes 4-7 of the setup packet from the host."]
pub mod setup_packet_high;
#[doc = "EP_CONTROL (rw) register accessor: an alias for `Reg<EP_CONTROL_SPEC>`"]
pub type EP_CONTROL = crate::Reg<ep_control::EP_CONTROL_SPEC>;
#[doc = "-"]
pub mod ep_control;
#[doc = "EP_BUFFER_CONTROL (rw) register accessor: an alias for `Reg<EP_BUFFER_CONTROL_SPEC>`"]
pub type EP_BUFFER_CONTROL = crate::Reg<ep_buffer_control::EP_BUFFER_CONTROL_SPEC>;
#[doc = "-"]
pub mod ep_buffer_control;
#[doc = "EPX_CONTROL (rw) register accessor: an alias for `Reg<EPX_CONTROL_SPEC>`"]
pub type EPX_CONTROL = crate::Reg<epx_control::EPX_CONTROL_SPEC>;
#[doc = "EPx Control (Host-mode only!)"]
pub mod epx_control;
