#[doc = "Register `SM_EXECCTRL` reader"]
pub struct R(crate::R<SM_EXECCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_EXECCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_EXECCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_EXECCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM_EXECCTRL` writer"]
pub struct W(crate::W<SM_EXECCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM_EXECCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SM_EXECCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SM_EXECCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXEC_STALLED` reader - If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes."]
pub struct EXEC_STALLED_R(crate::FieldReader<bool, bool>);
impl EXEC_STALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXEC_STALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEC_STALLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDE_EN` reader - If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
pub struct SIDE_EN_R(crate::FieldReader<bool, bool>);
impl SIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDE_EN` writer - If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
pub struct SIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SIDE_PINDIR` reader - If 1, side-set data is asserted to pin directions, instead of pin values"]
pub struct SIDE_PINDIR_R(crate::FieldReader<bool, bool>);
impl SIDE_PINDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIDE_PINDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDE_PINDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDE_PINDIR` writer - If 1, side-set data is asserted to pin directions, instead of pin values"]
pub struct SIDE_PINDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDE_PINDIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `JMP_PIN` reader - The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
pub struct JMP_PIN_R(crate::FieldReader<u8, u8>);
impl JMP_PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JMP_PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMP_PIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMP_PIN` writer - The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
pub struct JMP_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> JMP_PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `OUT_EN_SEL` reader - Which data bit to use for inline OUT enable"]
pub struct OUT_EN_SEL_R(crate::FieldReader<u8, u8>);
impl OUT_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EN_SEL` writer - Which data bit to use for inline OUT enable"]
pub struct OUT_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `INLINE_OUT_EN` reader - If 1, use a bit of OUT data as an auxiliary write enable  
 When used in conjunction with OUT_STICKY, writes with an enable of 0 will  
 deassert the latest pin write. This can create useful masking/override behaviour  
 due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
pub struct INLINE_OUT_EN_R(crate::FieldReader<bool, bool>);
impl INLINE_OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINE_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINE_OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINE_OUT_EN` writer - If 1, use a bit of OUT data as an auxiliary write enable  
 When used in conjunction with OUT_STICKY, writes with an enable of 0 will  
 deassert the latest pin write. This can create useful masking/override behaviour  
 due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
pub struct INLINE_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINE_OUT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OUT_STICKY` reader - Continuously assert the most recent OUT/SET to the pins"]
pub struct OUT_STICKY_R(crate::FieldReader<bool, bool>);
impl OUT_STICKY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_STICKY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_STICKY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_STICKY` writer - Continuously assert the most recent OUT/SET to the pins"]
pub struct OUT_STICKY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_STICKY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `WRAP_TOP` reader - After reaching this address, execution is wrapped to wrap_bottom.  
 If the instruction is a jump, and the jump condition is true, the jump takes priority."]
pub struct WRAP_TOP_R(crate::FieldReader<u8, u8>);
impl WRAP_TOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WRAP_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRAP_TOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRAP_TOP` writer - After reaching this address, execution is wrapped to wrap_bottom.  
 If the instruction is a jump, and the jump condition is true, the jump takes priority."]
pub struct WRAP_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAP_TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
#[doc = "Field `WRAP_BOTTOM` reader - After reaching wrap_top, execution is wrapped to this address."]
pub struct WRAP_BOTTOM_R(crate::FieldReader<u8, u8>);
impl WRAP_BOTTOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WRAP_BOTTOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRAP_BOTTOM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRAP_BOTTOM` writer - After reaching wrap_top, execution is wrapped to this address."]
pub struct WRAP_BOTTOM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAP_BOTTOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | ((value as u32 & 0x1f) << 7);
        self.w
    }
}
#[doc = "Comparison used for the MOV x, STATUS instruction.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_SEL_A {
    #[doc = "0: All-ones if TX FIFO level < N, otherwise all-zeroes"]
    TXLEVEL = 0,
    #[doc = "1: All-ones if RX FIFO level < N, otherwise all-zeroes"]
    RXLEVEL = 1,
}
impl From<STATUS_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS_SEL` reader - Comparison used for the MOV x, STATUS instruction."]
pub struct STATUS_SEL_R(crate::FieldReader<bool, STATUS_SEL_A>);
impl STATUS_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_SEL_A {
        match self.bits {
            false => STATUS_SEL_A::TXLEVEL,
            true => STATUS_SEL_A::RXLEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `TXLEVEL`"]
    #[inline(always)]
    pub fn is_txlevel(&self) -> bool {
        **self == STATUS_SEL_A::TXLEVEL
    }
    #[doc = "Checks if the value of the field is `RXLEVEL`"]
    #[inline(always)]
    pub fn is_rxlevel(&self) -> bool {
        **self == STATUS_SEL_A::RXLEVEL
    }
}
impl core::ops::Deref for STATUS_SEL_R {
    type Target = crate::FieldReader<bool, STATUS_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_SEL` writer - Comparison used for the MOV x, STATUS instruction."]
pub struct STATUS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATUS_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All-ones if TX FIFO level < N, otherwise all-zeroes"]
    #[inline(always)]
    pub fn txlevel(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::TXLEVEL)
    }
    #[doc = "All-ones if RX FIFO level < N, otherwise all-zeroes"]
    #[inline(always)]
    pub fn rxlevel(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::RXLEVEL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `STATUS_N` reader - Comparison level for the MOV x, STATUS instruction"]
pub struct STATUS_N_R(crate::FieldReader<u8, u8>);
impl STATUS_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_N` writer - Comparison level for the MOV x, STATUS instruction"]
pub struct STATUS_N_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes."]
    #[inline(always)]
    pub fn exec_stalled(&self) -> EXEC_STALLED_R {
        EXEC_STALLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
    #[inline(always)]
    pub fn side_en(&self) -> SIDE_EN_R {
        SIDE_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - If 1, side-set data is asserted to pin directions, instead of pin values"]
    #[inline(always)]
    pub fn side_pindir(&self) -> SIDE_PINDIR_R {
        SIDE_PINDIR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
    #[inline(always)]
    pub fn jmp_pin(&self) -> JMP_PIN_R {
        JMP_PIN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Which data bit to use for inline OUT enable"]
    #[inline(always)]
    pub fn out_en_sel(&self) -> OUT_EN_SEL_R {
        OUT_EN_SEL_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - If 1, use a bit of OUT data as an auxiliary write enable  
 When used in conjunction with OUT_STICKY, writes with an enable of 0 will  
 deassert the latest pin write. This can create useful masking/override behaviour  
 due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
    #[inline(always)]
    pub fn inline_out_en(&self) -> INLINE_OUT_EN_R {
        INLINE_OUT_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Continuously assert the most recent OUT/SET to the pins"]
    #[inline(always)]
    pub fn out_sticky(&self) -> OUT_STICKY_R {
        OUT_STICKY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 12:16 - After reaching this address, execution is wrapped to wrap_bottom.  
 If the instruction is a jump, and the jump condition is true, the jump takes priority."]
    #[inline(always)]
    pub fn wrap_top(&self) -> WRAP_TOP_R {
        WRAP_TOP_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - After reaching wrap_top, execution is wrapped to this address."]
    #[inline(always)]
    pub fn wrap_bottom(&self) -> WRAP_BOTTOM_R {
        WRAP_BOTTOM_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 4 - Comparison used for the MOV x, STATUS instruction."]
    #[inline(always)]
    pub fn status_sel(&self) -> STATUS_SEL_R {
        STATUS_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Comparison level for the MOV x, STATUS instruction"]
    #[inline(always)]
    pub fn status_n(&self) -> STATUS_N_R {
        STATUS_N_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
    #[inline(always)]
    pub fn side_en(&mut self) -> SIDE_EN_W {
        SIDE_EN_W { w: self }
    }
    #[doc = "Bit 29 - If 1, side-set data is asserted to pin directions, instead of pin values"]
    #[inline(always)]
    pub fn side_pindir(&mut self) -> SIDE_PINDIR_W {
        SIDE_PINDIR_W { w: self }
    }
    #[doc = "Bits 24:28 - The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
    #[inline(always)]
    pub fn jmp_pin(&mut self) -> JMP_PIN_W {
        JMP_PIN_W { w: self }
    }
    #[doc = "Bits 19:23 - Which data bit to use for inline OUT enable"]
    #[inline(always)]
    pub fn out_en_sel(&mut self) -> OUT_EN_SEL_W {
        OUT_EN_SEL_W { w: self }
    }
    #[doc = "Bit 18 - If 1, use a bit of OUT data as an auxiliary write enable  
 When used in conjunction with OUT_STICKY, writes with an enable of 0 will  
 deassert the latest pin write. This can create useful masking/override behaviour  
 due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
    #[inline(always)]
    pub fn inline_out_en(&mut self) -> INLINE_OUT_EN_W {
        INLINE_OUT_EN_W { w: self }
    }
    #[doc = "Bit 17 - Continuously assert the most recent OUT/SET to the pins"]
    #[inline(always)]
    pub fn out_sticky(&mut self) -> OUT_STICKY_W {
        OUT_STICKY_W { w: self }
    }
    #[doc = "Bits 12:16 - After reaching this address, execution is wrapped to wrap_bottom.  
 If the instruction is a jump, and the jump condition is true, the jump takes priority."]
    #[inline(always)]
    pub fn wrap_top(&mut self) -> WRAP_TOP_W {
        WRAP_TOP_W { w: self }
    }
    #[doc = "Bits 7:11 - After reaching wrap_top, execution is wrapped to this address."]
    #[inline(always)]
    pub fn wrap_bottom(&mut self) -> WRAP_BOTTOM_W {
        WRAP_BOTTOM_W { w: self }
    }
    #[doc = "Bit 4 - Comparison used for the MOV x, STATUS instruction."]
    #[inline(always)]
    pub fn status_sel(&mut self) -> STATUS_SEL_W {
        STATUS_SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - Comparison level for the MOV x, STATUS instruction"]
    #[inline(always)]
    pub fn status_n(&mut self) -> STATUS_N_W {
        STATUS_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Execution/behavioural settings for state machine 0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sm_execctrl](index.html) module"]
pub struct SM_EXECCTRL_SPEC;
impl crate::RegisterSpec for SM_EXECCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm_execctrl::R](R) reader structure"]
impl crate::Readable for SM_EXECCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm_execctrl::W](W) writer structure"]
impl crate::Writable for SM_EXECCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SM_EXECCTRL to value 0x0001_f000"]
impl crate::Resettable for SM_EXECCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_f000
    }
}
