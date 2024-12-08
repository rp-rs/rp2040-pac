#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xe010],
    syst_csr: SYST_CSR,
    syst_rvr: SYST_RVR,
    syst_cvr: SYST_CVR,
    syst_calib: SYST_CALIB,
    _reserved4: [u8; 0xe0],
    nvic_iser: NVIC_ISER,
    _reserved5: [u8; 0x7c],
    nvic_icer: NVIC_ICER,
    _reserved6: [u8; 0x7c],
    nvic_ispr: NVIC_ISPR,
    _reserved7: [u8; 0x7c],
    nvic_icpr: NVIC_ICPR,
    _reserved8: [u8; 0x017c],
    nvic_ipr0: NVIC_IPR0,
    nvic_ipr1: NVIC_IPR1,
    nvic_ipr2: NVIC_IPR2,
    nvic_ipr3: NVIC_IPR3,
    nvic_ipr4: NVIC_IPR4,
    nvic_ipr5: NVIC_IPR5,
    nvic_ipr6: NVIC_IPR6,
    nvic_ipr7: NVIC_IPR7,
    _reserved16: [u8; 0x08e0],
    cpuid: CPUID,
    icsr: ICSR,
    vtor: VTOR,
    aircr: AIRCR,
    scr: SCR,
    ccr: CCR,
    _reserved22: [u8; 0x04],
    shpr2: SHPR2,
    shpr3: SHPR3,
    shcsr: SHCSR,
    _reserved25: [u8; 0x68],
    mpu_type: MPU_TYPE,
    mpu_ctrl: MPU_CTRL,
    mpu_rnr: MPU_RNR,
    mpu_rbar: MPU_RBAR,
    mpu_rasr: MPU_RASR,
}
impl RegisterBlock {
    #[doc = "0xe010 - Use the SysTick Control and Status Register to enable the SysTick features."]
    #[inline(always)]
    pub const fn syst_csr(&self) -> &SYST_CSR {
        &self.syst_csr
    }
    #[doc = "0xe014 - Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
    #[inline(always)]
    pub const fn syst_rvr(&self) -> &SYST_RVR {
        &self.syst_rvr
    }
    #[doc = "0xe018 - Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
    #[inline(always)]
    pub const fn syst_cvr(&self) -> &SYST_CVR {
        &self.syst_cvr
    }
    #[doc = "0xe01c - Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
    #[inline(always)]
    pub const fn syst_calib(&self) -> &SYST_CALIB {
        &self.syst_calib
    }
    #[doc = "0xe100 - Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled. If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
    #[inline(always)]
    pub const fn nvic_iser(&self) -> &NVIC_ISER {
        &self.nvic_iser
    }
    #[doc = "0xe180 - Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
    #[inline(always)]
    pub const fn nvic_icer(&self) -> &NVIC_ICER {
        &self.nvic_icer
    }
    #[doc = "0xe200 - The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
    #[inline(always)]
    pub const fn nvic_ispr(&self) -> &NVIC_ISPR {
        &self.nvic_ispr
    }
    #[doc = "0xe280 - Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
    #[inline(always)]
    pub const fn nvic_icpr(&self) -> &NVIC_ICPR {
        &self.nvic_icpr
    }
    #[doc = "0xe400 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest. Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt. These registers are only word-accessible"]
    #[inline(always)]
    pub const fn nvic_ipr0(&self) -> &NVIC_IPR0 {
        &self.nvic_ipr0
    }
    #[doc = "0xe404 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    #[inline(always)]
    pub const fn nvic_ipr1(&self) -> &NVIC_IPR1 {
        &self.nvic_ipr1
    }
    #[doc = "0xe408 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    #[inline(always)]
    pub const fn nvic_ipr2(&self) -> &NVIC_IPR2 {
        &self.nvic_ipr2
    }
    #[doc = "0xe40c - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    #[inline(always)]
    pub const fn nvic_ipr3(&self) -> &NVIC_IPR3 {
        &self.nvic_ipr3
    }
    #[doc = "0xe410 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    #[inline(always)]
    pub const fn nvic_ipr4(&self) -> &NVIC_IPR4 {
        &self.nvic_ipr4
    }
    #[doc = "0xe414 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    #[inline(always)]
    pub const fn nvic_ipr5(&self) -> &NVIC_IPR5 {
        &self.nvic_ipr5
    }
    #[doc = "0xe418 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    #[inline(always)]
    pub const fn nvic_ipr6(&self) -> &NVIC_IPR6 {
        &self.nvic_ipr6
    }
    #[doc = "0xe41c - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    #[inline(always)]
    pub const fn nvic_ipr7(&self) -> &NVIC_IPR7 {
        &self.nvic_ipr7
    }
    #[doc = "0xed00 - Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core."]
    #[inline(always)]
    pub const fn cpuid(&self) -> &CPUID {
        &self.cpuid
    }
    #[doc = "0xed04 - Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception."]
    #[inline(always)]
    pub const fn icsr(&self) -> &ICSR {
        &self.icsr
    }
    #[doc = "0xed08 - The VTOR holds the vector table offset address."]
    #[inline(always)]
    pub const fn vtor(&self) -> &VTOR {
        &self.vtor
    }
    #[doc = "0xed0c - Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
    #[inline(always)]
    pub const fn aircr(&self) -> &AIRCR {
        &self.aircr
    }
    #[doc = "0xed10 - System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0xed14 - The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault."]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0xed1c - System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
    #[inline(always)]
    pub const fn shpr2(&self) -> &SHPR2 {
        &self.shpr2
    }
    #[doc = "0xed20 - System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
    #[inline(always)]
    pub const fn shpr3(&self) -> &SHPR3 {
        &self.shpr3
    }
    #[doc = "0xed24 - Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
    #[inline(always)]
    pub const fn shcsr(&self) -> &SHCSR {
        &self.shcsr
    }
    #[doc = "0xed90 - Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports."]
    #[inline(always)]
    pub const fn mpu_type(&self) -> &MPU_TYPE {
        &self.mpu_type
    }
    #[doc = "0xed94 - Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs."]
    #[inline(always)]
    pub const fn mpu_ctrl(&self) -> &MPU_CTRL {
        &self.mpu_ctrl
    }
    #[doc = "0xed98 - Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR."]
    #[inline(always)]
    pub const fn mpu_rnr(&self) -> &MPU_RNR {
        &self.mpu_rnr
    }
    #[doc = "0xed9c - Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated."]
    #[inline(always)]
    pub const fn mpu_rbar(&self) -> &MPU_RBAR {
        &self.mpu_rbar
    }
    #[doc = "0xeda0 - Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region."]
    #[inline(always)]
    pub const fn mpu_rasr(&self) -> &MPU_RASR {
        &self.mpu_rasr
    }
}
#[doc = "SYST_CSR (rw) register accessor: Use the SysTick Control and Status Register to enable the SysTick features.  

