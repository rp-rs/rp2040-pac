#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bytes 0-3 of the SETUP packet from the host."]
    pub setup_packet_low: SETUP_PACKET_LOW,
    #[doc = "0x04 - Bytes 4-7 of the setup packet from the host."]
    pub setup_packet_high: SETUP_PACKET_HIGH,
    #[doc = "0x08 - "]
    pub ep1_in_control: EP1_IN_CONTROL,
    #[doc = "0x0c - "]
    pub ep1_out_control: EP1_OUT_CONTROL,
    #[doc = "0x10 - "]
    pub ep2_in_control: EP2_IN_CONTROL,
    #[doc = "0x14 - "]
    pub ep2_out_control: EP2_OUT_CONTROL,
    #[doc = "0x18 - "]
    pub ep3_in_control: EP3_IN_CONTROL,
    #[doc = "0x1c - "]
    pub ep3_out_control: EP3_OUT_CONTROL,
    #[doc = "0x20 - "]
    pub ep4_in_control: EP4_IN_CONTROL,
    #[doc = "0x24 - "]
    pub ep4_out_control: EP4_OUT_CONTROL,
    #[doc = "0x28 - "]
    pub ep5_in_control: EP5_IN_CONTROL,
    #[doc = "0x2c - "]
    pub ep5_out_control: EP5_OUT_CONTROL,
    #[doc = "0x30 - "]
    pub ep6_in_control: EP6_IN_CONTROL,
    #[doc = "0x34 - "]
    pub ep6_out_control: EP6_OUT_CONTROL,
    #[doc = "0x38 - "]
    pub ep7_in_control: EP7_IN_CONTROL,
    #[doc = "0x3c - "]
    pub ep7_out_control: EP7_OUT_CONTROL,
    #[doc = "0x40 - "]
    pub ep8_in_control: EP8_IN_CONTROL,
    #[doc = "0x44 - "]
    pub ep8_out_control: EP8_OUT_CONTROL,
    #[doc = "0x48 - "]
    pub ep9_in_control: EP9_IN_CONTROL,
    #[doc = "0x4c - "]
    pub ep9_out_control: EP9_OUT_CONTROL,
    #[doc = "0x50 - "]
    pub ep10_in_control: EP10_IN_CONTROL,
    #[doc = "0x54 - "]
    pub ep10_out_control: EP10_OUT_CONTROL,
    #[doc = "0x58 - "]
    pub ep11_in_control: EP11_IN_CONTROL,
    #[doc = "0x5c - "]
    pub ep11_out_control: EP11_OUT_CONTROL,
    #[doc = "0x60 - "]
    pub ep12_in_control: EP12_IN_CONTROL,
    #[doc = "0x64 - "]
    pub ep12_out_control: EP12_OUT_CONTROL,
    #[doc = "0x68 - "]
    pub ep13_in_control: EP13_IN_CONTROL,
    #[doc = "0x6c - "]
    pub ep13_out_control: EP13_OUT_CONTROL,
    #[doc = "0x70 - "]
    pub ep14_in_control: EP14_IN_CONTROL,
    #[doc = "0x74 - "]
    pub ep14_out_control: EP14_OUT_CONTROL,
    #[doc = "0x78 - "]
    pub ep15_in_control: EP15_IN_CONTROL,
    #[doc = "0x7c - "]
    pub ep15_out_control: EP15_OUT_CONTROL,
    #[doc = "0x80 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep0_in_buffer_control: EP0_IN_BUFFER_CONTROL,
    #[doc = "0x84 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep0_out_buffer_control: EP0_OUT_BUFFER_CONTROL,
    #[doc = "0x88 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep1_in_buffer_control: EP1_IN_BUFFER_CONTROL,
    #[doc = "0x8c - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep1_out_buffer_control: EP1_OUT_BUFFER_CONTROL,
    #[doc = "0x90 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep2_in_buffer_control: EP2_IN_BUFFER_CONTROL,
    #[doc = "0x94 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep2_out_buffer_control: EP2_OUT_BUFFER_CONTROL,
    #[doc = "0x98 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep3_in_buffer_control: EP3_IN_BUFFER_CONTROL,
    #[doc = "0x9c - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep3_out_buffer_control: EP3_OUT_BUFFER_CONTROL,
    #[doc = "0xa0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep4_in_buffer_control: EP4_IN_BUFFER_CONTROL,
    #[doc = "0xa4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep4_out_buffer_control: EP4_OUT_BUFFER_CONTROL,
    #[doc = "0xa8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep5_in_buffer_control: EP5_IN_BUFFER_CONTROL,
    #[doc = "0xac - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep5_out_buffer_control: EP5_OUT_BUFFER_CONTROL,
    #[doc = "0xb0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep6_in_buffer_control: EP6_IN_BUFFER_CONTROL,
    #[doc = "0xb4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep6_out_buffer_control: EP6_OUT_BUFFER_CONTROL,
    #[doc = "0xb8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep7_in_buffer_control: EP7_IN_BUFFER_CONTROL,
    #[doc = "0xbc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep7_out_buffer_control: EP7_OUT_BUFFER_CONTROL,
    #[doc = "0xc0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep8_in_buffer_control: EP8_IN_BUFFER_CONTROL,
    #[doc = "0xc4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep8_out_buffer_control: EP8_OUT_BUFFER_CONTROL,
    #[doc = "0xc8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep9_in_buffer_control: EP9_IN_BUFFER_CONTROL,
    #[doc = "0xcc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep9_out_buffer_control: EP9_OUT_BUFFER_CONTROL,
    #[doc = "0xd0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep10_in_buffer_control: EP10_IN_BUFFER_CONTROL,
    #[doc = "0xd4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep10_out_buffer_control: EP10_OUT_BUFFER_CONTROL,
    #[doc = "0xd8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep11_in_buffer_control: EP11_IN_BUFFER_CONTROL,
    #[doc = "0xdc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep11_out_buffer_control: EP11_OUT_BUFFER_CONTROL,
    #[doc = "0xe0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep12_in_buffer_control: EP12_IN_BUFFER_CONTROL,
    #[doc = "0xe4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep12_out_buffer_control: EP12_OUT_BUFFER_CONTROL,
    #[doc = "0xe8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep13_in_buffer_control: EP13_IN_BUFFER_CONTROL,
    #[doc = "0xec - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep13_out_buffer_control: EP13_OUT_BUFFER_CONTROL,
    #[doc = "0xf0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep14_in_buffer_control: EP14_IN_BUFFER_CONTROL,
    #[doc = "0xf4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep14_out_buffer_control: EP14_OUT_BUFFER_CONTROL,
    #[doc = "0xf8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep15_in_buffer_control: EP15_IN_BUFFER_CONTROL,
    #[doc = "0xfc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    pub ep15_out_buffer_control: EP15_OUT_BUFFER_CONTROL,
}
#[doc = "Bytes 0-3 of the SETUP packet from the host.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup_packet_low](setup_packet_low) module"]
pub type SETUP_PACKET_LOW = crate::Reg<u32, _SETUP_PACKET_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_PACKET_LOW;
#[doc = "`read()` method returns [setup_packet_low::R](setup_packet_low::R) reader structure"]
impl crate::Readable for SETUP_PACKET_LOW {}
#[doc = "`write(|w| ..)` method takes [setup_packet_low::W](setup_packet_low::W) writer structure"]
impl crate::Writable for SETUP_PACKET_LOW {}
#[doc = "Bytes 0-3 of the SETUP packet from the host."]
pub mod setup_packet_low;
#[doc = "Bytes 4-7 of the setup packet from the host.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup_packet_high](setup_packet_high) module"]
pub type SETUP_PACKET_HIGH = crate::Reg<u32, _SETUP_PACKET_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_PACKET_HIGH;
#[doc = "`read()` method returns [setup_packet_high::R](setup_packet_high::R) reader structure"]
impl crate::Readable for SETUP_PACKET_HIGH {}
#[doc = "`write(|w| ..)` method takes [setup_packet_high::W](setup_packet_high::W) writer structure"]
impl crate::Writable for SETUP_PACKET_HIGH {}
#[doc = "Bytes 4-7 of the setup packet from the host."]
pub mod setup_packet_high;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1_in_control](ep1_in_control) module"]
pub type EP1_IN_CONTROL = crate::Reg<u32, _EP1_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1_IN_CONTROL;
#[doc = "`read()` method returns [ep1_in_control::R](ep1_in_control::R) reader structure"]
impl crate::Readable for EP1_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep1_in_control::W](ep1_in_control::W) writer structure"]
impl crate::Writable for EP1_IN_CONTROL {}
#[doc = ""]
pub mod ep1_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1_out_control](ep1_out_control) module"]
pub type EP1_OUT_CONTROL = crate::Reg<u32, _EP1_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1_OUT_CONTROL;
#[doc = "`read()` method returns [ep1_out_control::R](ep1_out_control::R) reader structure"]
impl crate::Readable for EP1_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep1_out_control::W](ep1_out_control::W) writer structure"]
impl crate::Writable for EP1_OUT_CONTROL {}
#[doc = ""]
pub mod ep1_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2_in_control](ep2_in_control) module"]
pub type EP2_IN_CONTROL = crate::Reg<u32, _EP2_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2_IN_CONTROL;
#[doc = "`read()` method returns [ep2_in_control::R](ep2_in_control::R) reader structure"]
impl crate::Readable for EP2_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep2_in_control::W](ep2_in_control::W) writer structure"]
impl crate::Writable for EP2_IN_CONTROL {}
#[doc = ""]
pub mod ep2_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2_out_control](ep2_out_control) module"]
pub type EP2_OUT_CONTROL = crate::Reg<u32, _EP2_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2_OUT_CONTROL;
#[doc = "`read()` method returns [ep2_out_control::R](ep2_out_control::R) reader structure"]
impl crate::Readable for EP2_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep2_out_control::W](ep2_out_control::W) writer structure"]
impl crate::Writable for EP2_OUT_CONTROL {}
#[doc = ""]
pub mod ep2_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3_in_control](ep3_in_control) module"]
pub type EP3_IN_CONTROL = crate::Reg<u32, _EP3_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3_IN_CONTROL;
#[doc = "`read()` method returns [ep3_in_control::R](ep3_in_control::R) reader structure"]
impl crate::Readable for EP3_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep3_in_control::W](ep3_in_control::W) writer structure"]
impl crate::Writable for EP3_IN_CONTROL {}
#[doc = ""]
pub mod ep3_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3_out_control](ep3_out_control) module"]
pub type EP3_OUT_CONTROL = crate::Reg<u32, _EP3_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3_OUT_CONTROL;
#[doc = "`read()` method returns [ep3_out_control::R](ep3_out_control::R) reader structure"]
impl crate::Readable for EP3_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep3_out_control::W](ep3_out_control::W) writer structure"]
impl crate::Writable for EP3_OUT_CONTROL {}
#[doc = ""]
pub mod ep3_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4_in_control](ep4_in_control) module"]
pub type EP4_IN_CONTROL = crate::Reg<u32, _EP4_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4_IN_CONTROL;
#[doc = "`read()` method returns [ep4_in_control::R](ep4_in_control::R) reader structure"]
impl crate::Readable for EP4_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep4_in_control::W](ep4_in_control::W) writer structure"]
impl crate::Writable for EP4_IN_CONTROL {}
#[doc = ""]
pub mod ep4_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4_out_control](ep4_out_control) module"]
pub type EP4_OUT_CONTROL = crate::Reg<u32, _EP4_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4_OUT_CONTROL;
#[doc = "`read()` method returns [ep4_out_control::R](ep4_out_control::R) reader structure"]
impl crate::Readable for EP4_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep4_out_control::W](ep4_out_control::W) writer structure"]
impl crate::Writable for EP4_OUT_CONTROL {}
#[doc = ""]
pub mod ep4_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5_in_control](ep5_in_control) module"]
pub type EP5_IN_CONTROL = crate::Reg<u32, _EP5_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5_IN_CONTROL;
#[doc = "`read()` method returns [ep5_in_control::R](ep5_in_control::R) reader structure"]
impl crate::Readable for EP5_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep5_in_control::W](ep5_in_control::W) writer structure"]
impl crate::Writable for EP5_IN_CONTROL {}
#[doc = ""]
pub mod ep5_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5_out_control](ep5_out_control) module"]
pub type EP5_OUT_CONTROL = crate::Reg<u32, _EP5_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5_OUT_CONTROL;
#[doc = "`read()` method returns [ep5_out_control::R](ep5_out_control::R) reader structure"]
impl crate::Readable for EP5_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep5_out_control::W](ep5_out_control::W) writer structure"]
impl crate::Writable for EP5_OUT_CONTROL {}
#[doc = ""]
pub mod ep5_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6_in_control](ep6_in_control) module"]
pub type EP6_IN_CONTROL = crate::Reg<u32, _EP6_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6_IN_CONTROL;
#[doc = "`read()` method returns [ep6_in_control::R](ep6_in_control::R) reader structure"]
impl crate::Readable for EP6_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep6_in_control::W](ep6_in_control::W) writer structure"]
impl crate::Writable for EP6_IN_CONTROL {}
#[doc = ""]
pub mod ep6_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6_out_control](ep6_out_control) module"]
pub type EP6_OUT_CONTROL = crate::Reg<u32, _EP6_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6_OUT_CONTROL;
#[doc = "`read()` method returns [ep6_out_control::R](ep6_out_control::R) reader structure"]
impl crate::Readable for EP6_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep6_out_control::W](ep6_out_control::W) writer structure"]
impl crate::Writable for EP6_OUT_CONTROL {}
#[doc = ""]
pub mod ep6_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7_in_control](ep7_in_control) module"]
pub type EP7_IN_CONTROL = crate::Reg<u32, _EP7_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7_IN_CONTROL;
#[doc = "`read()` method returns [ep7_in_control::R](ep7_in_control::R) reader structure"]
impl crate::Readable for EP7_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep7_in_control::W](ep7_in_control::W) writer structure"]
impl crate::Writable for EP7_IN_CONTROL {}
#[doc = ""]
pub mod ep7_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7_out_control](ep7_out_control) module"]
pub type EP7_OUT_CONTROL = crate::Reg<u32, _EP7_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7_OUT_CONTROL;
#[doc = "`read()` method returns [ep7_out_control::R](ep7_out_control::R) reader structure"]
impl crate::Readable for EP7_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep7_out_control::W](ep7_out_control::W) writer structure"]
impl crate::Writable for EP7_OUT_CONTROL {}
#[doc = ""]
pub mod ep7_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep8_in_control](ep8_in_control) module"]
pub type EP8_IN_CONTROL = crate::Reg<u32, _EP8_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP8_IN_CONTROL;
#[doc = "`read()` method returns [ep8_in_control::R](ep8_in_control::R) reader structure"]
impl crate::Readable for EP8_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep8_in_control::W](ep8_in_control::W) writer structure"]
impl crate::Writable for EP8_IN_CONTROL {}
#[doc = ""]
pub mod ep8_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep8_out_control](ep8_out_control) module"]
pub type EP8_OUT_CONTROL = crate::Reg<u32, _EP8_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP8_OUT_CONTROL;
#[doc = "`read()` method returns [ep8_out_control::R](ep8_out_control::R) reader structure"]
impl crate::Readable for EP8_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep8_out_control::W](ep8_out_control::W) writer structure"]
impl crate::Writable for EP8_OUT_CONTROL {}
#[doc = ""]
pub mod ep8_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep9_in_control](ep9_in_control) module"]
pub type EP9_IN_CONTROL = crate::Reg<u32, _EP9_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP9_IN_CONTROL;
#[doc = "`read()` method returns [ep9_in_control::R](ep9_in_control::R) reader structure"]
impl crate::Readable for EP9_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep9_in_control::W](ep9_in_control::W) writer structure"]
impl crate::Writable for EP9_IN_CONTROL {}
#[doc = ""]
pub mod ep9_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep9_out_control](ep9_out_control) module"]
pub type EP9_OUT_CONTROL = crate::Reg<u32, _EP9_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP9_OUT_CONTROL;
#[doc = "`read()` method returns [ep9_out_control::R](ep9_out_control::R) reader structure"]
impl crate::Readable for EP9_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep9_out_control::W](ep9_out_control::W) writer structure"]
impl crate::Writable for EP9_OUT_CONTROL {}
#[doc = ""]
pub mod ep9_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep10_in_control](ep10_in_control) module"]
pub type EP10_IN_CONTROL = crate::Reg<u32, _EP10_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP10_IN_CONTROL;
#[doc = "`read()` method returns [ep10_in_control::R](ep10_in_control::R) reader structure"]
impl crate::Readable for EP10_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep10_in_control::W](ep10_in_control::W) writer structure"]
impl crate::Writable for EP10_IN_CONTROL {}
#[doc = ""]
pub mod ep10_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep10_out_control](ep10_out_control) module"]
pub type EP10_OUT_CONTROL = crate::Reg<u32, _EP10_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP10_OUT_CONTROL;
#[doc = "`read()` method returns [ep10_out_control::R](ep10_out_control::R) reader structure"]
impl crate::Readable for EP10_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep10_out_control::W](ep10_out_control::W) writer structure"]
impl crate::Writable for EP10_OUT_CONTROL {}
#[doc = ""]
pub mod ep10_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep11_in_control](ep11_in_control) module"]
pub type EP11_IN_CONTROL = crate::Reg<u32, _EP11_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP11_IN_CONTROL;
#[doc = "`read()` method returns [ep11_in_control::R](ep11_in_control::R) reader structure"]
impl crate::Readable for EP11_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep11_in_control::W](ep11_in_control::W) writer structure"]
impl crate::Writable for EP11_IN_CONTROL {}
#[doc = ""]
pub mod ep11_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep11_out_control](ep11_out_control) module"]
pub type EP11_OUT_CONTROL = crate::Reg<u32, _EP11_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP11_OUT_CONTROL;
#[doc = "`read()` method returns [ep11_out_control::R](ep11_out_control::R) reader structure"]
impl crate::Readable for EP11_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep11_out_control::W](ep11_out_control::W) writer structure"]
impl crate::Writable for EP11_OUT_CONTROL {}
#[doc = ""]
pub mod ep11_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep12_in_control](ep12_in_control) module"]
pub type EP12_IN_CONTROL = crate::Reg<u32, _EP12_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP12_IN_CONTROL;
#[doc = "`read()` method returns [ep12_in_control::R](ep12_in_control::R) reader structure"]
impl crate::Readable for EP12_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep12_in_control::W](ep12_in_control::W) writer structure"]
impl crate::Writable for EP12_IN_CONTROL {}
#[doc = ""]
pub mod ep12_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep12_out_control](ep12_out_control) module"]
pub type EP12_OUT_CONTROL = crate::Reg<u32, _EP12_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP12_OUT_CONTROL;
#[doc = "`read()` method returns [ep12_out_control::R](ep12_out_control::R) reader structure"]
impl crate::Readable for EP12_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep12_out_control::W](ep12_out_control::W) writer structure"]
impl crate::Writable for EP12_OUT_CONTROL {}
#[doc = ""]
pub mod ep12_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep13_in_control](ep13_in_control) module"]
pub type EP13_IN_CONTROL = crate::Reg<u32, _EP13_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP13_IN_CONTROL;
#[doc = "`read()` method returns [ep13_in_control::R](ep13_in_control::R) reader structure"]
impl crate::Readable for EP13_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep13_in_control::W](ep13_in_control::W) writer structure"]
impl crate::Writable for EP13_IN_CONTROL {}
#[doc = ""]
pub mod ep13_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep13_out_control](ep13_out_control) module"]
pub type EP13_OUT_CONTROL = crate::Reg<u32, _EP13_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP13_OUT_CONTROL;
#[doc = "`read()` method returns [ep13_out_control::R](ep13_out_control::R) reader structure"]
impl crate::Readable for EP13_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep13_out_control::W](ep13_out_control::W) writer structure"]
impl crate::Writable for EP13_OUT_CONTROL {}
#[doc = ""]
pub mod ep13_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep14_in_control](ep14_in_control) module"]
pub type EP14_IN_CONTROL = crate::Reg<u32, _EP14_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP14_IN_CONTROL;
#[doc = "`read()` method returns [ep14_in_control::R](ep14_in_control::R) reader structure"]
impl crate::Readable for EP14_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep14_in_control::W](ep14_in_control::W) writer structure"]
impl crate::Writable for EP14_IN_CONTROL {}
#[doc = ""]
pub mod ep14_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep14_out_control](ep14_out_control) module"]
pub type EP14_OUT_CONTROL = crate::Reg<u32, _EP14_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP14_OUT_CONTROL;
#[doc = "`read()` method returns [ep14_out_control::R](ep14_out_control::R) reader structure"]
impl crate::Readable for EP14_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep14_out_control::W](ep14_out_control::W) writer structure"]
impl crate::Writable for EP14_OUT_CONTROL {}
#[doc = ""]
pub mod ep14_out_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep15_in_control](ep15_in_control) module"]
pub type EP15_IN_CONTROL = crate::Reg<u32, _EP15_IN_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP15_IN_CONTROL;
#[doc = "`read()` method returns [ep15_in_control::R](ep15_in_control::R) reader structure"]
impl crate::Readable for EP15_IN_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep15_in_control::W](ep15_in_control::W) writer structure"]
impl crate::Writable for EP15_IN_CONTROL {}
#[doc = ""]
pub mod ep15_in_control;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep15_out_control](ep15_out_control) module"]
pub type EP15_OUT_CONTROL = crate::Reg<u32, _EP15_OUT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP15_OUT_CONTROL;
#[doc = "`read()` method returns [ep15_out_control::R](ep15_out_control::R) reader structure"]
impl crate::Readable for EP15_OUT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep15_out_control::W](ep15_out_control::W) writer structure"]
impl crate::Writable for EP15_OUT_CONTROL {}
#[doc = ""]
pub mod ep15_out_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_in_buffer_control](ep0_in_buffer_control) module"]
pub type EP0_IN_BUFFER_CONTROL = crate::Reg<u32, _EP0_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep0_in_buffer_control::R](ep0_in_buffer_control::R) reader structure"]
impl crate::Readable for EP0_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep0_in_buffer_control::W](ep0_in_buffer_control::W) writer structure"]
impl crate::Writable for EP0_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep0_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_out_buffer_control](ep0_out_buffer_control) module"]
pub type EP0_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP0_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep0_out_buffer_control::R](ep0_out_buffer_control::R) reader structure"]
impl crate::Readable for EP0_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep0_out_buffer_control::W](ep0_out_buffer_control::W) writer structure"]
impl crate::Writable for EP0_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep0_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1_in_buffer_control](ep1_in_buffer_control) module"]
pub type EP1_IN_BUFFER_CONTROL = crate::Reg<u32, _EP1_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep1_in_buffer_control::R](ep1_in_buffer_control::R) reader structure"]
impl crate::Readable for EP1_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep1_in_buffer_control::W](ep1_in_buffer_control::W) writer structure"]
impl crate::Writable for EP1_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep1_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1_out_buffer_control](ep1_out_buffer_control) module"]
pub type EP1_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP1_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep1_out_buffer_control::R](ep1_out_buffer_control::R) reader structure"]
impl crate::Readable for EP1_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep1_out_buffer_control::W](ep1_out_buffer_control::W) writer structure"]
impl crate::Writable for EP1_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep1_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2_in_buffer_control](ep2_in_buffer_control) module"]
pub type EP2_IN_BUFFER_CONTROL = crate::Reg<u32, _EP2_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep2_in_buffer_control::R](ep2_in_buffer_control::R) reader structure"]
impl crate::Readable for EP2_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep2_in_buffer_control::W](ep2_in_buffer_control::W) writer structure"]
impl crate::Writable for EP2_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep2_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2_out_buffer_control](ep2_out_buffer_control) module"]
pub type EP2_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP2_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep2_out_buffer_control::R](ep2_out_buffer_control::R) reader structure"]
impl crate::Readable for EP2_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep2_out_buffer_control::W](ep2_out_buffer_control::W) writer structure"]
impl crate::Writable for EP2_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep2_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3_in_buffer_control](ep3_in_buffer_control) module"]
pub type EP3_IN_BUFFER_CONTROL = crate::Reg<u32, _EP3_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep3_in_buffer_control::R](ep3_in_buffer_control::R) reader structure"]
impl crate::Readable for EP3_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep3_in_buffer_control::W](ep3_in_buffer_control::W) writer structure"]
impl crate::Writable for EP3_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep3_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3_out_buffer_control](ep3_out_buffer_control) module"]
pub type EP3_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP3_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep3_out_buffer_control::R](ep3_out_buffer_control::R) reader structure"]
impl crate::Readable for EP3_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep3_out_buffer_control::W](ep3_out_buffer_control::W) writer structure"]
impl crate::Writable for EP3_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep3_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4_in_buffer_control](ep4_in_buffer_control) module"]
pub type EP4_IN_BUFFER_CONTROL = crate::Reg<u32, _EP4_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep4_in_buffer_control::R](ep4_in_buffer_control::R) reader structure"]
impl crate::Readable for EP4_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep4_in_buffer_control::W](ep4_in_buffer_control::W) writer structure"]
impl crate::Writable for EP4_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep4_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4_out_buffer_control](ep4_out_buffer_control) module"]
pub type EP4_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP4_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep4_out_buffer_control::R](ep4_out_buffer_control::R) reader structure"]
impl crate::Readable for EP4_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep4_out_buffer_control::W](ep4_out_buffer_control::W) writer structure"]
impl crate::Writable for EP4_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep4_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5_in_buffer_control](ep5_in_buffer_control) module"]
pub type EP5_IN_BUFFER_CONTROL = crate::Reg<u32, _EP5_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep5_in_buffer_control::R](ep5_in_buffer_control::R) reader structure"]
impl crate::Readable for EP5_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep5_in_buffer_control::W](ep5_in_buffer_control::W) writer structure"]
impl crate::Writable for EP5_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep5_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5_out_buffer_control](ep5_out_buffer_control) module"]
pub type EP5_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP5_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep5_out_buffer_control::R](ep5_out_buffer_control::R) reader structure"]
impl crate::Readable for EP5_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep5_out_buffer_control::W](ep5_out_buffer_control::W) writer structure"]
impl crate::Writable for EP5_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep5_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6_in_buffer_control](ep6_in_buffer_control) module"]
pub type EP6_IN_BUFFER_CONTROL = crate::Reg<u32, _EP6_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep6_in_buffer_control::R](ep6_in_buffer_control::R) reader structure"]
impl crate::Readable for EP6_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep6_in_buffer_control::W](ep6_in_buffer_control::W) writer structure"]
impl crate::Writable for EP6_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep6_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6_out_buffer_control](ep6_out_buffer_control) module"]
pub type EP6_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP6_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep6_out_buffer_control::R](ep6_out_buffer_control::R) reader structure"]
impl crate::Readable for EP6_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep6_out_buffer_control::W](ep6_out_buffer_control::W) writer structure"]
impl crate::Writable for EP6_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep6_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7_in_buffer_control](ep7_in_buffer_control) module"]
pub type EP7_IN_BUFFER_CONTROL = crate::Reg<u32, _EP7_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep7_in_buffer_control::R](ep7_in_buffer_control::R) reader structure"]
impl crate::Readable for EP7_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep7_in_buffer_control::W](ep7_in_buffer_control::W) writer structure"]
impl crate::Writable for EP7_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep7_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7_out_buffer_control](ep7_out_buffer_control) module"]
pub type EP7_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP7_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep7_out_buffer_control::R](ep7_out_buffer_control::R) reader structure"]
impl crate::Readable for EP7_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep7_out_buffer_control::W](ep7_out_buffer_control::W) writer structure"]
impl crate::Writable for EP7_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep7_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep8_in_buffer_control](ep8_in_buffer_control) module"]
pub type EP8_IN_BUFFER_CONTROL = crate::Reg<u32, _EP8_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP8_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep8_in_buffer_control::R](ep8_in_buffer_control::R) reader structure"]
impl crate::Readable for EP8_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep8_in_buffer_control::W](ep8_in_buffer_control::W) writer structure"]
impl crate::Writable for EP8_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep8_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep8_out_buffer_control](ep8_out_buffer_control) module"]
pub type EP8_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP8_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP8_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep8_out_buffer_control::R](ep8_out_buffer_control::R) reader structure"]
impl crate::Readable for EP8_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep8_out_buffer_control::W](ep8_out_buffer_control::W) writer structure"]
impl crate::Writable for EP8_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep8_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep9_in_buffer_control](ep9_in_buffer_control) module"]
pub type EP9_IN_BUFFER_CONTROL = crate::Reg<u32, _EP9_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP9_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep9_in_buffer_control::R](ep9_in_buffer_control::R) reader structure"]
impl crate::Readable for EP9_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep9_in_buffer_control::W](ep9_in_buffer_control::W) writer structure"]
impl crate::Writable for EP9_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep9_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep9_out_buffer_control](ep9_out_buffer_control) module"]
pub type EP9_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP9_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP9_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep9_out_buffer_control::R](ep9_out_buffer_control::R) reader structure"]
impl crate::Readable for EP9_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep9_out_buffer_control::W](ep9_out_buffer_control::W) writer structure"]
impl crate::Writable for EP9_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep9_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep10_in_buffer_control](ep10_in_buffer_control) module"]
pub type EP10_IN_BUFFER_CONTROL = crate::Reg<u32, _EP10_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP10_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep10_in_buffer_control::R](ep10_in_buffer_control::R) reader structure"]
impl crate::Readable for EP10_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep10_in_buffer_control::W](ep10_in_buffer_control::W) writer structure"]
impl crate::Writable for EP10_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep10_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep10_out_buffer_control](ep10_out_buffer_control) module"]
pub type EP10_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP10_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP10_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep10_out_buffer_control::R](ep10_out_buffer_control::R) reader structure"]
impl crate::Readable for EP10_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep10_out_buffer_control::W](ep10_out_buffer_control::W) writer structure"]
impl crate::Writable for EP10_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep10_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep11_in_buffer_control](ep11_in_buffer_control) module"]
pub type EP11_IN_BUFFER_CONTROL = crate::Reg<u32, _EP11_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP11_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep11_in_buffer_control::R](ep11_in_buffer_control::R) reader structure"]
impl crate::Readable for EP11_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep11_in_buffer_control::W](ep11_in_buffer_control::W) writer structure"]
impl crate::Writable for EP11_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep11_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep11_out_buffer_control](ep11_out_buffer_control) module"]
pub type EP11_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP11_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP11_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep11_out_buffer_control::R](ep11_out_buffer_control::R) reader structure"]
impl crate::Readable for EP11_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep11_out_buffer_control::W](ep11_out_buffer_control::W) writer structure"]
impl crate::Writable for EP11_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep11_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep12_in_buffer_control](ep12_in_buffer_control) module"]
pub type EP12_IN_BUFFER_CONTROL = crate::Reg<u32, _EP12_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP12_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep12_in_buffer_control::R](ep12_in_buffer_control::R) reader structure"]
impl crate::Readable for EP12_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep12_in_buffer_control::W](ep12_in_buffer_control::W) writer structure"]
impl crate::Writable for EP12_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep12_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep12_out_buffer_control](ep12_out_buffer_control) module"]
pub type EP12_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP12_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP12_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep12_out_buffer_control::R](ep12_out_buffer_control::R) reader structure"]
impl crate::Readable for EP12_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep12_out_buffer_control::W](ep12_out_buffer_control::W) writer structure"]
impl crate::Writable for EP12_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep12_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep13_in_buffer_control](ep13_in_buffer_control) module"]
pub type EP13_IN_BUFFER_CONTROL = crate::Reg<u32, _EP13_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP13_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep13_in_buffer_control::R](ep13_in_buffer_control::R) reader structure"]
impl crate::Readable for EP13_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep13_in_buffer_control::W](ep13_in_buffer_control::W) writer structure"]
impl crate::Writable for EP13_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep13_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep13_out_buffer_control](ep13_out_buffer_control) module"]
pub type EP13_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP13_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP13_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep13_out_buffer_control::R](ep13_out_buffer_control::R) reader structure"]
impl crate::Readable for EP13_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep13_out_buffer_control::W](ep13_out_buffer_control::W) writer structure"]
impl crate::Writable for EP13_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep13_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep14_in_buffer_control](ep14_in_buffer_control) module"]
pub type EP14_IN_BUFFER_CONTROL = crate::Reg<u32, _EP14_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP14_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep14_in_buffer_control::R](ep14_in_buffer_control::R) reader structure"]
impl crate::Readable for EP14_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep14_in_buffer_control::W](ep14_in_buffer_control::W) writer structure"]
impl crate::Writable for EP14_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep14_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep14_out_buffer_control](ep14_out_buffer_control) module"]
pub type EP14_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP14_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP14_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep14_out_buffer_control::R](ep14_out_buffer_control::R) reader structure"]
impl crate::Readable for EP14_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep14_out_buffer_control::W](ep14_out_buffer_control::W) writer structure"]
impl crate::Writable for EP14_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep14_out_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep15_in_buffer_control](ep15_in_buffer_control) module"]
pub type EP15_IN_BUFFER_CONTROL = crate::Reg<u32, _EP15_IN_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP15_IN_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep15_in_buffer_control::R](ep15_in_buffer_control::R) reader structure"]
impl crate::Readable for EP15_IN_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep15_in_buffer_control::W](ep15_in_buffer_control::W) writer structure"]
impl crate::Writable for EP15_IN_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep15_in_buffer_control;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep15_out_buffer_control](ep15_out_buffer_control) module"]
pub type EP15_OUT_BUFFER_CONTROL = crate::Reg<u32, _EP15_OUT_BUFFER_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP15_OUT_BUFFER_CONTROL;
#[doc = "`read()` method returns [ep15_out_buffer_control::R](ep15_out_buffer_control::R) reader structure"]
impl crate::Readable for EP15_OUT_BUFFER_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ep15_out_buffer_control::W](ep15_out_buffer_control::W) writer structure"]
impl crate::Writable for EP15_OUT_BUFFER_CONTROL {}
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1.\\n Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep15_out_buffer_control;
