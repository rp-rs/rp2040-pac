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
 UDIVISOR/SDIVISOR are aliases of the same internal register. The U alias starts an  
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
    #[doc = "0x100..0x180 - Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
    pub spinlock: [crate::Reg<spinlock::SPINLOCK_SPEC>; 32],
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
 UDIVISOR/SDIVISOR are aliases of the same internal register. The U alias starts an  
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
#[doc = "SPINLOCK register accessor: an alias for `Reg<SPINLOCK_SPEC>`"]
pub type SPINLOCK = crate::Reg<spinlock::SPINLOCK_SPEC>;
#[doc = "Reading from a spinlock address will:  
 - Return 0 if lock is already locked  
 - Otherwise return nonzero, and simultaneously claim the lock  

 Writing (any value) releases the lock.  
 If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.  
 The value returned on success is 0x1 << lock number."]
pub mod spinlock;
