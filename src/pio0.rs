#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - FIFO status register"]
    pub fstat: FSTAT,
    #[doc = "0x08 - FIFO debug register"]
    pub fdebug: FDEBUG,
    #[doc = "0x0c - FIFO levels"]
    pub flevel: FLEVEL,
    #[doc = "0x10 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub txf0: TXF0,
    #[doc = "0x14 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub txf1: TXF1,
    #[doc = "0x18 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub txf2: TXF2,
    #[doc = "0x1c - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub txf3: TXF3,
    #[doc = "0x20 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub rxf0: RXF0,
    #[doc = "0x24 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub rxf1: RXF1,
    #[doc = "0x28 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub rxf2: RXF2,
    #[doc = "0x2c - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub rxf3: RXF3,
    #[doc = "0x30 - Interrupt request register. Write 1 to clear"]
    pub irq: IRQ,
    #[doc = "0x34 - Writing a 1 to each of these bits will forcibly assert the corresponding IRQ.\\n Note this is different to the INTF register: writing here affects PIO internal\\n state. INTF just asserts the processor-facing IRQ signal for testing ISRs,\\n and is not visible to the state machines."]
    pub irq_force: IRQ_FORCE,
    #[doc = "0x38 - There is a 2-flipflop synchronizer on each GPIO input, which protects\\n PIO logic from metastabilities. This increases input delay, and for fast\\n synchronous IO (e.g. SPI) these synchronizers may need to be bypassed.\\n Each bit in this register corresponds to one GPIO.\\n 0 -> input is synchronized (default)\\n 1 -> synchronizer is bypassed\\n If in doubt, leave this register as all zeroes."]
    pub input_sync_bypass: INPUT_SYNC_BYPASS,
    #[doc = "0x3c - Read to sample the pad output values PIO is currently driving to the GPIOs."]
    pub dbg_padout: DBG_PADOUT,
    #[doc = "0x40 - Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs."]
    pub dbg_padoe: DBG_PADOE,
    #[doc = "0x44 - The PIO hardware has some free parameters that may vary between chip products.\\n These should be provided in the chip datasheet, but are also exposed here."]
    pub dbg_cfginfo: DBG_CFGINFO,
    #[doc = "0x48 - Write-only access to instruction memory location 0"]
    pub instr_mem0: INSTR_MEM0,
    #[doc = "0x4c - Write-only access to instruction memory location 1"]
    pub instr_mem1: INSTR_MEM1,
    #[doc = "0x50 - Write-only access to instruction memory location 2"]
    pub instr_mem2: INSTR_MEM2,
    #[doc = "0x54 - Write-only access to instruction memory location 3"]
    pub instr_mem3: INSTR_MEM3,
    #[doc = "0x58 - Write-only access to instruction memory location 4"]
    pub instr_mem4: INSTR_MEM4,
    #[doc = "0x5c - Write-only access to instruction memory location 5"]
    pub instr_mem5: INSTR_MEM5,
    #[doc = "0x60 - Write-only access to instruction memory location 6"]
    pub instr_mem6: INSTR_MEM6,
    #[doc = "0x64 - Write-only access to instruction memory location 7"]
    pub instr_mem7: INSTR_MEM7,
    #[doc = "0x68 - Write-only access to instruction memory location 8"]
    pub instr_mem8: INSTR_MEM8,
    #[doc = "0x6c - Write-only access to instruction memory location 9"]
    pub instr_mem9: INSTR_MEM9,
    #[doc = "0x70 - Write-only access to instruction memory location 10"]
    pub instr_mem10: INSTR_MEM10,
    #[doc = "0x74 - Write-only access to instruction memory location 11"]
    pub instr_mem11: INSTR_MEM11,
    #[doc = "0x78 - Write-only access to instruction memory location 12"]
    pub instr_mem12: INSTR_MEM12,
    #[doc = "0x7c - Write-only access to instruction memory location 13"]
    pub instr_mem13: INSTR_MEM13,
    #[doc = "0x80 - Write-only access to instruction memory location 14"]
    pub instr_mem14: INSTR_MEM14,
    #[doc = "0x84 - Write-only access to instruction memory location 15"]
    pub instr_mem15: INSTR_MEM15,
    #[doc = "0x88 - Write-only access to instruction memory location 16"]
    pub instr_mem16: INSTR_MEM16,
    #[doc = "0x8c - Write-only access to instruction memory location 17"]
    pub instr_mem17: INSTR_MEM17,
    #[doc = "0x90 - Write-only access to instruction memory location 18"]
    pub instr_mem18: INSTR_MEM18,
    #[doc = "0x94 - Write-only access to instruction memory location 19"]
    pub instr_mem19: INSTR_MEM19,
    #[doc = "0x98 - Write-only access to instruction memory location 20"]
    pub instr_mem20: INSTR_MEM20,
    #[doc = "0x9c - Write-only access to instruction memory location 21"]
    pub instr_mem21: INSTR_MEM21,
    #[doc = "0xa0 - Write-only access to instruction memory location 22"]
    pub instr_mem22: INSTR_MEM22,
    #[doc = "0xa4 - Write-only access to instruction memory location 23"]
    pub instr_mem23: INSTR_MEM23,
    #[doc = "0xa8 - Write-only access to instruction memory location 24"]
    pub instr_mem24: INSTR_MEM24,
    #[doc = "0xac - Write-only access to instruction memory location 25"]
    pub instr_mem25: INSTR_MEM25,
    #[doc = "0xb0 - Write-only access to instruction memory location 26"]
    pub instr_mem26: INSTR_MEM26,
    #[doc = "0xb4 - Write-only access to instruction memory location 27"]
    pub instr_mem27: INSTR_MEM27,
    #[doc = "0xb8 - Write-only access to instruction memory location 28"]
    pub instr_mem28: INSTR_MEM28,
    #[doc = "0xbc - Write-only access to instruction memory location 29"]
    pub instr_mem29: INSTR_MEM29,
    #[doc = "0xc0 - Write-only access to instruction memory location 30"]
    pub instr_mem30: INSTR_MEM30,
    #[doc = "0xc4 - Write-only access to instruction memory location 31"]
    pub instr_mem31: INSTR_MEM31,
    #[doc = "0xc8 - Clock divider register for state machine 0\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm0_clkdiv: SM0_CLKDIV,
    #[doc = "0xcc - Execution/behavioural settings for state machine 0"]
    pub sm0_execctrl: SM0_EXECCTRL,
    #[doc = "0xd0 - Control behaviour of the input/output shift registers for state machine 0"]
    pub sm0_shiftctrl: SM0_SHIFTCTRL,
    #[doc = "0xd4 - Current instruction address of state machine 0"]
    pub sm0_addr: SM0_ADDR,
    #[doc = "0xd8 - Instruction currently being executed by state machine 0\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm0_instr: SM0_INSTR,
    #[doc = "0xdc - State machine pin control"]
    pub sm0_pinctrl: SM0_PINCTRL,
    #[doc = "0xe0 - Clock divider register for state machine 1\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm1_clkdiv: SM1_CLKDIV,
    #[doc = "0xe4 - Execution/behavioural settings for state machine 1"]
    pub sm1_execctrl: SM1_EXECCTRL,
    #[doc = "0xe8 - Control behaviour of the input/output shift registers for state machine 1"]
    pub sm1_shiftctrl: SM1_SHIFTCTRL,
    #[doc = "0xec - Current instruction address of state machine 1"]
    pub sm1_addr: SM1_ADDR,
    #[doc = "0xf0 - Instruction currently being executed by state machine 1\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm1_instr: SM1_INSTR,
    #[doc = "0xf4 - State machine pin control"]
    pub sm1_pinctrl: SM1_PINCTRL,
    #[doc = "0xf8 - Clock divider register for state machine 2\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm2_clkdiv: SM2_CLKDIV,
    #[doc = "0xfc - Execution/behavioural settings for state machine 2"]
    pub sm2_execctrl: SM2_EXECCTRL,
    #[doc = "0x100 - Control behaviour of the input/output shift registers for state machine 2"]
    pub sm2_shiftctrl: SM2_SHIFTCTRL,
    #[doc = "0x104 - Current instruction address of state machine 2"]
    pub sm2_addr: SM2_ADDR,
    #[doc = "0x108 - Instruction currently being executed by state machine 2\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm2_instr: SM2_INSTR,
    #[doc = "0x10c - State machine pin control"]
    pub sm2_pinctrl: SM2_PINCTRL,
    #[doc = "0x110 - Clock divider register for state machine 3\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm3_clkdiv: SM3_CLKDIV,
    #[doc = "0x114 - Execution/behavioural settings for state machine 3"]
    pub sm3_execctrl: SM3_EXECCTRL,
    #[doc = "0x118 - Control behaviour of the input/output shift registers for state machine 3"]
    pub sm3_shiftctrl: SM3_SHIFTCTRL,
    #[doc = "0x11c - Current instruction address of state machine 3"]
    pub sm3_addr: SM3_ADDR,
    #[doc = "0x120 - Instruction currently being executed by state machine 3\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm3_instr: SM3_INSTR,
    #[doc = "0x124 - State machine pin control"]
    pub sm3_pinctrl: SM3_PINCTRL,
    #[doc = "0x128 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0x12c - Interrupt Enable for irq0"]
    pub irq0_inte: IRQ0_INTE,
    #[doc = "0x130 - Interrupt Force for irq0"]
    pub irq0_intf: IRQ0_INTF,
    #[doc = "0x134 - Interrupt status after masking & forcing for irq0"]
    pub irq0_ints: IRQ0_INTS,
    #[doc = "0x138 - Interrupt Enable for irq1"]
    pub irq1_inte: IRQ1_INTE,
    #[doc = "0x13c - Interrupt Force for irq1"]
    pub irq1_intf: IRQ1_INTF,
    #[doc = "0x140 - Interrupt status after masking & forcing for irq1"]
    pub irq1_ints: IRQ1_INTS,
}
#[doc = "PIO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "PIO control register"]
pub mod ctrl;
#[doc = "FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstat](fstat) module"]
pub type FSTAT = crate::Reg<u32, _FSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTAT;
#[doc = "`read()` method returns [fstat::R](fstat::R) reader structure"]
impl crate::Readable for FSTAT {}
#[doc = "FIFO status register"]
pub mod fstat;
#[doc = "FIFO debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdebug](fdebug) module"]
pub type FDEBUG = crate::Reg<u32, _FDEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDEBUG;
#[doc = "`read()` method returns [fdebug::R](fdebug::R) reader structure"]
impl crate::Readable for FDEBUG {}
#[doc = "`write(|w| ..)` method takes [fdebug::W](fdebug::W) writer structure"]
impl crate::Writable for FDEBUG {}
#[doc = "FIFO debug register"]
pub mod fdebug;
#[doc = "FIFO levels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flevel](flevel) module"]
pub type FLEVEL = crate::Reg<u32, _FLEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEVEL;
#[doc = "`read()` method returns [flevel::R](flevel::R) reader structure"]
impl crate::Readable for FLEVEL {}
#[doc = "FIFO levels"]
pub mod flevel;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf0](txf0) module"]
pub type TXF0 = crate::Reg<u32, _TXF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXF0;
#[doc = "`write(|w| ..)` method takes [txf0::W](txf0::W) writer structure"]
impl crate::Writable for TXF0 {}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
pub mod txf0;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf1](txf1) module"]
pub type TXF1 = crate::Reg<u32, _TXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXF1;
#[doc = "`write(|w| ..)` method takes [txf1::W](txf1::W) writer structure"]
impl crate::Writable for TXF1 {}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
pub mod txf1;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf2](txf2) module"]
pub type TXF2 = crate::Reg<u32, _TXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXF2;
#[doc = "`write(|w| ..)` method takes [txf2::W](txf2::W) writer structure"]
impl crate::Writable for TXF2 {}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
pub mod txf2;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf3](txf3) module"]
pub type TXF3 = crate::Reg<u32, _TXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXF3;
#[doc = "`write(|w| ..)` method takes [txf3::W](txf3::W) writer structure"]
impl crate::Writable for TXF3 {}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
pub mod txf3;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0](rxf0) module"]
pub type RXF0 = crate::Reg<u32, _RXF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0;
#[doc = "`read()` method returns [rxf0::R](rxf0::R) reader structure"]
impl crate::Readable for RXF0 {}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
pub mod rxf0;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1](rxf1) module"]
pub type RXF1 = crate::Reg<u32, _RXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1;
#[doc = "`read()` method returns [rxf1::R](rxf1::R) reader structure"]
impl crate::Readable for RXF1 {}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
pub mod rxf1;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf2](rxf2) module"]
pub type RXF2 = crate::Reg<u32, _RXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF2;
#[doc = "`read()` method returns [rxf2::R](rxf2::R) reader structure"]
impl crate::Readable for RXF2 {}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
pub mod rxf2;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf3](rxf3) module"]
pub type RXF3 = crate::Reg<u32, _RXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF3;
#[doc = "`read()` method returns [rxf3::R](rxf3::R) reader structure"]
impl crate::Readable for RXF3 {}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
pub mod rxf3;
#[doc = "Interrupt request register. Write 1 to clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq](irq) module"]
pub type IRQ = crate::Reg<u32, _IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ;
#[doc = "`read()` method returns [irq::R](irq::R) reader structure"]
impl crate::Readable for IRQ {}
#[doc = "`write(|w| ..)` method takes [irq::W](irq::W) writer structure"]
impl crate::Writable for IRQ {}
#[doc = "Interrupt request register. Write 1 to clear"]
pub mod irq;
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ.\\n Note this is different to the INTF register: writing here affects PIO internal\\n state. INTF just asserts the processor-facing IRQ signal for testing ISRs,\\n and is not visible to the state machines.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_force](irq_force) module"]
pub type IRQ_FORCE = crate::Reg<u32, _IRQ_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_FORCE;
#[doc = "`write(|w| ..)` method takes [irq_force::W](irq_force::W) writer structure"]
impl crate::Writable for IRQ_FORCE {}
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ.\\n Note this is different to the INTF register: writing here affects PIO internal\\n state. INTF just asserts the processor-facing IRQ signal for testing ISRs,\\n and is not visible to the state machines."]
pub mod irq_force;
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects\\n PIO logic from metastabilities. This increases input delay, and for fast\\n synchronous IO (e.g. SPI) these synchronizers may need to be bypassed.\\n Each bit in this register corresponds to one GPIO.\\n 0 -> input is synchronized (default)\\n 1 -> synchronizer is bypassed\\n If in doubt, leave this register as all zeroes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input_sync_bypass](input_sync_bypass) module"]
pub type INPUT_SYNC_BYPASS = crate::Reg<u32, _INPUT_SYNC_BYPASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUT_SYNC_BYPASS;
#[doc = "`read()` method returns [input_sync_bypass::R](input_sync_bypass::R) reader structure"]
impl crate::Readable for INPUT_SYNC_BYPASS {}
#[doc = "`write(|w| ..)` method takes [input_sync_bypass::W](input_sync_bypass::W) writer structure"]
impl crate::Writable for INPUT_SYNC_BYPASS {}
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects\\n PIO logic from metastabilities. This increases input delay, and for fast\\n synchronous IO (e.g. SPI) these synchronizers may need to be bypassed.\\n Each bit in this register corresponds to one GPIO.\\n 0 -> input is synchronized (default)\\n 1 -> synchronizer is bypassed\\n If in doubt, leave this register as all zeroes."]
pub mod input_sync_bypass;
#[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_padout](dbg_padout) module"]
pub type DBG_PADOUT = crate::Reg<u32, _DBG_PADOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_PADOUT;
#[doc = "`read()` method returns [dbg_padout::R](dbg_padout::R) reader structure"]
impl crate::Readable for DBG_PADOUT {}
#[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs."]
pub mod dbg_padout;
#[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_padoe](dbg_padoe) module"]
pub type DBG_PADOE = crate::Reg<u32, _DBG_PADOE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_PADOE;
#[doc = "`read()` method returns [dbg_padoe::R](dbg_padoe::R) reader structure"]
impl crate::Readable for DBG_PADOE {}
#[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs."]
pub mod dbg_padoe;
#[doc = "The PIO hardware has some free parameters that may vary between chip products.\\n These should be provided in the chip datasheet, but are also exposed here.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_cfginfo](dbg_cfginfo) module"]
pub type DBG_CFGINFO = crate::Reg<u32, _DBG_CFGINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_CFGINFO;
#[doc = "`read()` method returns [dbg_cfginfo::R](dbg_cfginfo::R) reader structure"]
impl crate::Readable for DBG_CFGINFO {}
#[doc = "The PIO hardware has some free parameters that may vary between chip products.\\n These should be provided in the chip datasheet, but are also exposed here."]
pub mod dbg_cfginfo;
#[doc = "Write-only access to instruction memory location 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem0](instr_mem0) module"]
pub type INSTR_MEM0 = crate::Reg<u32, _INSTR_MEM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM0;
#[doc = "`read()` method returns [instr_mem0::R](instr_mem0::R) reader structure"]
impl crate::Readable for INSTR_MEM0 {}
#[doc = "`write(|w| ..)` method takes [instr_mem0::W](instr_mem0::W) writer structure"]
impl crate::Writable for INSTR_MEM0 {}
#[doc = "Write-only access to instruction memory location 0"]
pub mod instr_mem0;
#[doc = "Write-only access to instruction memory location 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem1](instr_mem1) module"]
pub type INSTR_MEM1 = crate::Reg<u32, _INSTR_MEM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM1;
#[doc = "`read()` method returns [instr_mem1::R](instr_mem1::R) reader structure"]
impl crate::Readable for INSTR_MEM1 {}
#[doc = "`write(|w| ..)` method takes [instr_mem1::W](instr_mem1::W) writer structure"]
impl crate::Writable for INSTR_MEM1 {}
#[doc = "Write-only access to instruction memory location 1"]
pub mod instr_mem1;
#[doc = "Write-only access to instruction memory location 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem2](instr_mem2) module"]
pub type INSTR_MEM2 = crate::Reg<u32, _INSTR_MEM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM2;
#[doc = "`read()` method returns [instr_mem2::R](instr_mem2::R) reader structure"]
impl crate::Readable for INSTR_MEM2 {}
#[doc = "`write(|w| ..)` method takes [instr_mem2::W](instr_mem2::W) writer structure"]
impl crate::Writable for INSTR_MEM2 {}
#[doc = "Write-only access to instruction memory location 2"]
pub mod instr_mem2;
#[doc = "Write-only access to instruction memory location 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem3](instr_mem3) module"]
pub type INSTR_MEM3 = crate::Reg<u32, _INSTR_MEM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM3;
#[doc = "`read()` method returns [instr_mem3::R](instr_mem3::R) reader structure"]
impl crate::Readable for INSTR_MEM3 {}
#[doc = "`write(|w| ..)` method takes [instr_mem3::W](instr_mem3::W) writer structure"]
impl crate::Writable for INSTR_MEM3 {}
#[doc = "Write-only access to instruction memory location 3"]
pub mod instr_mem3;
#[doc = "Write-only access to instruction memory location 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem4](instr_mem4) module"]
pub type INSTR_MEM4 = crate::Reg<u32, _INSTR_MEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM4;
#[doc = "`read()` method returns [instr_mem4::R](instr_mem4::R) reader structure"]
impl crate::Readable for INSTR_MEM4 {}
#[doc = "`write(|w| ..)` method takes [instr_mem4::W](instr_mem4::W) writer structure"]
impl crate::Writable for INSTR_MEM4 {}
#[doc = "Write-only access to instruction memory location 4"]
pub mod instr_mem4;
#[doc = "Write-only access to instruction memory location 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem5](instr_mem5) module"]
pub type INSTR_MEM5 = crate::Reg<u32, _INSTR_MEM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM5;
#[doc = "`read()` method returns [instr_mem5::R](instr_mem5::R) reader structure"]
impl crate::Readable for INSTR_MEM5 {}
#[doc = "`write(|w| ..)` method takes [instr_mem5::W](instr_mem5::W) writer structure"]
impl crate::Writable for INSTR_MEM5 {}
#[doc = "Write-only access to instruction memory location 5"]
pub mod instr_mem5;
#[doc = "Write-only access to instruction memory location 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem6](instr_mem6) module"]
pub type INSTR_MEM6 = crate::Reg<u32, _INSTR_MEM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM6;
#[doc = "`read()` method returns [instr_mem6::R](instr_mem6::R) reader structure"]
impl crate::Readable for INSTR_MEM6 {}
#[doc = "`write(|w| ..)` method takes [instr_mem6::W](instr_mem6::W) writer structure"]
impl crate::Writable for INSTR_MEM6 {}
#[doc = "Write-only access to instruction memory location 6"]
pub mod instr_mem6;
#[doc = "Write-only access to instruction memory location 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem7](instr_mem7) module"]
pub type INSTR_MEM7 = crate::Reg<u32, _INSTR_MEM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM7;
#[doc = "`read()` method returns [instr_mem7::R](instr_mem7::R) reader structure"]
impl crate::Readable for INSTR_MEM7 {}
#[doc = "`write(|w| ..)` method takes [instr_mem7::W](instr_mem7::W) writer structure"]
impl crate::Writable for INSTR_MEM7 {}
#[doc = "Write-only access to instruction memory location 7"]
pub mod instr_mem7;
#[doc = "Write-only access to instruction memory location 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem8](instr_mem8) module"]
pub type INSTR_MEM8 = crate::Reg<u32, _INSTR_MEM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM8;
#[doc = "`read()` method returns [instr_mem8::R](instr_mem8::R) reader structure"]
impl crate::Readable for INSTR_MEM8 {}
#[doc = "`write(|w| ..)` method takes [instr_mem8::W](instr_mem8::W) writer structure"]
impl crate::Writable for INSTR_MEM8 {}
#[doc = "Write-only access to instruction memory location 8"]
pub mod instr_mem8;
#[doc = "Write-only access to instruction memory location 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem9](instr_mem9) module"]
pub type INSTR_MEM9 = crate::Reg<u32, _INSTR_MEM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM9;
#[doc = "`read()` method returns [instr_mem9::R](instr_mem9::R) reader structure"]
impl crate::Readable for INSTR_MEM9 {}
#[doc = "`write(|w| ..)` method takes [instr_mem9::W](instr_mem9::W) writer structure"]
impl crate::Writable for INSTR_MEM9 {}
#[doc = "Write-only access to instruction memory location 9"]
pub mod instr_mem9;
#[doc = "Write-only access to instruction memory location 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem10](instr_mem10) module"]
pub type INSTR_MEM10 = crate::Reg<u32, _INSTR_MEM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM10;
#[doc = "`read()` method returns [instr_mem10::R](instr_mem10::R) reader structure"]
impl crate::Readable for INSTR_MEM10 {}
#[doc = "`write(|w| ..)` method takes [instr_mem10::W](instr_mem10::W) writer structure"]
impl crate::Writable for INSTR_MEM10 {}
#[doc = "Write-only access to instruction memory location 10"]
pub mod instr_mem10;
#[doc = "Write-only access to instruction memory location 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem11](instr_mem11) module"]
pub type INSTR_MEM11 = crate::Reg<u32, _INSTR_MEM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM11;
#[doc = "`read()` method returns [instr_mem11::R](instr_mem11::R) reader structure"]
impl crate::Readable for INSTR_MEM11 {}
#[doc = "`write(|w| ..)` method takes [instr_mem11::W](instr_mem11::W) writer structure"]
impl crate::Writable for INSTR_MEM11 {}
#[doc = "Write-only access to instruction memory location 11"]
pub mod instr_mem11;
#[doc = "Write-only access to instruction memory location 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem12](instr_mem12) module"]
pub type INSTR_MEM12 = crate::Reg<u32, _INSTR_MEM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM12;
#[doc = "`read()` method returns [instr_mem12::R](instr_mem12::R) reader structure"]
impl crate::Readable for INSTR_MEM12 {}
#[doc = "`write(|w| ..)` method takes [instr_mem12::W](instr_mem12::W) writer structure"]
impl crate::Writable for INSTR_MEM12 {}
#[doc = "Write-only access to instruction memory location 12"]
pub mod instr_mem12;
#[doc = "Write-only access to instruction memory location 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem13](instr_mem13) module"]
pub type INSTR_MEM13 = crate::Reg<u32, _INSTR_MEM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM13;
#[doc = "`read()` method returns [instr_mem13::R](instr_mem13::R) reader structure"]
impl crate::Readable for INSTR_MEM13 {}
#[doc = "`write(|w| ..)` method takes [instr_mem13::W](instr_mem13::W) writer structure"]
impl crate::Writable for INSTR_MEM13 {}
#[doc = "Write-only access to instruction memory location 13"]
pub mod instr_mem13;
#[doc = "Write-only access to instruction memory location 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem14](instr_mem14) module"]
pub type INSTR_MEM14 = crate::Reg<u32, _INSTR_MEM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM14;
#[doc = "`read()` method returns [instr_mem14::R](instr_mem14::R) reader structure"]
impl crate::Readable for INSTR_MEM14 {}
#[doc = "`write(|w| ..)` method takes [instr_mem14::W](instr_mem14::W) writer structure"]
impl crate::Writable for INSTR_MEM14 {}
#[doc = "Write-only access to instruction memory location 14"]
pub mod instr_mem14;
#[doc = "Write-only access to instruction memory location 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem15](instr_mem15) module"]
pub type INSTR_MEM15 = crate::Reg<u32, _INSTR_MEM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM15;
#[doc = "`read()` method returns [instr_mem15::R](instr_mem15::R) reader structure"]
impl crate::Readable for INSTR_MEM15 {}
#[doc = "`write(|w| ..)` method takes [instr_mem15::W](instr_mem15::W) writer structure"]
impl crate::Writable for INSTR_MEM15 {}
#[doc = "Write-only access to instruction memory location 15"]
pub mod instr_mem15;
#[doc = "Write-only access to instruction memory location 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem16](instr_mem16) module"]
pub type INSTR_MEM16 = crate::Reg<u32, _INSTR_MEM16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM16;
#[doc = "`read()` method returns [instr_mem16::R](instr_mem16::R) reader structure"]
impl crate::Readable for INSTR_MEM16 {}
#[doc = "`write(|w| ..)` method takes [instr_mem16::W](instr_mem16::W) writer structure"]
impl crate::Writable for INSTR_MEM16 {}
#[doc = "Write-only access to instruction memory location 16"]
pub mod instr_mem16;
#[doc = "Write-only access to instruction memory location 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem17](instr_mem17) module"]
pub type INSTR_MEM17 = crate::Reg<u32, _INSTR_MEM17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM17;
#[doc = "`read()` method returns [instr_mem17::R](instr_mem17::R) reader structure"]
impl crate::Readable for INSTR_MEM17 {}
#[doc = "`write(|w| ..)` method takes [instr_mem17::W](instr_mem17::W) writer structure"]
impl crate::Writable for INSTR_MEM17 {}
#[doc = "Write-only access to instruction memory location 17"]
pub mod instr_mem17;
#[doc = "Write-only access to instruction memory location 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem18](instr_mem18) module"]
pub type INSTR_MEM18 = crate::Reg<u32, _INSTR_MEM18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM18;
#[doc = "`read()` method returns [instr_mem18::R](instr_mem18::R) reader structure"]
impl crate::Readable for INSTR_MEM18 {}
#[doc = "`write(|w| ..)` method takes [instr_mem18::W](instr_mem18::W) writer structure"]
impl crate::Writable for INSTR_MEM18 {}
#[doc = "Write-only access to instruction memory location 18"]
pub mod instr_mem18;
#[doc = "Write-only access to instruction memory location 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem19](instr_mem19) module"]
pub type INSTR_MEM19 = crate::Reg<u32, _INSTR_MEM19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM19;
#[doc = "`read()` method returns [instr_mem19::R](instr_mem19::R) reader structure"]
impl crate::Readable for INSTR_MEM19 {}
#[doc = "`write(|w| ..)` method takes [instr_mem19::W](instr_mem19::W) writer structure"]
impl crate::Writable for INSTR_MEM19 {}
#[doc = "Write-only access to instruction memory location 19"]
pub mod instr_mem19;
#[doc = "Write-only access to instruction memory location 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem20](instr_mem20) module"]
pub type INSTR_MEM20 = crate::Reg<u32, _INSTR_MEM20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM20;
#[doc = "`read()` method returns [instr_mem20::R](instr_mem20::R) reader structure"]
impl crate::Readable for INSTR_MEM20 {}
#[doc = "`write(|w| ..)` method takes [instr_mem20::W](instr_mem20::W) writer structure"]
impl crate::Writable for INSTR_MEM20 {}
#[doc = "Write-only access to instruction memory location 20"]
pub mod instr_mem20;
#[doc = "Write-only access to instruction memory location 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem21](instr_mem21) module"]
pub type INSTR_MEM21 = crate::Reg<u32, _INSTR_MEM21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM21;
#[doc = "`read()` method returns [instr_mem21::R](instr_mem21::R) reader structure"]
impl crate::Readable for INSTR_MEM21 {}
#[doc = "`write(|w| ..)` method takes [instr_mem21::W](instr_mem21::W) writer structure"]
impl crate::Writable for INSTR_MEM21 {}
#[doc = "Write-only access to instruction memory location 21"]
pub mod instr_mem21;
#[doc = "Write-only access to instruction memory location 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem22](instr_mem22) module"]
pub type INSTR_MEM22 = crate::Reg<u32, _INSTR_MEM22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM22;
#[doc = "`read()` method returns [instr_mem22::R](instr_mem22::R) reader structure"]
impl crate::Readable for INSTR_MEM22 {}
#[doc = "`write(|w| ..)` method takes [instr_mem22::W](instr_mem22::W) writer structure"]
impl crate::Writable for INSTR_MEM22 {}
#[doc = "Write-only access to instruction memory location 22"]
pub mod instr_mem22;
#[doc = "Write-only access to instruction memory location 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem23](instr_mem23) module"]
pub type INSTR_MEM23 = crate::Reg<u32, _INSTR_MEM23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM23;
#[doc = "`read()` method returns [instr_mem23::R](instr_mem23::R) reader structure"]
impl crate::Readable for INSTR_MEM23 {}
#[doc = "`write(|w| ..)` method takes [instr_mem23::W](instr_mem23::W) writer structure"]
impl crate::Writable for INSTR_MEM23 {}
#[doc = "Write-only access to instruction memory location 23"]
pub mod instr_mem23;
#[doc = "Write-only access to instruction memory location 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem24](instr_mem24) module"]
pub type INSTR_MEM24 = crate::Reg<u32, _INSTR_MEM24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM24;
#[doc = "`read()` method returns [instr_mem24::R](instr_mem24::R) reader structure"]
impl crate::Readable for INSTR_MEM24 {}
#[doc = "`write(|w| ..)` method takes [instr_mem24::W](instr_mem24::W) writer structure"]
impl crate::Writable for INSTR_MEM24 {}
#[doc = "Write-only access to instruction memory location 24"]
pub mod instr_mem24;
#[doc = "Write-only access to instruction memory location 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem25](instr_mem25) module"]
pub type INSTR_MEM25 = crate::Reg<u32, _INSTR_MEM25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM25;
#[doc = "`read()` method returns [instr_mem25::R](instr_mem25::R) reader structure"]
impl crate::Readable for INSTR_MEM25 {}
#[doc = "`write(|w| ..)` method takes [instr_mem25::W](instr_mem25::W) writer structure"]
impl crate::Writable for INSTR_MEM25 {}
#[doc = "Write-only access to instruction memory location 25"]
pub mod instr_mem25;
#[doc = "Write-only access to instruction memory location 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem26](instr_mem26) module"]
pub type INSTR_MEM26 = crate::Reg<u32, _INSTR_MEM26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM26;
#[doc = "`read()` method returns [instr_mem26::R](instr_mem26::R) reader structure"]
impl crate::Readable for INSTR_MEM26 {}
#[doc = "`write(|w| ..)` method takes [instr_mem26::W](instr_mem26::W) writer structure"]
impl crate::Writable for INSTR_MEM26 {}
#[doc = "Write-only access to instruction memory location 26"]
pub mod instr_mem26;
#[doc = "Write-only access to instruction memory location 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem27](instr_mem27) module"]
pub type INSTR_MEM27 = crate::Reg<u32, _INSTR_MEM27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM27;
#[doc = "`read()` method returns [instr_mem27::R](instr_mem27::R) reader structure"]
impl crate::Readable for INSTR_MEM27 {}
#[doc = "`write(|w| ..)` method takes [instr_mem27::W](instr_mem27::W) writer structure"]
impl crate::Writable for INSTR_MEM27 {}
#[doc = "Write-only access to instruction memory location 27"]
pub mod instr_mem27;
#[doc = "Write-only access to instruction memory location 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem28](instr_mem28) module"]
pub type INSTR_MEM28 = crate::Reg<u32, _INSTR_MEM28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM28;
#[doc = "`read()` method returns [instr_mem28::R](instr_mem28::R) reader structure"]
impl crate::Readable for INSTR_MEM28 {}
#[doc = "`write(|w| ..)` method takes [instr_mem28::W](instr_mem28::W) writer structure"]
impl crate::Writable for INSTR_MEM28 {}
#[doc = "Write-only access to instruction memory location 28"]
pub mod instr_mem28;
#[doc = "Write-only access to instruction memory location 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem29](instr_mem29) module"]
pub type INSTR_MEM29 = crate::Reg<u32, _INSTR_MEM29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM29;
#[doc = "`read()` method returns [instr_mem29::R](instr_mem29::R) reader structure"]
impl crate::Readable for INSTR_MEM29 {}
#[doc = "`write(|w| ..)` method takes [instr_mem29::W](instr_mem29::W) writer structure"]
impl crate::Writable for INSTR_MEM29 {}
#[doc = "Write-only access to instruction memory location 29"]
pub mod instr_mem29;
#[doc = "Write-only access to instruction memory location 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem30](instr_mem30) module"]
pub type INSTR_MEM30 = crate::Reg<u32, _INSTR_MEM30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM30;
#[doc = "`read()` method returns [instr_mem30::R](instr_mem30::R) reader structure"]
impl crate::Readable for INSTR_MEM30 {}
#[doc = "`write(|w| ..)` method takes [instr_mem30::W](instr_mem30::W) writer structure"]
impl crate::Writable for INSTR_MEM30 {}
#[doc = "Write-only access to instruction memory location 30"]
pub mod instr_mem30;
#[doc = "Write-only access to instruction memory location 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem31](instr_mem31) module"]
pub type INSTR_MEM31 = crate::Reg<u32, _INSTR_MEM31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM31;
#[doc = "`read()` method returns [instr_mem31::R](instr_mem31::R) reader structure"]
impl crate::Readable for INSTR_MEM31 {}
#[doc = "`write(|w| ..)` method takes [instr_mem31::W](instr_mem31::W) writer structure"]
impl crate::Writable for INSTR_MEM31 {}
#[doc = "Write-only access to instruction memory location 31"]
pub mod instr_mem31;
#[doc = "Clock divider register for state machine 0\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm0_clkdiv](sm0_clkdiv) module"]
pub type SM0_CLKDIV = crate::Reg<u32, _SM0_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0_CLKDIV;
#[doc = "`read()` method returns [sm0_clkdiv::R](sm0_clkdiv::R) reader structure"]
impl crate::Readable for SM0_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [sm0_clkdiv::W](sm0_clkdiv::W) writer structure"]
impl crate::Writable for SM0_CLKDIV {}
#[doc = "Clock divider register for state machine 0\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm0_clkdiv;
#[doc = "Execution/behavioural settings for state machine 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm0_execctrl](sm0_execctrl) module"]
pub type SM0_EXECCTRL = crate::Reg<u32, _SM0_EXECCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0_EXECCTRL;
#[doc = "`read()` method returns [sm0_execctrl::R](sm0_execctrl::R) reader structure"]
impl crate::Readable for SM0_EXECCTRL {}
#[doc = "`write(|w| ..)` method takes [sm0_execctrl::W](sm0_execctrl::W) writer structure"]
impl crate::Writable for SM0_EXECCTRL {}
#[doc = "Execution/behavioural settings for state machine 0"]
pub mod sm0_execctrl;
#[doc = "Control behaviour of the input/output shift registers for state machine 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm0_shiftctrl](sm0_shiftctrl) module"]
pub type SM0_SHIFTCTRL = crate::Reg<u32, _SM0_SHIFTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0_SHIFTCTRL;
#[doc = "`read()` method returns [sm0_shiftctrl::R](sm0_shiftctrl::R) reader structure"]
impl crate::Readable for SM0_SHIFTCTRL {}
#[doc = "`write(|w| ..)` method takes [sm0_shiftctrl::W](sm0_shiftctrl::W) writer structure"]
impl crate::Writable for SM0_SHIFTCTRL {}
#[doc = "Control behaviour of the input/output shift registers for state machine 0"]
pub mod sm0_shiftctrl;
#[doc = "Current instruction address of state machine 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm0_addr](sm0_addr) module"]
pub type SM0_ADDR = crate::Reg<u32, _SM0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0_ADDR;
#[doc = "`read()` method returns [sm0_addr::R](sm0_addr::R) reader structure"]
impl crate::Readable for SM0_ADDR {}
#[doc = "Current instruction address of state machine 0"]
pub mod sm0_addr;
#[doc = "Instruction currently being executed by state machine 0\\n Write to execute an instruction immediately (including jumps) and then resume execution.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm0_instr](sm0_instr) module"]
pub type SM0_INSTR = crate::Reg<u32, _SM0_INSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0_INSTR;
#[doc = "`read()` method returns [sm0_instr::R](sm0_instr::R) reader structure"]
impl crate::Readable for SM0_INSTR {}
#[doc = "`write(|w| ..)` method takes [sm0_instr::W](sm0_instr::W) writer structure"]
impl crate::Writable for SM0_INSTR {}
#[doc = "Instruction currently being executed by state machine 0\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm0_instr;
#[doc = "State machine pin control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm0_pinctrl](sm0_pinctrl) module"]
pub type SM0_PINCTRL = crate::Reg<u32, _SM0_PINCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0_PINCTRL;
#[doc = "`read()` method returns [sm0_pinctrl::R](sm0_pinctrl::R) reader structure"]
impl crate::Readable for SM0_PINCTRL {}
#[doc = "`write(|w| ..)` method takes [sm0_pinctrl::W](sm0_pinctrl::W) writer structure"]
impl crate::Writable for SM0_PINCTRL {}
#[doc = "State machine pin control"]
pub mod sm0_pinctrl;
#[doc = "Clock divider register for state machine 1\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_clkdiv](sm1_clkdiv) module"]
pub type SM1_CLKDIV = crate::Reg<u32, _SM1_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1_CLKDIV;
#[doc = "`read()` method returns [sm1_clkdiv::R](sm1_clkdiv::R) reader structure"]
impl crate::Readable for SM1_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [sm1_clkdiv::W](sm1_clkdiv::W) writer structure"]
impl crate::Writable for SM1_CLKDIV {}
#[doc = "Clock divider register for state machine 1\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm1_clkdiv;
#[doc = "Execution/behavioural settings for state machine 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_execctrl](sm1_execctrl) module"]
pub type SM1_EXECCTRL = crate::Reg<u32, _SM1_EXECCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1_EXECCTRL;
#[doc = "`read()` method returns [sm1_execctrl::R](sm1_execctrl::R) reader structure"]
impl crate::Readable for SM1_EXECCTRL {}
#[doc = "`write(|w| ..)` method takes [sm1_execctrl::W](sm1_execctrl::W) writer structure"]
impl crate::Writable for SM1_EXECCTRL {}
#[doc = "Execution/behavioural settings for state machine 1"]
pub mod sm1_execctrl;
#[doc = "Control behaviour of the input/output shift registers for state machine 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_shiftctrl](sm1_shiftctrl) module"]
pub type SM1_SHIFTCTRL = crate::Reg<u32, _SM1_SHIFTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1_SHIFTCTRL;
#[doc = "`read()` method returns [sm1_shiftctrl::R](sm1_shiftctrl::R) reader structure"]
impl crate::Readable for SM1_SHIFTCTRL {}
#[doc = "`write(|w| ..)` method takes [sm1_shiftctrl::W](sm1_shiftctrl::W) writer structure"]
impl crate::Writable for SM1_SHIFTCTRL {}
#[doc = "Control behaviour of the input/output shift registers for state machine 1"]
pub mod sm1_shiftctrl;
#[doc = "Current instruction address of state machine 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_addr](sm1_addr) module"]
pub type SM1_ADDR = crate::Reg<u32, _SM1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1_ADDR;
#[doc = "`read()` method returns [sm1_addr::R](sm1_addr::R) reader structure"]
impl crate::Readable for SM1_ADDR {}
#[doc = "Current instruction address of state machine 1"]
pub mod sm1_addr;
#[doc = "Instruction currently being executed by state machine 1\\n Write to execute an instruction immediately (including jumps) and then resume execution.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_instr](sm1_instr) module"]
pub type SM1_INSTR = crate::Reg<u32, _SM1_INSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1_INSTR;
#[doc = "`read()` method returns [sm1_instr::R](sm1_instr::R) reader structure"]
impl crate::Readable for SM1_INSTR {}
#[doc = "`write(|w| ..)` method takes [sm1_instr::W](sm1_instr::W) writer structure"]
impl crate::Writable for SM1_INSTR {}
#[doc = "Instruction currently being executed by state machine 1\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm1_instr;
#[doc = "State machine pin control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_pinctrl](sm1_pinctrl) module"]
pub type SM1_PINCTRL = crate::Reg<u32, _SM1_PINCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1_PINCTRL;
#[doc = "`read()` method returns [sm1_pinctrl::R](sm1_pinctrl::R) reader structure"]
impl crate::Readable for SM1_PINCTRL {}
#[doc = "`write(|w| ..)` method takes [sm1_pinctrl::W](sm1_pinctrl::W) writer structure"]
impl crate::Writable for SM1_PINCTRL {}
#[doc = "State machine pin control"]
pub mod sm1_pinctrl;
#[doc = "Clock divider register for state machine 2\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm2_clkdiv](sm2_clkdiv) module"]
pub type SM2_CLKDIV = crate::Reg<u32, _SM2_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2_CLKDIV;
#[doc = "`read()` method returns [sm2_clkdiv::R](sm2_clkdiv::R) reader structure"]
impl crate::Readable for SM2_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [sm2_clkdiv::W](sm2_clkdiv::W) writer structure"]
impl crate::Writable for SM2_CLKDIV {}
#[doc = "Clock divider register for state machine 2\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm2_clkdiv;
#[doc = "Execution/behavioural settings for state machine 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm2_execctrl](sm2_execctrl) module"]
pub type SM2_EXECCTRL = crate::Reg<u32, _SM2_EXECCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2_EXECCTRL;
#[doc = "`read()` method returns [sm2_execctrl::R](sm2_execctrl::R) reader structure"]
impl crate::Readable for SM2_EXECCTRL {}
#[doc = "`write(|w| ..)` method takes [sm2_execctrl::W](sm2_execctrl::W) writer structure"]
impl crate::Writable for SM2_EXECCTRL {}
#[doc = "Execution/behavioural settings for state machine 2"]
pub mod sm2_execctrl;
#[doc = "Control behaviour of the input/output shift registers for state machine 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm2_shiftctrl](sm2_shiftctrl) module"]
pub type SM2_SHIFTCTRL = crate::Reg<u32, _SM2_SHIFTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2_SHIFTCTRL;
#[doc = "`read()` method returns [sm2_shiftctrl::R](sm2_shiftctrl::R) reader structure"]
impl crate::Readable for SM2_SHIFTCTRL {}
#[doc = "`write(|w| ..)` method takes [sm2_shiftctrl::W](sm2_shiftctrl::W) writer structure"]
impl crate::Writable for SM2_SHIFTCTRL {}
#[doc = "Control behaviour of the input/output shift registers for state machine 2"]
pub mod sm2_shiftctrl;
#[doc = "Current instruction address of state machine 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm2_addr](sm2_addr) module"]
pub type SM2_ADDR = crate::Reg<u32, _SM2_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2_ADDR;
#[doc = "`read()` method returns [sm2_addr::R](sm2_addr::R) reader structure"]
impl crate::Readable for SM2_ADDR {}
#[doc = "Current instruction address of state machine 2"]
pub mod sm2_addr;
#[doc = "Instruction currently being executed by state machine 2\\n Write to execute an instruction immediately (including jumps) and then resume execution.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm2_instr](sm2_instr) module"]
pub type SM2_INSTR = crate::Reg<u32, _SM2_INSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2_INSTR;
#[doc = "`read()` method returns [sm2_instr::R](sm2_instr::R) reader structure"]
impl crate::Readable for SM2_INSTR {}
#[doc = "`write(|w| ..)` method takes [sm2_instr::W](sm2_instr::W) writer structure"]
impl crate::Writable for SM2_INSTR {}
#[doc = "Instruction currently being executed by state machine 2\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm2_instr;
#[doc = "State machine pin control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm2_pinctrl](sm2_pinctrl) module"]
pub type SM2_PINCTRL = crate::Reg<u32, _SM2_PINCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2_PINCTRL;
#[doc = "`read()` method returns [sm2_pinctrl::R](sm2_pinctrl::R) reader structure"]
impl crate::Readable for SM2_PINCTRL {}
#[doc = "`write(|w| ..)` method takes [sm2_pinctrl::W](sm2_pinctrl::W) writer structure"]
impl crate::Writable for SM2_PINCTRL {}
#[doc = "State machine pin control"]
pub mod sm2_pinctrl;
#[doc = "Clock divider register for state machine 3\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm3_clkdiv](sm3_clkdiv) module"]
pub type SM3_CLKDIV = crate::Reg<u32, _SM3_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3_CLKDIV;
#[doc = "`read()` method returns [sm3_clkdiv::R](sm3_clkdiv::R) reader structure"]
impl crate::Readable for SM3_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [sm3_clkdiv::W](sm3_clkdiv::W) writer structure"]
impl crate::Writable for SM3_CLKDIV {}
#[doc = "Clock divider register for state machine 3\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm3_clkdiv;
#[doc = "Execution/behavioural settings for state machine 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm3_execctrl](sm3_execctrl) module"]
pub type SM3_EXECCTRL = crate::Reg<u32, _SM3_EXECCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3_EXECCTRL;
#[doc = "`read()` method returns [sm3_execctrl::R](sm3_execctrl::R) reader structure"]
impl crate::Readable for SM3_EXECCTRL {}
#[doc = "`write(|w| ..)` method takes [sm3_execctrl::W](sm3_execctrl::W) writer structure"]
impl crate::Writable for SM3_EXECCTRL {}
#[doc = "Execution/behavioural settings for state machine 3"]
pub mod sm3_execctrl;
#[doc = "Control behaviour of the input/output shift registers for state machine 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm3_shiftctrl](sm3_shiftctrl) module"]
pub type SM3_SHIFTCTRL = crate::Reg<u32, _SM3_SHIFTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3_SHIFTCTRL;
#[doc = "`read()` method returns [sm3_shiftctrl::R](sm3_shiftctrl::R) reader structure"]
impl crate::Readable for SM3_SHIFTCTRL {}
#[doc = "`write(|w| ..)` method takes [sm3_shiftctrl::W](sm3_shiftctrl::W) writer structure"]
impl crate::Writable for SM3_SHIFTCTRL {}
#[doc = "Control behaviour of the input/output shift registers for state machine 3"]
pub mod sm3_shiftctrl;
#[doc = "Current instruction address of state machine 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm3_addr](sm3_addr) module"]
pub type SM3_ADDR = crate::Reg<u32, _SM3_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3_ADDR;
#[doc = "`read()` method returns [sm3_addr::R](sm3_addr::R) reader structure"]
impl crate::Readable for SM3_ADDR {}
#[doc = "Current instruction address of state machine 3"]
pub mod sm3_addr;
#[doc = "Instruction currently being executed by state machine 3\\n Write to execute an instruction immediately (including jumps) and then resume execution.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm3_instr](sm3_instr) module"]
pub type SM3_INSTR = crate::Reg<u32, _SM3_INSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3_INSTR;
#[doc = "`read()` method returns [sm3_instr::R](sm3_instr::R) reader structure"]
impl crate::Readable for SM3_INSTR {}
#[doc = "`write(|w| ..)` method takes [sm3_instr::W](sm3_instr::W) writer structure"]
impl crate::Writable for SM3_INSTR {}
#[doc = "Instruction currently being executed by state machine 3\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm3_instr;
#[doc = "State machine pin control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm3_pinctrl](sm3_pinctrl) module"]
pub type SM3_PINCTRL = crate::Reg<u32, _SM3_PINCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3_PINCTRL;
#[doc = "`read()` method returns [sm3_pinctrl::R](sm3_pinctrl::R) reader structure"]
impl crate::Readable for SM3_PINCTRL {}
#[doc = "`write(|w| ..)` method takes [sm3_pinctrl::W](sm3_pinctrl::W) writer structure"]
impl crate::Writable for SM3_PINCTRL {}
#[doc = "State machine pin control"]
pub mod sm3_pinctrl;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "Interrupt Enable for irq0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq0_inte](irq0_inte) module"]
pub type IRQ0_INTE = crate::Reg<u32, _IRQ0_INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ0_INTE;
#[doc = "`read()` method returns [irq0_inte::R](irq0_inte::R) reader structure"]
impl crate::Readable for IRQ0_INTE {}
#[doc = "`write(|w| ..)` method takes [irq0_inte::W](irq0_inte::W) writer structure"]
impl crate::Writable for IRQ0_INTE {}
#[doc = "Interrupt Enable for irq0"]
pub mod irq0_inte;
#[doc = "Interrupt Force for irq0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq0_intf](irq0_intf) module"]
pub type IRQ0_INTF = crate::Reg<u32, _IRQ0_INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ0_INTF;
#[doc = "`read()` method returns [irq0_intf::R](irq0_intf::R) reader structure"]
impl crate::Readable for IRQ0_INTF {}
#[doc = "`write(|w| ..)` method takes [irq0_intf::W](irq0_intf::W) writer structure"]
impl crate::Writable for IRQ0_INTF {}
#[doc = "Interrupt Force for irq0"]
pub mod irq0_intf;
#[doc = "Interrupt status after masking & forcing for irq0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq0_ints](irq0_ints) module"]
pub type IRQ0_INTS = crate::Reg<u32, _IRQ0_INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ0_INTS;
#[doc = "`read()` method returns [irq0_ints::R](irq0_ints::R) reader structure"]
impl crate::Readable for IRQ0_INTS {}
#[doc = "Interrupt status after masking & forcing for irq0"]
pub mod irq0_ints;
#[doc = "Interrupt Enable for irq1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq1_inte](irq1_inte) module"]
pub type IRQ1_INTE = crate::Reg<u32, _IRQ1_INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ1_INTE;
#[doc = "`read()` method returns [irq1_inte::R](irq1_inte::R) reader structure"]
impl crate::Readable for IRQ1_INTE {}
#[doc = "`write(|w| ..)` method takes [irq1_inte::W](irq1_inte::W) writer structure"]
impl crate::Writable for IRQ1_INTE {}
#[doc = "Interrupt Enable for irq1"]
pub mod irq1_inte;
#[doc = "Interrupt Force for irq1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq1_intf](irq1_intf) module"]
pub type IRQ1_INTF = crate::Reg<u32, _IRQ1_INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ1_INTF;
#[doc = "`read()` method returns [irq1_intf::R](irq1_intf::R) reader structure"]
impl crate::Readable for IRQ1_INTF {}
#[doc = "`write(|w| ..)` method takes [irq1_intf::W](irq1_intf::W) writer structure"]
impl crate::Writable for IRQ1_INTF {}
#[doc = "Interrupt Force for irq1"]
pub mod irq1_intf;
#[doc = "Interrupt status after masking & forcing for irq1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq1_ints](irq1_ints) module"]
pub type IRQ1_INTS = crate::Reg<u32, _IRQ1_INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ1_INTS;
#[doc = "`read()` method returns [irq1_ints::R](irq1_ints::R) reader structure"]
impl crate::Readable for IRQ1_INTS {}
#[doc = "Interrupt status after masking & forcing for irq1"]
pub mod irq1_ints;
