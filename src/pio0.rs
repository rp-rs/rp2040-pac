#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - FIFO status register"]
    pub fstat: crate::Reg<fstat::FSTAT_SPEC>,
    #[doc = "0x08 - FIFO debug register"]
    pub fdebug: crate::Reg<fdebug::FDEBUG_SPEC>,
    #[doc = "0x0c - FIFO levels"]
    pub flevel: crate::Reg<flevel::FLEVEL_SPEC>,
    #[doc = "0x10..0x20 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
    pub txf: [crate::Reg<txf::TXF_SPEC>; 4],
    #[doc = "0x20..0x30 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
    pub rxf: [crate::Reg<rxf::RXF_SPEC>; 4],
    #[doc = "0x30 - State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag.  

 Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
    pub irq: crate::Reg<irq::IRQ_SPEC>,
    #[doc = "0x34 - Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
    pub irq_force: crate::Reg<irq_force::IRQ_FORCE_SPEC>,
    #[doc = "0x38 - There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO.  
 0 -> input is synchronized (default)  
 1 -> synchronizer is bypassed  
 If in doubt, leave this register as all zeroes."]
    pub input_sync_bypass: crate::Reg<input_sync_bypass::INPUT_SYNC_BYPASS_SPEC>,
    #[doc = "0x3c - Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
    pub dbg_padout: crate::Reg<dbg_padout::DBG_PADOUT_SPEC>,
    #[doc = "0x40 - Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
    pub dbg_padoe: crate::Reg<dbg_padoe::DBG_PADOE_SPEC>,
    #[doc = "0x44 - The PIO hardware has some free parameters that may vary between chip products.  
 These should be provided in the chip datasheet, but are also exposed here."]
    pub dbg_cfginfo: crate::Reg<dbg_cfginfo::DBG_CFGINFO_SPEC>,
    #[doc = "0x48..0xc8 - Write-only access to instruction memory location %s"]
    pub instr_mem: [crate::Reg<instr_mem::INSTR_MEM_SPEC>; 32],
    #[doc = "0xc8..0x128 - Cluster SM%s, containing SM*_CLKDIV, SM*_EXECCTRL, SM*_SHIFTCTRL, SM*_ADDR, SM*_INSTR, SM*_PINCTRL"]
    pub sm: [SM; 4],
    #[doc = "0x128 - Raw Interrupts"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x12c..0x144 - Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
    pub sm_irq: [SM_IRQ; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SM {
    #[doc = "0x00 - Clock divisor register for state machine 0  
 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm_clkdiv: crate::Reg<self::sm::sm_clkdiv::SM_CLKDIV_SPEC>,
    #[doc = "0x04 - Execution/behavioural settings for state machine 0"]
    pub sm_execctrl: crate::Reg<self::sm::sm_execctrl::SM_EXECCTRL_SPEC>,
    #[doc = "0x08 - Control behaviour of the input/output shift registers for state machine 0"]
    pub sm_shiftctrl: crate::Reg<self::sm::sm_shiftctrl::SM_SHIFTCTRL_SPEC>,
    #[doc = "0x0c - Current instruction address of state machine 0"]
    pub sm_addr: crate::Reg<self::sm::sm_addr::SM_ADDR_SPEC>,
    #[doc = "0x10 - Read to see the instruction currently addressed by state machine 0's program counter  
 Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm_instr: crate::Reg<self::sm::sm_instr::SM_INSTR_SPEC>,
    #[doc = "0x14 - State machine pin control"]
    pub sm_pinctrl: crate::Reg<self::sm::sm_pinctrl::SM_PINCTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster SM%s, containing SM*_CLKDIV, SM*_EXECCTRL, SM*_SHIFTCTRL, SM*_ADDR, SM*_INSTR, SM*_PINCTRL"]
pub mod sm;
#[doc = r"Register block"]
#[repr(C)]
pub struct SM_IRQ {
    #[doc = "0x00 - Interrupt Enable for irq0"]
    pub irq_inte: crate::Reg<self::sm_irq::irq_inte::IRQ_INTE_SPEC>,
    #[doc = "0x04 - Interrupt Force for irq0"]
    pub irq_intf: crate::Reg<self::sm_irq::irq_intf::IRQ_INTF_SPEC>,
    #[doc = "0x08 - Interrupt status after masking & forcing for irq0"]
    pub irq_ints: crate::Reg<self::sm_irq::irq_ints::IRQ_INTS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
pub mod sm_irq;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "PIO control register"]
pub mod ctrl;
#[doc = "FSTAT register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "FIFO status register"]
pub mod fstat;
#[doc = "FDEBUG register accessor: an alias for `Reg<FDEBUG_SPEC>`"]
pub type FDEBUG = crate::Reg<fdebug::FDEBUG_SPEC>;
#[doc = "FIFO debug register"]
pub mod fdebug;
#[doc = "FLEVEL register accessor: an alias for `Reg<FLEVEL_SPEC>`"]
pub type FLEVEL = crate::Reg<flevel::FLEVEL_SPEC>;
#[doc = "FIFO levels"]
pub mod flevel;
#[doc = "TXF register accessor: an alias for `Reg<TXF_SPEC>`"]
pub type TXF = crate::Reg<txf::TXF_SPEC>;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
pub mod txf;
#[doc = "RXF register accessor: an alias for `Reg<RXF_SPEC>`"]
pub type RXF = crate::Reg<rxf::RXF_SPEC>;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
pub mod rxf;
#[doc = "IRQ register accessor: an alias for `Reg<IRQ_SPEC>`"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag.  

 Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
pub mod irq;
#[doc = "IRQ_FORCE register accessor: an alias for `Reg<IRQ_FORCE_SPEC>`"]
pub type IRQ_FORCE = crate::Reg<irq_force::IRQ_FORCE_SPEC>;
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
pub mod irq_force;
#[doc = "INPUT_SYNC_BYPASS register accessor: an alias for `Reg<INPUT_SYNC_BYPASS_SPEC>`"]
pub type INPUT_SYNC_BYPASS = crate::Reg<input_sync_bypass::INPUT_SYNC_BYPASS_SPEC>;
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO.  
 0 -> input is synchronized (default)  
 1 -> synchronizer is bypassed  
 If in doubt, leave this register as all zeroes."]
pub mod input_sync_bypass;
#[doc = "DBG_PADOUT register accessor: an alias for `Reg<DBG_PADOUT_SPEC>`"]
pub type DBG_PADOUT = crate::Reg<dbg_padout::DBG_PADOUT_SPEC>;
#[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
pub mod dbg_padout;
#[doc = "DBG_PADOE register accessor: an alias for `Reg<DBG_PADOE_SPEC>`"]
pub type DBG_PADOE = crate::Reg<dbg_padoe::DBG_PADOE_SPEC>;
#[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
pub mod dbg_padoe;
#[doc = "DBG_CFGINFO register accessor: an alias for `Reg<DBG_CFGINFO_SPEC>`"]
pub type DBG_CFGINFO = crate::Reg<dbg_cfginfo::DBG_CFGINFO_SPEC>;
#[doc = "The PIO hardware has some free parameters that may vary between chip products.  
 These should be provided in the chip datasheet, but are also exposed here."]
pub mod dbg_cfginfo;
#[doc = "INSTR_MEM register accessor: an alias for `Reg<INSTR_MEM_SPEC>`"]
pub type INSTR_MEM = crate::Reg<instr_mem::INSTR_MEM_SPEC>;
#[doc = "Write-only access to instruction memory location %s"]
pub mod instr_mem;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