You can [`read`](crate::generic::Reg::read) this register and get [`syst_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syst_csr`]
module"]
pub type SYST_CSR = crate::Reg<syst_csr::SYST_CSR_SPEC>;
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features."]
pub mod syst_csr;
#[doc = "SYST_RVR (rw) register accessor: Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99.  

You can [`read`](crate::generic::Reg::read) this register and get [`syst_rvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_rvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syst_rvr`]
module"]
pub type SYST_RVR = crate::Reg<syst_rvr::SYST_RVR_SPEC>;
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
pub mod syst_rvr;
#[doc = "SYST_CVR (rw) register accessor: Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN.  

You can [`read`](crate::generic::Reg::read) this register and get [`syst_cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syst_cvr`]
module"]
pub type SYST_CVR = crate::Reg<syst_cvr::SYST_CVR_SPEC>;
#[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
pub mod syst_cvr;
#[doc = "SYST_CALIB (rw) register accessor: Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply.  

You can [`read`](crate::generic::Reg::read) this register and get [`syst_calib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_calib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syst_calib`]
module"]
pub type SYST_CALIB = crate::Reg<syst_calib::SYST_CALIB_SPEC>;
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
pub mod syst_calib;
#[doc = "NVIC_ISER (rw) register accessor: Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled. If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_iser`]
module"]
pub type NVIC_ISER = crate::Reg<nvic_iser::NVIC_ISER_SPEC>;
#[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled. If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
pub mod nvic_iser;
#[doc = "NVIC_ICER (rw) register accessor: Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_icer`]
module"]
pub type NVIC_ICER = crate::Reg<nvic_icer::NVIC_ICER_SPEC>;
#[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer;
#[doc = "NVIC_ISPR (rw) register accessor: The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ispr`]
module"]
pub type NVIC_ISPR = crate::Reg<nvic_ispr::NVIC_ISPR_SPEC>;
#[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
pub mod nvic_ispr;
#[doc = "NVIC_ICPR (rw) register accessor: Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_icpr`]
module"]
pub type NVIC_ICPR = crate::Reg<nvic_icpr::NVIC_ICPR_SPEC>;
#[doc = "Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr;
#[doc = "NVIC_IPR0 (rw) register accessor: Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest. Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt. These registers are only word-accessible  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr0`]
module"]
pub type NVIC_IPR0 = crate::Reg<nvic_ipr0::NVIC_IPR0_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest. Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt. These registers are only word-accessible"]
pub mod nvic_ipr0;
#[doc = "NVIC_IPR1 (rw) register accessor: Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr1`]
module"]
pub type NVIC_IPR1 = crate::Reg<nvic_ipr1::NVIC_IPR1_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr1;
#[doc = "NVIC_IPR2 (rw) register accessor: Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr2`]
module"]
pub type NVIC_IPR2 = crate::Reg<nvic_ipr2::NVIC_IPR2_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr2;
#[doc = "NVIC_IPR3 (rw) register accessor: Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr3`]
module"]
pub type NVIC_IPR3 = crate::Reg<nvic_ipr3::NVIC_IPR3_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr3;
#[doc = "NVIC_IPR4 (rw) register accessor: Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr4`]
module"]
pub type NVIC_IPR4 = crate::Reg<nvic_ipr4::NVIC_IPR4_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr4;
#[doc = "NVIC_IPR5 (rw) register accessor: Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr5`]
module"]
pub type NVIC_IPR5 = crate::Reg<nvic_ipr5::NVIC_IPR5_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr5;
#[doc = "NVIC_IPR6 (rw) register accessor: Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr6`]
module"]
pub type NVIC_IPR6 = crate::Reg<nvic_ipr6::NVIC_IPR6_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr6;
#[doc = "NVIC_IPR7 (rw) register accessor: Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr7`]
module"]
pub type NVIC_IPR7 = crate::Reg<nvic_ipr7::NVIC_IPR7_SPEC>;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr7;
#[doc = "CPUID (rw) register accessor: Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core.  

