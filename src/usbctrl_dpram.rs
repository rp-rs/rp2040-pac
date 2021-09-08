#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bytes 0-3 of the SETUP packet from the host."]
    pub setup_packet_low: crate::Reg<setup_packet_low::SETUP_PACKET_LOW_SPEC>,
    #[doc = "0x04 - Bytes 4-7 of the setup packet from the host."]
    pub setup_packet_high: crate::Reg<setup_packet_high::SETUP_PACKET_HIGH_SPEC>,
    #[doc = "0x08..0x80 - %s-%s"]
    pub ep_control: [crate::Reg<ep_control::EP_CONTROL_SPEC>; 30],
    #[doc = "0x80..0x100 - %s-%s"]
    pub ep_buffer_control: [crate::Reg<ep_buffer_control::EP_BUFFER_CONTROL_SPEC>; 32],
}
#[doc = "SETUP_PACKET_LOW register accessor: an alias for `Reg<SETUP_PACKET_LOW_SPEC>`"]
pub type SETUP_PACKET_LOW = crate::Reg<setup_packet_low::SETUP_PACKET_LOW_SPEC>;
#[doc = "Bytes 0-3 of the SETUP packet from the host."]
pub mod setup_packet_low;
#[doc = "SETUP_PACKET_HIGH register accessor: an alias for `Reg<SETUP_PACKET_HIGH_SPEC>`"]
pub type SETUP_PACKET_HIGH = crate::Reg<setup_packet_high::SETUP_PACKET_HIGH_SPEC>;
#[doc = "Bytes 4-7 of the setup packet from the host."]
pub mod setup_packet_high;
#[doc = "EP_CONTROL register accessor: an alias for `Reg<EP_CONTROL_SPEC>`"]
pub type EP_CONTROL = crate::Reg<ep_control::EP_CONTROL_SPEC>;
#[doc = "%s-%s"]
pub mod ep_control;
#[doc = "EP_BUFFER_CONTROL register accessor: an alias for `Reg<EP_BUFFER_CONTROL_SPEC>`"]
pub type EP_BUFFER_CONTROL = crate::Reg<ep_buffer_control::EP_BUFFER_CONTROL_SPEC>;
#[doc = "%s-%s"]
pub mod ep_buffer_control;
