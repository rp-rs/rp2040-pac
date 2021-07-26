#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Processor core identifier  
 Value is 0 when read from processor core 0, and 1 when read from processor core 1."]
    pub cpuid: crate::Reg<cpuid::CPUID_SPEC>,
    #[doc = "0x04 - Input value for GPIO pins"]
    pub gpio_in: crate::Reg<gpio_in::GPIO_IN_SPEC>,
    #[doc = "0x08 - Input value for QSPI pins"]
    pub gpio_hi_in: crate::Reg<gpio_hi_in::GPIO_HI_IN_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - GPIO output value"]
    pub gpio_out: crate::Reg<gpio_out::GPIO_OUT_SPEC>,
    #[doc = "0x14 - GPIO output value set"]
    pub gpio_out_set: crate::Reg<gpio_out_set::GPIO_OUT_SET_SPEC>,
    #[doc = "0x18 - GPIO output value clear"]
    pub gpio_out_clr: crate::Reg<gpio_out_clr::GPIO_OUT_CLR_SPEC>,
    #[doc = "0x1c - GPIO output value XOR"]
    pub gpio_out_xor: crate::Reg<gpio_out_xor::GPIO_OUT_XOR_SPEC>,
    #[doc = "0x20 - GPIO output enable"]
    pub gpio_oe: crate::Reg<gpio_oe::GPIO_OE_SPEC>,
    #[doc = "0x24 - GPIO output enable set"]
    pub gpio_oe_set: crate::Reg<gpio_oe_set::GPIO_OE_SET_SPEC>,
    #[doc = "0x28 - GPIO output enable clear"]
    pub gpio_oe_clr: crate::Reg<gpio_oe_clr::GPIO_OE_CLR_SPEC>,
    #[doc = "0x2c - GPIO output enable XOR"]
    pub gpio_oe_xor: crate::Reg<gpio_oe_xor::GPIO_OE_XOR_SPEC>,
    #[doc = "0x30 - QSPI output value"]
    pub gpio_hi_out: crate::Reg<gpio_hi_out::GPIO_HI_OUT_SPEC>,
    #[doc = "0x34 - QSPI output value set"]
    pub gpio_hi_out_set: crate::Reg<gpio_hi_out_set::GPIO_HI_OUT_SET_SPEC>,
    #[doc = "0x38 - QSPI output value clear"]
    pub gpio_hi_out_clr: crate::Reg<gpio_hi_out_clr::GPIO_HI_OUT_CLR_SPEC>,
    #[doc = "0x3c - QSPI output value XOR"]
    pub gpio_hi_out_xor: crate::Reg<gpio_hi_out_xor::GPIO_HI_OUT_XOR_SPEC>,
    #[doc = "0x40 - QSPI output enable"]
    pub gpio_hi_oe: crate::Reg<gpio_hi_oe::GPIO_HI_OE_SPEC>,
    #[doc = "0x44 - QSPI output enable set"]
    pub gpio_hi_oe_set: crate::Reg<gpio_hi_oe_set::GPIO_HI_OE_SET_SPEC>,
    #[doc = "0x48 - QSPI output enable clear"]
    pub gpio_hi_oe_clr: crate::Reg<gpio_hi_oe_clr::GPIO_HI_OE_CLR_SPEC>,
    #[doc = "0x4c - QSPI output enable XOR"]
    pub gpio_hi_oe_xor: crate::Reg<gpio_hi_oe_xor::GPIO_HI_OE_XOR_SPEC>,
    #[doc = "0x50 - Status register for inter-core FIFOs (mailboxes).  
 There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.  
 Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).  
 Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).  
 The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
    pub fifo_st: crate::Reg<fifo_st::FIFO_ST_SPEC>,
    #[doc = "0x54 - Write access to this core's TX FIFO"]
    pub fifo_wr: crate::Reg<fifo_wr::FIFO_WR_SPEC>,
    #[doc = "0x58 - Read access to this core's RX FIFO"]
    pub fifo_rd: crate::Reg<fifo_rd::FIFO_RD_SPEC>,
    #[doc = "0x5c - Spinlock state  
 A bitmap containing the state of all 32 spinlocks (1=locked).  
 Mainly intended for debugging."]
    pub spinlock_st: crate::Reg<spinlock_st::SPINLOCK_ST_SPEC>,
    #[doc = "0x60 - Divider unsigned dividend  
 Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`.  
 Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.  
 UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an  
 unsigned calculation, and the S alias starts a signed calculation."]
    pub div_udividend: crate::Reg<div_udividend::DIV_UDIVIDEND_SPEC>,
    #[doc = "0x64 - Divider unsigned divisor  
 Write to the DIVISOR operand of the divider, i.e. the q in `p / q`.  
 Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.  
 UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an  
 unsigned calculation, and the S alias starts a signed calculation."]
    pub div_udivisor: crate::Reg<div_udivisor::DIV_UDIVISOR_SPEC>,
    #[doc = "0x68 - Divider signed dividend  
 The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
    pub div_sdividend: crate::Reg<div_sdividend::DIV_SDIVIDEND_SPEC>,
    #[doc = "0x6c - Divider signed divisor  
 The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
    pub div_sdivisor: crate::Reg<div_sdivisor::DIV_SDIVISOR_SPEC>,
    #[doc = "0x70 - Divider result quotient  
 The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low.  
 For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ.  
 This register can be written to directly, for context save/restore purposes. This halts any  
 in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.  
 Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order  
 REMAINDER, QUOTIENT if CSR_DIRTY is used."]
    pub div_quotient: crate::Reg<div_quotient::DIV_QUOTIENT_SPEC>,
    #[doc = "0x74 - Divider result remainder  
 The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low.  
 For signed calculations, REMAINDER is negative only when DIVIDEND is negative.  
 This register can be written to directly, for context save/restore purposes. This halts any  
 in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
    pub div_remainder: crate::Reg<div_remainder::DIV_REMAINDER_SPEC>,
    #[doc = "0x78 - Control and status register for divider."]
    pub div_csr: crate::Reg<div_csr::DIV_CSR_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x80 - Read/write access to accumulator 0"]
    pub interp0_accum0: crate::Reg<interp0_accum0::INTERP0_ACCUM0_SPEC>,
    #[doc = "0x84 - Read/write access to accumulator 1"]
    pub interp0_accum1: crate::Reg<interp0_accum1::INTERP0_ACCUM1_SPEC>,
    #[doc = "0x88 - Read/write access to BASE0 register."]
    pub interp0_base0: crate::Reg<interp0_base0::INTERP0_BASE0_SPEC>,
    #[doc = "0x8c - Read/write access to BASE1 register."]
    pub interp0_base1: crate::Reg<interp0_base1::INTERP0_BASE1_SPEC>,
    #[doc = "0x90 - Read/write access to BASE2 register."]
    pub interp0_base2: crate::Reg<interp0_base2::INTERP0_BASE2_SPEC>,
    #[doc = "0x94 - Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp0_pop_lane0: crate::Reg<interp0_pop_lane0::INTERP0_POP_LANE0_SPEC>,
    #[doc = "0x98 - Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp0_pop_lane1: crate::Reg<interp0_pop_lane1::INTERP0_POP_LANE1_SPEC>,
    #[doc = "0x9c - Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp0_pop_full: crate::Reg<interp0_pop_full::INTERP0_POP_FULL_SPEC>,
    #[doc = "0xa0 - Read LANE0 result, without altering any internal state (PEEK)."]
    pub interp0_peek_lane0: crate::Reg<interp0_peek_lane0::INTERP0_PEEK_LANE0_SPEC>,
    #[doc = "0xa4 - Read LANE1 result, without altering any internal state (PEEK)."]
    pub interp0_peek_lane1: crate::Reg<interp0_peek_lane1::INTERP0_PEEK_LANE1_SPEC>,
    #[doc = "0xa8 - Read FULL result, without altering any internal state (PEEK)."]
    pub interp0_peek_full: crate::Reg<interp0_peek_full::INTERP0_PEEK_FULL_SPEC>,
    #[doc = "0xac - Control register for lane 0"]
    pub interp0_ctrl_lane0: crate::Reg<interp0_ctrl_lane0::INTERP0_CTRL_LANE0_SPEC>,
    #[doc = "0xb0 - Control register for lane 1"]
    pub interp0_ctrl_lane1: crate::Reg<interp0_ctrl_lane1::INTERP0_CTRL_LANE1_SPEC>,
    #[doc = "0xb4 - Values written here are atomically added to ACCUM0  
 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    pub interp0_accum0_add: crate::Reg<interp0_accum0_add::INTERP0_ACCUM0_ADD_SPEC>,
    #[doc = "0xb8 - Values written here are atomically added to ACCUM1  
 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    pub interp0_accum1_add: crate::Reg<interp0_accum1_add::INTERP0_ACCUM1_ADD_SPEC>,
    #[doc = "0xbc - On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.  
 Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    pub interp0_base_1and0: crate::Reg<interp0_base_1and0::INTERP0_BASE_1AND0_SPEC>,
    #[doc = "0xc0 - Read/write access to accumulator 0"]
    pub interp1_accum0: crate::Reg<interp1_accum0::INTERP1_ACCUM0_SPEC>,
    #[doc = "0xc4 - Read/write access to accumulator 1"]
    pub interp1_accum1: crate::Reg<interp1_accum1::INTERP1_ACCUM1_SPEC>,
    #[doc = "0xc8 - Read/write access to BASE0 register."]
    pub interp1_base0: crate::Reg<interp1_base0::INTERP1_BASE0_SPEC>,
    #[doc = "0xcc - Read/write access to BASE1 register."]
    pub interp1_base1: crate::Reg<interp1_base1::INTERP1_BASE1_SPEC>,
    #[doc = "0xd0 - Read/write access to BASE2 register."]
    pub interp1_base2: crate::Reg<interp1_base2::INTERP1_BASE2_SPEC>,
    #[doc = "0xd4 - Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp1_pop_lane0: crate::Reg<interp1_pop_lane0::INTERP1_POP_LANE0_SPEC>,
    #[doc = "0xd8 - Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp1_pop_lane1: crate::Reg<interp1_pop_lane1::INTERP1_POP_LANE1_SPEC>,
    #[doc = "0xdc - Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    pub interp1_pop_full: crate::Reg<interp1_pop_full::INTERP1_POP_FULL_SPEC>,
    #[doc = "0xe0 - Read LANE0 result, without altering any internal state (PEEK)."]
    pub interp1_peek_lane0: crate::Reg<interp1_peek_lane0::INTERP1_PEEK_LANE0_SPEC>,
    #[doc = "0xe4 - Read LANE1 result, without altering any internal state (PEEK)."]
    pub interp1_peek_lane1: crate::Reg<interp1_peek_lane1::INTERP1_PEEK_LANE1_SPEC>,
    #[doc = "0xe8 - Read FULL result, without altering any internal state (PEEK)."]
    pub interp1_peek_full: crate::Reg<interp1_peek_full::INTERP1_PEEK_FULL_SPEC>,
    #[doc = "0xec - Control register for lane 0"]
    pub interp1_ctrl_lane0: crate::Reg<interp1_ctrl_lane0::INTERP1_CTRL_LANE0_SPEC>,
    #[doc = "0xf0 - Control register for lane 1"]
    pub interp1_ctrl_lane1: crate::Reg<interp1_ctrl_lane1::INTERP1_CTRL_LANE1_SPEC>,
    #[doc = "0xf4 - Values written here are atomically added to ACCUM0  
 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    pub interp1_accum0_add: crate::Reg<interp1_accum0_add::INTERP1_ACCUM0_ADD_SPEC>,
    #[doc = "0xf8 - Values written here are atomically added to ACCUM1  
 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    pub interp1_accum1_add: crate::Reg<interp1_accum1_add::INTERP1_ACCUM1_ADD_SPEC>,
    #[doc = "0xfc - On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.  
 Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    pub interp1_base_1and0: crate::Reg<interp1_base_1and0::INTERP1_BASE_1AND0_SPEC>,
    #[doc = "0x100 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock0: crate::Reg<spinlock0::SPINLOCK0_SPEC>,
    #[doc = "0x104 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock1: crate::Reg<spinlock1::SPINLOCK1_SPEC>,
    #[doc = "0x108 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock2: crate::Reg<spinlock2::SPINLOCK2_SPEC>,
    #[doc = "0x10c - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock3: crate::Reg<spinlock3::SPINLOCK3_SPEC>,
    #[doc = "0x110 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock4: crate::Reg<spinlock4::SPINLOCK4_SPEC>,
    #[doc = "0x114 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock5: crate::Reg<spinlock5::SPINLOCK5_SPEC>,
    #[doc = "0x118 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock6: crate::Reg<spinlock6::SPINLOCK6_SPEC>,
    #[doc = "0x11c - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock7: crate::Reg<spinlock7::SPINLOCK7_SPEC>,
    #[doc = "0x120 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock8: crate::Reg<spinlock8::SPINLOCK8_SPEC>,
    #[doc = "0x124 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock9: crate::Reg<spinlock9::SPINLOCK9_SPEC>,
    #[doc = "0x128 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock10: crate::Reg<spinlock10::SPINLOCK10_SPEC>,
    #[doc = "0x12c - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock11: crate::Reg<spinlock11::SPINLOCK11_SPEC>,
    #[doc = "0x130 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock12: crate::Reg<spinlock12::SPINLOCK12_SPEC>,
    #[doc = "0x134 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock13: crate::Reg<spinlock13::SPINLOCK13_SPEC>,
    #[doc = "0x138 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock14: crate::Reg<spinlock14::SPINLOCK14_SPEC>,
    #[doc = "0x13c - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock15: crate::Reg<spinlock15::SPINLOCK15_SPEC>,
    #[doc = "0x140 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock16: crate::Reg<spinlock16::SPINLOCK16_SPEC>,
    #[doc = "0x144 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock17: crate::Reg<spinlock17::SPINLOCK17_SPEC>,
    #[doc = "0x148 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock18: crate::Reg<spinlock18::SPINLOCK18_SPEC>,
    #[doc = "0x14c - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock19: crate::Reg<spinlock19::SPINLOCK19_SPEC>,
    #[doc = "0x150 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock20: crate::Reg<spinlock20::SPINLOCK20_SPEC>,
    #[doc = "0x154 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock21: crate::Reg<spinlock21::SPINLOCK21_SPEC>,
    #[doc = "0x158 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock22: crate::Reg<spinlock22::SPINLOCK22_SPEC>,
    #[doc = "0x15c - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock23: crate::Reg<spinlock23::SPINLOCK23_SPEC>,
    #[doc = "0x160 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock24: crate::Reg<spinlock24::SPINLOCK24_SPEC>,
    #[doc = "0x164 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock25: crate::Reg<spinlock25::SPINLOCK25_SPEC>,
    #[doc = "0x168 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock26: crate::Reg<spinlock26::SPINLOCK26_SPEC>,
    #[doc = "0x16c - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock27: crate::Reg<spinlock27::SPINLOCK27_SPEC>,
    #[doc = "0x170 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock28: crate::Reg<spinlock28::SPINLOCK28_SPEC>,
    #[doc = "0x174 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock29: crate::Reg<spinlock29::SPINLOCK29_SPEC>,
    #[doc = "0x178 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock30: crate::Reg<spinlock30::SPINLOCK30_SPEC>,
    #[doc = "0x17c - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock31: crate::Reg<spinlock31::SPINLOCK31_SPEC>,
}
#[doc = "CPUID register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "Processor core identifier  
 Value is 0 when read from processor core 0, and 1 when read from processor core 1."]
pub mod cpuid;
#[doc = "GPIO_IN register accessor: an alias for `Reg<GPIO_IN_SPEC>`"]
pub type GPIO_IN = crate::Reg<gpio_in::GPIO_IN_SPEC>;
#[doc = "Input value for GPIO pins"]
pub mod gpio_in;
#[doc = "GPIO_HI_IN register accessor: an alias for `Reg<GPIO_HI_IN_SPEC>`"]
pub type GPIO_HI_IN = crate::Reg<gpio_hi_in::GPIO_HI_IN_SPEC>;
#[doc = "Input value for QSPI pins"]
pub mod gpio_hi_in;
#[doc = "GPIO_OUT register accessor: an alias for `Reg<GPIO_OUT_SPEC>`"]
pub type GPIO_OUT = crate::Reg<gpio_out::GPIO_OUT_SPEC>;
#[doc = "GPIO output value"]
pub mod gpio_out;
#[doc = "GPIO_OUT_SET register accessor: an alias for `Reg<GPIO_OUT_SET_SPEC>`"]
pub type GPIO_OUT_SET = crate::Reg<gpio_out_set::GPIO_OUT_SET_SPEC>;
#[doc = "GPIO output value set"]
pub mod gpio_out_set;
#[doc = "GPIO_OUT_CLR register accessor: an alias for `Reg<GPIO_OUT_CLR_SPEC>`"]
pub type GPIO_OUT_CLR = crate::Reg<gpio_out_clr::GPIO_OUT_CLR_SPEC>;
#[doc = "GPIO output value clear"]
pub mod gpio_out_clr;
#[doc = "GPIO_OUT_XOR register accessor: an alias for `Reg<GPIO_OUT_XOR_SPEC>`"]
pub type GPIO_OUT_XOR = crate::Reg<gpio_out_xor::GPIO_OUT_XOR_SPEC>;
#[doc = "GPIO output value XOR"]
pub mod gpio_out_xor;
#[doc = "GPIO_OE register accessor: an alias for `Reg<GPIO_OE_SPEC>`"]
pub type GPIO_OE = crate::Reg<gpio_oe::GPIO_OE_SPEC>;
#[doc = "GPIO output enable"]
pub mod gpio_oe;
#[doc = "GPIO_OE_SET register accessor: an alias for `Reg<GPIO_OE_SET_SPEC>`"]
pub type GPIO_OE_SET = crate::Reg<gpio_oe_set::GPIO_OE_SET_SPEC>;
#[doc = "GPIO output enable set"]
pub mod gpio_oe_set;
#[doc = "GPIO_OE_CLR register accessor: an alias for `Reg<GPIO_OE_CLR_SPEC>`"]
pub type GPIO_OE_CLR = crate::Reg<gpio_oe_clr::GPIO_OE_CLR_SPEC>;
#[doc = "GPIO output enable clear"]
pub mod gpio_oe_clr;
#[doc = "GPIO_OE_XOR register accessor: an alias for `Reg<GPIO_OE_XOR_SPEC>`"]
pub type GPIO_OE_XOR = crate::Reg<gpio_oe_xor::GPIO_OE_XOR_SPEC>;
#[doc = "GPIO output enable XOR"]
pub mod gpio_oe_xor;
#[doc = "GPIO_HI_OUT register accessor: an alias for `Reg<GPIO_HI_OUT_SPEC>`"]
pub type GPIO_HI_OUT = crate::Reg<gpio_hi_out::GPIO_HI_OUT_SPEC>;
#[doc = "QSPI output value"]
pub mod gpio_hi_out;
#[doc = "GPIO_HI_OUT_SET register accessor: an alias for `Reg<GPIO_HI_OUT_SET_SPEC>`"]
pub type GPIO_HI_OUT_SET = crate::Reg<gpio_hi_out_set::GPIO_HI_OUT_SET_SPEC>;
#[doc = "QSPI output value set"]
pub mod gpio_hi_out_set;
#[doc = "GPIO_HI_OUT_CLR register accessor: an alias for `Reg<GPIO_HI_OUT_CLR_SPEC>`"]
pub type GPIO_HI_OUT_CLR = crate::Reg<gpio_hi_out_clr::GPIO_HI_OUT_CLR_SPEC>;
#[doc = "QSPI output value clear"]
pub mod gpio_hi_out_clr;
#[doc = "GPIO_HI_OUT_XOR register accessor: an alias for `Reg<GPIO_HI_OUT_XOR_SPEC>`"]
pub type GPIO_HI_OUT_XOR = crate::Reg<gpio_hi_out_xor::GPIO_HI_OUT_XOR_SPEC>;
#[doc = "QSPI output value XOR"]
pub mod gpio_hi_out_xor;
#[doc = "GPIO_HI_OE register accessor: an alias for `Reg<GPIO_HI_OE_SPEC>`"]
pub type GPIO_HI_OE = crate::Reg<gpio_hi_oe::GPIO_HI_OE_SPEC>;
#[doc = "QSPI output enable"]
pub mod gpio_hi_oe;
#[doc = "GPIO_HI_OE_SET register accessor: an alias for `Reg<GPIO_HI_OE_SET_SPEC>`"]
pub type GPIO_HI_OE_SET = crate::Reg<gpio_hi_oe_set::GPIO_HI_OE_SET_SPEC>;
#[doc = "QSPI output enable set"]
pub mod gpio_hi_oe_set;
#[doc = "GPIO_HI_OE_CLR register accessor: an alias for `Reg<GPIO_HI_OE_CLR_SPEC>`"]
pub type GPIO_HI_OE_CLR = crate::Reg<gpio_hi_oe_clr::GPIO_HI_OE_CLR_SPEC>;
#[doc = "QSPI output enable clear"]
pub mod gpio_hi_oe_clr;
#[doc = "GPIO_HI_OE_XOR register accessor: an alias for `Reg<GPIO_HI_OE_XOR_SPEC>`"]
pub type GPIO_HI_OE_XOR = crate::Reg<gpio_hi_oe_xor::GPIO_HI_OE_XOR_SPEC>;
#[doc = "QSPI output enable XOR"]
pub mod gpio_hi_oe_xor;
#[doc = "FIFO_ST register accessor: an alias for `Reg<FIFO_ST_SPEC>`"]
pub type FIFO_ST = crate::Reg<fifo_st::FIFO_ST_SPEC>;
#[doc = "Status register for inter-core FIFOs (mailboxes).  
 There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.  
 Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).  
 Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).  
 The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
pub mod fifo_st;
#[doc = "FIFO_WR register accessor: an alias for `Reg<FIFO_WR_SPEC>`"]
pub type FIFO_WR = crate::Reg<fifo_wr::FIFO_WR_SPEC>;
#[doc = "Write access to this core's TX FIFO"]
pub mod fifo_wr;
#[doc = "FIFO_RD register accessor: an alias for `Reg<FIFO_RD_SPEC>`"]
pub type FIFO_RD = crate::Reg<fifo_rd::FIFO_RD_SPEC>;
#[doc = "Read access to this core's RX FIFO"]
pub mod fifo_rd;
#[doc = "SPINLOCK_ST register accessor: an alias for `Reg<SPINLOCK_ST_SPEC>`"]
pub type SPINLOCK_ST = crate::Reg<spinlock_st::SPINLOCK_ST_SPEC>;
#[doc = "Spinlock state  
 A bitmap containing the state of all 32 spinlocks (1=locked).  
 Mainly intended for debugging."]
pub mod spinlock_st;
#[doc = "DIV_UDIVIDEND register accessor: an alias for `Reg<DIV_UDIVIDEND_SPEC>`"]
pub type DIV_UDIVIDEND = crate::Reg<div_udividend::DIV_UDIVIDEND_SPEC>;
#[doc = "Divider unsigned dividend  
 Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`.  
 Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.  
 UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an  
 unsigned calculation, and the S alias starts a signed calculation."]
