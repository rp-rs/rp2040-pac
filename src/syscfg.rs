#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Processor core 0 NMI source mask  
 Set a bit high to enable NMI from that IRQ"]
    pub proc0_nmi_mask: crate::Reg<proc0_nmi_mask::PROC0_NMI_MASK_SPEC>,
    #[doc = "0x04 - Processor core 1 NMI source mask  
 Set a bit high to enable NMI from that IRQ"]
    pub proc1_nmi_mask: crate::Reg<proc1_nmi_mask::PROC1_NMI_MASK_SPEC>,
    #[doc = "0x08 - Configuration for processors"]
    pub proc_config: crate::Reg<proc_config::PROC_CONFIG_SPEC>,
    #[doc = "0x0c - For each bit, if 1, bypass the input synchronizer between that GPIO  
 and the GPIO input register in the SIO. The input synchronizers should  
 generally be unbypassed, to avoid injecting metastabilities into processors.  
 If you're feeling brave, you can bypass to save two cycles of input  
 latency. This register applies to GPIO 0...29."]
    pub proc_in_sync_bypass: crate::Reg<proc_in_sync_bypass::PROC_IN_SYNC_BYPASS_SPEC>,
    #[doc = "0x10 - For each bit, if 1, bypass the input synchronizer between that GPIO  
 and the GPIO input register in the SIO. The input synchronizers should  
 generally be unbypassed, to avoid injecting metastabilities into processors.  
 If you're feeling brave, you can bypass to save two cycles of input  
 latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
    pub proc_in_sync_bypass_hi: crate::Reg<proc_in_sync_bypass_hi::PROC_IN_SYNC_BYPASS_HI_SPEC>,
    #[doc = "0x14 - Directly control the SWD debug port of either processor"]
    pub dbgforce: crate::Reg<dbgforce::DBGFORCE_SPEC>,
    #[doc = "0x18 - Control power downs to memories. Set high to power down memories.  
 Use with extreme caution"]
    pub mempowerdown: crate::Reg<mempowerdown::MEMPOWERDOWN_SPEC>,
}
#[doc = "PROC0_NMI_MASK register accessor: an alias for `Reg<PROC0_NMI_MASK_SPEC>`"]
pub type PROC0_NMI_MASK = crate::Reg<proc0_nmi_mask::PROC0_NMI_MASK_SPEC>;
#[doc = "Processor core 0 NMI source mask  
 Set a bit high to enable NMI from that IRQ"]
pub mod proc0_nmi_mask;
#[doc = "PROC1_NMI_MASK register accessor: an alias for `Reg<PROC1_NMI_MASK_SPEC>`"]
pub type PROC1_NMI_MASK = crate::Reg<proc1_nmi_mask::PROC1_NMI_MASK_SPEC>;
#[doc = "Processor core 1 NMI source mask  
 Set a bit high to enable NMI from that IRQ"]
pub mod proc1_nmi_mask;
#[doc = "PROC_CONFIG register accessor: an alias for `Reg<PROC_CONFIG_SPEC>`"]
pub type PROC_CONFIG = crate::Reg<proc_config::PROC_CONFIG_SPEC>;
#[doc = "Configuration for processors"]
pub mod proc_config;
#[doc = "PROC_IN_SYNC_BYPASS register accessor: an alias for `Reg<PROC_IN_SYNC_BYPASS_SPEC>`"]
pub type PROC_IN_SYNC_BYPASS = crate::Reg<proc_in_sync_bypass::PROC_IN_SYNC_BYPASS_SPEC>;
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO  
 and the GPIO input register in the SIO. The input synchronizers should  
 generally be unbypassed, to avoid injecting metastabilities into processors.  
 If you're feeling brave, you can bypass to save two cycles of input  
 latency. This register applies to GPIO 0...29."]
pub mod proc_in_sync_bypass;
#[doc = "PROC_IN_SYNC_BYPASS_HI register accessor: an alias for `Reg<PROC_IN_SYNC_BYPASS_HI_SPEC>`"]
pub type PROC_IN_SYNC_BYPASS_HI = crate::Reg<proc_in_sync_bypass_hi::PROC_IN_SYNC_BYPASS_HI_SPEC>;
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO  
 and the GPIO input register in the SIO. The input synchronizers should  
 generally be unbypassed, to avoid injecting metastabilities into processors.  
 If you're feeling brave, you can bypass to save two cycles of input  
 latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
pub mod proc_in_sync_bypass_hi;
#[doc = "DBGFORCE register accessor: an alias for `Reg<DBGFORCE_SPEC>`"]
pub type DBGFORCE = crate::Reg<dbgforce::DBGFORCE_SPEC>;
#[doc = "Directly control the SWD debug port of either processor"]
pub mod dbgforce;
#[doc = "MEMPOWERDOWN register accessor: an alias for `Reg<MEMPOWERDOWN_SPEC>`"]
pub type MEMPOWERDOWN = crate::Reg<mempowerdown::MEMPOWERDOWN_SPEC>;
#[doc = "Control power downs to memories. Set high to power down memories.  
 Use with extreme caution"]
pub mod mempowerdown;
