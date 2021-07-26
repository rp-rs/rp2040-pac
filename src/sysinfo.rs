#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JEDEC JEP-106 compliant chip identifier."]
    pub chip_id: crate::Reg<chip_id::CHIP_ID_SPEC>,
    #[doc = "0x04 - Platform register. Allows software to know what environment it is running in."]
    pub platform: crate::Reg<platform::PLATFORM_SPEC>,
    _reserved2: [u8; 0x38],
    #[doc = "0x40 - Git hash of the chip source. Used to identify chip version."]
    pub gitref_rp2040: crate::Reg<gitref_rp2040::GITREF_RP2040_SPEC>,
}
#[doc = "CHIP_ID register accessor: an alias for `Reg<CHIP_ID_SPEC>`"]
pub type CHIP_ID = crate::Reg<chip_id::CHIP_ID_SPEC>;
#[doc = "JEDEC JEP-106 compliant chip identifier."]
pub mod chip_id;
#[doc = "PLATFORM register accessor: an alias for `Reg<PLATFORM_SPEC>`"]
pub type PLATFORM = crate::Reg<platform::PLATFORM_SPEC>;
#[doc = "Platform register. Allows software to know what environment it is running in."]
pub mod platform;
#[doc = "GITREF_RP2040 register accessor: an alias for `Reg<GITREF_RP2040_SPEC>`"]
pub type GITREF_RP2040 = crate::Reg<gitref_rp2040::GITREF_RP2040_SPEC>;
#[doc = "Git hash of the chip source. Used to identify chip version."]
pub mod gitref_rp2040;