pub mod div_udividend;
#[doc = "DIV_UDIVISOR register accessor: an alias for `Reg<DIV_UDIVISOR_SPEC>`"]
pub type DIV_UDIVISOR = crate::Reg<div_udivisor::DIV_UDIVISOR_SPEC>;
#[doc = "Divider unsigned divisor  
 Write to the DIVISOR operand of the divider, i.e. the q in `p / q`.  
 Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER.  
 UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an  
 unsigned calculation, and the S alias starts a signed calculation."]
pub mod div_udivisor;
#[doc = "DIV_SDIVIDEND register accessor: an alias for `Reg<DIV_SDIVIDEND_SPEC>`"]
pub type DIV_SDIVIDEND = crate::Reg<div_sdividend::DIV_SDIVIDEND_SPEC>;
#[doc = "Divider signed dividend  
 The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
pub mod div_sdividend;
#[doc = "DIV_SDIVISOR register accessor: an alias for `Reg<DIV_SDIVISOR_SPEC>`"]
pub type DIV_SDIVISOR = crate::Reg<div_sdivisor::DIV_SDIVISOR_SPEC>;
#[doc = "Divider signed divisor  
 The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
pub mod div_sdivisor;
#[doc = "DIV_QUOTIENT register accessor: an alias for `Reg<DIV_QUOTIENT_SPEC>`"]
pub type DIV_QUOTIENT = crate::Reg<div_quotient::DIV_QUOTIENT_SPEC>;
#[doc = "Divider result quotient  
 The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low.  
 For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ.  
 This register can be written to directly, for context save/restore purposes. This halts any  
 in-progress calculation and sets the CSR_READY and CSR_DIRTY flags.  
 Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order  
 REMAINDER, QUOTIENT if CSR_DIRTY is used."]
