#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cs: CS,
    result: RESULT,
    fcs: FCS,
    fifo: FIFO,
    div: DIV,
    intr: INTR,
    inte: INTE,
    intf: INTF,
    ints: INTS,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Control and Status"]
    #[inline(always)]
    pub const fn cs(&self) -> &CS {
        &self.cs
    }
    #[doc = "0x04 - Result of most recent ADC conversion"]
    #[inline(always)]
    pub const fn result(&self) -> &RESULT {
        &self.result
    }
    #[doc = "0x08 - FIFO control and status"]
    #[inline(always)]
    pub const fn fcs(&self) -> &FCS {
        &self.fcs
    }
    #[doc = "0x0c - Conversion result FIFO"]
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
    #[doc = "0x10 - Clock divider. If non-zero, CS_START_MANY will start conversions  
 at regular intervals rather than back-to-back.  
 The divider is reset when either of these fields are written.  
 Total period is 1 + INT + FRAC / 256"]
    #[inline(always)]
    pub const fn div(&self) -> &DIV {
        &self.div
    }
    #[doc = "0x14 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x18 - Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0x1c - Interrupt Force"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0x20 - Interrupt status after masking &amp; forcing"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
}
#[doc = "CS (rw) register accessor: ADC Control and Status  

You can [`read`](crate::Reg::read) this register and get [`cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cs`]
module"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "ADC Control and Status"]
pub mod cs;
#[doc = "RESULT (r) register accessor: Result of most recent ADC conversion  

You can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@result`]
module"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result of most recent ADC conversion"]
pub mod result;
#[doc = "FCS (rw) register accessor: FIFO control and status  

You can [`read`](crate::Reg::read) this register and get [`fcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fcs`]
module"]
pub type FCS = crate::Reg<fcs::FCS_SPEC>;
#[doc = "FIFO control and status"]
pub mod fcs;
#[doc = "FIFO (r) register accessor: Conversion result FIFO  

You can [`read`](crate::Reg::read) this register and get [`fifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Conversion result FIFO"]
pub mod fifo;
#[doc = "DIV (rw) register accessor: Clock divider. If non-zero, CS_START_MANY will start conversions  
 at regular intervals rather than back-to-back.  
 The divider is reset when either of these fields are written.  
 Total period is 1 + INT + FRAC / 256  

You can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions  
 at regular intervals rather than back-to-back.  
 The divider is reset when either of these fields are written.  
 Total period is 1 + INT + FRAC / 256"]
pub mod div;
#[doc = "INTR (r) register accessor: Raw Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE (rw) register accessor: Interrupt Enable  

You can [`read`](crate::Reg::read) this register and get [`inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte`]
module"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF (rw) register accessor: Interrupt Force  

You can [`read`](crate::Reg::read) this register and get [`intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS (r) register accessor: Interrupt status after masking &amp; forcing  

You can [`read`](crate::Reg::read) this register and get [`ints::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints`]
module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing"]
pub mod ints;
