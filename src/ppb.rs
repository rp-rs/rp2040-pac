#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xe010],
    #[doc = "0xe010 - Use the SysTick Control and Status Register to enable the SysTick features."]
    pub syst_csr: SYST_CSR,
    #[doc = "0xe014 - Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN.  
 To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
    pub syst_rvr: SYST_RVR,
    #[doc = "0xe018 - Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
    pub syst_cvr: SYST_CVR,
    #[doc = "0xe01c - Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
    pub syst_calib: SYST_CALIB,
    _reserved4: [u8; 0xe0],
    #[doc = "0xe100 - Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled.  
 If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
    pub nvic_iser: NVIC_ISER,
    _reserved5: [u8; 0x7c],
    #[doc = "0xe180 - Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
    pub nvic_icer: NVIC_ICER,
    _reserved6: [u8; 0x7c],
    #[doc = "0xe200 - The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
    pub nvic_ispr: NVIC_ISPR,
    _reserved7: [u8; 0x7c],
    #[doc = "0xe280 - Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
    pub nvic_icpr: NVIC_ICPR,
    _reserved8: [u8; 0x017c],
    #[doc = "0xe400 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  
 Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt.  
 These registers are only word-accessible"]
    pub nvic_ipr0: NVIC_IPR0,
    #[doc = "0xe404 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub nvic_ipr1: NVIC_IPR1,
    #[doc = "0xe408 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub nvic_ipr2: NVIC_IPR2,
    #[doc = "0xe40c - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub nvic_ipr3: NVIC_IPR3,
    #[doc = "0xe410 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub nvic_ipr4: NVIC_IPR4,
    #[doc = "0xe414 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub nvic_ipr5: NVIC_IPR5,
    #[doc = "0xe418 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub nvic_ipr6: NVIC_IPR6,
    #[doc = "0xe41c - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub nvic_ipr7: NVIC_IPR7,
    _reserved16: [u8; 0x08e0],
    #[doc = "0xed00 - Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core."]
    pub cpuid: CPUID,
    #[doc = "0xed04 - Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception."]
    pub icsr: ICSR,
    #[doc = "0xed08 - The VTOR holds the vector table offset address."]
    pub vtor: VTOR,
    #[doc = "0xed0c - Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
    pub aircr: AIRCR,
    #[doc = "0xed10 - System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
    pub scr: SCR,
    #[doc = "0xed14 - The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault."]
    pub ccr: CCR,
    _reserved22: [u8; 0x04],
    #[doc = "0xed1c - System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
    pub shpr2: SHPR2,
    #[doc = "0xed20 - System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
    pub shpr3: SHPR3,
    #[doc = "0xed24 - Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
    pub shcsr: SHCSR,
    _reserved25: [u8; 0x68],
    #[doc = "0xed90 - Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports."]
    pub mpu_type: MPU_TYPE,
    #[doc = "0xed94 - Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs."]
    pub mpu_ctrl: MPU_CTRL,
    #[doc = "0xed98 - Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR."]
    pub mpu_rnr: MPU_RNR,
    #[doc = "0xed9c - Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated."]
    pub mpu_rbar: MPU_RBAR,
    #[doc = "0xeda0 - Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region."]
    pub mpu_rasr: MPU_RASR,
}
#[doc = "SYST_CSR (rw) register accessor: an alias for `Reg<SYST_CSR_SPEC>`"]
pub type SYST_CSR = crate::Reg<syst_csr::SYST_CSR_SPEC>;
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features."]
pub mod syst_csr;
#[doc = "SYST_RVR (rw) register accessor: an alias for `Reg<SYST_RVR_SPEC>`"]
pub type SYST_RVR = crate::Reg<syst_rvr::SYST_RVR_SPEC>;
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN.  
 To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
pub mod syst_rvr;
#[doc = "SYST_CVR (rw) register accessor: an alias for `Reg<SYST_CVR_SPEC>`"]
pub type SYST_CVR = crate::Reg<syst_cvr::SYST_CVR_SPEC>;
#[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
pub mod syst_cvr;
#[doc = "SYST_CALIB (r) register accessor: an alias for `Reg<SYST_CALIB_SPEC>`"]
pub type SYST_CALIB = crate::Reg<syst_calib::SYST_CALIB_SPEC>;
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
pub mod syst_calib;
#[doc = "NVIC_ISER (rw) register accessor: an alias for `Reg<NVIC_ISER_SPEC>`"]
pub type NVIC_ISER = crate::Reg<nvic_iser::NVIC_ISER_SPEC>;
#[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled.  
 If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
