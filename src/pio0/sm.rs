#[doc = "SM_CLKDIV register accessor: an alias for `Reg<SM_CLKDIV_SPEC>`"]
pub type SM_CLKDIV = crate::Reg<sm_clkdiv::SM_CLKDIV_SPEC>;
#[doc = "Clock divisor register for state machine 0  
 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm_clkdiv;
#[doc = "SM_EXECCTRL register accessor: an alias for `Reg<SM_EXECCTRL_SPEC>`"]
pub type SM_EXECCTRL = crate::Reg<sm_execctrl::SM_EXECCTRL_SPEC>;
#[doc = "Execution/behavioural settings for state machine 0"]
pub mod sm_execctrl;
#[doc = "SM_SHIFTCTRL register accessor: an alias for `Reg<SM_SHIFTCTRL_SPEC>`"]
pub type SM_SHIFTCTRL = crate::Reg<sm_shiftctrl::SM_SHIFTCTRL_SPEC>;
#[doc = "Control behaviour of the input/output shift registers for state machine 0"]
pub mod sm_shiftctrl;
#[doc = "SM_ADDR register accessor: an alias for `Reg<SM_ADDR_SPEC>`"]
pub type SM_ADDR = crate::Reg<sm_addr::SM_ADDR_SPEC>;
#[doc = "Current instruction address of state machine 0"]
pub mod sm_addr;
#[doc = "SM_INSTR register accessor: an alias for `Reg<SM_INSTR_SPEC>`"]
pub type SM_INSTR = crate::Reg<sm_instr::SM_INSTR_SPEC>;
#[doc = "Read to see the instruction currently addressed by state machine 0's program counter  
 Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm_instr;
#[doc = "SM_PINCTRL register accessor: an alias for `Reg<SM_PINCTRL_SPEC>`"]
pub type SM_PINCTRL = crate::Reg<sm_pinctrl::SM_PINCTRL_SPEC>;
#[doc = "State machine pin control"]
pub mod sm_pinctrl;
