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
    #[doc = "Bits 8:11 - Force clock dividers to restart their count and clear fractional\\n accumulators. Restart multiple dividers to synchronise them."]
    #[inline(always)]
    pub fn clkdiv_restart(&self) -> CLKDIV_RESTART_R {
        CLKDIV_RESTART_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Clear internal SM state which is otherwise difficult to access\\n (e.g. shift counters). Self-clearing."]
    #[inline(always)]
    pub fn sm_restart(&self) -> SM_RESTART_R {
        SM_RESTART_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Enable state machine"]
    #[inline(always)]
    pub fn sm_enable(&self) -> SM_ENABLE_R {
        SM_ENABLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Force clock dividers to restart their count and clear fractional\\n accumulators. Restart multiple dividers to synchronise them."]
    #[inline(always)]
    pub fn clkdiv_restart(&mut self) -> CLKDIV_RESTART_W {
        CLKDIV_RESTART_W { w: self }
    }
    #[doc = "Bits 4:7 - Clear internal SM state which is otherwise difficult to access\\n (e.g. shift counters). Self-clearing."]
    #[inline(always)]
    pub fn sm_restart(&mut self) -> SM_RESTART_W {
        SM_RESTART_W { w: self }
    }
    #[doc = "Bits 0:3 - Enable state machine"]
    #[inline(always)]
    pub fn sm_enable(&mut self) -> SM_ENABLE_W {
        SM_ENABLE_W { w: self }
    }
}
