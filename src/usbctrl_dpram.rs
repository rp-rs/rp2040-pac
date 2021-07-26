#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bytes 0-3 of the SETUP packet from the host."]
    pub setup_packet_low: crate::Reg<setup_packet_low::SETUP_PACKET_LOW_SPEC>,
    #[doc = "0x04 - Bytes 4-7 of the setup packet from the host."]
    pub setup_packet_high: crate::Reg<setup_packet_high::SETUP_PACKET_HIGH_SPEC>,
    #[doc = "0x08 - "]
    pub ep1_in_control: crate::Reg<ep1_in_control::EP1_IN_CONTROL_SPEC>,
    #[doc = "0x0c - "]
    pub ep1_out_control: crate::Reg<ep1_out_control::EP1_OUT_CONTROL_SPEC>,
    #[doc = "0x10 - "]
    pub ep2_in_control: crate::Reg<ep2_in_control::EP2_IN_CONTROL_SPEC>,
    #[doc = "0x14 - "]
    pub ep2_out_control: crate::Reg<ep2_out_control::EP2_OUT_CONTROL_SPEC>,
    #[doc = "0x18 - "]
    pub ep3_in_control: crate::Reg<ep3_in_control::EP3_IN_CONTROL_SPEC>,
    #[doc = "0x1c - "]
    pub ep3_out_control: crate::Reg<ep3_out_control::EP3_OUT_CONTROL_SPEC>,
    #[doc = "0x20 - "]
    pub ep4_in_control: crate::Reg<ep4_in_control::EP4_IN_CONTROL_SPEC>,
    #[doc = "0x24 - "]
    pub ep4_out_control: crate::Reg<ep4_out_control::EP4_OUT_CONTROL_SPEC>,
    #[doc = "0x28 - "]
    pub ep5_in_control: crate::Reg<ep5_in_control::EP5_IN_CONTROL_SPEC>,
    #[doc = "0x2c - "]
    pub ep5_out_control: crate::Reg<ep5_out_control::EP5_OUT_CONTROL_SPEC>,
    #[doc = "0x30 - "]
    pub ep6_in_control: crate::Reg<ep6_in_control::EP6_IN_CONTROL_SPEC>,
    #[doc = "0x34 - "]
    pub ep6_out_control: crate::Reg<ep6_out_control::EP6_OUT_CONTROL_SPEC>,
    #[doc = "0x38 - "]
    pub ep7_in_control: crate::Reg<ep7_in_control::EP7_IN_CONTROL_SPEC>,
    #[doc = "0x3c - "]
    pub ep7_out_control: crate::Reg<ep7_out_control::EP7_OUT_CONTROL_SPEC>,
    #[doc = "0x40 - "]
    pub ep8_in_control: crate::Reg<ep8_in_control::EP8_IN_CONTROL_SPEC>,
    #[doc = "0x44 - "]
    pub ep8_out_control: crate::Reg<ep8_out_control::EP8_OUT_CONTROL_SPEC>,
    #[doc = "0x48 - "]
    pub ep9_in_control: crate::Reg<ep9_in_control::EP9_IN_CONTROL_SPEC>,
    #[doc = "0x4c - "]
    pub ep9_out_control: crate::Reg<ep9_out_control::EP9_OUT_CONTROL_SPEC>,
    #[doc = "0x50 - "]
    pub ep10_in_control: crate::Reg<ep10_in_control::EP10_IN_CONTROL_SPEC>,
    #[doc = "0x54 - "]
    pub ep10_out_control: crate::Reg<ep10_out_control::EP10_OUT_CONTROL_SPEC>,
    #[doc = "0x58 - "]
    pub ep11_in_control: crate::Reg<ep11_in_control::EP11_IN_CONTROL_SPEC>,
    #[doc = "0x5c - "]
    pub ep11_out_control: crate::Reg<ep11_out_control::EP11_OUT_CONTROL_SPEC>,
    #[doc = "0x60 - "]
    pub ep12_in_control: crate::Reg<ep12_in_control::EP12_IN_CONTROL_SPEC>,
    #[doc = "0x64 - "]
    pub ep12_out_control: crate::Reg<ep12_out_control::EP12_OUT_CONTROL_SPEC>,
    #[doc = "0x68 - "]
    pub ep13_in_control: crate::Reg<ep13_in_control::EP13_IN_CONTROL_SPEC>,
    #[doc = "0x6c - "]
    pub ep13_out_control: crate::Reg<ep13_out_control::EP13_OUT_CONTROL_SPEC>,
    #[doc = "0x70 - "]
    pub ep14_in_control: crate::Reg<ep14_in_control::EP14_IN_CONTROL_SPEC>,
    #[doc = "0x74 - "]
    pub ep14_out_control: crate::Reg<ep14_out_control::EP14_OUT_CONTROL_SPEC>,
    #[doc = "0x78 - "]
    pub ep15_in_control: crate::Reg<ep15_in_control::EP15_IN_CONTROL_SPEC>,
    #[doc = "0x7c - "]
    pub ep15_out_control: crate::Reg<ep15_out_control::EP15_OUT_CONTROL_SPEC>,
    #[doc = "0x80 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep0_in_buffer_control: crate::Reg<ep0_in_buffer_control::EP0_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0x84 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep0_out_buffer_control: crate::Reg<ep0_out_buffer_control::EP0_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0x88 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep1_in_buffer_control: crate::Reg<ep1_in_buffer_control::EP1_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0x8c - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep1_out_buffer_control: crate::Reg<ep1_out_buffer_control::EP1_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0x90 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep2_in_buffer_control: crate::Reg<ep2_in_buffer_control::EP2_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0x94 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep2_out_buffer_control: crate::Reg<ep2_out_buffer_control::EP2_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0x98 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep3_in_buffer_control: crate::Reg<ep3_in_buffer_control::EP3_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0x9c - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep3_out_buffer_control: crate::Reg<ep3_out_buffer_control::EP3_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xa0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep4_in_buffer_control: crate::Reg<ep4_in_buffer_control::EP4_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xa4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep4_out_buffer_control: crate::Reg<ep4_out_buffer_control::EP4_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xa8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep5_in_buffer_control: crate::Reg<ep5_in_buffer_control::EP5_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xac - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep5_out_buffer_control: crate::Reg<ep5_out_buffer_control::EP5_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xb0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep6_in_buffer_control: crate::Reg<ep6_in_buffer_control::EP6_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xb4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep6_out_buffer_control: crate::Reg<ep6_out_buffer_control::EP6_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xb8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep7_in_buffer_control: crate::Reg<ep7_in_buffer_control::EP7_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xbc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep7_out_buffer_control: crate::Reg<ep7_out_buffer_control::EP7_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xc0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep8_in_buffer_control: crate::Reg<ep8_in_buffer_control::EP8_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xc4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep8_out_buffer_control: crate::Reg<ep8_out_buffer_control::EP8_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xc8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep9_in_buffer_control: crate::Reg<ep9_in_buffer_control::EP9_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xcc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep9_out_buffer_control: crate::Reg<ep9_out_buffer_control::EP9_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xd0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep10_in_buffer_control: crate::Reg<ep10_in_buffer_control::EP10_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xd4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep10_out_buffer_control: crate::Reg<ep10_out_buffer_control::EP10_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xd8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep11_in_buffer_control: crate::Reg<ep11_in_buffer_control::EP11_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xdc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep11_out_buffer_control: crate::Reg<ep11_out_buffer_control::EP11_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xe0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep12_in_buffer_control: crate::Reg<ep12_in_buffer_control::EP12_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xe4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep12_out_buffer_control: crate::Reg<ep12_out_buffer_control::EP12_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xe8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep13_in_buffer_control: crate::Reg<ep13_in_buffer_control::EP13_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xec - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep13_out_buffer_control: crate::Reg<ep13_out_buffer_control::EP13_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xf0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep14_in_buffer_control: crate::Reg<ep14_in_buffer_control::EP14_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xf4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep14_out_buffer_control: crate::Reg<ep14_out_buffer_control::EP14_OUT_BUFFER_CONTROL_SPEC>,
    #[doc = "0xf8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep15_in_buffer_control: crate::Reg<ep15_in_buffer_control::EP15_IN_BUFFER_CONTROL_SPEC>,
    #[doc = "0xfc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep15_out_buffer_control: crate::Reg<ep15_out_buffer_control::EP15_OUT_BUFFER_CONTROL_SPEC>,
}
#[doc = "SETUP_PACKET_LOW register accessor: an alias for `Reg<SETUP_PACKET_LOW_SPEC>`"]
pub type SETUP_PACKET_LOW = crate::Reg<setup_packet_low::SETUP_PACKET_LOW_SPEC>;
#[doc = "Bytes 0-3 of the SETUP packet from the host."]
pub mod setup_packet_low;
#[doc = "SETUP_PACKET_HIGH register accessor: an alias for `Reg<SETUP_PACKET_HIGH_SPEC>`"]
pub type SETUP_PACKET_HIGH = crate::Reg<setup_packet_high::SETUP_PACKET_HIGH_SPEC>;
#[doc = "Bytes 4-7 of the setup packet from the host."]
pub mod setup_packet_high;
#[doc = "EP1_IN_CONTROL register accessor: an alias for `Reg<EP1_IN_CONTROL_SPEC>`"]
pub type EP1_IN_CONTROL = crate::Reg<ep1_in_control::EP1_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep1_in_control;
#[doc = "EP1_OUT_CONTROL register accessor: an alias for `Reg<EP1_OUT_CONTROL_SPEC>`"]
pub type EP1_OUT_CONTROL = crate::Reg<ep1_out_control::EP1_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep1_out_control;
#[doc = "EP2_IN_CONTROL register accessor: an alias for `Reg<EP2_IN_CONTROL_SPEC>`"]
pub type EP2_IN_CONTROL = crate::Reg<ep2_in_control::EP2_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep2_in_control;
#[doc = "EP2_OUT_CONTROL register accessor: an alias for `Reg<EP2_OUT_CONTROL_SPEC>`"]
pub type EP2_OUT_CONTROL = crate::Reg<ep2_out_control::EP2_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep2_out_control;
#[doc = "EP3_IN_CONTROL register accessor: an alias for `Reg<EP3_IN_CONTROL_SPEC>`"]
pub type EP3_IN_CONTROL = crate::Reg<ep3_in_control::EP3_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep3_in_control;
#[doc = "EP3_OUT_CONTROL register accessor: an alias for `Reg<EP3_OUT_CONTROL_SPEC>`"]
pub type EP3_OUT_CONTROL = crate::Reg<ep3_out_control::EP3_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep3_out_control;
#[doc = "EP4_IN_CONTROL register accessor: an alias for `Reg<EP4_IN_CONTROL_SPEC>`"]
pub type EP4_IN_CONTROL = crate::Reg<ep4_in_control::EP4_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep4_in_control;
#[doc = "EP4_OUT_CONTROL register accessor: an alias for `Reg<EP4_OUT_CONTROL_SPEC>`"]
pub type EP4_OUT_CONTROL = crate::Reg<ep4_out_control::EP4_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep4_out_control;
#[doc = "EP5_IN_CONTROL register accessor: an alias for `Reg<EP5_IN_CONTROL_SPEC>`"]
pub type EP5_IN_CONTROL = crate::Reg<ep5_in_control::EP5_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep5_in_control;
#[doc = "EP5_OUT_CONTROL register accessor: an alias for `Reg<EP5_OUT_CONTROL_SPEC>`"]
pub type EP5_OUT_CONTROL = crate::Reg<ep5_out_control::EP5_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep5_out_control;
#[doc = "EP6_IN_CONTROL register accessor: an alias for `Reg<EP6_IN_CONTROL_SPEC>`"]
pub type EP6_IN_CONTROL = crate::Reg<ep6_in_control::EP6_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep6_in_control;
#[doc = "EP6_OUT_CONTROL register accessor: an alias for `Reg<EP6_OUT_CONTROL_SPEC>`"]
pub type EP6_OUT_CONTROL = crate::Reg<ep6_out_control::EP6_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep6_out_control;
#[doc = "EP7_IN_CONTROL register accessor: an alias for `Reg<EP7_IN_CONTROL_SPEC>`"]
pub type EP7_IN_CONTROL = crate::Reg<ep7_in_control::EP7_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep7_in_control;
#[doc = "EP7_OUT_CONTROL register accessor: an alias for `Reg<EP7_OUT_CONTROL_SPEC>`"]
pub type EP7_OUT_CONTROL = crate::Reg<ep7_out_control::EP7_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep7_out_control;
#[doc = "EP8_IN_CONTROL register accessor: an alias for `Reg<EP8_IN_CONTROL_SPEC>`"]
pub type EP8_IN_CONTROL = crate::Reg<ep8_in_control::EP8_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep8_in_control;
#[doc = "EP8_OUT_CONTROL register accessor: an alias for `Reg<EP8_OUT_CONTROL_SPEC>`"]
pub type EP8_OUT_CONTROL = crate::Reg<ep8_out_control::EP8_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep8_out_control;
#[doc = "EP9_IN_CONTROL register accessor: an alias for `Reg<EP9_IN_CONTROL_SPEC>`"]
pub type EP9_IN_CONTROL = crate::Reg<ep9_in_control::EP9_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep9_in_control;
#[doc = "EP9_OUT_CONTROL register accessor: an alias for `Reg<EP9_OUT_CONTROL_SPEC>`"]
pub type EP9_OUT_CONTROL = crate::Reg<ep9_out_control::EP9_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep9_out_control;
#[doc = "EP10_IN_CONTROL register accessor: an alias for `Reg<EP10_IN_CONTROL_SPEC>`"]
pub type EP10_IN_CONTROL = crate::Reg<ep10_in_control::EP10_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep10_in_control;
#[doc = "EP10_OUT_CONTROL register accessor: an alias for `Reg<EP10_OUT_CONTROL_SPEC>`"]
pub type EP10_OUT_CONTROL = crate::Reg<ep10_out_control::EP10_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep10_out_control;
#[doc = "EP11_IN_CONTROL register accessor: an alias for `Reg<EP11_IN_CONTROL_SPEC>`"]
pub type EP11_IN_CONTROL = crate::Reg<ep11_in_control::EP11_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep11_in_control;
#[doc = "EP11_OUT_CONTROL register accessor: an alias for `Reg<EP11_OUT_CONTROL_SPEC>`"]
pub type EP11_OUT_CONTROL = crate::Reg<ep11_out_control::EP11_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep11_out_control;
#[doc = "EP12_IN_CONTROL register accessor: an alias for `Reg<EP12_IN_CONTROL_SPEC>`"]
pub type EP12_IN_CONTROL = crate::Reg<ep12_in_control::EP12_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep12_in_control;
#[doc = "EP12_OUT_CONTROL register accessor: an alias for `Reg<EP12_OUT_CONTROL_SPEC>`"]
pub type EP12_OUT_CONTROL = crate::Reg<ep12_out_control::EP12_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep12_out_control;
#[doc = "EP13_IN_CONTROL register accessor: an alias for `Reg<EP13_IN_CONTROL_SPEC>`"]
pub type EP13_IN_CONTROL = crate::Reg<ep13_in_control::EP13_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep13_in_control;
#[doc = "EP13_OUT_CONTROL register accessor: an alias for `Reg<EP13_OUT_CONTROL_SPEC>`"]
pub type EP13_OUT_CONTROL = crate::Reg<ep13_out_control::EP13_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep13_out_control;
#[doc = "EP14_IN_CONTROL register accessor: an alias for `Reg<EP14_IN_CONTROL_SPEC>`"]
pub type EP14_IN_CONTROL = crate::Reg<ep14_in_control::EP14_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep14_in_control;
#[doc = "EP14_OUT_CONTROL register accessor: an alias for `Reg<EP14_OUT_CONTROL_SPEC>`"]
pub type EP14_OUT_CONTROL = crate::Reg<ep14_out_control::EP14_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep14_out_control;
#[doc = "EP15_IN_CONTROL register accessor: an alias for `Reg<EP15_IN_CONTROL_SPEC>`"]
pub type EP15_IN_CONTROL = crate::Reg<ep15_in_control::EP15_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep15_in_control;
#[doc = "EP15_OUT_CONTROL register accessor: an alias for `Reg<EP15_OUT_CONTROL_SPEC>`"]
pub type EP15_OUT_CONTROL = crate::Reg<ep15_out_control::EP15_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep15_out_control;
#[doc = "EP0_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP0_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP0_IN_BUFFER_CONTROL = crate::Reg<ep0_in_buffer_control::EP0_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep0_in_buffer_control;
#[doc = "EP0_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP0_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP0_OUT_BUFFER_CONTROL = crate::Reg<ep0_out_buffer_control::EP0_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep0_out_buffer_control;
#[doc = "EP1_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP1_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP1_IN_BUFFER_CONTROL = crate::Reg<ep1_in_buffer_control::EP1_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep1_in_buffer_control;
#[doc = "EP1_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP1_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP1_OUT_BUFFER_CONTROL = crate::Reg<ep1_out_buffer_control::EP1_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep1_out_buffer_control;
#[doc = "EP2_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP2_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP2_IN_BUFFER_CONTROL = crate::Reg<ep2_in_buffer_control::EP2_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep2_in_buffer_control;
#[doc = "EP2_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP2_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP2_OUT_BUFFER_CONTROL = crate::Reg<ep2_out_buffer_control::EP2_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep2_out_buffer_control;
#[doc = "EP3_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP3_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP3_IN_BUFFER_CONTROL = crate::Reg<ep3_in_buffer_control::EP3_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep3_in_buffer_control;
#[doc = "EP3_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP3_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP3_OUT_BUFFER_CONTROL = crate::Reg<ep3_out_buffer_control::EP3_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep3_out_buffer_control;
#[doc = "EP4_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP4_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP4_IN_BUFFER_CONTROL = crate::Reg<ep4_in_buffer_control::EP4_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep4_in_buffer_control;
#[doc = "EP4_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP4_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP4_OUT_BUFFER_CONTROL = crate::Reg<ep4_out_buffer_control::EP4_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep4_out_buffer_control;
#[doc = "EP5_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP5_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP5_IN_BUFFER_CONTROL = crate::Reg<ep5_in_buffer_control::EP5_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep5_in_buffer_control;
#[doc = "EP5_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP5_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP5_OUT_BUFFER_CONTROL = crate::Reg<ep5_out_buffer_control::EP5_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep5_out_buffer_control;
#[doc = "EP6_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP6_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP6_IN_BUFFER_CONTROL = crate::Reg<ep6_in_buffer_control::EP6_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep6_in_buffer_control;
#[doc = "EP6_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP6_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP6_OUT_BUFFER_CONTROL = crate::Reg<ep6_out_buffer_control::EP6_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep6_out_buffer_control;
#[doc = "EP7_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP7_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP7_IN_BUFFER_CONTROL = crate::Reg<ep7_in_buffer_control::EP7_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep7_in_buffer_control;
#[doc = "EP7_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP7_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP7_OUT_BUFFER_CONTROL = crate::Reg<ep7_out_buffer_control::EP7_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep7_out_buffer_control;
#[doc = "EP8_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP8_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP8_IN_BUFFER_CONTROL = crate::Reg<ep8_in_buffer_control::EP8_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep8_in_buffer_control;
#[doc = "EP8_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP8_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP8_OUT_BUFFER_CONTROL = crate::Reg<ep8_out_buffer_control::EP8_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep8_out_buffer_control;
#[doc = "EP9_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP9_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP9_IN_BUFFER_CONTROL = crate::Reg<ep9_in_buffer_control::EP9_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep9_in_buffer_control;
#[doc = "EP9_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP9_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP9_OUT_BUFFER_CONTROL = crate::Reg<ep9_out_buffer_control::EP9_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep9_out_buffer_control;
#[doc = "EP10_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP10_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP10_IN_BUFFER_CONTROL = crate::Reg<ep10_in_buffer_control::EP10_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep10_in_buffer_control;
#[doc = "EP10_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP10_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP10_OUT_BUFFER_CONTROL =
    crate::Reg<ep10_out_buffer_control::EP10_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep10_out_buffer_control;
