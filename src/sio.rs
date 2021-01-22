#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Processor core identifier\\n Value is 0 when read from processor core 0, and 1 when read from processor core 1."]
    pub cpuid: CPUID,
    #[doc = "0x04 - Input value for GPIO pins"]
    pub gpio_in: GPIO_IN,
    #[doc = "0x08 - Input value for QSPI pins"]
    pub gpio_hi_in: GPIO_HI_IN,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - GPIO output value"]
    pub gpio_out: GPIO_OUT,
    #[doc = "0x14 - GPIO output value set"]
    pub gpio_out_set: GPIO_OUT_SET,
    #[doc = "0x18 - GPIO output value clear"]
    pub gpio_out_clr: GPIO_OUT_CLR,
    #[doc = "0x1c - GPIO output value XOR"]
    pub gpio_out_xor: GPIO_OUT_XOR,
    #[doc = "0x20 - GPIO output enable"]
    pub gpio_oe: GPIO_OE,
    #[doc = "0x24 - GPIO output enable set"]
    pub gpio_oe_set: GPIO_OE_SET,
    #[doc = "0x28 - GPIO output enable clear"]
    pub gpio_oe_clr: GPIO_OE_CLR,
    #[doc = "0x2c - GPIO output enable XOR"]
    pub gpio_oe_xor: GPIO_OE_XOR,
    #[doc = "0x30 - QSPI output value"]
    pub gpio_hi_out: GPIO_HI_OUT,
    #[doc = "0x34 - QSPI output value set"]
    pub gpio_hi_out_set: GPIO_HI_OUT_SET,
    #[doc = "0x38 - QSPI output value clear"]
    pub gpio_hi_out_clr: GPIO_HI_OUT_CLR,
    #[doc = "0x3c - QSPI output value XOR"]
    pub gpio_hi_out_xor: GPIO_HI_OUT_XOR,
    #[doc = "0x40 - QSPI output enable"]
    pub gpio_hi_oe: GPIO_HI_OE,
    #[doc = "0x44 - QSPI output enable set"]
    pub gpio_hi_oe_set: GPIO_HI_OE_SET,
    #[doc = "0x48 - QSPI output enable clear"]
    pub gpio_hi_oe_clr: GPIO_HI_OE_CLR,
    #[doc = "0x4c - QSPI output enable XOR"]
    pub gpio_hi_oe_xor: GPIO_HI_OE_XOR,
    #[doc = "0x50 - Status register for inter-core FIFOs (mailboxes).\\n There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.\\n Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).\\n Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).\\n The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
    pub fifo_st: FIFO_ST,
    #[doc = "0x54 - Write access to this core's TX FIFO"]
    pub fifo_wr: FIFO_WR,
    #[doc = "0x58 - Read access to this core's RX FIFO"]
    pub fifo_rd: FIFO_RD,
    #[doc = "0x5c - Spinlock state\\n A bitmap containing the state of all 32 spinlocks (1=locked).\\n Mainly intended for debugging."]
    pub spinlock_st: SPINLOCK_ST,
    #[doc = "0x60 - Divider unsigned dividend\\n Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`.\\n Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.\\n UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an\\n unsigned calculation, and the S alias starts a signed calculation."]
    pub div_udividend: DIV_UDIVIDEND,
    #[doc = "0x64 - Divider unsigned divisor\\n Write to the DIVISOR operand of the divider, i.e. the q in `p / q`.\\n Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.\\n UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an\\n unsigned calculation, and the S alias starts a signed calculation."]
    pub div_udivisor: DIV_UDIVISOR,
    #[doc = "0x68 - Divider signed dividend\\n The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
    pub div_sdividend: DIV_SDIVIDEND,
    #[doc = "0x6c - Divider signed divisor\\n The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
    pub div_sdivisor: DIV_SDIVISOR,
    #[doc = "0x70 - Divider result quotient\\n The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low.\\n For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ.\\n This register can be written to directly, for context save/restore purposes. This halts any\\n in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.\\n Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order\\n REMAINDER, QUOTIENT if CSR_DIRTY is used."]
    pub div_quotient: DIV_QUOTIENT,
    #[doc = "0x74 - Divider result remainder\\n The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low.\\n For signed calculations, REMAINDER is negative only when DIVIDEND is negative.\\n This register can be written to directly, for context save/restore purposes. This halts any\\n in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
    pub div_remainder: DIV_REMAINDER,
    #[doc = "0x78 - Control and status register for divider."]
    pub div_csr: DIV_CSR,
    _reserved30: [u8; 4usize],
    #[doc = "0x80 - Read/write access to accumulator 0"]
    pub interp0_accum0: INTERP0_ACCUM0,
    #[doc = "0x84 - Read/write access to accumulator 1"]
    pub interp0_accum1: INTERP0_ACCUM1,
    #[doc = "0x88 - Read/write access to BASE0 register."]
    pub interp0_base0: INTERP0_BASE0,
    #[doc = "0x8c - Read/write access to BASE1 register."]
    pub interp0_base1: INTERP0_BASE1,
    #[doc = "0x90 - Read/write access to BASE2 register."]
    pub interp0_base2: INTERP0_BASE2,
    #[doc = "0x94 - Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp0_pop_lane0: INTERP0_POP_LANE0,
    #[doc = "0x98 - Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp0_pop_lane1: INTERP0_POP_LANE1,
    #[doc = "0x9c - Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp0_pop_full: INTERP0_POP_FULL,
    #[doc = "0xa0 - Read LANE0 result, without altering any internal state (PEEK)."]
    pub interp0_peek_lane0: INTERP0_PEEK_LANE0,
    #[doc = "0xa4 - Read LANE1 result, without altering any internal state (PEEK)."]
    pub interp0_peek_lane1: INTERP0_PEEK_LANE1,
    #[doc = "0xa8 - Read FULL result, without altering any internal state (PEEK)."]
    pub interp0_peek_full: INTERP0_PEEK_FULL,
    #[doc = "0xac - Control register for lane 0"]
    pub interp0_ctrl_lane0: INTERP0_CTRL_LANE0,
    #[doc = "0xb0 - Control register for lane 1"]
    pub interp0_ctrl_lane1: INTERP0_CTRL_LANE1,
    #[doc = "0xb4 - Values written here are atomically added to ACCUM0\\n Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    pub interp0_accum0_add: INTERP0_ACCUM0_ADD,
    #[doc = "0xb8 - Values written here are atomically added to ACCUM1\\n Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    pub interp0_accum1_add: INTERP0_ACCUM1_ADD,
    #[doc = "0xbc - On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.\\n Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    pub interp0_base_1and0: INTERP0_BASE_1AND0,
    #[doc = "0xc0 - Read/write access to accumulator 0"]
    pub interp1_accum0: INTERP1_ACCUM0,
    #[doc = "0xc4 - Read/write access to accumulator 1"]
    pub interp1_accum1: INTERP1_ACCUM1,
    #[doc = "0xc8 - Read/write access to BASE0 register."]
    pub interp1_base0: INTERP1_BASE0,
    #[doc = "0xcc - Read/write access to BASE1 register."]
    pub interp1_base1: INTERP1_BASE1,
    #[doc = "0xd0 - Read/write access to BASE2 register."]
    pub interp1_base2: INTERP1_BASE2,
    #[doc = "0xd4 - Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp1_pop_lane0: INTERP1_POP_LANE0,
    #[doc = "0xd8 - Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp1_pop_lane1: INTERP1_POP_LANE1,
    #[doc = "0xdc - Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp1_pop_full: INTERP1_POP_FULL,
    #[doc = "0xe0 - Read LANE0 result, without altering any internal state (PEEK)."]
    pub interp1_peek_lane0: INTERP1_PEEK_LANE0,
    #[doc = "0xe4 - Read LANE1 result, without altering any internal state (PEEK)."]
    pub interp1_peek_lane1: INTERP1_PEEK_LANE1,
    #[doc = "0xe8 - Read FULL result, without altering any internal state (PEEK)."]
    pub interp1_peek_full: INTERP1_PEEK_FULL,
    #[doc = "0xec - Control register for lane 0"]
    pub interp1_ctrl_lane0: INTERP1_CTRL_LANE0,
    #[doc = "0xf0 - Control register for lane 1"]
    pub interp1_ctrl_lane1: INTERP1_CTRL_LANE1,
    #[doc = "0xf4 - Values written here are atomically added to ACCUM0\\n Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    pub interp1_accum0_add: INTERP1_ACCUM0_ADD,
    #[doc = "0xf8 - Values written here are atomically added to ACCUM1\\n Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    pub interp1_accum1_add: INTERP1_ACCUM1_ADD,
    #[doc = "0xfc - On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.\\n Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    pub interp1_base_1and0: INTERP1_BASE_1AND0,
    #[doc = "0x100 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock0: SPINLOCK0,
    #[doc = "0x104 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock1: SPINLOCK1,
    #[doc = "0x108 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock2: SPINLOCK2,
    #[doc = "0x10c - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock3: SPINLOCK3,
    #[doc = "0x110 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock4: SPINLOCK4,
    #[doc = "0x114 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock5: SPINLOCK5,
    #[doc = "0x118 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock6: SPINLOCK6,
    #[doc = "0x11c - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock7: SPINLOCK7,
    #[doc = "0x120 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock8: SPINLOCK8,
    #[doc = "0x124 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock9: SPINLOCK9,
    #[doc = "0x128 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock10: SPINLOCK10,
    #[doc = "0x12c - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock11: SPINLOCK11,
    #[doc = "0x130 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock12: SPINLOCK12,
    #[doc = "0x134 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock13: SPINLOCK13,
    #[doc = "0x138 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock14: SPINLOCK14,
    #[doc = "0x13c - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock15: SPINLOCK15,
    #[doc = "0x140 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock16: SPINLOCK16,
    #[doc = "0x144 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock17: SPINLOCK17,
    #[doc = "0x148 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock18: SPINLOCK18,
    #[doc = "0x14c - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock19: SPINLOCK19,
    #[doc = "0x150 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock20: SPINLOCK20,
    #[doc = "0x154 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock21: SPINLOCK21,
    #[doc = "0x158 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock22: SPINLOCK22,
    #[doc = "0x15c - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock23: SPINLOCK23,
    #[doc = "0x160 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock24: SPINLOCK24,
    #[doc = "0x164 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock25: SPINLOCK25,
    #[doc = "0x168 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock26: SPINLOCK26,
    #[doc = "0x16c - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock27: SPINLOCK27,
    #[doc = "0x170 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock28: SPINLOCK28,
    #[doc = "0x174 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock29: SPINLOCK29,
    #[doc = "0x178 - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock30: SPINLOCK30,
    #[doc = "0x17c - Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
    pub spinlock31: SPINLOCK31,
}
#[doc = "Processor core identifier\\n Value is 0 when read from processor core 0, and 1 when read from processor core 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "Processor core identifier\\n Value is 0 when read from processor core 0, and 1 when read from processor core 1."]
pub mod cpuid;
#[doc = "Input value for GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_in](gpio_in) module"]
pub type GPIO_IN = crate::Reg<u32, _GPIO_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_IN;
#[doc = "`read()` method returns [gpio_in::R](gpio_in::R) reader structure"]
impl crate::Readable for GPIO_IN {}
#[doc = "Input value for GPIO pins"]
pub mod gpio_in;
#[doc = "Input value for QSPI pins\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_in](gpio_hi_in) module"]
pub type GPIO_HI_IN = crate::Reg<u32, _GPIO_HI_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_IN;
#[doc = "`read()` method returns [gpio_hi_in::R](gpio_hi_in::R) reader structure"]
impl crate::Readable for GPIO_HI_IN {}
#[doc = "Input value for QSPI pins"]
pub mod gpio_hi_in;
#[doc = "GPIO output value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out](gpio_out) module"]
pub type GPIO_OUT = crate::Reg<u32, _GPIO_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT;
#[doc = "`read()` method returns [gpio_out::R](gpio_out::R) reader structure"]
impl crate::Readable for GPIO_OUT {}
#[doc = "`write(|w| ..)` method takes [gpio_out::W](gpio_out::W) writer structure"]
impl crate::Writable for GPIO_OUT {}
#[doc = "GPIO output value"]
pub mod gpio_out;
#[doc = "GPIO output value set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out_set](gpio_out_set) module"]
pub type GPIO_OUT_SET = crate::Reg<u32, _GPIO_OUT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT_SET;
#[doc = "`read()` method returns [gpio_out_set::R](gpio_out_set::R) reader structure"]
impl crate::Readable for GPIO_OUT_SET {}
#[doc = "`write(|w| ..)` method takes [gpio_out_set::W](gpio_out_set::W) writer structure"]
impl crate::Writable for GPIO_OUT_SET {}
#[doc = "GPIO output value set"]
pub mod gpio_out_set;
#[doc = "GPIO output value clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out_clr](gpio_out_clr) module"]
pub type GPIO_OUT_CLR = crate::Reg<u32, _GPIO_OUT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT_CLR;
#[doc = "`read()` method returns [gpio_out_clr::R](gpio_out_clr::R) reader structure"]
impl crate::Readable for GPIO_OUT_CLR {}
#[doc = "`write(|w| ..)` method takes [gpio_out_clr::W](gpio_out_clr::W) writer structure"]
impl crate::Writable for GPIO_OUT_CLR {}
#[doc = "GPIO output value clear"]
pub mod gpio_out_clr;
#[doc = "GPIO output value XOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out_xor](gpio_out_xor) module"]
pub type GPIO_OUT_XOR = crate::Reg<u32, _GPIO_OUT_XOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT_XOR;
#[doc = "`read()` method returns [gpio_out_xor::R](gpio_out_xor::R) reader structure"]
impl crate::Readable for GPIO_OUT_XOR {}
#[doc = "`write(|w| ..)` method takes [gpio_out_xor::W](gpio_out_xor::W) writer structure"]
impl crate::Writable for GPIO_OUT_XOR {}
#[doc = "GPIO output value XOR"]
pub mod gpio_out_xor;
#[doc = "GPIO output enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_oe](gpio_oe) module"]
pub type GPIO_OE = crate::Reg<u32, _GPIO_OE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OE;
#[doc = "`read()` method returns [gpio_oe::R](gpio_oe::R) reader structure"]
impl crate::Readable for GPIO_OE {}
#[doc = "`write(|w| ..)` method takes [gpio_oe::W](gpio_oe::W) writer structure"]
impl crate::Writable for GPIO_OE {}
#[doc = "GPIO output enable"]
pub mod gpio_oe;
#[doc = "GPIO output enable set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_oe_set](gpio_oe_set) module"]
pub type GPIO_OE_SET = crate::Reg<u32, _GPIO_OE_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OE_SET;
#[doc = "`read()` method returns [gpio_oe_set::R](gpio_oe_set::R) reader structure"]
impl crate::Readable for GPIO_OE_SET {}
#[doc = "`write(|w| ..)` method takes [gpio_oe_set::W](gpio_oe_set::W) writer structure"]
impl crate::Writable for GPIO_OE_SET {}
#[doc = "GPIO output enable set"]
pub mod gpio_oe_set;
#[doc = "GPIO output enable clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_oe_clr](gpio_oe_clr) module"]
pub type GPIO_OE_CLR = crate::Reg<u32, _GPIO_OE_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OE_CLR;
#[doc = "`read()` method returns [gpio_oe_clr::R](gpio_oe_clr::R) reader structure"]
impl crate::Readable for GPIO_OE_CLR {}
#[doc = "`write(|w| ..)` method takes [gpio_oe_clr::W](gpio_oe_clr::W) writer structure"]
impl crate::Writable for GPIO_OE_CLR {}
#[doc = "GPIO output enable clear"]
pub mod gpio_oe_clr;
#[doc = "GPIO output enable XOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_oe_xor](gpio_oe_xor) module"]
pub type GPIO_OE_XOR = crate::Reg<u32, _GPIO_OE_XOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OE_XOR;
#[doc = "`read()` method returns [gpio_oe_xor::R](gpio_oe_xor::R) reader structure"]
impl crate::Readable for GPIO_OE_XOR {}
#[doc = "`write(|w| ..)` method takes [gpio_oe_xor::W](gpio_oe_xor::W) writer structure"]
impl crate::Writable for GPIO_OE_XOR {}
#[doc = "GPIO output enable XOR"]
pub mod gpio_oe_xor;
#[doc = "QSPI output value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_out](gpio_hi_out) module"]
pub type GPIO_HI_OUT = crate::Reg<u32, _GPIO_HI_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_OUT;
#[doc = "`read()` method returns [gpio_hi_out::R](gpio_hi_out::R) reader structure"]
impl crate::Readable for GPIO_HI_OUT {}
#[doc = "`write(|w| ..)` method takes [gpio_hi_out::W](gpio_hi_out::W) writer structure"]
impl crate::Writable for GPIO_HI_OUT {}
#[doc = "QSPI output value"]
pub mod gpio_hi_out;
#[doc = "QSPI output value set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_out_set](gpio_hi_out_set) module"]
pub type GPIO_HI_OUT_SET = crate::Reg<u32, _GPIO_HI_OUT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_OUT_SET;
#[doc = "`read()` method returns [gpio_hi_out_set::R](gpio_hi_out_set::R) reader structure"]
impl crate::Readable for GPIO_HI_OUT_SET {}
#[doc = "`write(|w| ..)` method takes [gpio_hi_out_set::W](gpio_hi_out_set::W) writer structure"]
impl crate::Writable for GPIO_HI_OUT_SET {}
#[doc = "QSPI output value set"]
pub mod gpio_hi_out_set;
#[doc = "QSPI output value clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_out_clr](gpio_hi_out_clr) module"]
pub type GPIO_HI_OUT_CLR = crate::Reg<u32, _GPIO_HI_OUT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_OUT_CLR;
#[doc = "`read()` method returns [gpio_hi_out_clr::R](gpio_hi_out_clr::R) reader structure"]
impl crate::Readable for GPIO_HI_OUT_CLR {}
#[doc = "`write(|w| ..)` method takes [gpio_hi_out_clr::W](gpio_hi_out_clr::W) writer structure"]
impl crate::Writable for GPIO_HI_OUT_CLR {}
#[doc = "QSPI output value clear"]
pub mod gpio_hi_out_clr;
#[doc = "QSPI output value XOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_out_xor](gpio_hi_out_xor) module"]
pub type GPIO_HI_OUT_XOR = crate::Reg<u32, _GPIO_HI_OUT_XOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_OUT_XOR;
#[doc = "`read()` method returns [gpio_hi_out_xor::R](gpio_hi_out_xor::R) reader structure"]
impl crate::Readable for GPIO_HI_OUT_XOR {}
#[doc = "`write(|w| ..)` method takes [gpio_hi_out_xor::W](gpio_hi_out_xor::W) writer structure"]
impl crate::Writable for GPIO_HI_OUT_XOR {}
#[doc = "QSPI output value XOR"]
pub mod gpio_hi_out_xor;
#[doc = "QSPI output enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_oe](gpio_hi_oe) module"]
pub type GPIO_HI_OE = crate::Reg<u32, _GPIO_HI_OE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_OE;
#[doc = "`read()` method returns [gpio_hi_oe::R](gpio_hi_oe::R) reader structure"]
impl crate::Readable for GPIO_HI_OE {}
#[doc = "`write(|w| ..)` method takes [gpio_hi_oe::W](gpio_hi_oe::W) writer structure"]
impl crate::Writable for GPIO_HI_OE {}
#[doc = "QSPI output enable"]
pub mod gpio_hi_oe;
#[doc = "QSPI output enable set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_oe_set](gpio_hi_oe_set) module"]
pub type GPIO_HI_OE_SET = crate::Reg<u32, _GPIO_HI_OE_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_OE_SET;
#[doc = "`read()` method returns [gpio_hi_oe_set::R](gpio_hi_oe_set::R) reader structure"]
impl crate::Readable for GPIO_HI_OE_SET {}
#[doc = "`write(|w| ..)` method takes [gpio_hi_oe_set::W](gpio_hi_oe_set::W) writer structure"]
impl crate::Writable for GPIO_HI_OE_SET {}
#[doc = "QSPI output enable set"]
pub mod gpio_hi_oe_set;
#[doc = "QSPI output enable clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_oe_clr](gpio_hi_oe_clr) module"]
pub type GPIO_HI_OE_CLR = crate::Reg<u32, _GPIO_HI_OE_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_OE_CLR;
#[doc = "`read()` method returns [gpio_hi_oe_clr::R](gpio_hi_oe_clr::R) reader structure"]
impl crate::Readable for GPIO_HI_OE_CLR {}
#[doc = "`write(|w| ..)` method takes [gpio_hi_oe_clr::W](gpio_hi_oe_clr::W) writer structure"]
impl crate::Writable for GPIO_HI_OE_CLR {}
#[doc = "QSPI output enable clear"]
pub mod gpio_hi_oe_clr;
#[doc = "QSPI output enable XOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hi_oe_xor](gpio_hi_oe_xor) module"]
pub type GPIO_HI_OE_XOR = crate::Reg<u32, _GPIO_HI_OE_XOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_HI_OE_XOR;
#[doc = "`read()` method returns [gpio_hi_oe_xor::R](gpio_hi_oe_xor::R) reader structure"]
impl crate::Readable for GPIO_HI_OE_XOR {}
#[doc = "`write(|w| ..)` method takes [gpio_hi_oe_xor::W](gpio_hi_oe_xor::W) writer structure"]
impl crate::Writable for GPIO_HI_OE_XOR {}
#[doc = "QSPI output enable XOR"]
pub mod gpio_hi_oe_xor;
#[doc = "Status register for inter-core FIFOs (mailboxes).\\n There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.\\n Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).\\n Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).\\n The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_st](fifo_st) module"]
pub type FIFO_ST = crate::Reg<u32, _FIFO_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_ST;
#[doc = "`read()` method returns [fifo_st::R](fifo_st::R) reader structure"]
impl crate::Readable for FIFO_ST {}
#[doc = "`write(|w| ..)` method takes [fifo_st::W](fifo_st::W) writer structure"]
impl crate::Writable for FIFO_ST {}
#[doc = "Status register for inter-core FIFOs (mailboxes).\\n There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.\\n Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).\\n Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).\\n The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
pub mod fifo_st;
#[doc = "Write access to this core's TX FIFO\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_wr](fifo_wr) module"]
pub type FIFO_WR = crate::Reg<u32, _FIFO_WR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_WR;
#[doc = "`write(|w| ..)` method takes [fifo_wr::W](fifo_wr::W) writer structure"]
impl crate::Writable for FIFO_WR {}
#[doc = "Write access to this core's TX FIFO"]
pub mod fifo_wr;
#[doc = "Read access to this core's RX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_rd](fifo_rd) module"]
pub type FIFO_RD = crate::Reg<u32, _FIFO_RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_RD;
#[doc = "`read()` method returns [fifo_rd::R](fifo_rd::R) reader structure"]
impl crate::Readable for FIFO_RD {}
#[doc = "Read access to this core's RX FIFO"]
pub mod fifo_rd;
#[doc = "Spinlock state\\n A bitmap containing the state of all 32 spinlocks (1=locked).\\n Mainly intended for debugging.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock_st](spinlock_st) module"]
pub type SPINLOCK_ST = crate::Reg<u32, _SPINLOCK_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK_ST;
#[doc = "`read()` method returns [spinlock_st::R](spinlock_st::R) reader structure"]
impl crate::Readable for SPINLOCK_ST {}
#[doc = "Spinlock state\\n A bitmap containing the state of all 32 spinlocks (1=locked).\\n Mainly intended for debugging."]
pub mod spinlock_st;
#[doc = "Divider unsigned dividend\\n Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`.\\n Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.\\n UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an\\n unsigned calculation, and the S alias starts a signed calculation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_udividend](div_udividend) module"]
pub type DIV_UDIVIDEND = crate::Reg<u32, _DIV_UDIVIDEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_UDIVIDEND;
#[doc = "`read()` method returns [div_udividend::R](div_udividend::R) reader structure"]
impl crate::Readable for DIV_UDIVIDEND {}
#[doc = "`write(|w| ..)` method takes [div_udividend::W](div_udividend::W) writer structure"]
impl crate::Writable for DIV_UDIVIDEND {}
#[doc = "Divider unsigned dividend\\n Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`.\\n Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.\\n UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an\\n unsigned calculation, and the S alias starts a signed calculation."]
pub mod div_udividend;
#[doc = "Divider unsigned divisor\\n Write to the DIVISOR operand of the divider, i.e. the q in `p / q`.\\n Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.\\n UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an\\n unsigned calculation, and the S alias starts a signed calculation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_udivisor](div_udivisor) module"]
pub type DIV_UDIVISOR = crate::Reg<u32, _DIV_UDIVISOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_UDIVISOR;
#[doc = "`read()` method returns [div_udivisor::R](div_udivisor::R) reader structure"]
impl crate::Readable for DIV_UDIVISOR {}
#[doc = "`write(|w| ..)` method takes [div_udivisor::W](div_udivisor::W) writer structure"]
impl crate::Writable for DIV_UDIVISOR {}
#[doc = "Divider unsigned divisor\\n Write to the DIVISOR operand of the divider, i.e. the q in `p / q`.\\n Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.\\n UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an\\n unsigned calculation, and the S alias starts a signed calculation."]
pub mod div_udivisor;
#[doc = "Divider signed dividend\\n The same as UDIVIDEND, but starts a signed calculation, rather than unsigned.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_sdividend](div_sdividend) module"]
pub type DIV_SDIVIDEND = crate::Reg<u32, _DIV_SDIVIDEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_SDIVIDEND;
#[doc = "`read()` method returns [div_sdividend::R](div_sdividend::R) reader structure"]
impl crate::Readable for DIV_SDIVIDEND {}
#[doc = "`write(|w| ..)` method takes [div_sdividend::W](div_sdividend::W) writer structure"]
impl crate::Writable for DIV_SDIVIDEND {}
#[doc = "Divider signed dividend\\n The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
pub mod div_sdividend;
#[doc = "Divider signed divisor\\n The same as UDIVISOR, but starts a signed calculation, rather than unsigned.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_sdivisor](div_sdivisor) module"]
pub type DIV_SDIVISOR = crate::Reg<u32, _DIV_SDIVISOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_SDIVISOR;
#[doc = "`read()` method returns [div_sdivisor::R](div_sdivisor::R) reader structure"]
impl crate::Readable for DIV_SDIVISOR {}
#[doc = "`write(|w| ..)` method takes [div_sdivisor::W](div_sdivisor::W) writer structure"]
impl crate::Writable for DIV_SDIVISOR {}
#[doc = "Divider signed divisor\\n The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
pub mod div_sdivisor;
#[doc = "Divider result quotient\\n The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low.\\n For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ.\\n This register can be written to directly, for context save/restore purposes. This halts any\\n in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.\\n Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order\\n REMAINDER, QUOTIENT if CSR_DIRTY is used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_quotient](div_quotient) module"]
pub type DIV_QUOTIENT = crate::Reg<u32, _DIV_QUOTIENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_QUOTIENT;
#[doc = "`read()` method returns [div_quotient::R](div_quotient::R) reader structure"]
impl crate::Readable for DIV_QUOTIENT {}
#[doc = "`write(|w| ..)` method takes [div_quotient::W](div_quotient::W) writer structure"]
impl crate::Writable for DIV_QUOTIENT {}
#[doc = "Divider result quotient\\n The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low.\\n For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ.\\n This register can be written to directly, for context save/restore purposes. This halts any\\n in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.\\n Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order\\n REMAINDER, QUOTIENT if CSR_DIRTY is used."]
pub mod div_quotient;
#[doc = "Divider result remainder\\n The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low.\\n For signed calculations, REMAINDER is negative only when DIVIDEND is negative.\\n This register can be written to directly, for context save/restore purposes. This halts any\\n in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_remainder](div_remainder) module"]
pub type DIV_REMAINDER = crate::Reg<u32, _DIV_REMAINDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_REMAINDER;
#[doc = "`read()` method returns [div_remainder::R](div_remainder::R) reader structure"]
impl crate::Readable for DIV_REMAINDER {}
#[doc = "`write(|w| ..)` method takes [div_remainder::W](div_remainder::W) writer structure"]
impl crate::Writable for DIV_REMAINDER {}
#[doc = "Divider result remainder\\n The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low.\\n For signed calculations, REMAINDER is negative only when DIVIDEND is negative.\\n This register can be written to directly, for context save/restore purposes. This halts any\\n in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
pub mod div_remainder;
#[doc = "Control and status register for divider.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_csr](div_csr) module"]
pub type DIV_CSR = crate::Reg<u32, _DIV_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_CSR;
#[doc = "`read()` method returns [div_csr::R](div_csr::R) reader structure"]
impl crate::Readable for DIV_CSR {}
#[doc = "Control and status register for divider."]
pub mod div_csr;
#[doc = "Read/write access to accumulator 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_accum0](interp0_accum0) module"]
pub type INTERP0_ACCUM0 = crate::Reg<u32, _INTERP0_ACCUM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_ACCUM0;
#[doc = "`read()` method returns [interp0_accum0::R](interp0_accum0::R) reader structure"]
impl crate::Readable for INTERP0_ACCUM0 {}
#[doc = "`write(|w| ..)` method takes [interp0_accum0::W](interp0_accum0::W) writer structure"]
impl crate::Writable for INTERP0_ACCUM0 {}
#[doc = "Read/write access to accumulator 0"]
pub mod interp0_accum0;
#[doc = "Read/write access to accumulator 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_accum1](interp0_accum1) module"]
pub type INTERP0_ACCUM1 = crate::Reg<u32, _INTERP0_ACCUM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_ACCUM1;
#[doc = "`read()` method returns [interp0_accum1::R](interp0_accum1::R) reader structure"]
impl crate::Readable for INTERP0_ACCUM1 {}
#[doc = "`write(|w| ..)` method takes [interp0_accum1::W](interp0_accum1::W) writer structure"]
impl crate::Writable for INTERP0_ACCUM1 {}
#[doc = "Read/write access to accumulator 1"]
pub mod interp0_accum1;
#[doc = "Read/write access to BASE0 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_base0](interp0_base0) module"]
pub type INTERP0_BASE0 = crate::Reg<u32, _INTERP0_BASE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_BASE0;
#[doc = "`read()` method returns [interp0_base0::R](interp0_base0::R) reader structure"]
impl crate::Readable for INTERP0_BASE0 {}
#[doc = "`write(|w| ..)` method takes [interp0_base0::W](interp0_base0::W) writer structure"]
impl crate::Writable for INTERP0_BASE0 {}
#[doc = "Read/write access to BASE0 register."]
pub mod interp0_base0;
#[doc = "Read/write access to BASE1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_base1](interp0_base1) module"]
pub type INTERP0_BASE1 = crate::Reg<u32, _INTERP0_BASE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_BASE1;
#[doc = "`read()` method returns [interp0_base1::R](interp0_base1::R) reader structure"]
impl crate::Readable for INTERP0_BASE1 {}
#[doc = "`write(|w| ..)` method takes [interp0_base1::W](interp0_base1::W) writer structure"]
impl crate::Writable for INTERP0_BASE1 {}
#[doc = "Read/write access to BASE1 register."]
pub mod interp0_base1;
#[doc = "Read/write access to BASE2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_base2](interp0_base2) module"]
pub type INTERP0_BASE2 = crate::Reg<u32, _INTERP0_BASE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_BASE2;
#[doc = "`read()` method returns [interp0_base2::R](interp0_base2::R) reader structure"]
impl crate::Readable for INTERP0_BASE2 {}
#[doc = "`write(|w| ..)` method takes [interp0_base2::W](interp0_base2::W) writer structure"]
impl crate::Writable for INTERP0_BASE2 {}
#[doc = "Read/write access to BASE2 register."]
pub mod interp0_base2;
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_pop_lane0](interp0_pop_lane0) module"]
pub type INTERP0_POP_LANE0 = crate::Reg<u32, _INTERP0_POP_LANE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_POP_LANE0;
#[doc = "`read()` method returns [interp0_pop_lane0::R](interp0_pop_lane0::R) reader structure"]
impl crate::Readable for INTERP0_POP_LANE0 {}
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_lane0;
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_pop_lane1](interp0_pop_lane1) module"]
pub type INTERP0_POP_LANE1 = crate::Reg<u32, _INTERP0_POP_LANE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_POP_LANE1;
#[doc = "`read()` method returns [interp0_pop_lane1::R](interp0_pop_lane1::R) reader structure"]
impl crate::Readable for INTERP0_POP_LANE1 {}
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_lane1;
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_pop_full](interp0_pop_full) module"]
pub type INTERP0_POP_FULL = crate::Reg<u32, _INTERP0_POP_FULL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_POP_FULL;
#[doc = "`read()` method returns [interp0_pop_full::R](interp0_pop_full::R) reader structure"]
impl crate::Readable for INTERP0_POP_FULL {}
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_full;
#[doc = "Read LANE0 result, without altering any internal state (PEEK).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_peek_lane0](interp0_peek_lane0) module"]
pub type INTERP0_PEEK_LANE0 = crate::Reg<u32, _INTERP0_PEEK_LANE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_PEEK_LANE0;
#[doc = "`read()` method returns [interp0_peek_lane0::R](interp0_peek_lane0::R) reader structure"]
impl crate::Readable for INTERP0_PEEK_LANE0 {}
#[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
pub mod interp0_peek_lane0;
#[doc = "Read LANE1 result, without altering any internal state (PEEK).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_peek_lane1](interp0_peek_lane1) module"]
pub type INTERP0_PEEK_LANE1 = crate::Reg<u32, _INTERP0_PEEK_LANE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_PEEK_LANE1;
#[doc = "`read()` method returns [interp0_peek_lane1::R](interp0_peek_lane1::R) reader structure"]
impl crate::Readable for INTERP0_PEEK_LANE1 {}
#[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
pub mod interp0_peek_lane1;
#[doc = "Read FULL result, without altering any internal state (PEEK).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_peek_full](interp0_peek_full) module"]
pub type INTERP0_PEEK_FULL = crate::Reg<u32, _INTERP0_PEEK_FULL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_PEEK_FULL;
#[doc = "`read()` method returns [interp0_peek_full::R](interp0_peek_full::R) reader structure"]
impl crate::Readable for INTERP0_PEEK_FULL {}
#[doc = "Read FULL result, without altering any internal state (PEEK)."]
pub mod interp0_peek_full;
#[doc = "Control register for lane 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_ctrl_lane0](interp0_ctrl_lane0) module"]
pub type INTERP0_CTRL_LANE0 = crate::Reg<u32, _INTERP0_CTRL_LANE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_CTRL_LANE0;
#[doc = "`read()` method returns [interp0_ctrl_lane0::R](interp0_ctrl_lane0::R) reader structure"]
impl crate::Readable for INTERP0_CTRL_LANE0 {}
#[doc = "`write(|w| ..)` method takes [interp0_ctrl_lane0::W](interp0_ctrl_lane0::W) writer structure"]
impl crate::Writable for INTERP0_CTRL_LANE0 {}
#[doc = "Control register for lane 0"]
pub mod interp0_ctrl_lane0;
#[doc = "Control register for lane 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_ctrl_lane1](interp0_ctrl_lane1) module"]
pub type INTERP0_CTRL_LANE1 = crate::Reg<u32, _INTERP0_CTRL_LANE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_CTRL_LANE1;
#[doc = "`read()` method returns [interp0_ctrl_lane1::R](interp0_ctrl_lane1::R) reader structure"]
impl crate::Readable for INTERP0_CTRL_LANE1 {}
#[doc = "`write(|w| ..)` method takes [interp0_ctrl_lane1::W](interp0_ctrl_lane1::W) writer structure"]
impl crate::Writable for INTERP0_CTRL_LANE1 {}
#[doc = "Control register for lane 1"]
pub mod interp0_ctrl_lane1;
#[doc = "Values written here are atomically added to ACCUM0\\n Reading yields lane 0's raw shift and mask value (BASE0 not added).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_accum0_add](interp0_accum0_add) module"]
pub type INTERP0_ACCUM0_ADD = crate::Reg<u32, _INTERP0_ACCUM0_ADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_ACCUM0_ADD;
#[doc = "`read()` method returns [interp0_accum0_add::R](interp0_accum0_add::R) reader structure"]
impl crate::Readable for INTERP0_ACCUM0_ADD {}
#[doc = "`write(|w| ..)` method takes [interp0_accum0_add::W](interp0_accum0_add::W) writer structure"]
impl crate::Writable for INTERP0_ACCUM0_ADD {}
#[doc = "Values written here are atomically added to ACCUM0\\n Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
pub mod interp0_accum0_add;
#[doc = "Values written here are atomically added to ACCUM1\\n Reading yields lane 1's raw shift and mask value (BASE1 not added).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_accum1_add](interp0_accum1_add) module"]
pub type INTERP0_ACCUM1_ADD = crate::Reg<u32, _INTERP0_ACCUM1_ADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_ACCUM1_ADD;
#[doc = "`read()` method returns [interp0_accum1_add::R](interp0_accum1_add::R) reader structure"]
impl crate::Readable for INTERP0_ACCUM1_ADD {}
#[doc = "`write(|w| ..)` method takes [interp0_accum1_add::W](interp0_accum1_add::W) writer structure"]
impl crate::Writable for INTERP0_ACCUM1_ADD {}
#[doc = "Values written here are atomically added to ACCUM1\\n Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
pub mod interp0_accum1_add;
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.\\n Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp0_base_1and0](interp0_base_1and0) module"]
pub type INTERP0_BASE_1AND0 = crate::Reg<u32, _INTERP0_BASE_1AND0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP0_BASE_1AND0;
#[doc = "`read()` method returns [interp0_base_1and0::R](interp0_base_1and0::R) reader structure"]
impl crate::Readable for INTERP0_BASE_1AND0 {}
#[doc = "`write(|w| ..)` method takes [interp0_base_1and0::W](interp0_base_1and0::W) writer structure"]
impl crate::Writable for INTERP0_BASE_1AND0 {}
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.\\n Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
pub mod interp0_base_1and0;
#[doc = "Read/write access to accumulator 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_accum0](interp1_accum0) module"]
pub type INTERP1_ACCUM0 = crate::Reg<u32, _INTERP1_ACCUM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_ACCUM0;
#[doc = "`read()` method returns [interp1_accum0::R](interp1_accum0::R) reader structure"]
impl crate::Readable for INTERP1_ACCUM0 {}
#[doc = "`write(|w| ..)` method takes [interp1_accum0::W](interp1_accum0::W) writer structure"]
impl crate::Writable for INTERP1_ACCUM0 {}
#[doc = "Read/write access to accumulator 0"]
pub mod interp1_accum0;
#[doc = "Read/write access to accumulator 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_accum1](interp1_accum1) module"]
pub type INTERP1_ACCUM1 = crate::Reg<u32, _INTERP1_ACCUM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_ACCUM1;
#[doc = "`read()` method returns [interp1_accum1::R](interp1_accum1::R) reader structure"]
impl crate::Readable for INTERP1_ACCUM1 {}
#[doc = "`write(|w| ..)` method takes [interp1_accum1::W](interp1_accum1::W) writer structure"]
impl crate::Writable for INTERP1_ACCUM1 {}
#[doc = "Read/write access to accumulator 1"]
pub mod interp1_accum1;
#[doc = "Read/write access to BASE0 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_base0](interp1_base0) module"]
pub type INTERP1_BASE0 = crate::Reg<u32, _INTERP1_BASE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_BASE0;
#[doc = "`read()` method returns [interp1_base0::R](interp1_base0::R) reader structure"]
impl crate::Readable for INTERP1_BASE0 {}
#[doc = "`write(|w| ..)` method takes [interp1_base0::W](interp1_base0::W) writer structure"]
impl crate::Writable for INTERP1_BASE0 {}
#[doc = "Read/write access to BASE0 register."]
pub mod interp1_base0;
#[doc = "Read/write access to BASE1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_base1](interp1_base1) module"]
pub type INTERP1_BASE1 = crate::Reg<u32, _INTERP1_BASE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_BASE1;
#[doc = "`read()` method returns [interp1_base1::R](interp1_base1::R) reader structure"]
impl crate::Readable for INTERP1_BASE1 {}
#[doc = "`write(|w| ..)` method takes [interp1_base1::W](interp1_base1::W) writer structure"]
impl crate::Writable for INTERP1_BASE1 {}
#[doc = "Read/write access to BASE1 register."]
pub mod interp1_base1;
#[doc = "Read/write access to BASE2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_base2](interp1_base2) module"]
pub type INTERP1_BASE2 = crate::Reg<u32, _INTERP1_BASE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_BASE2;
#[doc = "`read()` method returns [interp1_base2::R](interp1_base2::R) reader structure"]
impl crate::Readable for INTERP1_BASE2 {}
#[doc = "`write(|w| ..)` method takes [interp1_base2::W](interp1_base2::W) writer structure"]
impl crate::Writable for INTERP1_BASE2 {}
#[doc = "Read/write access to BASE2 register."]
pub mod interp1_base2;
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_pop_lane0](interp1_pop_lane0) module"]
pub type INTERP1_POP_LANE0 = crate::Reg<u32, _INTERP1_POP_LANE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_POP_LANE0;
#[doc = "`read()` method returns [interp1_pop_lane0::R](interp1_pop_lane0::R) reader structure"]
impl crate::Readable for INTERP1_POP_LANE0 {}
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_lane0;
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_pop_lane1](interp1_pop_lane1) module"]
pub type INTERP1_POP_LANE1 = crate::Reg<u32, _INTERP1_POP_LANE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_POP_LANE1;
#[doc = "`read()` method returns [interp1_pop_lane1::R](interp1_pop_lane1::R) reader structure"]
impl crate::Readable for INTERP1_POP_LANE1 {}
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_lane1;
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_pop_full](interp1_pop_full) module"]
pub type INTERP1_POP_FULL = crate::Reg<u32, _INTERP1_POP_FULL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_POP_FULL;
#[doc = "`read()` method returns [interp1_pop_full::R](interp1_pop_full::R) reader structure"]
impl crate::Readable for INTERP1_POP_FULL {}
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_full;
#[doc = "Read LANE0 result, without altering any internal state (PEEK).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_peek_lane0](interp1_peek_lane0) module"]
pub type INTERP1_PEEK_LANE0 = crate::Reg<u32, _INTERP1_PEEK_LANE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_PEEK_LANE0;
#[doc = "`read()` method returns [interp1_peek_lane0::R](interp1_peek_lane0::R) reader structure"]
impl crate::Readable for INTERP1_PEEK_LANE0 {}
#[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
pub mod interp1_peek_lane0;
#[doc = "Read LANE1 result, without altering any internal state (PEEK).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_peek_lane1](interp1_peek_lane1) module"]
pub type INTERP1_PEEK_LANE1 = crate::Reg<u32, _INTERP1_PEEK_LANE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_PEEK_LANE1;
#[doc = "`read()` method returns [interp1_peek_lane1::R](interp1_peek_lane1::R) reader structure"]
impl crate::Readable for INTERP1_PEEK_LANE1 {}
#[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
pub mod interp1_peek_lane1;
#[doc = "Read FULL result, without altering any internal state (PEEK).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_peek_full](interp1_peek_full) module"]
pub type INTERP1_PEEK_FULL = crate::Reg<u32, _INTERP1_PEEK_FULL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_PEEK_FULL;
#[doc = "`read()` method returns [interp1_peek_full::R](interp1_peek_full::R) reader structure"]
impl crate::Readable for INTERP1_PEEK_FULL {}
#[doc = "Read FULL result, without altering any internal state (PEEK)."]
pub mod interp1_peek_full;
#[doc = "Control register for lane 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_ctrl_lane0](interp1_ctrl_lane0) module"]
pub type INTERP1_CTRL_LANE0 = crate::Reg<u32, _INTERP1_CTRL_LANE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_CTRL_LANE0;
#[doc = "`read()` method returns [interp1_ctrl_lane0::R](interp1_ctrl_lane0::R) reader structure"]
impl crate::Readable for INTERP1_CTRL_LANE0 {}
#[doc = "`write(|w| ..)` method takes [interp1_ctrl_lane0::W](interp1_ctrl_lane0::W) writer structure"]
impl crate::Writable for INTERP1_CTRL_LANE0 {}
#[doc = "Control register for lane 0"]
pub mod interp1_ctrl_lane0;
#[doc = "Control register for lane 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_ctrl_lane1](interp1_ctrl_lane1) module"]
pub type INTERP1_CTRL_LANE1 = crate::Reg<u32, _INTERP1_CTRL_LANE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_CTRL_LANE1;
#[doc = "`read()` method returns [interp1_ctrl_lane1::R](interp1_ctrl_lane1::R) reader structure"]
impl crate::Readable for INTERP1_CTRL_LANE1 {}
#[doc = "`write(|w| ..)` method takes [interp1_ctrl_lane1::W](interp1_ctrl_lane1::W) writer structure"]
impl crate::Writable for INTERP1_CTRL_LANE1 {}
#[doc = "Control register for lane 1"]
pub mod interp1_ctrl_lane1;
#[doc = "Values written here are atomically added to ACCUM0\\n Reading yields lane 0's raw shift and mask value (BASE0 not added).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_accum0_add](interp1_accum0_add) module"]
pub type INTERP1_ACCUM0_ADD = crate::Reg<u32, _INTERP1_ACCUM0_ADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_ACCUM0_ADD;
#[doc = "`read()` method returns [interp1_accum0_add::R](interp1_accum0_add::R) reader structure"]
impl crate::Readable for INTERP1_ACCUM0_ADD {}
#[doc = "`write(|w| ..)` method takes [interp1_accum0_add::W](interp1_accum0_add::W) writer structure"]
impl crate::Writable for INTERP1_ACCUM0_ADD {}
#[doc = "Values written here are atomically added to ACCUM0\\n Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
pub mod interp1_accum0_add;
#[doc = "Values written here are atomically added to ACCUM1\\n Reading yields lane 1's raw shift and mask value (BASE1 not added).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_accum1_add](interp1_accum1_add) module"]
pub type INTERP1_ACCUM1_ADD = crate::Reg<u32, _INTERP1_ACCUM1_ADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_ACCUM1_ADD;
#[doc = "`read()` method returns [interp1_accum1_add::R](interp1_accum1_add::R) reader structure"]
impl crate::Readable for INTERP1_ACCUM1_ADD {}
#[doc = "`write(|w| ..)` method takes [interp1_accum1_add::W](interp1_accum1_add::W) writer structure"]
impl crate::Writable for INTERP1_ACCUM1_ADD {}
#[doc = "Values written here are atomically added to ACCUM1\\n Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
pub mod interp1_accum1_add;
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.\\n Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interp1_base_1and0](interp1_base_1and0) module"]
pub type INTERP1_BASE_1AND0 = crate::Reg<u32, _INTERP1_BASE_1AND0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERP1_BASE_1AND0;
#[doc = "`read()` method returns [interp1_base_1and0::R](interp1_base_1and0::R) reader structure"]
impl crate::Readable for INTERP1_BASE_1AND0 {}
#[doc = "`write(|w| ..)` method takes [interp1_base_1and0::W](interp1_base_1and0::W) writer structure"]
impl crate::Writable for INTERP1_BASE_1AND0 {}
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.\\n Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
pub mod interp1_base_1and0;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock0](spinlock0) module"]
pub type SPINLOCK0 = crate::Reg<u32, _SPINLOCK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK0;
#[doc = "`read()` method returns [spinlock0::R](spinlock0::R) reader structure"]
impl crate::Readable for SPINLOCK0 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock0;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock1](spinlock1) module"]
pub type SPINLOCK1 = crate::Reg<u32, _SPINLOCK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK1;
#[doc = "`read()` method returns [spinlock1::R](spinlock1::R) reader structure"]
impl crate::Readable for SPINLOCK1 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock1;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock2](spinlock2) module"]
pub type SPINLOCK2 = crate::Reg<u32, _SPINLOCK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK2;
#[doc = "`read()` method returns [spinlock2::R](spinlock2::R) reader structure"]
impl crate::Readable for SPINLOCK2 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock2;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock3](spinlock3) module"]
pub type SPINLOCK3 = crate::Reg<u32, _SPINLOCK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK3;
#[doc = "`read()` method returns [spinlock3::R](spinlock3::R) reader structure"]
impl crate::Readable for SPINLOCK3 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock3;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock4](spinlock4) module"]
pub type SPINLOCK4 = crate::Reg<u32, _SPINLOCK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK4;
#[doc = "`read()` method returns [spinlock4::R](spinlock4::R) reader structure"]
impl crate::Readable for SPINLOCK4 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock4;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock5](spinlock5) module"]
pub type SPINLOCK5 = crate::Reg<u32, _SPINLOCK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK5;
#[doc = "`read()` method returns [spinlock5::R](spinlock5::R) reader structure"]
impl crate::Readable for SPINLOCK5 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock5;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock6](spinlock6) module"]
pub type SPINLOCK6 = crate::Reg<u32, _SPINLOCK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK6;
#[doc = "`read()` method returns [spinlock6::R](spinlock6::R) reader structure"]
impl crate::Readable for SPINLOCK6 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock6;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock7](spinlock7) module"]
pub type SPINLOCK7 = crate::Reg<u32, _SPINLOCK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK7;
#[doc = "`read()` method returns [spinlock7::R](spinlock7::R) reader structure"]
impl crate::Readable for SPINLOCK7 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock7;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock8](spinlock8) module"]
pub type SPINLOCK8 = crate::Reg<u32, _SPINLOCK8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK8;
#[doc = "`read()` method returns [spinlock8::R](spinlock8::R) reader structure"]
impl crate::Readable for SPINLOCK8 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock8;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock9](spinlock9) module"]
pub type SPINLOCK9 = crate::Reg<u32, _SPINLOCK9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK9;
#[doc = "`read()` method returns [spinlock9::R](spinlock9::R) reader structure"]
impl crate::Readable for SPINLOCK9 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock9;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock10](spinlock10) module"]
pub type SPINLOCK10 = crate::Reg<u32, _SPINLOCK10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK10;
#[doc = "`read()` method returns [spinlock10::R](spinlock10::R) reader structure"]
impl crate::Readable for SPINLOCK10 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock10;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock11](spinlock11) module"]
pub type SPINLOCK11 = crate::Reg<u32, _SPINLOCK11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK11;
#[doc = "`read()` method returns [spinlock11::R](spinlock11::R) reader structure"]
impl crate::Readable for SPINLOCK11 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock11;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock12](spinlock12) module"]
pub type SPINLOCK12 = crate::Reg<u32, _SPINLOCK12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK12;
#[doc = "`read()` method returns [spinlock12::R](spinlock12::R) reader structure"]
impl crate::Readable for SPINLOCK12 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock12;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock13](spinlock13) module"]
pub type SPINLOCK13 = crate::Reg<u32, _SPINLOCK13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK13;
#[doc = "`read()` method returns [spinlock13::R](spinlock13::R) reader structure"]
impl crate::Readable for SPINLOCK13 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock13;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock14](spinlock14) module"]
pub type SPINLOCK14 = crate::Reg<u32, _SPINLOCK14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK14;
#[doc = "`read()` method returns [spinlock14::R](spinlock14::R) reader structure"]
impl crate::Readable for SPINLOCK14 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock14;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock15](spinlock15) module"]
pub type SPINLOCK15 = crate::Reg<u32, _SPINLOCK15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK15;
#[doc = "`read()` method returns [spinlock15::R](spinlock15::R) reader structure"]
impl crate::Readable for SPINLOCK15 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock15;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock16](spinlock16) module"]
pub type SPINLOCK16 = crate::Reg<u32, _SPINLOCK16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK16;
#[doc = "`read()` method returns [spinlock16::R](spinlock16::R) reader structure"]
impl crate::Readable for SPINLOCK16 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock16;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock17](spinlock17) module"]
pub type SPINLOCK17 = crate::Reg<u32, _SPINLOCK17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK17;
#[doc = "`read()` method returns [spinlock17::R](spinlock17::R) reader structure"]
impl crate::Readable for SPINLOCK17 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock17;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock18](spinlock18) module"]
pub type SPINLOCK18 = crate::Reg<u32, _SPINLOCK18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK18;
#[doc = "`read()` method returns [spinlock18::R](spinlock18::R) reader structure"]
impl crate::Readable for SPINLOCK18 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock18;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock19](spinlock19) module"]
pub type SPINLOCK19 = crate::Reg<u32, _SPINLOCK19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK19;
#[doc = "`read()` method returns [spinlock19::R](spinlock19::R) reader structure"]
impl crate::Readable for SPINLOCK19 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock19;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock20](spinlock20) module"]
pub type SPINLOCK20 = crate::Reg<u32, _SPINLOCK20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK20;
#[doc = "`read()` method returns [spinlock20::R](spinlock20::R) reader structure"]
impl crate::Readable for SPINLOCK20 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock20;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock21](spinlock21) module"]
pub type SPINLOCK21 = crate::Reg<u32, _SPINLOCK21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK21;
#[doc = "`read()` method returns [spinlock21::R](spinlock21::R) reader structure"]
impl crate::Readable for SPINLOCK21 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock21;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock22](spinlock22) module"]
pub type SPINLOCK22 = crate::Reg<u32, _SPINLOCK22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK22;
#[doc = "`read()` method returns [spinlock22::R](spinlock22::R) reader structure"]
impl crate::Readable for SPINLOCK22 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock22;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock23](spinlock23) module"]
pub type SPINLOCK23 = crate::Reg<u32, _SPINLOCK23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK23;
#[doc = "`read()` method returns [spinlock23::R](spinlock23::R) reader structure"]
impl crate::Readable for SPINLOCK23 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock23;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock24](spinlock24) module"]
pub type SPINLOCK24 = crate::Reg<u32, _SPINLOCK24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK24;
#[doc = "`read()` method returns [spinlock24::R](spinlock24::R) reader structure"]
impl crate::Readable for SPINLOCK24 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock24;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock25](spinlock25) module"]
pub type SPINLOCK25 = crate::Reg<u32, _SPINLOCK25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK25;
#[doc = "`read()` method returns [spinlock25::R](spinlock25::R) reader structure"]
impl crate::Readable for SPINLOCK25 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock25;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock26](spinlock26) module"]
pub type SPINLOCK26 = crate::Reg<u32, _SPINLOCK26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK26;
#[doc = "`read()` method returns [spinlock26::R](spinlock26::R) reader structure"]
impl crate::Readable for SPINLOCK26 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock26;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock27](spinlock27) module"]
pub type SPINLOCK27 = crate::Reg<u32, _SPINLOCK27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK27;
#[doc = "`read()` method returns [spinlock27::R](spinlock27::R) reader structure"]
impl crate::Readable for SPINLOCK27 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock27;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock28](spinlock28) module"]
pub type SPINLOCK28 = crate::Reg<u32, _SPINLOCK28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK28;
#[doc = "`read()` method returns [spinlock28::R](spinlock28::R) reader structure"]
impl crate::Readable for SPINLOCK28 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock28;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock29](spinlock29) module"]
pub type SPINLOCK29 = crate::Reg<u32, _SPINLOCK29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK29;
#[doc = "`read()` method returns [spinlock29::R](spinlock29::R) reader structure"]
impl crate::Readable for SPINLOCK29 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock29;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock30](spinlock30) module"]
pub type SPINLOCK30 = crate::Reg<u32, _SPINLOCK30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK30;
#[doc = "`read()` method returns [spinlock30::R](spinlock30::R) reader structure"]
impl crate::Readable for SPINLOCK30 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock30;
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock31](spinlock31) module"]
pub type SPINLOCK31 = crate::Reg<u32, _SPINLOCK31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPINLOCK31;
#[doc = "`read()` method returns [spinlock31::R](spinlock31::R) reader structure"]
impl crate::Readable for SPINLOCK31 {}
#[doc = "Reading from a spinlock address will:\\n - Return 0 if lock is already locked\\n - Otherwise return nonzero, and simultaneously claim the lock\\n\\n Writing (any value) releases the lock.\\n If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.\\n The value returned on success is 0x1 << lock number."]
pub mod spinlock31;
