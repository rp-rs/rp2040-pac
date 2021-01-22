#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 57360usize],
    #[doc = "0xe010 - Use the SysTick Control and Status Register to enable the SysTick features."]
    pub syst_csr: SYST_CSR,
    #[doc = "0xe014 - Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN.\\n To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
    pub syst_rvr: SYST_RVR,
    #[doc = "0xe018 - Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
    pub syst_cvr: SYST_CVR,
    #[doc = "0xe01c - Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
    pub syst_calib: SYST_CALIB,
    _reserved4: [u8; 224usize],
    #[doc = "0xe100 - Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled.\\n If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
    pub nvic_iser: NVIC_ISER,
    _reserved5: [u8; 124usize],
    #[doc = "0xe180 - Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
    pub nvic_icer: NVIC_ICER,
    _reserved6: [u8; 124usize],
    #[doc = "0xe200 - The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
    pub nvic_ispr: NVIC_ISPR,
    _reserved7: [u8; 124usize],
    #[doc = "0xe280 - Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
    pub nvic_icpr: NVIC_ICPR,
    _reserved8: [u8; 380usize],
    #[doc = "0xe400 - Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\\n Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt.\\n These registers are only word-accessible"]
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
    _reserved16: [u8; 2272usize],
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
    _reserved22: [u8; 4usize],
    #[doc = "0xed1c - System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
    pub shpr2: SHPR2,
    #[doc = "0xed20 - System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
    pub shpr3: SHPR3,
    #[doc = "0xed24 - Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
    pub shcsr: SHCSR,
    _reserved25: [u8; 104usize],
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
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_csr](syst_csr) module"]
pub type SYST_CSR = crate::Reg<u32, _SYST_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CSR;
#[doc = "`read()` method returns [syst_csr::R](syst_csr::R) reader structure"]
impl crate::Readable for SYST_CSR {}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](syst_csr::W) writer structure"]
impl crate::Writable for SYST_CSR {}
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features."]
pub mod syst_csr;
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN.\\n To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_rvr](syst_rvr) module"]
pub type SYST_RVR = crate::Reg<u32, _SYST_RVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_RVR;
#[doc = "`read()` method returns [syst_rvr::R](syst_rvr::R) reader structure"]
impl crate::Readable for SYST_RVR {}
#[doc = "`write(|w| ..)` method takes [syst_rvr::W](syst_rvr::W) writer structure"]
impl crate::Writable for SYST_RVR {}
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN.\\n To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
pub mod syst_rvr;
#[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_cvr](syst_cvr) module"]
pub type SYST_CVR = crate::Reg<u32, _SYST_CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CVR;
#[doc = "`read()` method returns [syst_cvr::R](syst_cvr::R) reader structure"]
impl crate::Readable for SYST_CVR {}
#[doc = "`write(|w| ..)` method takes [syst_cvr::W](syst_cvr::W) writer structure"]
impl crate::Writable for SYST_CVR {}
#[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
pub mod syst_cvr;
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_calib](syst_calib) module"]
pub type SYST_CALIB = crate::Reg<u32, _SYST_CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CALIB;
#[doc = "`read()` method returns [syst_calib::R](syst_calib::R) reader structure"]
impl crate::Readable for SYST_CALIB {}
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
pub mod syst_calib;
#[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled.\\n If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iser](nvic_iser) module"]
pub type NVIC_ISER = crate::Reg<u32, _NVIC_ISER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER;
#[doc = "`read()` method returns [nvic_iser::R](nvic_iser::R) reader structure"]
impl crate::Readable for NVIC_ISER {}
#[doc = "`write(|w| ..)` method takes [nvic_iser::W](nvic_iser::W) writer structure"]
impl crate::Writable for NVIC_ISER {}
#[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled.\\n If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
pub mod nvic_iser;
#[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icer](nvic_icer) module"]
pub type NVIC_ICER = crate::Reg<u32, _NVIC_ICER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER;
#[doc = "`read()` method returns [nvic_icer::R](nvic_icer::R) reader structure"]
impl crate::Readable for NVIC_ICER {}
#[doc = "`write(|w| ..)` method takes [nvic_icer::W](nvic_icer::W) writer structure"]
impl crate::Writable for NVIC_ICER {}
#[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer;
#[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ispr](nvic_ispr) module"]
pub type NVIC_ISPR = crate::Reg<u32, _NVIC_ISPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR;
#[doc = "`read()` method returns [nvic_ispr::R](nvic_ispr::R) reader structure"]
impl crate::Readable for NVIC_ISPR {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr::W](nvic_ispr::W) writer structure"]
impl crate::Writable for NVIC_ISPR {}
#[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
pub mod nvic_ispr;
#[doc = "Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icpr](nvic_icpr) module"]
pub type NVIC_ICPR = crate::Reg<u32, _NVIC_ICPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR;
#[doc = "`read()` method returns [nvic_icpr::R](nvic_icpr::R) reader structure"]
impl crate::Readable for NVIC_ICPR {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr::W](nvic_icpr::W) writer structure"]
impl crate::Writable for NVIC_ICPR {}
#[doc = "Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\\n Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt.\\n These registers are only word-accessible\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr0](nvic_ipr0) module"]
pub type NVIC_IPR0 = crate::Reg<u32, _NVIC_IPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR0;
#[doc = "`read()` method returns [nvic_ipr0::R](nvic_ipr0::R) reader structure"]
impl crate::Readable for NVIC_IPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr0::W](nvic_ipr0::W) writer structure"]
impl crate::Writable for NVIC_IPR0 {}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\\n Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt.\\n These registers are only word-accessible"]
pub mod nvic_ipr0;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr1](nvic_ipr1) module"]
pub type NVIC_IPR1 = crate::Reg<u32, _NVIC_IPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR1;
#[doc = "`read()` method returns [nvic_ipr1::R](nvic_ipr1::R) reader structure"]
impl crate::Readable for NVIC_IPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr1::W](nvic_ipr1::W) writer structure"]
impl crate::Writable for NVIC_IPR1 {}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr1;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr2](nvic_ipr2) module"]
pub type NVIC_IPR2 = crate::Reg<u32, _NVIC_IPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR2;
#[doc = "`read()` method returns [nvic_ipr2::R](nvic_ipr2::R) reader structure"]
impl crate::Readable for NVIC_IPR2 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr2::W](nvic_ipr2::W) writer structure"]
impl crate::Writable for NVIC_IPR2 {}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr2;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr3](nvic_ipr3) module"]
pub type NVIC_IPR3 = crate::Reg<u32, _NVIC_IPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR3;
#[doc = "`read()` method returns [nvic_ipr3::R](nvic_ipr3::R) reader structure"]
impl crate::Readable for NVIC_IPR3 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr3::W](nvic_ipr3::W) writer structure"]
impl crate::Writable for NVIC_IPR3 {}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr3;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr4](nvic_ipr4) module"]
pub type NVIC_IPR4 = crate::Reg<u32, _NVIC_IPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR4;
#[doc = "`read()` method returns [nvic_ipr4::R](nvic_ipr4::R) reader structure"]
impl crate::Readable for NVIC_IPR4 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr4::W](nvic_ipr4::W) writer structure"]
impl crate::Writable for NVIC_IPR4 {}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr4;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr5](nvic_ipr5) module"]
pub type NVIC_IPR5 = crate::Reg<u32, _NVIC_IPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR5;
#[doc = "`read()` method returns [nvic_ipr5::R](nvic_ipr5::R) reader structure"]
impl crate::Readable for NVIC_IPR5 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr5::W](nvic_ipr5::W) writer structure"]
impl crate::Writable for NVIC_IPR5 {}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr5;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr6](nvic_ipr6) module"]
pub type NVIC_IPR6 = crate::Reg<u32, _NVIC_IPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR6;
#[doc = "`read()` method returns [nvic_ipr6::R](nvic_ipr6::R) reader structure"]
impl crate::Readable for NVIC_IPR6 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr6::W](nvic_ipr6::W) writer structure"]
impl crate::Writable for NVIC_IPR6 {}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr6;
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr7](nvic_ipr7) module"]
pub type NVIC_IPR7 = crate::Reg<u32, _NVIC_IPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR7;
#[doc = "`read()` method returns [nvic_ipr7::R](nvic_ipr7::R) reader structure"]
impl crate::Readable for NVIC_IPR7 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr7::W](nvic_ipr7::W) writer structure"]
impl crate::Writable for NVIC_IPR7 {}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
pub mod nvic_ipr7;
#[doc = "Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core."]
pub mod cpuid;
#[doc = "Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](icsr) module"]
pub type ICSR = crate::Reg<u32, _ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSR;
#[doc = "`read()` method returns [icsr::R](icsr::R) reader structure"]
impl crate::Readable for ICSR {}
#[doc = "`write(|w| ..)` method takes [icsr::W](icsr::W) writer structure"]
impl crate::Writable for ICSR {}
#[doc = "Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception."]
pub mod icsr;
#[doc = "The VTOR holds the vector table offset address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtor](vtor) module"]
pub type VTOR = crate::Reg<u32, _VTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VTOR;
#[doc = "`read()` method returns [vtor::R](vtor::R) reader structure"]
impl crate::Readable for VTOR {}
#[doc = "`write(|w| ..)` method takes [vtor::W](vtor::W) writer structure"]
impl crate::Writable for VTOR {}
#[doc = "The VTOR holds the vector table offset address."]
pub mod vtor;
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](aircr) module"]
pub type AIRCR = crate::Reg<u32, _AIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIRCR;
#[doc = "`read()` method returns [aircr::R](aircr::R) reader structure"]
impl crate::Readable for AIRCR {}
#[doc = "`write(|w| ..)` method takes [aircr::W](aircr::W) writer structure"]
impl crate::Writable for AIRCR {}
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
pub mod aircr;
#[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
pub mod scr;
#[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault."]
pub mod ccr;
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr2](shpr2) module"]
pub type SHPR2 = crate::Reg<u32, _SHPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR2;
#[doc = "`read()` method returns [shpr2::R](shpr2::R) reader structure"]
impl crate::Readable for SHPR2 {}
#[doc = "`write(|w| ..)` method takes [shpr2::W](shpr2::W) writer structure"]
impl crate::Writable for SHPR2 {}
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
pub mod shpr2;
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr3](shpr3) module"]
pub type SHPR3 = crate::Reg<u32, _SHPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR3;
#[doc = "`read()` method returns [shpr3::R](shpr3::R) reader structure"]
impl crate::Readable for SHPR3 {}
#[doc = "`write(|w| ..)` method takes [shpr3::W](shpr3::W) writer structure"]
impl crate::Writable for SHPR3 {}
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
pub mod shpr3;
#[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcsr](shcsr) module"]
pub type SHCSR = crate::Reg<u32, _SHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHCSR;
#[doc = "`read()` method returns [shcsr::R](shcsr::R) reader structure"]
impl crate::Readable for SHCSR {}
#[doc = "`write(|w| ..)` method takes [shcsr::W](shcsr::W) writer structure"]
impl crate::Writable for SHCSR {}
#[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
pub mod shcsr;
#[doc = "Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_type](mpu_type) module"]
pub type MPU_TYPE = crate::Reg<u32, _MPU_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_TYPE;
#[doc = "`read()` method returns [mpu_type::R](mpu_type::R) reader structure"]
impl crate::Readable for MPU_TYPE {}
#[doc = "Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports."]
pub mod mpu_type;
#[doc = "Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_ctrl](mpu_ctrl) module"]
pub type MPU_CTRL = crate::Reg<u32, _MPU_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_CTRL;
#[doc = "`read()` method returns [mpu_ctrl::R](mpu_ctrl::R) reader structure"]
impl crate::Readable for MPU_CTRL {}
#[doc = "`write(|w| ..)` method takes [mpu_ctrl::W](mpu_ctrl::W) writer structure"]
impl crate::Writable for MPU_CTRL {}
#[doc = "Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs."]
pub mod mpu_ctrl;
#[doc = "Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rnr](mpu_rnr) module"]
pub type MPU_RNR = crate::Reg<u32, _MPU_RNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RNR;
#[doc = "`read()` method returns [mpu_rnr::R](mpu_rnr::R) reader structure"]
impl crate::Readable for MPU_RNR {}
#[doc = "`write(|w| ..)` method takes [mpu_rnr::W](mpu_rnr::W) writer structure"]
impl crate::Writable for MPU_RNR {}
#[doc = "Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR."]
pub mod mpu_rnr;
#[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rbar](mpu_rbar) module"]
pub type MPU_RBAR = crate::Reg<u32, _MPU_RBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR;
#[doc = "`read()` method returns [mpu_rbar::R](mpu_rbar::R) reader structure"]
impl crate::Readable for MPU_RBAR {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar::W](mpu_rbar::W) writer structure"]
impl crate::Writable for MPU_RBAR {}
#[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated."]
pub mod mpu_rbar;
#[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr](mpu_rasr) module"]
pub type MPU_RASR = crate::Reg<u32, _MPU_RASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR;
#[doc = "`read()` method returns [mpu_rasr::R](mpu_rasr::R) reader structure"]
impl crate::Readable for MPU_RASR {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr::W](mpu_rasr::W) writer structure"]
impl crate::Writable for MPU_RASR {}
#[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region."]
pub mod mpu_rasr;