#[doc = "EP11_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP11_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP11_IN_BUFFER_CONTROL = crate::Reg<ep11_in_buffer_control::EP11_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep11_in_buffer_control;
#[doc = "EP11_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP11_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP11_OUT_BUFFER_CONTROL =
    crate::Reg<ep11_out_buffer_control::EP11_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep11_out_buffer_control;
#[doc = "EP12_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP12_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP12_IN_BUFFER_CONTROL = crate::Reg<ep12_in_buffer_control::EP12_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep12_in_buffer_control;
#[doc = "EP12_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP12_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP12_OUT_BUFFER_CONTROL =
    crate::Reg<ep12_out_buffer_control::EP12_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep12_out_buffer_control;
#[doc = "EP13_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP13_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP13_IN_BUFFER_CONTROL = crate::Reg<ep13_in_buffer_control::EP13_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep13_in_buffer_control;
#[doc = "EP13_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP13_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP13_OUT_BUFFER_CONTROL =
    crate::Reg<ep13_out_buffer_control::EP13_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep13_out_buffer_control;
#[doc = "EP14_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP14_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP14_IN_BUFFER_CONTROL = crate::Reg<ep14_in_buffer_control::EP14_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep14_in_buffer_control;
#[doc = "EP14_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP14_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP14_OUT_BUFFER_CONTROL =
    crate::Reg<ep14_out_buffer_control::EP14_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep14_out_buffer_control;
#[doc = "EP15_IN_BUFFER_CONTROL register accessor: an alias for `Reg<EP15_IN_BUFFER_CONTROL_SPEC>`"]
pub type EP15_IN_BUFFER_CONTROL = crate::Reg<ep15_in_buffer_control::EP15_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep15_in_buffer_control;
#[doc = "EP15_OUT_BUFFER_CONTROL register accessor: an alias for `Reg<EP15_OUT_BUFFER_CONTROL_SPEC>`"]
pub type EP15_OUT_BUFFER_CONTROL =
    crate::Reg<ep15_out_buffer_control::EP15_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.  
 Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep15_out_buffer_control;
