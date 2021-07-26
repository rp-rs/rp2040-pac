#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Indicates the type of platform in use"]
    pub platform: crate::Reg<platform::PLATFORM_SPEC>,
}
#[doc = "PLATFORM register accessor: an alias for `Reg<PLATFORM_SPEC>`"]
pub type PLATFORM = crate::Reg<platform::PLATFORM_SPEC>;
#[doc = "Indicates the type of platform in use"]
pub mod platform;
