#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKDIV_RESTART`"]
pub type CLKDIV_RESTART_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV_RESTART`"]
pub struct CLKDIV_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_RESTART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SM_RESTART`"]
pub type SM_RESTART_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SM_RESTART`"]
pub struct SM_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_RESTART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SM_ENABLE`"]
pub type SM_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SM_ENABLE`"]
pub struct SM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep.\\n\\n Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync.\\n\\n Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly."]
    #[inline(always)]
    pub fn clkdiv_restart(&self) -> CLKDIV_RESTART_R {
        CLKDIV_RESTART_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution.\\n\\n Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY."]
    #[inline(always)]
    pub fn sm_restart(&self) -> SM_RESTART_R {
        SM_RESTART_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously."]
    #[inline(always)]
    pub fn sm_enable(&self) -> SM_ENABLE_R {
        SM_ENABLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep.\\n\\n Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync.\\n\\n Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly."]
    #[inline(always)]
    pub fn clkdiv_restart(&mut self) -> CLKDIV_RESTART_W {
        CLKDIV_RESTART_W { w: self }
    }
    #[doc = "Bits 4:7 - Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution.\\n\\n Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY."]
    #[inline(always)]
    pub fn sm_restart(&mut self) -> SM_RESTART_W {
        SM_RESTART_W { w: self }
    }
    #[doc = "Bits 0:3 - Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously."]
    #[inline(always)]
    pub fn sm_enable(&mut self) -> SM_ENABLE_W {
        SM_ENABLE_W { w: self }
    }
}
