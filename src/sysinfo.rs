#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chip_id: CHIP_ID,
    platform: PLATFORM,
    _reserved2: [u8; 0x08],
    gitref_rp2040: GITREF_RP2040,
}
impl RegisterBlock {
    #[doc = "0x00 - JEDEC JEP-106 compliant chip identifier."]
    #[inline(always)]
    pub const fn chip_id(&self) -> &CHIP_ID {
        &self.chip_id
    }
    #[doc = "0x04 - Platform register. Allows software to know what environment it is running in."]
    #[inline(always)]
    pub const fn platform(&self) -> &PLATFORM {
        &self.platform
    }
    #[doc = "0x10 - Git hash of the chip source. Used to identify chip version."]
    #[inline(always)]
    pub const fn gitref_rp2040(&self) -> &GITREF_RP2040 {
        &self.gitref_rp2040
    }
}
#[doc = "CHIP_ID (rw) register accessor: JEDEC JEP-106 compliant chip identifier.  

You can [`read`](crate::generic::Reg::read) this register and get [`chip_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chip_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chip_id`]
module"]
pub type CHIP_ID = crate::Reg<chip_id::CHIP_ID_SPEC>;
#[doc = "JEDEC JEP-106 compliant chip identifier."]
pub mod chip_id;
#[doc = "PLATFORM (rw) register accessor: Platform register. Allows software to know what environment it is running in.  

You can [`read`](crate::generic::Reg::read) this register and get [`platform::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`platform::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@platform`]
module"]
pub type PLATFORM = crate::Reg<platform::PLATFORM_SPEC>;
#[doc = "Platform register. Allows software to know what environment it is running in."]
pub mod platform;
#[doc = "GITREF_RP2040 (rw) register accessor: Git hash of the chip source. Used to identify chip version.  

You can [`read`](crate::generic::Reg::read) this register and get [`gitref_rp2040::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gitref_rp2040::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gitref_rp2040`]
module"]
pub type GITREF_RP2040 = crate::Reg<gitref_rp2040::GITREF_RP2040_SPEC>;
#[doc = "Git hash of the chip source. Used to identify chip version."]
pub mod gitref_rp2040;
