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
    #[doc = "0x10 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
    pub txf: [TXF; 4],
    #[doc = "0x20 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
    pub rxf: [RXF; 4],
    #[doc = "0x30 - State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag.\\n\\n Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
    pub irq: IRQ,
    #[doc = "0x34 - Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
    pub irq_force: IRQ_FORCE,
    #[doc = "0x38 - There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO.\\n 0 -> input is synchronized (default)\\n 1 -> synchronizer is bypassed\\n If in doubt, leave this register as all zeroes."]
    pub input_sync_bypass: INPUT_SYNC_BYPASS,
    #[doc = "0x3c - Read to sample the pad output values PIO is currently driving to the GPIOs."]
    pub dbg_padout: DBG_PADOUT,
    #[doc = "0x40 - Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs."]
    pub dbg_padoe: DBG_PADOE,
    #[doc = "0x44 - The PIO hardware has some free parameters that may vary between chip products.\\n These should be provided in the chip datasheet, but are also exposed here."]
    pub dbg_cfginfo: DBG_CFGINFO,
    #[doc = "0x48 - Write-only access to instruction memory location %s"]
    pub instr_mem: [INSTR_MEM; 32],
    #[doc = "0xc8 - Cluster SM%s, containing SM*_CLKDIV, SM*_EXECCTRL, SM*_SHIFTCTRL, SM*_ADDR, SM*_INSTR, SM*_PINCTRL"]
    pub sm: [SM; 4],
    #[doc = "0x128 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0x12c - Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
    pub sm_irq: [SM_IRQ; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SM {
    #[doc = "0x00 - Clock divisor register for state machine 0\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm_clkdiv: self::sm::SM_CLKDIV,
    #[doc = "0x04 - Execution/behavioural settings for state machine 0"]
    pub sm_execctrl: self::sm::SM_EXECCTRL,
    #[doc = "0x08 - Control behaviour of the input/output shift registers for state machine 0"]
    pub sm_shiftctrl: self::sm::SM_SHIFTCTRL,
    #[doc = "0x0c - Current instruction address of state machine 0"]
    pub sm_addr: self::sm::SM_ADDR,
    #[doc = "0x10 - Read to see the instruction currently addressed by state machine 0's program counter\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm_instr: self::sm::SM_INSTR,
    #[doc = "0x14 - State machine pin control"]
    pub sm_pinctrl: self::sm::SM_PINCTRL,
}
#[doc = r"Register block"]
#[doc = "Cluster SM%s, containing SM*_CLKDIV, SM*_EXECCTRL, SM*_SHIFTCTRL, SM*_ADDR, SM*_INSTR, SM*_PINCTRL"]
pub mod sm;
#[doc = r"Register block"]
#[repr(C)]
pub struct SM_IRQ {
    #[doc = "0x00 - Interrupt Enable for irq0"]
    pub irq_inte: self::sm_irq::IRQ_INTE,
    #[doc = "0x04 - Interrupt Force for irq0"]
    pub irq_intf: self::sm_irq::IRQ_INTF,
    #[doc = "0x08 - Interrupt status after masking & forcing for irq0"]
    pub irq_ints: self::sm_irq::IRQ_INTS,
}
#[doc = r"Register block"]
#[doc = "Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
pub mod sm_irq;
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
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf](txf) module"]
pub type TXF = crate::Reg<u32, _TXF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXF;
#[doc = "`write(|w| ..)` method takes [txf::W](txf::W) writer structure"]
impl crate::Writable for TXF {}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
pub mod txf;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf](rxf) module"]
pub type RXF = crate::Reg<u32, _RXF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF;
#[doc = "`read()` method returns [rxf::R](rxf::R) reader structure"]
impl crate::Readable for RXF {}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
pub mod rxf;
#[doc = "State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag.\\n\\n Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq](irq) module"]
pub type IRQ = crate::Reg<u32, _IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ;
#[doc = "`read()` method returns [irq::R](irq::R) reader structure"]
impl crate::Readable for IRQ {}
#[doc = "`write(|w| ..)` method takes [irq::W](irq::W) writer structure"]
impl crate::Writable for IRQ {}
#[doc = "State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag.\\n\\n Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
pub mod irq;
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_force](irq_force) module"]
pub type IRQ_FORCE = crate::Reg<u32, _IRQ_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_FORCE;
#[doc = "`write(|w| ..)` method takes [irq_force::W](irq_force::W) writer structure"]
impl crate::Writable for IRQ_FORCE {}
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
pub mod irq_force;
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO.\\n 0 -> input is synchronized (default)\\n 1 -> synchronizer is bypassed\\n If in doubt, leave this register as all zeroes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input_sync_bypass](input_sync_bypass) module"]
pub type INPUT_SYNC_BYPASS = crate::Reg<u32, _INPUT_SYNC_BYPASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUT_SYNC_BYPASS;
#[doc = "`read()` method returns [input_sync_bypass::R](input_sync_bypass::R) reader structure"]
impl crate::Readable for INPUT_SYNC_BYPASS {}
#[doc = "`write(|w| ..)` method takes [input_sync_bypass::W](input_sync_bypass::W) writer structure"]
impl crate::Writable for INPUT_SYNC_BYPASS {}
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO.\\n 0 -> input is synchronized (default)\\n 1 -> synchronizer is bypassed\\n If in doubt, leave this register as all zeroes."]
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
#[doc = "Write-only access to instruction memory location %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem](instr_mem) module"]
pub type INSTR_MEM = crate::Reg<u32, _INSTR_MEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSTR_MEM;
#[doc = "`read()` method returns [instr_mem::R](instr_mem::R) reader structure"]
impl crate::Readable for INSTR_MEM {}
#[doc = "`write(|w| ..)` method takes [instr_mem::W](instr_mem::W) writer structure"]
impl crate::Writable for INSTR_MEM {}
#[doc = "Write-only access to instruction memory location %s"]
pub mod instr_mem;
#[doc = "Raw Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "Raw Interrupts"]
pub mod intr;
