#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    platform: PLATFORM,
}
impl RegisterBlock {
    #[doc = "0x00 - Indicates the type of platform in use"]
    #[inline(always)]
    pub const fn platform(&self) -> &PLATFORM {
        &self.platform
    }
}
#[doc = "PLATFORM (r) register accessor: Indicates the type of platform in use  

You can [`read`](crate::Reg::read) this register and get [`platform::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@platform`]
module"]
pub type PLATFORM = crate::Reg<platform::PLATFORM_SPEC>;
#[doc = "Indicates the type of platform in use"]
pub mod platform;
