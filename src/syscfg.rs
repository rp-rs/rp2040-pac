#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    proc0_nmi_mask: PROC0_NMI_MASK,
    proc1_nmi_mask: PROC1_NMI_MASK,
    proc_config: PROC_CONFIG,
    proc_in_sync_bypass: PROC_IN_SYNC_BYPASS,
    proc_in_sync_bypass_hi: PROC_IN_SYNC_BYPASS_HI,
    dbgforce: DBGFORCE,
    mempowerdown: MEMPOWERDOWN,
}
impl RegisterBlock {
    #[doc = "0x00 - Processor core 0 NMI source mask"]
    #[inline(always)]
    pub const fn proc0_nmi_mask(&self) -> &PROC0_NMI_MASK {
        &self.proc0_nmi_mask
    }
    #[doc = "0x04 - Processor core 1 NMI source mask"]
    #[inline(always)]
    pub const fn proc1_nmi_mask(&self) -> &PROC1_NMI_MASK {
        &self.proc1_nmi_mask
    }
    #[doc = "0x08 - Configuration for processors"]
    #[inline(always)]
    pub const fn proc_config(&self) -> &PROC_CONFIG {
        &self.proc_config
    }
    #[doc = "0x0c - For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...29."]
    #[inline(always)]
    pub const fn proc_in_sync_bypass(&self) -> &PROC_IN_SYNC_BYPASS {
        &self.proc_in_sync_bypass
    }
    #[doc = "0x10 - For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
    #[inline(always)]
    pub const fn proc_in_sync_bypass_hi(&self) -> &PROC_IN_SYNC_BYPASS_HI {
        &self.proc_in_sync_bypass_hi
    }
    #[doc = "0x14 - Directly control the SWD debug port of either processor"]
    #[inline(always)]
    pub const fn dbgforce(&self) -> &DBGFORCE {
        &self.dbgforce
    }
    #[doc = "0x18 - Control power downs to memories. Set high to power down memories. Use with extreme caution"]
    #[inline(always)]
    pub const fn mempowerdown(&self) -> &MEMPOWERDOWN {
        &self.mempowerdown
    }
}
#[doc = "PROC0_NMI_MASK (rw) register accessor: Processor core 0 NMI source mask  

You can [`read`](crate::generic::Reg::read) this register and get [`proc0_nmi_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc0_nmi_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc0_nmi_mask`]
module"]
pub type PROC0_NMI_MASK = crate::Reg<proc0_nmi_mask::PROC0_NMI_MASK_SPEC>;
#[doc = "Processor core 0 NMI source mask"]
pub mod proc0_nmi_mask;
#[doc = "PROC1_NMI_MASK (rw) register accessor: Processor core 1 NMI source mask  

You can [`read`](crate::generic::Reg::read) this register and get [`proc1_nmi_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc1_nmi_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc1_nmi_mask`]
module"]
pub type PROC1_NMI_MASK = crate::Reg<proc1_nmi_mask::PROC1_NMI_MASK_SPEC>;
#[doc = "Processor core 1 NMI source mask"]
pub mod proc1_nmi_mask;
#[doc = "PROC_CONFIG (rw) register accessor: Configuration for processors  

You can [`read`](crate::generic::Reg::read) this register and get [`proc_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc_config`]
module"]
pub type PROC_CONFIG = crate::Reg<proc_config::PROC_CONFIG_SPEC>;
#[doc = "Configuration for processors"]
pub mod proc_config;
#[doc = "PROC_IN_SYNC_BYPASS (rw) register accessor: For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...29.  

You can [`read`](crate::generic::Reg::read) this register and get [`proc_in_sync_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc_in_sync_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc_in_sync_bypass`]
module"]
pub type PROC_IN_SYNC_BYPASS = crate::Reg<proc_in_sync_bypass::PROC_IN_SYNC_BYPASS_SPEC>;
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...29."]
pub mod proc_in_sync_bypass;
#[doc = "PROC_IN_SYNC_BYPASS_HI (rw) register accessor: For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 30...35 (the QSPI IOs).  

You can [`read`](crate::generic::Reg::read) this register and get [`proc_in_sync_bypass_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc_in_sync_bypass_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc_in_sync_bypass_hi`]
module"]
pub type PROC_IN_SYNC_BYPASS_HI = crate::Reg<proc_in_sync_bypass_hi::PROC_IN_SYNC_BYPASS_HI_SPEC>;
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
pub mod proc_in_sync_bypass_hi;
#[doc = "DBGFORCE (rw) register accessor: Directly control the SWD debug port of either processor  

You can [`read`](crate::generic::Reg::read) this register and get [`dbgforce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgforce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbgforce`]
module"]
pub type DBGFORCE = crate::Reg<dbgforce::DBGFORCE_SPEC>;
#[doc = "Directly control the SWD debug port of either processor"]
pub mod dbgforce;
#[doc = "MEMPOWERDOWN (rw) register accessor: Control power downs to memories. Set high to power down memories. Use with extreme caution  

You can [`read`](crate::generic::Reg::read) this register and get [`mempowerdown::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mempowerdown::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mempowerdown`]
module"]
pub type MEMPOWERDOWN = crate::Reg<mempowerdown::MEMPOWERDOWN_SPEC>;
#[doc = "Control power downs to memories. Set high to power down memories. Use with extreme caution"]
pub mod mempowerdown;
