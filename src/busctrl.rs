#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Set the priority of each master for bus arbitration."]
    pub bus_priority: BUS_PRIORITY,
    #[doc = "0x04 - Bus priority acknowledge"]
    pub bus_priority_ack: BUS_PRIORITY_ACK,
    #[doc = "0x08 - Bus fabric performance counter 0"]
    pub perfctr0: PERFCTR0,
    #[doc = "0x0c - Bus fabric performance event select for PERFCTR0"]
    pub perfsel0: PERFSEL0,
    #[doc = "0x10 - Bus fabric performance counter 1"]
    pub perfctr1: PERFCTR1,
    #[doc = "0x14 - Bus fabric performance event select for PERFCTR1"]
    pub perfsel1: PERFSEL1,
    #[doc = "0x18 - Bus fabric performance counter 2"]
    pub perfctr2: PERFCTR2,
    #[doc = "0x1c - Bus fabric performance event select for PERFCTR2"]
    pub perfsel2: PERFSEL2,
    #[doc = "0x20 - Bus fabric performance counter 3"]
    pub perfctr3: PERFCTR3,
    #[doc = "0x24 - Bus fabric performance event select for PERFCTR3"]
    pub perfsel3: PERFSEL3,
}
#[doc = "Set the priority of each master for bus arbitration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_priority](bus_priority) module"]
pub type BUS_PRIORITY = crate::Reg<u32, _BUS_PRIORITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_PRIORITY;
#[doc = "`read()` method returns [bus_priority::R](bus_priority::R) reader structure"]
impl crate::Readable for BUS_PRIORITY {}
#[doc = "`write(|w| ..)` method takes [bus_priority::W](bus_priority::W) writer structure"]
impl crate::Writable for BUS_PRIORITY {}
#[doc = "Set the priority of each master for bus arbitration."]
pub mod bus_priority;
#[doc = "Bus priority acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_priority_ack](bus_priority_ack) module"]
pub type BUS_PRIORITY_ACK = crate::Reg<u32, _BUS_PRIORITY_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_PRIORITY_ACK;
#[doc = "`read()` method returns [bus_priority_ack::R](bus_priority_ack::R) reader structure"]
impl crate::Readable for BUS_PRIORITY_ACK {}
#[doc = "Bus priority acknowledge"]
pub mod bus_priority_ack;
#[doc = "Bus fabric performance counter 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfctr0](perfctr0) module"]
pub type PERFCTR0 = crate::Reg<u32, _PERFCTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFCTR0;
#[doc = "`read()` method returns [perfctr0::R](perfctr0::R) reader structure"]
impl crate::Readable for PERFCTR0 {}
#[doc = "`write(|w| ..)` method takes [perfctr0::W](perfctr0::W) writer structure"]
impl crate::Writable for PERFCTR0 {}
#[doc = "Bus fabric performance counter 0"]
pub mod perfctr0;
#[doc = "Bus fabric performance event select for PERFCTR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfsel0](perfsel0) module"]
pub type PERFSEL0 = crate::Reg<u32, _PERFSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFSEL0;
#[doc = "`read()` method returns [perfsel0::R](perfsel0::R) reader structure"]
impl crate::Readable for PERFSEL0 {}
#[doc = "`write(|w| ..)` method takes [perfsel0::W](perfsel0::W) writer structure"]
impl crate::Writable for PERFSEL0 {}
#[doc = "Bus fabric performance event select for PERFCTR0"]
pub mod perfsel0;
#[doc = "Bus fabric performance counter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfctr1](perfctr1) module"]
pub type PERFCTR1 = crate::Reg<u32, _PERFCTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFCTR1;
#[doc = "`read()` method returns [perfctr1::R](perfctr1::R) reader structure"]
impl crate::Readable for PERFCTR1 {}
#[doc = "`write(|w| ..)` method takes [perfctr1::W](perfctr1::W) writer structure"]
impl crate::Writable for PERFCTR1 {}
#[doc = "Bus fabric performance counter 1"]
pub mod perfctr1;
#[doc = "Bus fabric performance event select for PERFCTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfsel1](perfsel1) module"]
pub type PERFSEL1 = crate::Reg<u32, _PERFSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFSEL1;
#[doc = "`read()` method returns [perfsel1::R](perfsel1::R) reader structure"]
impl crate::Readable for PERFSEL1 {}
#[doc = "`write(|w| ..)` method takes [perfsel1::W](perfsel1::W) writer structure"]
impl crate::Writable for PERFSEL1 {}
#[doc = "Bus fabric performance event select for PERFCTR1"]
pub mod perfsel1;
#[doc = "Bus fabric performance counter 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfctr2](perfctr2) module"]
pub type PERFCTR2 = crate::Reg<u32, _PERFCTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFCTR2;
#[doc = "`read()` method returns [perfctr2::R](perfctr2::R) reader structure"]
impl crate::Readable for PERFCTR2 {}
#[doc = "`write(|w| ..)` method takes [perfctr2::W](perfctr2::W) writer structure"]
impl crate::Writable for PERFCTR2 {}
#[doc = "Bus fabric performance counter 2"]
pub mod perfctr2;
#[doc = "Bus fabric performance event select for PERFCTR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfsel2](perfsel2) module"]
pub type PERFSEL2 = crate::Reg<u32, _PERFSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFSEL2;
#[doc = "`read()` method returns [perfsel2::R](perfsel2::R) reader structure"]
impl crate::Readable for PERFSEL2 {}
#[doc = "`write(|w| ..)` method takes [perfsel2::W](perfsel2::W) writer structure"]
impl crate::Writable for PERFSEL2 {}
#[doc = "Bus fabric performance event select for PERFCTR2"]
pub mod perfsel2;
#[doc = "Bus fabric performance counter 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfctr3](perfctr3) module"]
pub type PERFCTR3 = crate::Reg<u32, _PERFCTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFCTR3;
#[doc = "`read()` method returns [perfctr3::R](perfctr3::R) reader structure"]
impl crate::Readable for PERFCTR3 {}
#[doc = "`write(|w| ..)` method takes [perfctr3::W](perfctr3::W) writer structure"]
impl crate::Writable for PERFCTR3 {}
#[doc = "Bus fabric performance counter 3"]
pub mod perfctr3;
#[doc = "Bus fabric performance event select for PERFCTR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfsel3](perfsel3) module"]
pub type PERFSEL3 = crate::Reg<u32, _PERFSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFSEL3;
#[doc = "`read()` method returns [perfsel3::R](perfsel3::R) reader structure"]
impl crate::Readable for PERFSEL3 {}
#[doc = "`write(|w| ..)` method takes [perfsel3::W](perfsel3::W) writer structure"]
impl crate::Writable for PERFSEL3 {}
#[doc = "Bus fabric performance event select for PERFCTR3"]
pub mod perfsel3;
