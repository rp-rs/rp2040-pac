#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Indicates the type of platform in use"]
    pub platform: PLATFORM,
}
#[doc = "PLATFORM (r) register accessor: Indicates the type of platform in use  

You can [`read`](crate::generic::Reg::read) this register and get [`platform::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@platform`]
module"]
pub type PLATFORM = crate::Reg<platform::PLATFORM_SPEC>;
#[doc = "Indicates the type of platform in use"]
pub mod platform;