pub mod div_quotient;
#[doc = "DIV_REMAINDER register accessor: an alias for `Reg<DIV_REMAINDER_SPEC>`"]
pub type DIV_REMAINDER = crate::Reg<div_remainder::DIV_REMAINDER_SPEC>;
#[doc = "Divider result remainder  
 The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low.  
 For signed calculations, REMAINDER is negative only when DIVIDEND is negative.  
 This register can be written to directly, for context save/restore purposes. This halts any  
 in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
pub mod div_remainder;
#[doc = "DIV_CSR register accessor: an alias for `Reg<DIV_CSR_SPEC>`"]
pub type DIV_CSR = crate::Reg<div_csr::DIV_CSR_SPEC>;
#[doc = "Control and status register for divider."]
pub mod div_csr;
#[doc = "INTERP0_ACCUM0 register accessor: an alias for `Reg<INTERP0_ACCUM0_SPEC>`"]
pub type INTERP0_ACCUM0 = crate::Reg<interp0_accum0::INTERP0_ACCUM0_SPEC>;
#[doc = "Read/write access to accumulator 0"]
pub mod interp0_accum0;
#[doc = "INTERP0_ACCUM1 register accessor: an alias for `Reg<INTERP0_ACCUM1_SPEC>`"]
pub type INTERP0_ACCUM1 = crate::Reg<interp0_accum1::INTERP0_ACCUM1_SPEC>;
#[doc = "Read/write access to accumulator 1"]
pub mod interp0_accum1;
#[doc = "INTERP0_BASE0 register accessor: an alias for `Reg<INTERP0_BASE0_SPEC>`"]
pub type INTERP0_BASE0 = crate::Reg<interp0_base0::INTERP0_BASE0_SPEC>;
#[doc = "Read/write access to BASE0 register."]
pub mod interp0_base0;
#[doc = "INTERP0_BASE1 register accessor: an alias for `Reg<INTERP0_BASE1_SPEC>`"]
pub type INTERP0_BASE1 = crate::Reg<interp0_base1::INTERP0_BASE1_SPEC>;
#[doc = "Read/write access to BASE1 register."]
pub mod interp0_base1;
#[doc = "INTERP0_BASE2 register accessor: an alias for `Reg<INTERP0_BASE2_SPEC>`"]
pub type INTERP0_BASE2 = crate::Reg<interp0_base2::INTERP0_BASE2_SPEC>;
#[doc = "Read/write access to BASE2 register."]
pub mod interp0_base2;
#[doc = "INTERP0_POP_LANE0 register accessor: an alias for `Reg<INTERP0_POP_LANE0_SPEC>`"]
pub type INTERP0_POP_LANE0 = crate::Reg<interp0_pop_lane0::INTERP0_POP_LANE0_SPEC>;
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_lane0;
#[doc = "INTERP0_POP_LANE1 register accessor: an alias for `Reg<INTERP0_POP_LANE1_SPEC>`"]
pub type INTERP0_POP_LANE1 = crate::Reg<interp0_pop_lane1::INTERP0_POP_LANE1_SPEC>;
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_lane1;
#[doc = "INTERP0_POP_FULL register accessor: an alias for `Reg<INTERP0_POP_FULL_SPEC>`"]
pub type INTERP0_POP_FULL = crate::Reg<interp0_pop_full::INTERP0_POP_FULL_SPEC>;
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_full;
#[doc = "INTERP0_PEEK_LANE0 register accessor: an alias for `Reg<INTERP0_PEEK_LANE0_SPEC>`"]
pub type INTERP0_PEEK_LANE0 = crate::Reg<interp0_peek_lane0::INTERP0_PEEK_LANE0_SPEC>;
#[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
pub mod interp0_peek_lane0;
#[doc = "INTERP0_PEEK_LANE1 register accessor: an alias for `Reg<INTERP0_PEEK_LANE1_SPEC>`"]
pub type INTERP0_PEEK_LANE1 = crate::Reg<interp0_peek_lane1::INTERP0_PEEK_LANE1_SPEC>;
#[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
pub mod interp0_peek_lane1;
#[doc = "INTERP0_PEEK_FULL register accessor: an alias for `Reg<INTERP0_PEEK_FULL_SPEC>`"]
pub type INTERP0_PEEK_FULL = crate::Reg<interp0_peek_full::INTERP0_PEEK_FULL_SPEC>;
#[doc = "Read FULL result, without altering any internal state (PEEK)."]
pub mod interp0_peek_full;
#[doc = "INTERP0_CTRL_LANE0 register accessor: an alias for `Reg<INTERP0_CTRL_LANE0_SPEC>`"]
pub type INTERP0_CTRL_LANE0 = crate::Reg<interp0_ctrl_lane0::INTERP0_CTRL_LANE0_SPEC>;
#[doc = "Control register for lane 0"]
pub mod interp0_ctrl_lane0;
#[doc = "INTERP0_CTRL_LANE1 register accessor: an alias for `Reg<INTERP0_CTRL_LANE1_SPEC>`"]
pub type INTERP0_CTRL_LANE1 = crate::Reg<interp0_ctrl_lane1::INTERP0_CTRL_LANE1_SPEC>;
#[doc = "Control register for lane 1"]
pub mod interp0_ctrl_lane1;
#[doc = "INTERP0_ACCUM0_ADD register accessor: an alias for `Reg<INTERP0_ACCUM0_ADD_SPEC>`"]
pub type INTERP0_ACCUM0_ADD = crate::Reg<interp0_accum0_add::INTERP0_ACCUM0_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM0  
 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