pub mod nvic_iser;
#[doc = "NVIC_ICER (rw) register accessor: an alias for `Reg<NVIC_ICER_SPEC>`"]
pub type NVIC_ICER = crate::Reg<nvic_icer::NVIC_ICER_SPEC>;
#[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer;
#[doc = "NVIC_ISPR (rw) register accessor: an alias for `Reg<NVIC_ISPR_SPEC>`"]
pub type NVIC_ISPR = crate::Reg<nvic_ispr::NVIC_ISPR_SPEC>;
#[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
pub mod nvic_ispr;
#[doc = "NVIC_ICPR (rw) register accessor: an alias for `Reg<NVIC_ICPR_SPEC>`"]
pub type NVIC_ICPR = crate::Reg<nvic_icpr::NVIC_ICPR_SPEC>;
#[doc = "Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr;
#[doc = "NVIC_IPR0 (rw) register accessor: an alias for `Reg<NVIC_IPR0_SPEC>`"]
pub type NVIC_IPR0 = crate::Reg<nvic_ipr0::NVIC_IPR0_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  
 Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt.  
 These registers are only word-accessible"]
pub mod nvic_ipr0;
#[doc = "NVIC_IPR1 (rw) register accessor: an alias for `Reg<NVIC_IPR1_SPEC>`"]
pub type NVIC_IPR1 = crate::Reg<nvic_ipr1::NVIC_IPR1_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr1;
#[doc = "NVIC_IPR2 (rw) register accessor: an alias for `Reg<NVIC_IPR2_SPEC>`"]
pub type NVIC_IPR2 = crate::Reg<nvic_ipr2::NVIC_IPR2_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr2;
#[doc = "NVIC_IPR3 (rw) register accessor: an alias for `Reg<NVIC_IPR3_SPEC>`"]
pub type NVIC_IPR3 = crate::Reg<nvic_ipr3::NVIC_IPR3_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr3;
#[doc = "NVIC_IPR4 (rw) register accessor: an alias for `Reg<NVIC_IPR4_SPEC>`"]
pub type NVIC_IPR4 = crate::Reg<nvic_ipr4::NVIC_IPR4_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr4;
#[doc = "NVIC_IPR5 (rw) register accessor: an alias for `Reg<NVIC_IPR5_SPEC>`"]
pub type NVIC_IPR5 = crate::Reg<nvic_ipr5::NVIC_IPR5_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr5;
#[doc = "NVIC_IPR6 (rw) register accessor: an alias for `Reg<NVIC_IPR6_SPEC>`"]
pub type NVIC_IPR6 = crate::Reg<nvic_ipr6::NVIC_IPR6_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr6;
#[doc = "NVIC_IPR7 (rw) register accessor: an alias for `Reg<NVIC_IPR7_SPEC>`"]
pub type NVIC_IPR7 = crate::Reg<nvic_ipr7::NVIC_IPR7_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr7;
#[doc = "CPUID (r) register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core."]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception."]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: an alias for `Reg<VTOR_SPEC>`"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "The VTOR holds the vector table offset address."]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: an alias for `Reg<AIRCR_SPEC>`"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
pub mod aircr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
pub mod scr;
#[doc = "CCR (r) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault."]
pub mod ccr;
#[doc = "SHPR2 (rw) register accessor: an alias for `Reg<SHPR2_SPEC>`"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: an alias for `Reg<SHPR3_SPEC>`"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: an alias for `Reg<SHCSR_SPEC>`"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
pub mod shcsr;
#[doc = "MPU_TYPE (r) register accessor: an alias for `Reg<MPU_TYPE_SPEC>`"]
pub type MPU_TYPE = crate::Reg<mpu_type::MPU_TYPE_SPEC>;
#[doc = "Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports."]
pub mod mpu_type;
#[doc = "MPU_CTRL (rw) register accessor: an alias for `Reg<MPU_CTRL_SPEC>`"]
pub type MPU_CTRL = crate::Reg<mpu_ctrl::MPU_CTRL_SPEC>;
#[doc = "Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs."]
pub mod mpu_ctrl;
#[doc = "MPU_RNR (rw) register accessor: an alias for `Reg<MPU_RNR_SPEC>`"]
pub type MPU_RNR = crate::Reg<mpu_rnr::MPU_RNR_SPEC>;
#[doc = "Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR."]
pub mod mpu_rnr;
#[doc = "MPU_RBAR (rw) register accessor: an alias for `Reg<MPU_RBAR_SPEC>`"]
pub type MPU_RBAR = crate::Reg<mpu_rbar::MPU_RBAR_SPEC>;
#[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated."]
pub mod mpu_rbar;
#[doc = "MPU_RASR (rw) register accessor: an alias for `Reg<MPU_RASR_SPEC>`"]
pub type MPU_RASR = crate::Reg<mpu_rasr::MPU_RASR_SPEC>;
#[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region."]
pub mod mpu_rasr;
