#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage regulator control and status"]
    pub vreg: VREG,
    #[doc = "0x04 - brown-out detection control"]
    pub bod: BOD,
    #[doc = "0x08 - Chip reset control and status"]
    pub chip_reset: CHIP_RESET,
}
#[doc = "VREG (rw) register accessor: Voltage regulator control and status  

You can [`read`](crate::generic::Reg::read) this register and get [`vreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vreg`]
module"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "Voltage regulator control and status"]
pub mod vreg;
#[doc = "BOD (rw) register accessor: brown-out detection control  

You can [`read`](crate::generic::Reg::read) this register and get [`bod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bod`]
module"]
pub type BOD = crate::Reg<bod::BOD_SPEC>;
#[doc = "brown-out detection control"]
pub mod bod;
#[doc = "CHIP_RESET (rw) register accessor: Chip reset control and status  

You can [`read`](crate::generic::Reg::read) this register and get [`chip_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chip_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chip_reset`]
module"]
pub type CHIP_RESET = crate::Reg<chip_reset::CHIP_RESET_SPEC>;
#[doc = "Chip reset control and status"]
pub mod chip_reset;
