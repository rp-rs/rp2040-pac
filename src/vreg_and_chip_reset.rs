#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vreg: VREG,
    bod: BOD,
    chip_reset: CHIP_RESET,
}
impl RegisterBlock {
    #[doc = "0x00 - Voltage regulator control and status"]
    #[inline(always)]
    pub const fn vreg(&self) -> &VREG {
        &self.vreg
    }
    #[doc = "0x04 - brown-out detection control"]
    #[inline(always)]
    pub const fn bod(&self) -> &BOD {
        &self.bod
    }
    #[doc = "0x08 - Chip reset control and status"]
    #[inline(always)]
    pub const fn chip_reset(&self) -> &CHIP_RESET {
        &self.chip_reset
    }
}
#[doc = "VREG (rw) register accessor: Voltage regulator control and status  

You can [`read`](crate::Reg::read) this register and get [`vreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vreg`]
module"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "Voltage regulator control and status"]
pub mod vreg;
#[doc = "BOD (rw) register accessor: brown-out detection control  

You can [`read`](crate::Reg::read) this register and get [`bod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bod`]
module"]
pub type BOD = crate::Reg<bod::BOD_SPEC>;
#[doc = "brown-out detection control"]
pub mod bod;
#[doc = "CHIP_RESET (rw) register accessor: Chip reset control and status  

You can [`read`](crate::Reg::read) this register and get [`chip_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chip_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chip_reset`]
module"]
pub type CHIP_RESET = crate::Reg<chip_reset::CHIP_RESET_SPEC>;
#[doc = "Chip reset control and status"]
pub mod chip_reset;
