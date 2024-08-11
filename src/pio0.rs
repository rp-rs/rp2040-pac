#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    fstat: FSTAT,
    fdebug: FDEBUG,
    flevel: FLEVEL,
    txf: [TXF; 4],
    rxf: [RXF; 4],
    irq: IRQ,
    irq_force: IRQ_FORCE,
    input_sync_bypass: INPUT_SYNC_BYPASS,
    dbg_padout: DBG_PADOUT,
    dbg_padoe: DBG_PADOE,
    dbg_cfginfo: DBG_CFGINFO,
    instr_mem: [INSTR_MEM; 32],
    sm: [SM; 4],
    intr: INTR,
    sm_irq: [SM_IRQ; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - PIO control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - FIFO status register"]
    #[inline(always)]
    pub const fn fstat(&self) -> &FSTAT {
        &self.fstat
    }
    #[doc = "0x08 - FIFO debug register"]
    #[inline(always)]
    pub const fn fdebug(&self) -> &FDEBUG {
        &self.fdebug
    }
    #[doc = "0x0c - FIFO levels"]
    #[inline(always)]
    pub const fn flevel(&self) -> &FLEVEL {
        &self.flevel
    }
    #[doc = "0x10..0x20 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
    #[inline(always)]
    pub const fn txf(&self, n: usize) -> &TXF {
        &self.txf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
    #[inline(always)]
    pub fn txf_iter(&self) -> impl Iterator<Item = &TXF> {
        self.txf.iter()
    }
    #[doc = "0x20..0x30 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
    #[inline(always)]
    pub const fn rxf(&self, n: usize) -> &RXF {
        &self.rxf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
    #[inline(always)]
    pub fn rxf_iter(&self) -> impl Iterator<Item = &RXF> {
        self.rxf.iter()
    }
    #[doc = "0x30 - State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag. Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
    #[inline(always)]
    pub const fn irq(&self) -> &IRQ {
        &self.irq
    }
    #[doc = "0x34 - Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
    #[inline(always)]
    pub const fn irq_force(&self) -> &IRQ_FORCE {
        &self.irq_force
    }
    #[doc = "0x38 - There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO. 0 -> input is synchronized (default) 1 -> synchronizer is bypassed If in doubt, leave this register as all zeroes."]
    #[inline(always)]
    pub const fn input_sync_bypass(&self) -> &INPUT_SYNC_BYPASS {
        &self.input_sync_bypass
    }
    #[doc = "0x3c - Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
    #[inline(always)]
    pub const fn dbg_padout(&self) -> &DBG_PADOUT {
        &self.dbg_padout
    }
    #[doc = "0x40 - Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
    #[inline(always)]
    pub const fn dbg_padoe(&self) -> &DBG_PADOE {
        &self.dbg_padoe
    }
    #[doc = "0x44 - The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."]
    #[inline(always)]
    pub const fn dbg_cfginfo(&self) -> &DBG_CFGINFO {
        &self.dbg_cfginfo
    }
    #[doc = "0x48..0xc8 - Write-only access to instruction memory location %s"]
    #[inline(always)]
    pub const fn instr_mem(&self, n: usize) -> &INSTR_MEM {
        &self.instr_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x48..0xc8 - Write-only access to instruction memory location %s"]
    #[inline(always)]
    pub fn instr_mem_iter(&self) -> impl Iterator<Item = &INSTR_MEM> {
        self.instr_mem.iter()
    }
    #[doc = "0xc8..0x128 - Cluster SM%s, containing SM*_CLKDIV, SM*_EXECCTRL, SM*_SHIFTCTRL, SM*_ADDR, SM*_INSTR, SM*_PINCTRL"]
    #[inline(always)]
    pub const fn sm(&self, n: usize) -> &SM {
        &self.sm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc8..0x128 - Cluster SM%s, containing SM*_CLKDIV, SM*_EXECCTRL, SM*_SHIFTCTRL, SM*_ADDR, SM*_INSTR, SM*_PINCTRL"]
    #[inline(always)]
    pub fn sm_iter(&self) -> impl Iterator<Item = &SM> {
        self.sm.iter()
    }
    #[doc = "0x128 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x12c..0x144 - Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
    #[inline(always)]
    pub const fn sm_irq(&self, n: usize) -> &SM_IRQ {
        &self.sm_irq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12c..0x144 - Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
    #[inline(always)]
    pub fn sm_irq_iter(&self) -> impl Iterator<Item = &SM_IRQ> {
        self.sm_irq.iter()
    }
}
#[doc = "CTRL (rw) register accessor: PIO control register  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "PIO control register"]
pub mod ctrl;
#[doc = "FSTAT (rw) register accessor: FIFO status register  

You can [`read`](crate::generic::Reg::read) this register and get [`fstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fstat`]
module"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "FIFO status register"]
pub mod fstat;
#[doc = "FDEBUG (rw) register accessor: FIFO debug register  

You can [`read`](crate::generic::Reg::read) this register and get [`fdebug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdebug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fdebug`]
module"]
pub type FDEBUG = crate::Reg<fdebug::FDEBUG_SPEC>;
#[doc = "FIFO debug register"]
pub mod fdebug;
#[doc = "FLEVEL (rw) register accessor: FIFO levels  

You can [`read`](crate::generic::Reg::read) this register and get [`flevel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flevel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@flevel`]
module"]
pub type FLEVEL = crate::Reg<flevel::FLEVEL_SPEC>;
#[doc = "FIFO levels"]
pub mod flevel;
#[doc = "TXF (rw) register accessor: Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.  

You can [`read`](crate::generic::Reg::read) this register and get [`txf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@txf`]
module"]
pub type TXF = crate::Reg<txf::TXF_SPEC>;
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
pub mod txf;
#[doc = "RXF (rw) register accessor: Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.  

You can [`read`](crate::generic::Reg::read) this register and get [`rxf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rxf`]
module"]
pub type RXF = crate::Reg<rxf::RXF_SPEC>;
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
pub mod rxf;
#[doc = "IRQ (rw) register accessor: State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag. Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE.  

You can [`read`](crate::generic::Reg::read) this register and get [`irq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq`]
module"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag. Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
pub mod irq;
#[doc = "IRQ_FORCE (rw) register accessor: Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines.  

You can [`read`](crate::generic::Reg::read) this register and get [`irq_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq_force`]
module"]
pub type IRQ_FORCE = crate::Reg<irq_force::IRQ_FORCE_SPEC>;
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
pub mod irq_force;
#[doc = "INPUT_SYNC_BYPASS (rw) register accessor: There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO. 0 -> input is synchronized (default) 1 -> synchronizer is bypassed If in doubt, leave this register as all zeroes.  

You can [`read`](crate::generic::Reg::read) this register and get [`input_sync_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input_sync_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@input_sync_bypass`]
module"]
pub type INPUT_SYNC_BYPASS = crate::Reg<input_sync_bypass::INPUT_SYNC_BYPASS_SPEC>;
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO. 0 -> input is synchronized (default) 1 -> synchronizer is bypassed If in doubt, leave this register as all zeroes."]
pub mod input_sync_bypass;
#[doc = "DBG_PADOUT (rw) register accessor: Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.  

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_padout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_padout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_padout`]
module"]
pub type DBG_PADOUT = crate::Reg<dbg_padout::DBG_PADOUT_SPEC>;
#[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
pub mod dbg_padout;
#[doc = "DBG_PADOE (rw) register accessor: Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.  

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_padoe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_padoe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_padoe`]
module"]
pub type DBG_PADOE = crate::Reg<dbg_padoe::DBG_PADOE_SPEC>;
#[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
pub mod dbg_padoe;
#[doc = "DBG_CFGINFO (rw) register accessor: The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here.  

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_cfginfo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_cfginfo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_cfginfo`]
module"]
pub type DBG_CFGINFO = crate::Reg<dbg_cfginfo::DBG_CFGINFO_SPEC>;
#[doc = "The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."]
pub mod dbg_cfginfo;
#[doc = "INSTR_MEM (rw) register accessor: Write-only access to instruction memory location %s  

You can [`read`](crate::generic::Reg::read) this register and get [`instr_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`instr_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@instr_mem`]
module"]
pub type INSTR_MEM = crate::Reg<instr_mem::INSTR_MEM_SPEC>;
#[doc = "Write-only access to instruction memory location %s"]
pub mod instr_mem;
#[doc = "Cluster SM%s, containing SM*_CLKDIV, SM*_EXECCTRL, SM*_SHIFTCTRL, SM*_ADDR, SM*_INSTR, SM*_PINCTRL"]
pub use self::sm::SM;
#[doc = r"Cluster"]
#[doc = "Cluster SM%s, containing SM*_CLKDIV, SM*_EXECCTRL, SM*_SHIFTCTRL, SM*_ADDR, SM*_INSTR, SM*_PINCTRL"]
pub mod sm;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
pub use self::sm_irq::SM_IRQ;
#[doc = r"Cluster"]
#[doc = "Cluster SM_IRQ%s, containing IRQ*_INTE, IRQ*_INTF, IRQ*_INTS"]
pub mod sm_irq;