You can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cpuid`]
module"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core."]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception.  

You can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@icsr`]
module"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception."]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: The VTOR holds the vector table offset address.  

You can [`read`](crate::generic::Reg::read) this register and get [`vtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vtor`]
module"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "The VTOR holds the vector table offset address."]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset.  

You can [`read`](crate::generic::Reg::read) this register and get [`aircr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aircr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@aircr`]
module"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
pub mod aircr;
#[doc = "SCR (rw) register accessor: System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states.  

You can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
pub mod scr;
#[doc = "CCR (rw) register accessor: The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault.  

You can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault."]
pub mod ccr;
#[doc = "SHPR2 (rw) register accessor: System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall.  

You can [`read`](crate::generic::Reg::read) this register and get [`shpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@shpr2`]
module"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick.  

You can [`read`](crate::generic::Reg::read) this register and get [`shpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@shpr3`]
module"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: Use the System Handler Control and State Register to determine or clear the pending status of SVCall.  

You can [`read`](crate::generic::Reg::read) this register and get [`shcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@shcsr`]
module"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
pub mod shcsr;
#[doc = "MPU_TYPE (rw) register accessor: Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports.  

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_type`]
module"]
pub type MPU_TYPE = crate::Reg<mpu_type::MPU_TYPE_SPEC>;
#[doc = "Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports."]
pub mod mpu_type;
#[doc = "MPU_CTRL (rw) register accessor: Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs.  

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_ctrl`]
module"]
pub type MPU_CTRL = crate::Reg<mpu_ctrl::MPU_CTRL_SPEC>;
#[doc = "Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs."]
pub mod mpu_ctrl;
#[doc = "MPU_RNR (rw) register accessor: Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR.  

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_rnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rnr`]
module"]
pub type MPU_RNR = crate::Reg<mpu_rnr::MPU_RNR_SPEC>;
#[doc = "Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR."]
pub mod mpu_rnr;
#[doc = "MPU_RBAR (rw) register accessor: Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated.  

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rbar`]
module"]
pub type MPU_RBAR = crate::Reg<mpu_rbar::MPU_RBAR_SPEC>;
#[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated."]
pub mod mpu_rbar;
#[doc = "MPU_RASR (rw) register accessor: Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region.  

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rasr`]
module"]
pub type MPU_RASR = crate::Reg<mpu_rasr::MPU_RASR_SPEC>;
#[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region."]
pub mod mpu_rasr;
