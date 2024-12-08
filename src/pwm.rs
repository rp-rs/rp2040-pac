#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [CH; 8],
    en: EN,
    intr: INTR,
    inte: INTE,
    intf: INTF,
    ints: INTS,
}
impl RegisterBlock {
    #[doc = "0x00..0xa0 - Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xa0 - Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0xa0 - This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
    #[inline(always)]
    pub const fn en(&self) -> &EN {
        &self.en
    }
    #[doc = "0xa4 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0xa8 - Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0xac - Interrupt Force"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0xb0 - Interrupt status after masking &amp; forcing"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
}
#[doc = "Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
pub mod ch;
#[doc = "EN (rw) register accessor: This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR.  

You can [`read`](crate::generic::Reg::read) this register and get [`en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@en`]
module"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
pub mod en;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE (rw) register accessor: Interrupt Enable  

You can [`read`](crate::generic::Reg::read) this register and get [`inte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte`]
module"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF (rw) register accessor: Interrupt Force  

You can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS (rw) register accessor: Interrupt status after masking &amp; forcing  

You can [`read`](crate::generic::Reg::read) this register and get [`ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints`]
module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing"]
pub mod ints;
