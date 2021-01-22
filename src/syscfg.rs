#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Processor core 0 NMI source mask\\n Set a bit high to enable NMI from that IRQ"]
    pub proc0_nmi_mask: PROC0_NMI_MASK,
    #[doc = "0x04 - Processor core 1 NMI source mask\\n Set a bit high to enable NMI from that IRQ"]
    pub proc1_nmi_mask: PROC1_NMI_MASK,
    #[doc = "0x08 - Configuration for processors"]
    pub proc_config: PROC_CONFIG,
    #[doc = "0x0c - For each bit, if 1, bypass the input synchronizer between that GPIO\\n and the GPIO input register in the SIO. The input synchronizers should\\n generally be unbypassed, to avoid injecting metastabilities into processors.\\n If you're feeling brave, you can bypass to save two cycles of input\\n latency. This register applies to GPIO 0...29."]
    pub proc_in_sync_bypass: PROC_IN_SYNC_BYPASS,
    #[doc = "0x10 - For each bit, if 1, bypass the input synchronizer between that GPIO\\n and the GPIO input register in the SIO. The input synchronizers should\\n generally be unbypassed, to avoid injecting metastabilities into processors.\\n If you're feeling brave, you can bypass to save two cycles of input\\n latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
    pub proc_in_sync_bypass_hi: PROC_IN_SYNC_BYPASS_HI,
    #[doc = "0x14 - Directly control the SWD debug port of either processor"]
    pub dbgforce: DBGFORCE,
    #[doc = "0x18 - Control power downs to memories. Set high to power down memories.\\n Use with extreme caution"]
    pub mempowerdown: MEMPOWERDOWN,
}
#[doc = "Processor core 0 NMI source mask\\n Set a bit high to enable NMI from that IRQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc0_nmi_mask](proc0_nmi_mask) module"]
pub type PROC0_NMI_MASK = crate::Reg<u32, _PROC0_NMI_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC0_NMI_MASK;
#[doc = "`read()` method returns [proc0_nmi_mask::R](proc0_nmi_mask::R) reader structure"]
impl crate::Readable for PROC0_NMI_MASK {}
#[doc = "`write(|w| ..)` method takes [proc0_nmi_mask::W](proc0_nmi_mask::W) writer structure"]
impl crate::Writable for PROC0_NMI_MASK {}
#[doc = "Processor core 0 NMI source mask\\n Set a bit high to enable NMI from that IRQ"]
pub mod proc0_nmi_mask;
#[doc = "Processor core 1 NMI source mask\\n Set a bit high to enable NMI from that IRQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc1_nmi_mask](proc1_nmi_mask) module"]
pub type PROC1_NMI_MASK = crate::Reg<u32, _PROC1_NMI_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC1_NMI_MASK;
#[doc = "`read()` method returns [proc1_nmi_mask::R](proc1_nmi_mask::R) reader structure"]
impl crate::Readable for PROC1_NMI_MASK {}
#[doc = "`write(|w| ..)` method takes [proc1_nmi_mask::W](proc1_nmi_mask::W) writer structure"]
impl crate::Writable for PROC1_NMI_MASK {}
#[doc = "Processor core 1 NMI source mask\\n Set a bit high to enable NMI from that IRQ"]
pub mod proc1_nmi_mask;
#[doc = "Configuration for processors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc_config](proc_config) module"]
pub type PROC_CONFIG = crate::Reg<u32, _PROC_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC_CONFIG;
#[doc = "`read()` method returns [proc_config::R](proc_config::R) reader structure"]
impl crate::Readable for PROC_CONFIG {}
#[doc = "`write(|w| ..)` method takes [proc_config::W](proc_config::W) writer structure"]
impl crate::Writable for PROC_CONFIG {}
#[doc = "Configuration for processors"]
pub mod proc_config;
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO\\n and the GPIO input register in the SIO. The input synchronizers should\\n generally be unbypassed, to avoid injecting metastabilities into processors.\\n If you're feeling brave, you can bypass to save two cycles of input\\n latency. This register applies to GPIO 0...29.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc_in_sync_bypass](proc_in_sync_bypass) module"]
pub type PROC_IN_SYNC_BYPASS = crate::Reg<u32, _PROC_IN_SYNC_BYPASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC_IN_SYNC_BYPASS;
#[doc = "`read()` method returns [proc_in_sync_bypass::R](proc_in_sync_bypass::R) reader structure"]
impl crate::Readable for PROC_IN_SYNC_BYPASS {}
#[doc = "`write(|w| ..)` method takes [proc_in_sync_bypass::W](proc_in_sync_bypass::W) writer structure"]
impl crate::Writable for PROC_IN_SYNC_BYPASS {}
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO\\n and the GPIO input register in the SIO. The input synchronizers should\\n generally be unbypassed, to avoid injecting metastabilities into processors.\\n If you're feeling brave, you can bypass to save two cycles of input\\n latency. This register applies to GPIO 0...29."]
pub mod proc_in_sync_bypass;
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO\\n and the GPIO input register in the SIO. The input synchronizers should\\n generally be unbypassed, to avoid injecting metastabilities into processors.\\n If you're feeling brave, you can bypass to save two cycles of input\\n latency. This register applies to GPIO 30...35 (the QSPI IOs).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc_in_sync_bypass_hi](proc_in_sync_bypass_hi) module"]
pub type PROC_IN_SYNC_BYPASS_HI = crate::Reg<u32, _PROC_IN_SYNC_BYPASS_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC_IN_SYNC_BYPASS_HI;
#[doc = "`read()` method returns [proc_in_sync_bypass_hi::R](proc_in_sync_bypass_hi::R) reader structure"]
impl crate::Readable for PROC_IN_SYNC_BYPASS_HI {}
#[doc = "`write(|w| ..)` method takes [proc_in_sync_bypass_hi::W](proc_in_sync_bypass_hi::W) writer structure"]
impl crate::Writable for PROC_IN_SYNC_BYPASS_HI {}
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO\\n and the GPIO input register in the SIO. The input synchronizers should\\n generally be unbypassed, to avoid injecting metastabilities into processors.\\n If you're feeling brave, you can bypass to save two cycles of input\\n latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
pub mod proc_in_sync_bypass_hi;
#[doc = "Directly control the SWD debug port of either processor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgforce](dbgforce) module"]
pub type DBGFORCE = crate::Reg<u32, _DBGFORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGFORCE;
#[doc = "`read()` method returns [dbgforce::R](dbgforce::R) reader structure"]
impl crate::Readable for DBGFORCE {}
#[doc = "`write(|w| ..)` method takes [dbgforce::W](dbgforce::W) writer structure"]
impl crate::Writable for DBGFORCE {}
#[doc = "Directly control the SWD debug port of either processor"]
pub mod dbgforce;
#[doc = "Control power downs to memories. Set high to power down memories.\\n Use with extreme caution\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mempowerdown](mempowerdown) module"]
pub type MEMPOWERDOWN = crate::Reg<u32, _MEMPOWERDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMPOWERDOWN;
#[doc = "`read()` method returns [mempowerdown::R](mempowerdown::R) reader structure"]
impl crate::Readable for MEMPOWERDOWN {}
#[doc = "`write(|w| ..)` method takes [mempowerdown::W](mempowerdown::W) writer structure"]
impl crate::Writable for MEMPOWERDOWN {}
#[doc = "Control power downs to memories. Set high to power down memories.\\n Use with extreme caution"]
pub mod mempowerdown;
