#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cs: CS,
    pwr: PWR,
    fbdiv_int: FBDIV_INT,
    prim: PRIM,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=750MHz, max=1600MHz"]
    #[inline(always)]
    pub const fn cs(&self) -> &CS {
        &self.cs
    }
    #[doc = "0x04 - Controls the PLL power modes."]
    #[inline(always)]
    pub const fn pwr(&self) -> &PWR {
        &self.pwr
    }
    #[doc = "0x08 - Feedback divisor (note: this PLL does not support fractional division)"]
    #[inline(always)]
    pub const fn fbdiv_int(&self) -> &FBDIV_INT {
        &self.fbdiv_int
    }
    #[doc = "0x0c - Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2"]
    #[inline(always)]
    pub const fn prim(&self) -> &PRIM {
        &self.prim
    }
}
#[doc = "CS (rw) register accessor: Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=750MHz, max=1600MHz  

You can [`read`](crate::generic::Reg::read) this register and get [`cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cs`]
module"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=750MHz, max=1600MHz"]
pub mod cs;
#[doc = "PWR (rw) register accessor: Controls the PLL power modes.  

You can [`read`](crate::generic::Reg::read) this register and get [`pwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pwr`]
module"]
pub type PWR = crate::Reg<pwr::PWR_SPEC>;
#[doc = "Controls the PLL power modes."]
pub mod pwr;
#[doc = "FBDIV_INT (rw) register accessor: Feedback divisor (note: this PLL does not support fractional division)  

You can [`read`](crate::generic::Reg::read) this register and get [`fbdiv_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbdiv_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fbdiv_int`]
module"]
pub type FBDIV_INT = crate::Reg<fbdiv_int::FBDIV_INT_SPEC>;
#[doc = "Feedback divisor (note: this PLL does not support fractional division)"]
pub mod fbdiv_int;
#[doc = "PRIM (rw) register accessor: Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2  

You can [`read`](crate::generic::Reg::read) this register and get [`prim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@prim`]
module"]
pub type PRIM = crate::Reg<prim::PRIM_SPEC>;
#[doc = "Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2"]
pub mod prim;
