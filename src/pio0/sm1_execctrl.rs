#[doc = "Reader of register SM1_EXECCTRL"]
pub type R = crate::R<u32, super::SM1_EXECCTRL>;
#[doc = "Writer for register SM1_EXECCTRL"]
pub type W = crate::W<u32, super::SM1_EXECCTRL>;
#[doc = "Register SM1_EXECCTRL `reset()`'s with value 0x0001_f000"]
impl crate::ResetValue for super::SM1_EXECCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_f000
    }
}
#[doc = "Reader of field `EXEC_STALLED`"]
pub type EXEC_STALLED_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIDE_EN`"]
pub type SIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIDE_EN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SIDE_PINDIR`"]
pub type SIDE_PINDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIDE_PINDIR`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `JMP_PIN`"]
pub type JMP_PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JMP_PIN`"]
pub struct JMP_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> JMP_PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `OUT_EN_SEL`"]
pub type OUT_EN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT_EN_SEL`"]
pub struct OUT_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `INLINE_OUT_EN`"]
pub type INLINE_OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INLINE_OUT_EN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OUT_STICKY`"]
pub type OUT_STICKY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_STICKY`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `WRAP_TOP`"]
pub type WRAP_TOP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRAP_TOP`"]
pub struct WRAP_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAP_TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `WRAP_BOTTOM`"]
pub type WRAP_BOTTOM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRAP_BOTTOM`"]
pub struct WRAP_BOTTOM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAP_BOTTOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Comparison used for the MOV x, STATUS instruction.\n\nValue on reset: 0"]
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
#[doc = "Reader of field `STATUS_SEL`"]
pub type STATUS_SEL_R = crate::R<bool, STATUS_SEL_A>;
impl STATUS_SEL_R {
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
        *self == STATUS_SEL_A::TXLEVEL
    }
    #[doc = "Checks if the value of the field is `RXLEVEL`"]
    #[inline(always)]
    pub fn is_rxlevel(&self) -> bool {
        *self == STATUS_SEL_A::RXLEVEL
    }
}
#[doc = "Write proxy for field `STATUS_SEL`"]
pub struct STATUS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATUS_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `STATUS_N`"]
pub type STATUS_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATUS_N`"]
pub struct STATUS_N_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - An instruction written to SMx_INSTR is stalled, and latched by the\\n state machine. Will clear once the instruction completes."]
    #[inline(always)]
    pub fn exec_stalled(&self) -> EXEC_STALLED_R {
        EXEC_STALLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - If 1, the delay MSB is used as side-set enable, rather than a\\n side-set data bit. This allows instructions to perform side-set optionally,\\n rather than on every instruction."]
    #[inline(always)]
    pub fn side_en(&self) -> SIDE_EN_R {
        SIDE_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Side-set data is asserted to pin OEs instead of pin values"]
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
    #[doc = "Bit 18 - If 1, use a bit of OUT data as an auxiliary write enable\\n When used in conjunction with OUT_STICKY, writes with an enable of 0 will\\n deassert the latest pin write. This can create useful masking/override behaviour\\n due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
    #[inline(always)]
    pub fn inline_out_en(&self) -> INLINE_OUT_EN_R {
        INLINE_OUT_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Continuously assert the most recent OUT/SET to the pins"]
    #[inline(always)]
    pub fn out_sticky(&self) -> OUT_STICKY_R {
        OUT_STICKY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 12:16 - After reaching this address, execution is wrapped to wrap_bottom.\\n If the instruction is a jump, and the jump condition is true, the jump takes priority."]
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
    #[doc = "Bit 30 - If 1, the delay MSB is used as side-set enable, rather than a\\n side-set data bit. This allows instructions to perform side-set optionally,\\n rather than on every instruction."]
    #[inline(always)]
    pub fn side_en(&mut self) -> SIDE_EN_W {
        SIDE_EN_W { w: self }
    }
    #[doc = "Bit 29 - Side-set data is asserted to pin OEs instead of pin values"]
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
    #[doc = "Bit 18 - If 1, use a bit of OUT data as an auxiliary write enable\\n When used in conjunction with OUT_STICKY, writes with an enable of 0 will\\n deassert the latest pin write. This can create useful masking/override behaviour\\n due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
    #[inline(always)]
    pub fn inline_out_en(&mut self) -> INLINE_OUT_EN_W {
        INLINE_OUT_EN_W { w: self }
    }
    #[doc = "Bit 17 - Continuously assert the most recent OUT/SET to the pins"]
    #[inline(always)]
    pub fn out_sticky(&mut self) -> OUT_STICKY_W {
        OUT_STICKY_W { w: self }
    }
    #[doc = "Bits 12:16 - After reaching this address, execution is wrapped to wrap_bottom.\\n If the instruction is a jump, and the jump condition is true, the jump takes priority."]
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
}
