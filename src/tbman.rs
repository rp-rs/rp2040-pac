#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Indicates the type of platform in use"]
    pub platform: PLATFORM,
}
#[doc = "Indicates the type of platform in use\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [platform](platform) module"]
pub type PLATFORM = crate::Reg<u32, _PLATFORM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLATFORM;
#[doc = "`read()` method returns [platform::R](platform::R) reader structure"]
impl crate::Readable for PLATFORM {}
#[doc = "Indicates the type of platform in use"]
pub mod platform;