pub mod interp0_accum0_add;
#[doc = "INTERP0_ACCUM1_ADD register accessor: an alias for `Reg<INTERP0_ACCUM1_ADD_SPEC>`"]
pub type INTERP0_ACCUM1_ADD = crate::Reg<interp0_accum1_add::INTERP0_ACCUM1_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM1  
 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
pub mod interp0_accum1_add;
#[doc = "INTERP0_BASE_1AND0 register accessor: an alias for `Reg<INTERP0_BASE_1AND0_SPEC>`"]
pub type INTERP0_BASE_1AND0 = crate::Reg<interp0_base_1and0::INTERP0_BASE_1AND0_SPEC>;
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.  
 Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
pub mod interp0_base_1and0;
#[doc = "INTERP1_ACCUM0 register accessor: an alias for `Reg<INTERP1_ACCUM0_SPEC>`"]
pub type INTERP1_ACCUM0 = crate::Reg<interp1_accum0::INTERP1_ACCUM0_SPEC>;
#[doc = "Read/write access to accumulator 0"]
pub mod interp1_accum0;
#[doc = "INTERP1_ACCUM1 register accessor: an alias for `Reg<INTERP1_ACCUM1_SPEC>`"]
pub type INTERP1_ACCUM1 = crate::Reg<interp1_accum1::INTERP1_ACCUM1_SPEC>;
#[doc = "Read/write access to accumulator 1"]
pub mod interp1_accum1;
#[doc = "INTERP1_BASE0 register accessor: an alias for `Reg<INTERP1_BASE0_SPEC>`"]
pub type INTERP1_BASE0 = crate::Reg<interp1_base0::INTERP1_BASE0_SPEC>;
#[doc = "Read/write access to BASE0 register."]
pub mod interp1_base0;
#[doc = "INTERP1_BASE1 register accessor: an alias for `Reg<INTERP1_BASE1_SPEC>`"]
pub type INTERP1_BASE1 = crate::Reg<interp1_base1::INTERP1_BASE1_SPEC>;
#[doc = "Read/write access to BASE1 register."]
pub mod interp1_base1;
#[doc = "INTERP1_BASE2 register accessor: an alias for `Reg<INTERP1_BASE2_SPEC>`"]
pub type INTERP1_BASE2 = crate::Reg<interp1_base2::INTERP1_BASE2_SPEC>;
#[doc = "Read/write access to BASE2 register."]
pub mod interp1_base2;
#[doc = "INTERP1_POP_LANE0 register accessor: an alias for `Reg<INTERP1_POP_LANE0_SPEC>`"]
pub type INTERP1_POP_LANE0 = crate::Reg<interp1_pop_lane0::INTERP1_POP_LANE0_SPEC>;
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_lane0;
#[doc = "INTERP1_POP_LANE1 register accessor: an alias for `Reg<INTERP1_POP_LANE1_SPEC>`"]
pub type INTERP1_POP_LANE1 = crate::Reg<interp1_pop_lane1::INTERP1_POP_LANE1_SPEC>;
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_lane1;
#[doc = "INTERP1_POP_FULL register accessor: an alias for `Reg<INTERP1_POP_FULL_SPEC>`"]
pub type INTERP1_POP_FULL = crate::Reg<interp1_pop_full::INTERP1_POP_FULL_SPEC>;
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_full;
#[doc = "INTERP1_PEEK_LANE0 register accessor: an alias for `Reg<INTERP1_PEEK_LANE0_SPEC>`"]
pub type INTERP1_PEEK_LANE0 = crate::Reg<interp1_peek_lane0::INTERP1_PEEK_LANE0_SPEC>;
#[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
pub mod interp1_peek_lane0;
#[doc = "INTERP1_PEEK_LANE1 register accessor: an alias for `Reg<INTERP1_PEEK_LANE1_SPEC>`"]
pub type INTERP1_PEEK_LANE1 = crate::Reg<interp1_peek_lane1::INTERP1_PEEK_LANE1_SPEC>;
#[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
pub mod interp1_peek_lane1;
#[doc = "INTERP1_PEEK_FULL register accessor: an alias for `Reg<INTERP1_PEEK_FULL_SPEC>`"]
pub type INTERP1_PEEK_FULL = crate::Reg<interp1_peek_full::INTERP1_PEEK_FULL_SPEC>;
#[doc = "Read FULL result, without altering any internal state (PEEK)."]
pub mod interp1_peek_full;
#[doc = "INTERP1_CTRL_LANE0 register accessor: an alias for `Reg<INTERP1_CTRL_LANE0_SPEC>`"]
pub type INTERP1_CTRL_LANE0 = crate::Reg<interp1_ctrl_lane0::INTERP1_CTRL_LANE0_SPEC>;
#[doc = "Control register for lane 0"]
pub mod interp1_ctrl_lane0;
#[doc = "INTERP1_CTRL_LANE1 register accessor: an alias for `Reg<INTERP1_CTRL_LANE1_SPEC>`"]
pub type INTERP1_CTRL_LANE1 = crate::Reg<interp1_ctrl_lane1::INTERP1_CTRL_LANE1_SPEC>;
#[doc = "Control register for lane 1"]
pub mod interp1_ctrl_lane1;
#[doc = "INTERP1_ACCUM0_ADD register accessor: an alias for `Reg<INTERP1_ACCUM0_ADD_SPEC>`"]
pub type INTERP1_ACCUM0_ADD = crate::Reg<interp1_accum0_add::INTERP1_ACCUM0_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM0  
 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
