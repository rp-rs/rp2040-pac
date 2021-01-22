#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JEDEC JEP-106 compliant chip identifier."]
    pub chip_id: CHIP_ID,
    #[doc = "0x04 - Platform register. Allows software to know what environment it is running in."]
    pub platform: PLATFORM,
    _reserved2: [u8; 56usize],
    #[doc = "0x40 - Git hash of the chip source. Used to identify chip version."]
    pub gitref_rp2040: GITREF_RP2040,
}
#[doc = "JEDEC JEP-106 compliant chip identifier.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_id](chip_id) module"]
pub type CHIP_ID = crate::Reg<u32, _CHIP_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIP_ID;
#[doc = "`read()` method returns [chip_id::R](chip_id::R) reader structure"]
impl crate::Readable for CHIP_ID {}
#[doc = "JEDEC JEP-106 compliant chip identifier."]
pub mod chip_id;
#[doc = "Platform register. Allows software to know what environment it is running in.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [platform](platform) module"]
pub type PLATFORM = crate::Reg<u32, _PLATFORM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLATFORM;
#[doc = "`read()` method returns [platform::R](platform::R) reader structure"]
impl crate::Readable for PLATFORM {}
#[doc = "Platform register. Allows software to know what environment it is running in."]
pub mod platform;
#[doc = "Git hash of the chip source. Used to identify chip version.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gitref_rp2040](gitref_rp2040) module"]
pub type GITREF_RP2040 = crate::Reg<u32, _GITREF_RP2040>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREF_RP2040;
#[doc = "`read()` method returns [gitref_rp2040::R](gitref_rp2040::R) reader structure"]
impl crate::Readable for GITREF_RP2040 {}
#[doc = "Git hash of the chip source. Used to identify chip version."]
pub mod gitref_rp2040;
