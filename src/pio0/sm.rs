#[doc = "Clock divisor register for state machine 0\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_clkdiv](sm_clkdiv) module"]
pub type SM_CLKDIV = crate::Reg<u32, _SM_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_CLKDIV;
#[doc = "`read()` method returns [sm_clkdiv::R](sm_clkdiv::R) reader structure"]
impl crate::Readable for SM_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [sm_clkdiv::W](sm_clkdiv::W) writer structure"]
impl crate::Writable for SM_CLKDIV {}
#[doc = "Clock divisor register for state machine 0\\n Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm_clkdiv;
#[doc = "Execution/behavioural settings for state machine 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_execctrl](sm_execctrl) module"]
pub type SM_EXECCTRL = crate::Reg<u32, _SM_EXECCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_EXECCTRL;
#[doc = "`read()` method returns [sm_execctrl::R](sm_execctrl::R) reader structure"]
impl crate::Readable for SM_EXECCTRL {}
#[doc = "`write(|w| ..)` method takes [sm_execctrl::W](sm_execctrl::W) writer structure"]
impl crate::Writable for SM_EXECCTRL {}
#[doc = "Execution/behavioural settings for state machine 0"]
pub mod sm_execctrl;
#[doc = "Control behaviour of the input/output shift registers for state machine 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_shiftctrl](sm_shiftctrl) module"]
pub type SM_SHIFTCTRL = crate::Reg<u32, _SM_SHIFTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_SHIFTCTRL;
#[doc = "`read()` method returns [sm_shiftctrl::R](sm_shiftctrl::R) reader structure"]
impl crate::Readable for SM_SHIFTCTRL {}
#[doc = "`write(|w| ..)` method takes [sm_shiftctrl::W](sm_shiftctrl::W) writer structure"]
impl crate::Writable for SM_SHIFTCTRL {}
#[doc = "Control behaviour of the input/output shift registers for state machine 0"]
pub mod sm_shiftctrl;
#[doc = "Current instruction address of state machine 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_addr](sm_addr) module"]
pub type SM_ADDR = crate::Reg<u32, _SM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_ADDR;
#[doc = "`read()` method returns [sm_addr::R](sm_addr::R) reader structure"]
impl crate::Readable for SM_ADDR {}
#[doc = "Current instruction address of state machine 0"]
pub mod sm_addr;
#[doc = "Read to see the instruction currently addressed by state machine 0's program counter\\n Write to execute an instruction immediately (including jumps) and then resume execution.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_instr](sm_instr) module"]
pub type SM_INSTR = crate::Reg<u32, _SM_INSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_INSTR;
#[doc = "`read()` method returns [sm_instr::R](sm_instr::R) reader structure"]
impl crate::Readable for SM_INSTR {}
#[doc = "`write(|w| ..)` method takes [sm_instr::W](sm_instr::W) writer structure"]
impl crate::Writable for SM_INSTR {}
#[doc = "Read to see the instruction currently addressed by state machine 0's program counter\\n Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm_instr;
#[doc = "State machine pin control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_pinctrl](sm_pinctrl) module"]
pub type SM_PINCTRL = crate::Reg<u32, _SM_PINCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM_PINCTRL;
#[doc = "`read()` method returns [sm_pinctrl::R](sm_pinctrl::R) reader structure"]
impl crate::Readable for SM_PINCTRL {}
#[doc = "`write(|w| ..)` method takes [sm_pinctrl::W](sm_pinctrl::W) writer structure"]
impl crate::Writable for SM_PINCTRL {}
#[doc = "State machine pin control"]
pub mod sm_pinctrl;