pub mod interp1_accum0_add;
#[doc = "INTERP1_ACCUM1_ADD register accessor: an alias for `Reg<INTERP1_ACCUM1_ADD_SPEC>`"]
pub type INTERP1_ACCUM1_ADD = crate::Reg<interp1_accum1_add::INTERP1_ACCUM1_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM1  
 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
pub mod interp1_accum1_add;
#[doc = "INTERP1_BASE_1AND0 register accessor: an alias for `Reg<INTERP1_BASE_1AND0_SPEC>`"]
pub type INTERP1_BASE_1AND0 = crate::Reg<interp1_base_1and0::INTERP1_BASE_1AND0_SPEC>;
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.  
 Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
pub mod interp1_base_1and0;
#[doc = "SPINLOCK0 register accessor: an alias for `Reg<SPINLOCK0_SPEC>`"]
pub type SPINLOCK0 = crate::Reg<spinlock0::SPINLOCK0_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock0;
#[doc = "SPINLOCK1 register accessor: an alias for `Reg<SPINLOCK1_SPEC>`"]
pub type SPINLOCK1 = crate::Reg<spinlock1::SPINLOCK1_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock1;
#[doc = "SPINLOCK2 register accessor: an alias for `Reg<SPINLOCK2_SPEC>`"]
pub type SPINLOCK2 = crate::Reg<spinlock2::SPINLOCK2_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock2;
#[doc = "SPINLOCK3 register accessor: an alias for `Reg<SPINLOCK3_SPEC>`"]
pub type SPINLOCK3 = crate::Reg<spinlock3::SPINLOCK3_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock3;
#[doc = "SPINLOCK4 register accessor: an alias for `Reg<SPINLOCK4_SPEC>`"]
pub type SPINLOCK4 = crate::Reg<spinlock4::SPINLOCK4_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock4;
#[doc = "SPINLOCK5 register accessor: an alias for `Reg<SPINLOCK5_SPEC>`"]
pub type SPINLOCK5 = crate::Reg<spinlock5::SPINLOCK5_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock5;
#[doc = "SPINLOCK6 register accessor: an alias for `Reg<SPINLOCK6_SPEC>`"]
pub type SPINLOCK6 = crate::Reg<spinlock6::SPINLOCK6_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock6;
#[doc = "SPINLOCK7 register accessor: an alias for `Reg<SPINLOCK7_SPEC>`"]
pub type SPINLOCK7 = crate::Reg<spinlock7::SPINLOCK7_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock7;
#[doc = "SPINLOCK8 register accessor: an alias for `Reg<SPINLOCK8_SPEC>`"]
pub type SPINLOCK8 = crate::Reg<spinlock8::SPINLOCK8_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock8;
#[doc = "SPINLOCK9 register accessor: an alias for `Reg<SPINLOCK9_SPEC>`"]
pub type SPINLOCK9 = crate::Reg<spinlock9::SPINLOCK9_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock9;
#[doc = "SPINLOCK10 register accessor: an alias for `Reg<SPINLOCK10_SPEC>`"]
pub type SPINLOCK10 = crate::Reg<spinlock10::SPINLOCK10_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock10;
#[doc = "SPINLOCK11 register accessor: an alias for `Reg<SPINLOCK11_SPEC>`"]
pub type SPINLOCK11 = crate::Reg<spinlock11::SPINLOCK11_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock11;
#[doc = "SPINLOCK12 register accessor: an alias for `Reg<SPINLOCK12_SPEC>`"]
pub type SPINLOCK12 = crate::Reg<spinlock12::SPINLOCK12_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock12;
#[doc = "SPINLOCK13 register accessor: an alias for `Reg<SPINLOCK13_SPEC>`"]
pub type SPINLOCK13 = crate::Reg<spinlock13::SPINLOCK13_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock13;
#[doc = "SPINLOCK14 register accessor: an alias for `Reg<SPINLOCK14_SPEC>`"]
pub type SPINLOCK14 = crate::Reg<spinlock14::SPINLOCK14_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock14;
#[doc = "SPINLOCK15 register accessor: an alias for `Reg<SPINLOCK15_SPEC>`"]
pub type SPINLOCK15 = crate::Reg<spinlock15::SPINLOCK15_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock15;
#[doc = "SPINLOCK16 register accessor: an alias for `Reg<SPINLOCK16_SPEC>`"]
pub type SPINLOCK16 = crate::Reg<spinlock16::SPINLOCK16_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock16;
#[doc = "SPINLOCK17 register accessor: an alias for `Reg<SPINLOCK17_SPEC>`"]
pub type SPINLOCK17 = crate::Reg<spinlock17::SPINLOCK17_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock17;
#[doc = "SPINLOCK18 register accessor: an alias for `Reg<SPINLOCK18_SPEC>`"]
pub type SPINLOCK18 = crate::Reg<spinlock18::SPINLOCK18_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock18;
#[doc = "SPINLOCK19 register accessor: an alias for `Reg<SPINLOCK19_SPEC>`"]
pub type SPINLOCK19 = crate::Reg<spinlock19::SPINLOCK19_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock19;
#[doc = "SPINLOCK20 register accessor: an alias for `Reg<SPINLOCK20_SPEC>`"]
pub type SPINLOCK20 = crate::Reg<spinlock20::SPINLOCK20_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock20;
#[doc = "SPINLOCK21 register accessor: an alias for `Reg<SPINLOCK21_SPEC>`"]
pub type SPINLOCK21 = crate::Reg<spinlock21::SPINLOCK21_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock21;
#[doc = "SPINLOCK22 register accessor: an alias for `Reg<SPINLOCK22_SPEC>`"]
pub type SPINLOCK22 = crate::Reg<spinlock22::SPINLOCK22_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock22;
#[doc = "SPINLOCK23 register accessor: an alias for `Reg<SPINLOCK23_SPEC>`"]
pub type SPINLOCK23 = crate::Reg<spinlock23::SPINLOCK23_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock23;
#[doc = "SPINLOCK24 register accessor: an alias for `Reg<SPINLOCK24_SPEC>`"]
pub type SPINLOCK24 = crate::Reg<spinlock24::SPINLOCK24_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock24;
#[doc = "SPINLOCK25 register accessor: an alias for `Reg<SPINLOCK25_SPEC>`"]
pub type SPINLOCK25 = crate::Reg<spinlock25::SPINLOCK25_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock25;
#[doc = "SPINLOCK26 register accessor: an alias for `Reg<SPINLOCK26_SPEC>`"]
pub type SPINLOCK26 = crate::Reg<spinlock26::SPINLOCK26_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock26;
#[doc = "SPINLOCK27 register accessor: an alias for `Reg<SPINLOCK27_SPEC>`"]
pub type SPINLOCK27 = crate::Reg<spinlock27::SPINLOCK27_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock27;
#[doc = "SPINLOCK28 register accessor: an alias for `Reg<SPINLOCK28_SPEC>`"]
pub type SPINLOCK28 = crate::Reg<spinlock28::SPINLOCK28_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock28;
#[doc = "SPINLOCK29 register accessor: an alias for `Reg<SPINLOCK29_SPEC>`"]
pub type SPINLOCK29 = crate::Reg<spinlock29::SPINLOCK29_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock29;
#[doc = "SPINLOCK30 register accessor: an alias for `Reg<SPINLOCK30_SPEC>`"]
pub type SPINLOCK30 = crate::Reg<spinlock30::SPINLOCK30_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock30;
#[doc = "SPINLOCK31 register accessor: an alias for `Reg<SPINLOCK31_SPEC>`"]
pub type SPINLOCK31 = crate::Reg<spinlock31::SPINLOCK31_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock31;
