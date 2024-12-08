#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpuid: CPUID,
    gpio_in: GPIO_IN,
    gpio_hi_in: GPIO_HI_IN,
    _reserved3: [u8; 0x04],
    gpio_out: GPIO_OUT,
    gpio_out_set: GPIO_OUT_SET,
    gpio_out_clr: GPIO_OUT_CLR,
    gpio_out_xor: GPIO_OUT_XOR,
    gpio_oe: GPIO_OE,
    gpio_oe_set: GPIO_OE_SET,
    gpio_oe_clr: GPIO_OE_CLR,
    gpio_oe_xor: GPIO_OE_XOR,
    gpio_hi_out: GPIO_HI_OUT,
    gpio_hi_out_set: GPIO_HI_OUT_SET,
    gpio_hi_out_clr: GPIO_HI_OUT_CLR,
    gpio_hi_out_xor: GPIO_HI_OUT_XOR,
    gpio_hi_oe: GPIO_HI_OE,
    gpio_hi_oe_set: GPIO_HI_OE_SET,
    gpio_hi_oe_clr: GPIO_HI_OE_CLR,
    gpio_hi_oe_xor: GPIO_HI_OE_XOR,
    fifo_st: FIFO_ST,
    fifo_wr: FIFO_WR,
    fifo_rd: FIFO_RD,
    spinlock_st: SPINLOCK_ST,
    div_udividend: DIV_UDIVIDEND,
    div_udivisor: DIV_UDIVISOR,
    div_sdividend: DIV_SDIVIDEND,
    div_sdivisor: DIV_SDIVISOR,
    div_quotient: DIV_QUOTIENT,
    div_remainder: DIV_REMAINDER,
    div_csr: DIV_CSR,
    _reserved30: [u8; 0x04],
    interp0_accum0: INTERP0_ACCUM0,
    interp0_accum1: INTERP0_ACCUM1,
    interp0_base0: INTERP0_BASE0,
    interp0_base1: INTERP0_BASE1,
    interp0_base2: INTERP0_BASE2,
    interp0_pop_lane0: INTERP0_POP_LANE0,
    interp0_pop_lane1: INTERP0_POP_LANE1,
    interp0_pop_full: INTERP0_POP_FULL,
    interp0_peek_lane0: INTERP0_PEEK_LANE0,
    interp0_peek_lane1: INTERP0_PEEK_LANE1,
    interp0_peek_full: INTERP0_PEEK_FULL,
    interp0_ctrl_lane0: INTERP0_CTRL_LANE0,
    interp0_ctrl_lane1: INTERP0_CTRL_LANE1,
    interp0_accum0_add: INTERP0_ACCUM0_ADD,
    interp0_accum1_add: INTERP0_ACCUM1_ADD,
    interp0_base_1and0: INTERP0_BASE_1AND0,
    interp1_accum0: INTERP1_ACCUM0,
    interp1_accum1: INTERP1_ACCUM1,
    interp1_base0: INTERP1_BASE0,
    interp1_base1: INTERP1_BASE1,
    interp1_base2: INTERP1_BASE2,
    interp1_pop_lane0: INTERP1_POP_LANE0,
    interp1_pop_lane1: INTERP1_POP_LANE1,
    interp1_pop_full: INTERP1_POP_FULL,
    interp1_peek_lane0: INTERP1_PEEK_LANE0,
    interp1_peek_lane1: INTERP1_PEEK_LANE1,
    interp1_peek_full: INTERP1_PEEK_FULL,
    interp1_ctrl_lane0: INTERP1_CTRL_LANE0,
    interp1_ctrl_lane1: INTERP1_CTRL_LANE1,
    interp1_accum0_add: INTERP1_ACCUM0_ADD,
    interp1_accum1_add: INTERP1_ACCUM1_ADD,
    interp1_base_1and0: INTERP1_BASE_1AND0,
    spinlock: [SPINLOCK; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Processor core identifier"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &CPUID {
        &self.cpuid
    }
    #[doc = "0x04 - Input value for GPIO pins"]
    #[inline(always)]
    pub const fn gpio_in(&self) -> &GPIO_IN {
        &self.gpio_in
    }
    #[doc = "0x08 - Input value for QSPI pins"]
    #[inline(always)]
    pub const fn gpio_hi_in(&self) -> &GPIO_HI_IN {
        &self.gpio_hi_in
    }
    #[doc = "0x10 - GPIO output value"]
    #[inline(always)]
    pub const fn gpio_out(&self) -> &GPIO_OUT {
        &self.gpio_out
    }
    #[doc = "0x14 - GPIO output value set"]
    #[inline(always)]
    pub const fn gpio_out_set(&self) -> &GPIO_OUT_SET {
        &self.gpio_out_set
    }
    #[doc = "0x18 - GPIO output value clear"]
    #[inline(always)]
    pub const fn gpio_out_clr(&self) -> &GPIO_OUT_CLR {
        &self.gpio_out_clr
    }
    #[doc = "0x1c - GPIO output value XOR"]
    #[inline(always)]
    pub const fn gpio_out_xor(&self) -> &GPIO_OUT_XOR {
        &self.gpio_out_xor
    }
    #[doc = "0x20 - GPIO output enable"]
    #[inline(always)]
    pub const fn gpio_oe(&self) -> &GPIO_OE {
        &self.gpio_oe
    }
    #[doc = "0x24 - GPIO output enable set"]
    #[inline(always)]
    pub const fn gpio_oe_set(&self) -> &GPIO_OE_SET {
        &self.gpio_oe_set
    }
    #[doc = "0x28 - GPIO output enable clear"]
    #[inline(always)]
    pub const fn gpio_oe_clr(&self) -> &GPIO_OE_CLR {
        &self.gpio_oe_clr
    }
    #[doc = "0x2c - GPIO output enable XOR"]
    #[inline(always)]
    pub const fn gpio_oe_xor(&self) -> &GPIO_OE_XOR {
        &self.gpio_oe_xor
    }
    #[doc = "0x30 - QSPI output value"]
    #[inline(always)]
    pub const fn gpio_hi_out(&self) -> &GPIO_HI_OUT {
        &self.gpio_hi_out
    }
    #[doc = "0x34 - QSPI output value set"]
    #[inline(always)]
    pub const fn gpio_hi_out_set(&self) -> &GPIO_HI_OUT_SET {
        &self.gpio_hi_out_set
    }
    #[doc = "0x38 - QSPI output value clear"]
    #[inline(always)]
    pub const fn gpio_hi_out_clr(&self) -> &GPIO_HI_OUT_CLR {
        &self.gpio_hi_out_clr
    }
    #[doc = "0x3c - QSPI output value XOR"]
    #[inline(always)]
    pub const fn gpio_hi_out_xor(&self) -> &GPIO_HI_OUT_XOR {
        &self.gpio_hi_out_xor
    }
    #[doc = "0x40 - QSPI output enable"]
    #[inline(always)]
    pub const fn gpio_hi_oe(&self) -> &GPIO_HI_OE {
        &self.gpio_hi_oe
    }
    #[doc = "0x44 - QSPI output enable set"]
    #[inline(always)]
    pub const fn gpio_hi_oe_set(&self) -> &GPIO_HI_OE_SET {
        &self.gpio_hi_oe_set
    }
    #[doc = "0x48 - QSPI output enable clear"]
    #[inline(always)]
    pub const fn gpio_hi_oe_clr(&self) -> &GPIO_HI_OE_CLR {
        &self.gpio_hi_oe_clr
    }
    #[doc = "0x4c - QSPI output enable XOR"]
    #[inline(always)]
    pub const fn gpio_hi_oe_xor(&self) -> &GPIO_HI_OE_XOR {
        &self.gpio_hi_oe_xor
    }
    #[doc = "0x50 - Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
    #[inline(always)]
    pub const fn fifo_st(&self) -> &FIFO_ST {
        &self.fifo_st
    }
    #[doc = "0x54 - Write access to this core's TX FIFO"]
    #[inline(always)]
    pub const fn fifo_wr(&self) -> &FIFO_WR {
        &self.fifo_wr
    }
    #[doc = "0x58 - Read access to this core's RX FIFO"]
    #[inline(always)]
    pub const fn fifo_rd(&self) -> &FIFO_RD {
        &self.fifo_rd
    }
    #[doc = "0x5c - Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
    #[inline(always)]
    pub const fn spinlock_st(&self) -> &SPINLOCK_ST {
        &self.spinlock_st
    }
    #[doc = "0x60 - Divider unsigned dividend Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    #[inline(always)]
    pub const fn div_udividend(&self) -> &DIV_UDIVIDEND {
        &self.div_udividend
    }
    #[doc = "0x64 - Divider unsigned divisor Write to the DIVISOR operand of the divider, i.e. the q in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVISOR/SDIVISOR are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    #[inline(always)]
    pub const fn div_udivisor(&self) -> &DIV_UDIVISOR {
        &self.div_udivisor
    }
    #[doc = "0x68 - Divider signed dividend The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
    #[inline(always)]
    pub const fn div_sdividend(&self) -> &DIV_SDIVIDEND {
        &self.div_sdividend
    }
    #[doc = "0x6c - Divider signed divisor The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
    #[inline(always)]
    pub const fn div_sdivisor(&self) -> &DIV_SDIVISOR {
        &self.div_sdivisor
    }
    #[doc = "0x70 - Divider result quotient The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low. For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags. Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order REMAINDER, QUOTIENT if CSR_DIRTY is used."]
    #[inline(always)]
    pub const fn div_quotient(&self) -> &DIV_QUOTIENT {
        &self.div_quotient
    }
    #[doc = "0x74 - Divider result remainder The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low. For signed calculations, REMAINDER is negative only when DIVIDEND is negative. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
    #[inline(always)]
    pub const fn div_remainder(&self) -> &DIV_REMAINDER {
        &self.div_remainder
    }
    #[doc = "0x78 - Control and status register for divider."]
    #[inline(always)]
    pub const fn div_csr(&self) -> &DIV_CSR {
        &self.div_csr
    }
    #[doc = "0x80 - Read/write access to accumulator 0"]
    #[inline(always)]
    pub const fn interp0_accum0(&self) -> &INTERP0_ACCUM0 {
        &self.interp0_accum0
    }
    #[doc = "0x84 - Read/write access to accumulator 1"]
    #[inline(always)]
    pub const fn interp0_accum1(&self) -> &INTERP0_ACCUM1 {
        &self.interp0_accum1
    }
    #[doc = "0x88 - Read/write access to BASE0 register."]
    #[inline(always)]
    pub const fn interp0_base0(&self) -> &INTERP0_BASE0 {
        &self.interp0_base0
    }
    #[doc = "0x8c - Read/write access to BASE1 register."]
    #[inline(always)]
    pub const fn interp0_base1(&self) -> &INTERP0_BASE1 {
        &self.interp0_base1
    }
    #[doc = "0x90 - Read/write access to BASE2 register."]
    #[inline(always)]
    pub const fn interp0_base2(&self) -> &INTERP0_BASE2 {
        &self.interp0_base2
    }
    #[doc = "0x94 - Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp0_pop_lane0(&self) -> &INTERP0_POP_LANE0 {
        &self.interp0_pop_lane0
    }
    #[doc = "0x98 - Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp0_pop_lane1(&self) -> &INTERP0_POP_LANE1 {
        &self.interp0_pop_lane1
    }
    #[doc = "0x9c - Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp0_pop_full(&self) -> &INTERP0_POP_FULL {
        &self.interp0_pop_full
    }
    #[doc = "0xa0 - Read LANE0 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp0_peek_lane0(&self) -> &INTERP0_PEEK_LANE0 {
        &self.interp0_peek_lane0
    }
    #[doc = "0xa4 - Read LANE1 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp0_peek_lane1(&self) -> &INTERP0_PEEK_LANE1 {
        &self.interp0_peek_lane1
    }
    #[doc = "0xa8 - Read FULL result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp0_peek_full(&self) -> &INTERP0_PEEK_FULL {
        &self.interp0_peek_full
    }
    #[doc = "0xac - Control register for lane 0"]
    #[inline(always)]
    pub const fn interp0_ctrl_lane0(&self) -> &INTERP0_CTRL_LANE0 {
        &self.interp0_ctrl_lane0
    }
    #[doc = "0xb0 - Control register for lane 1"]
    #[inline(always)]
    pub const fn interp0_ctrl_lane1(&self) -> &INTERP0_CTRL_LANE1 {
        &self.interp0_ctrl_lane1
    }
    #[doc = "0xb4 - Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    #[inline(always)]
    pub const fn interp0_accum0_add(&self) -> &INTERP0_ACCUM0_ADD {
        &self.interp0_accum0_add
    }
    #[doc = "0xb8 - Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    #[inline(always)]
    pub const fn interp0_accum1_add(&self) -> &INTERP0_ACCUM1_ADD {
        &self.interp0_accum1_add
    }
    #[doc = "0xbc - On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    #[inline(always)]
    pub const fn interp0_base_1and0(&self) -> &INTERP0_BASE_1AND0 {
        &self.interp0_base_1and0
    }
    #[doc = "0xc0 - Read/write access to accumulator 0"]
    #[inline(always)]
    pub const fn interp1_accum0(&self) -> &INTERP1_ACCUM0 {
        &self.interp1_accum0
    }
    #[doc = "0xc4 - Read/write access to accumulator 1"]
    #[inline(always)]
    pub const fn interp1_accum1(&self) -> &INTERP1_ACCUM1 {
        &self.interp1_accum1
    }
    #[doc = "0xc8 - Read/write access to BASE0 register."]
    #[inline(always)]
    pub const fn interp1_base0(&self) -> &INTERP1_BASE0 {
        &self.interp1_base0
    }
    #[doc = "0xcc - Read/write access to BASE1 register."]
    #[inline(always)]
    pub const fn interp1_base1(&self) -> &INTERP1_BASE1 {
        &self.interp1_base1
    }
    #[doc = "0xd0 - Read/write access to BASE2 register."]
    #[inline(always)]
    pub const fn interp1_base2(&self) -> &INTERP1_BASE2 {
        &self.interp1_base2
    }
    #[doc = "0xd4 - Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp1_pop_lane0(&self) -> &INTERP1_POP_LANE0 {
        &self.interp1_pop_lane0
    }
    #[doc = "0xd8 - Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp1_pop_lane1(&self) -> &INTERP1_POP_LANE1 {
        &self.interp1_pop_lane1
    }
    #[doc = "0xdc - Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp1_pop_full(&self) -> &INTERP1_POP_FULL {
        &self.interp1_pop_full
    }
    #[doc = "0xe0 - Read LANE0 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp1_peek_lane0(&self) -> &INTERP1_PEEK_LANE0 {
        &self.interp1_peek_lane0
    }
    #[doc = "0xe4 - Read LANE1 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp1_peek_lane1(&self) -> &INTERP1_PEEK_LANE1 {
        &self.interp1_peek_lane1
    }
    #[doc = "0xe8 - Read FULL result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp1_peek_full(&self) -> &INTERP1_PEEK_FULL {
        &self.interp1_peek_full
    }
    #[doc = "0xec - Control register for lane 0"]
    #[inline(always)]
    pub const fn interp1_ctrl_lane0(&self) -> &INTERP1_CTRL_LANE0 {
        &self.interp1_ctrl_lane0
    }
    #[doc = "0xf0 - Control register for lane 1"]
    #[inline(always)]
    pub const fn interp1_ctrl_lane1(&self) -> &INTERP1_CTRL_LANE1 {
        &self.interp1_ctrl_lane1
    }
    #[doc = "0xf4 - Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    #[inline(always)]
    pub const fn interp1_accum0_add(&self) -> &INTERP1_ACCUM0_ADD {
        &self.interp1_accum0_add
    }
    #[doc = "0xf8 - Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    #[inline(always)]
    pub const fn interp1_accum1_add(&self) -> &INTERP1_ACCUM1_ADD {
        &self.interp1_accum1_add
    }
    #[doc = "0xfc - On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    #[inline(always)]
    pub const fn interp1_base_1and0(&self) -> &INTERP1_BASE_1AND0 {
        &self.interp1_base_1and0
    }
    #[doc = "0x100..0x180 - Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 &lt;&lt; lock number."]
    #[inline(always)]
    pub const fn spinlock(&self, n: usize) -> &SPINLOCK {
        &self.spinlock[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 &lt;&lt; lock number."]
    #[inline(always)]
    pub fn spinlock_iter(&self) -> impl Iterator<Item = &SPINLOCK> {
        self.spinlock.iter()
    }
}
#[doc = "CPUID (rw) register accessor: Processor core identifier  

You can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cpuid`]
module"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "Processor core identifier"]
pub mod cpuid;
#[doc = "GPIO_IN (rw) register accessor: Input value for GPIO pins  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_in`]
module"]
pub type GPIO_IN = crate::Reg<gpio_in::GPIO_IN_SPEC>;
#[doc = "Input value for GPIO pins"]
pub mod gpio_in;
#[doc = "GPIO_HI_IN (rw) register accessor: Input value for QSPI pins  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_in`]
module"]
pub type GPIO_HI_IN = crate::Reg<gpio_hi_in::GPIO_HI_IN_SPEC>;
#[doc = "Input value for QSPI pins"]
pub mod gpio_hi_in;
#[doc = "GPIO_OUT (rw) register accessor: GPIO output value  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_out`]
module"]
pub type GPIO_OUT = crate::Reg<gpio_out::GPIO_OUT_SPEC>;
#[doc = "GPIO output value"]
pub mod gpio_out;
#[doc = "GPIO_OUT_SET (rw) register accessor: GPIO output value set  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_out_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_out_set`]
module"]
pub type GPIO_OUT_SET = crate::Reg<gpio_out_set::GPIO_OUT_SET_SPEC>;
#[doc = "GPIO output value set"]
pub mod gpio_out_set;
#[doc = "GPIO_OUT_CLR (rw) register accessor: GPIO output value clear  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_out_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_out_clr`]
module"]
pub type GPIO_OUT_CLR = crate::Reg<gpio_out_clr::GPIO_OUT_CLR_SPEC>;
#[doc = "GPIO output value clear"]
pub mod gpio_out_clr;
#[doc = "GPIO_OUT_XOR (rw) register accessor: GPIO output value XOR  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_out_xor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out_xor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_out_xor`]
module"]
pub type GPIO_OUT_XOR = crate::Reg<gpio_out_xor::GPIO_OUT_XOR_SPEC>;
#[doc = "GPIO output value XOR"]
pub mod gpio_out_xor;
#[doc = "GPIO_OE (rw) register accessor: GPIO output enable  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_oe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_oe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_oe`]
module"]
pub type GPIO_OE = crate::Reg<gpio_oe::GPIO_OE_SPEC>;
#[doc = "GPIO output enable"]
pub mod gpio_oe;
#[doc = "GPIO_OE_SET (rw) register accessor: GPIO output enable set  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_oe_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_oe_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_oe_set`]
module"]
pub type GPIO_OE_SET = crate::Reg<gpio_oe_set::GPIO_OE_SET_SPEC>;
#[doc = "GPIO output enable set"]
pub mod gpio_oe_set;
#[doc = "GPIO_OE_CLR (rw) register accessor: GPIO output enable clear  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_oe_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_oe_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_oe_clr`]
module"]
pub type GPIO_OE_CLR = crate::Reg<gpio_oe_clr::GPIO_OE_CLR_SPEC>;
#[doc = "GPIO output enable clear"]
pub mod gpio_oe_clr;
#[doc = "GPIO_OE_XOR (rw) register accessor: GPIO output enable XOR  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_oe_xor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_oe_xor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_oe_xor`]
module"]
pub type GPIO_OE_XOR = crate::Reg<gpio_oe_xor::GPIO_OE_XOR_SPEC>;
#[doc = "GPIO output enable XOR"]
pub mod gpio_oe_xor;
#[doc = "GPIO_HI_OUT (rw) register accessor: QSPI output value  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_out`]
module"]
pub type GPIO_HI_OUT = crate::Reg<gpio_hi_out::GPIO_HI_OUT_SPEC>;
#[doc = "QSPI output value"]
pub mod gpio_hi_out;
#[doc = "GPIO_HI_OUT_SET (rw) register accessor: QSPI output value set  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_out_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_out_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_out_set`]
module"]
pub type GPIO_HI_OUT_SET = crate::Reg<gpio_hi_out_set::GPIO_HI_OUT_SET_SPEC>;
#[doc = "QSPI output value set"]
pub mod gpio_hi_out_set;
#[doc = "GPIO_HI_OUT_CLR (rw) register accessor: QSPI output value clear  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_out_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_out_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_out_clr`]
module"]
pub type GPIO_HI_OUT_CLR = crate::Reg<gpio_hi_out_clr::GPIO_HI_OUT_CLR_SPEC>;
#[doc = "QSPI output value clear"]
pub mod gpio_hi_out_clr;
#[doc = "GPIO_HI_OUT_XOR (rw) register accessor: QSPI output value XOR  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_out_xor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_out_xor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_out_xor`]
module"]
pub type GPIO_HI_OUT_XOR = crate::Reg<gpio_hi_out_xor::GPIO_HI_OUT_XOR_SPEC>;
#[doc = "QSPI output value XOR"]
pub mod gpio_hi_out_xor;
#[doc = "GPIO_HI_OE (rw) register accessor: QSPI output enable  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_oe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_oe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_oe`]
module"]
pub type GPIO_HI_OE = crate::Reg<gpio_hi_oe::GPIO_HI_OE_SPEC>;
#[doc = "QSPI output enable"]
pub mod gpio_hi_oe;
#[doc = "GPIO_HI_OE_SET (rw) register accessor: QSPI output enable set  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_oe_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_oe_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_oe_set`]
module"]
pub type GPIO_HI_OE_SET = crate::Reg<gpio_hi_oe_set::GPIO_HI_OE_SET_SPEC>;
#[doc = "QSPI output enable set"]
pub mod gpio_hi_oe_set;
#[doc = "GPIO_HI_OE_CLR (rw) register accessor: QSPI output enable clear  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_oe_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_oe_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_oe_clr`]
module"]
pub type GPIO_HI_OE_CLR = crate::Reg<gpio_hi_oe_clr::GPIO_HI_OE_CLR_SPEC>;
#[doc = "QSPI output enable clear"]
pub mod gpio_hi_oe_clr;
#[doc = "GPIO_HI_OE_XOR (rw) register accessor: QSPI output enable XOR  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_oe_xor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_oe_xor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_oe_xor`]
module"]
pub type GPIO_HI_OE_XOR = crate::Reg<gpio_hi_oe_xor::GPIO_HI_OE_XOR_SPEC>;
#[doc = "QSPI output enable XOR"]
pub mod gpio_hi_oe_xor;
#[doc = "FIFO_ST (rw) register accessor: Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register.  

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo_st`]
module"]
pub type FIFO_ST = crate::Reg<fifo_st::FIFO_ST_SPEC>;
#[doc = "Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
pub mod fifo_st;
#[doc = "FIFO_WR (rw) register accessor: Write access to this core's TX FIFO  

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo_wr`]
module"]
pub type FIFO_WR = crate::Reg<fifo_wr::FIFO_WR_SPEC>;
#[doc = "Write access to this core's TX FIFO"]
pub mod fifo_wr;
#[doc = "FIFO_RD (rw) register accessor: Read access to this core's RX FIFO  

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo_rd`]
module"]
pub type FIFO_RD = crate::Reg<fifo_rd::FIFO_RD_SPEC>;
#[doc = "Read access to this core's RX FIFO"]
pub mod fifo_rd;
#[doc = "SPINLOCK_ST (rw) register accessor: Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging.  

You can [`read`](crate::generic::Reg::read) this register and get [`spinlock_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@spinlock_st`]
module"]
pub type SPINLOCK_ST = crate::Reg<spinlock_st::SPINLOCK_ST_SPEC>;
#[doc = "Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
pub mod spinlock_st;
#[doc = "DIV_UDIVIDEND (rw) register accessor: Divider unsigned dividend Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_udividend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_udividend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div_udividend`]
module"]
pub type DIV_UDIVIDEND = crate::Reg<div_udividend::DIV_UDIVIDEND_SPEC>;
#[doc = "Divider unsigned dividend Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
pub mod div_udividend;
#[doc = "DIV_UDIVISOR (rw) register accessor: Divider unsigned divisor Write to the DIVISOR operand of the divider, i.e. the q in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVISOR/SDIVISOR are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_udivisor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_udivisor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div_udivisor`]
module"]
pub type DIV_UDIVISOR = crate::Reg<div_udivisor::DIV_UDIVISOR_SPEC>;
#[doc = "Divider unsigned divisor Write to the DIVISOR operand of the divider, i.e. the q in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVISOR/SDIVISOR are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
pub mod div_udivisor;
#[doc = "DIV_SDIVIDEND (rw) register accessor: Divider signed dividend The same as UDIVIDEND, but starts a signed calculation, rather than unsigned.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_sdividend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_sdividend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div_sdividend`]
module"]
pub type DIV_SDIVIDEND = crate::Reg<div_sdividend::DIV_SDIVIDEND_SPEC>;
#[doc = "Divider signed dividend The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
pub mod div_sdividend;
#[doc = "DIV_SDIVISOR (rw) register accessor: Divider signed divisor The same as UDIVISOR, but starts a signed calculation, rather than unsigned.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_sdivisor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_sdivisor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div_sdivisor`]
module"]
pub type DIV_SDIVISOR = crate::Reg<div_sdivisor::DIV_SDIVISOR_SPEC>;
#[doc = "Divider signed divisor The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
pub mod div_sdivisor;
#[doc = "DIV_QUOTIENT (rw) register accessor: Divider result quotient The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low. For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags. Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order REMAINDER, QUOTIENT if CSR_DIRTY is used.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_quotient::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_quotient::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div_quotient`]
module"]
pub type DIV_QUOTIENT = crate::Reg<div_quotient::DIV_QUOTIENT_SPEC>;
#[doc = "Divider result quotient The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low. For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags. Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order REMAINDER, QUOTIENT if CSR_DIRTY is used."]
pub mod div_quotient;
#[doc = "DIV_REMAINDER (rw) register accessor: Divider result remainder The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low. For signed calculations, REMAINDER is negative only when DIVIDEND is negative. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_remainder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_remainder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div_remainder`]
module"]
pub type DIV_REMAINDER = crate::Reg<div_remainder::DIV_REMAINDER_SPEC>;
#[doc = "Divider result remainder The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low. For signed calculations, REMAINDER is negative only when DIVIDEND is negative. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
pub mod div_remainder;
#[doc = "DIV_CSR (rw) register accessor: Control and status register for divider.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div_csr`]
module"]
pub type DIV_CSR = crate::Reg<div_csr::DIV_CSR_SPEC>;
#[doc = "Control and status register for divider."]
pub mod div_csr;
#[doc = "INTERP0_ACCUM0 (rw) register accessor: Read/write access to accumulator 0  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_accum0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_accum0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_accum0`]
module"]
pub type INTERP0_ACCUM0 = crate::Reg<interp0_accum0::INTERP0_ACCUM0_SPEC>;
#[doc = "Read/write access to accumulator 0"]
pub mod interp0_accum0;
#[doc = "INTERP0_ACCUM1 (rw) register accessor: Read/write access to accumulator 1  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_accum1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_accum1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_accum1`]
module"]
pub type INTERP0_ACCUM1 = crate::Reg<interp0_accum1::INTERP0_ACCUM1_SPEC>;
#[doc = "Read/write access to accumulator 1"]
pub mod interp0_accum1;
#[doc = "INTERP0_BASE0 (rw) register accessor: Read/write access to BASE0 register.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_base0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_base0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_base0`]
module"]
pub type INTERP0_BASE0 = crate::Reg<interp0_base0::INTERP0_BASE0_SPEC>;
#[doc = "Read/write access to BASE0 register."]
pub mod interp0_base0;
#[doc = "INTERP0_BASE1 (rw) register accessor: Read/write access to BASE1 register.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_base1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_base1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_base1`]
module"]
pub type INTERP0_BASE1 = crate::Reg<interp0_base1::INTERP0_BASE1_SPEC>;
#[doc = "Read/write access to BASE1 register."]
pub mod interp0_base1;
#[doc = "INTERP0_BASE2 (rw) register accessor: Read/write access to BASE2 register.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_base2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_base2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_base2`]
module"]
pub type INTERP0_BASE2 = crate::Reg<interp0_base2::INTERP0_BASE2_SPEC>;
#[doc = "Read/write access to BASE2 register."]
pub mod interp0_base2;
#[doc = "INTERP0_POP_LANE0 (rw) register accessor: Read LANE0 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_pop_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_pop_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_pop_lane0`]
module"]
pub type INTERP0_POP_LANE0 = crate::Reg<interp0_pop_lane0::INTERP0_POP_LANE0_SPEC>;
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_lane0;
#[doc = "INTERP0_POP_LANE1 (rw) register accessor: Read LANE1 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_pop_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_pop_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_pop_lane1`]
module"]
pub type INTERP0_POP_LANE1 = crate::Reg<interp0_pop_lane1::INTERP0_POP_LANE1_SPEC>;
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_lane1;
#[doc = "INTERP0_POP_FULL (rw) register accessor: Read FULL result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_pop_full::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_pop_full::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_pop_full`]
module"]
pub type INTERP0_POP_FULL = crate::Reg<interp0_pop_full::INTERP0_POP_FULL_SPEC>;
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_full;
#[doc = "INTERP0_PEEK_LANE0 (rw) register accessor: Read LANE0 result, without altering any internal state (PEEK).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_peek_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_peek_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_peek_lane0`]
module"]
pub type INTERP0_PEEK_LANE0 = crate::Reg<interp0_peek_lane0::INTERP0_PEEK_LANE0_SPEC>;
#[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
pub mod interp0_peek_lane0;
#[doc = "INTERP0_PEEK_LANE1 (rw) register accessor: Read LANE1 result, without altering any internal state (PEEK).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_peek_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_peek_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_peek_lane1`]
module"]
pub type INTERP0_PEEK_LANE1 = crate::Reg<interp0_peek_lane1::INTERP0_PEEK_LANE1_SPEC>;
#[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
pub mod interp0_peek_lane1;
#[doc = "INTERP0_PEEK_FULL (rw) register accessor: Read FULL result, without altering any internal state (PEEK).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_peek_full::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_peek_full::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_peek_full`]
module"]
pub type INTERP0_PEEK_FULL = crate::Reg<interp0_peek_full::INTERP0_PEEK_FULL_SPEC>;
#[doc = "Read FULL result, without altering any internal state (PEEK)."]
pub mod interp0_peek_full;
#[doc = "INTERP0_CTRL_LANE0 (rw) register accessor: Control register for lane 0  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_ctrl_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_ctrl_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_ctrl_lane0`]
module"]
pub type INTERP0_CTRL_LANE0 = crate::Reg<interp0_ctrl_lane0::INTERP0_CTRL_LANE0_SPEC>;
#[doc = "Control register for lane 0"]
pub mod interp0_ctrl_lane0;
#[doc = "INTERP0_CTRL_LANE1 (rw) register accessor: Control register for lane 1  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_ctrl_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_ctrl_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_ctrl_lane1`]
module"]
pub type INTERP0_CTRL_LANE1 = crate::Reg<interp0_ctrl_lane1::INTERP0_CTRL_LANE1_SPEC>;
#[doc = "Control register for lane 1"]
pub mod interp0_ctrl_lane1;
#[doc = "INTERP0_ACCUM0_ADD (rw) register accessor: Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_accum0_add::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_accum0_add::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_accum0_add`]
module"]
pub type INTERP0_ACCUM0_ADD = crate::Reg<interp0_accum0_add::INTERP0_ACCUM0_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
pub mod interp0_accum0_add;
#[doc = "INTERP0_ACCUM1_ADD (rw) register accessor: Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_accum1_add::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_accum1_add::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_accum1_add`]
module"]
pub type INTERP0_ACCUM1_ADD = crate::Reg<interp0_accum1_add::INTERP0_ACCUM1_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
pub mod interp0_accum1_add;
#[doc = "INTERP0_BASE_1AND0 (rw) register accessor: On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_base_1and0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_base_1and0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_base_1and0`]
module"]
pub type INTERP0_BASE_1AND0 = crate::Reg<interp0_base_1and0::INTERP0_BASE_1AND0_SPEC>;
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
pub mod interp0_base_1and0;
#[doc = "INTERP1_ACCUM0 (rw) register accessor: Read/write access to accumulator 0  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_accum0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_accum0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_accum0`]
module"]
pub type INTERP1_ACCUM0 = crate::Reg<interp1_accum0::INTERP1_ACCUM0_SPEC>;
#[doc = "Read/write access to accumulator 0"]
pub mod interp1_accum0;
#[doc = "INTERP1_ACCUM1 (rw) register accessor: Read/write access to accumulator 1  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_accum1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_accum1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_accum1`]
module"]
pub type INTERP1_ACCUM1 = crate::Reg<interp1_accum1::INTERP1_ACCUM1_SPEC>;
#[doc = "Read/write access to accumulator 1"]
pub mod interp1_accum1;
#[doc = "INTERP1_BASE0 (rw) register accessor: Read/write access to BASE0 register.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_base0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_base0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_base0`]
module"]
pub type INTERP1_BASE0 = crate::Reg<interp1_base0::INTERP1_BASE0_SPEC>;
#[doc = "Read/write access to BASE0 register."]
pub mod interp1_base0;
#[doc = "INTERP1_BASE1 (rw) register accessor: Read/write access to BASE1 register.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_base1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_base1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_base1`]
module"]
pub type INTERP1_BASE1 = crate::Reg<interp1_base1::INTERP1_BASE1_SPEC>;
#[doc = "Read/write access to BASE1 register."]
pub mod interp1_base1;
#[doc = "INTERP1_BASE2 (rw) register accessor: Read/write access to BASE2 register.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_base2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_base2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_base2`]
module"]
pub type INTERP1_BASE2 = crate::Reg<interp1_base2::INTERP1_BASE2_SPEC>;
#[doc = "Read/write access to BASE2 register."]
pub mod interp1_base2;
#[doc = "INTERP1_POP_LANE0 (rw) register accessor: Read LANE0 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_pop_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_pop_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_pop_lane0`]
module"]
pub type INTERP1_POP_LANE0 = crate::Reg<interp1_pop_lane0::INTERP1_POP_LANE0_SPEC>;
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_lane0;
#[doc = "INTERP1_POP_LANE1 (rw) register accessor: Read LANE1 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_pop_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_pop_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_pop_lane1`]
module"]
pub type INTERP1_POP_LANE1 = crate::Reg<interp1_pop_lane1::INTERP1_POP_LANE1_SPEC>;
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_lane1;
#[doc = "INTERP1_POP_FULL (rw) register accessor: Read FULL result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_pop_full::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_pop_full::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_pop_full`]
module"]
pub type INTERP1_POP_FULL = crate::Reg<interp1_pop_full::INTERP1_POP_FULL_SPEC>;
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_full;
#[doc = "INTERP1_PEEK_LANE0 (rw) register accessor: Read LANE0 result, without altering any internal state (PEEK).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_peek_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_peek_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_peek_lane0`]
module"]
pub type INTERP1_PEEK_LANE0 = crate::Reg<interp1_peek_lane0::INTERP1_PEEK_LANE0_SPEC>;
#[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
pub mod interp1_peek_lane0;
#[doc = "INTERP1_PEEK_LANE1 (rw) register accessor: Read LANE1 result, without altering any internal state (PEEK).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_peek_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_peek_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_peek_lane1`]
module"]
pub type INTERP1_PEEK_LANE1 = crate::Reg<interp1_peek_lane1::INTERP1_PEEK_LANE1_SPEC>;
#[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
pub mod interp1_peek_lane1;
#[doc = "INTERP1_PEEK_FULL (rw) register accessor: Read FULL result, without altering any internal state (PEEK).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_peek_full::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_peek_full::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_peek_full`]
module"]
pub type INTERP1_PEEK_FULL = crate::Reg<interp1_peek_full::INTERP1_PEEK_FULL_SPEC>;
#[doc = "Read FULL result, without altering any internal state (PEEK)."]
pub mod interp1_peek_full;
#[doc = "INTERP1_CTRL_LANE0 (rw) register accessor: Control register for lane 0  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_ctrl_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_ctrl_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_ctrl_lane0`]
module"]
pub type INTERP1_CTRL_LANE0 = crate::Reg<interp1_ctrl_lane0::INTERP1_CTRL_LANE0_SPEC>;
#[doc = "Control register for lane 0"]
pub mod interp1_ctrl_lane0;
#[doc = "INTERP1_CTRL_LANE1 (rw) register accessor: Control register for lane 1  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_ctrl_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_ctrl_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_ctrl_lane1`]
module"]
pub type INTERP1_CTRL_LANE1 = crate::Reg<interp1_ctrl_lane1::INTERP1_CTRL_LANE1_SPEC>;
#[doc = "Control register for lane 1"]
pub mod interp1_ctrl_lane1;
#[doc = "INTERP1_ACCUM0_ADD (rw) register accessor: Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_accum0_add::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_accum0_add::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_accum0_add`]
module"]
pub type INTERP1_ACCUM0_ADD = crate::Reg<interp1_accum0_add::INTERP1_ACCUM0_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
pub mod interp1_accum0_add;
#[doc = "INTERP1_ACCUM1_ADD (rw) register accessor: Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_accum1_add::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_accum1_add::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_accum1_add`]
module"]
pub type INTERP1_ACCUM1_ADD = crate::Reg<interp1_accum1_add::INTERP1_ACCUM1_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
pub mod interp1_accum1_add;
#[doc = "INTERP1_BASE_1AND0 (rw) register accessor: On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_base_1and0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_base_1and0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_base_1and0`]
module"]
pub type INTERP1_BASE_1AND0 = crate::Reg<interp1_base_1and0::INTERP1_BASE_1AND0_SPEC>;
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
pub mod interp1_base_1and0;
#[doc = "SPINLOCK (rw) register accessor: Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 &lt;&lt; lock number.  

You can [`read`](crate::generic::Reg::read) this register and get [`spinlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@spinlock`]
module"]
pub type SPINLOCK = crate::Reg<spinlock::SPINLOCK_SPEC>;
#[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 &lt;&lt; lock number."]
pub mod spinlock;
