#[doc = "Reader of register SM1_INSTR"]
pub type R = crate::R<u32, super::SM1_INSTR>;
#[doc = "Writer for register SM1_INSTR"]
pub type W = crate::W<u32, super::SM1_INSTR>;
#[doc = "Register SM1_INSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SM1_INSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SM1_INSTR`"]
pub type SM1_INSTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SM1_INSTR`"]
pub struct SM1_INSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_INSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sm1_instr(&self) -> SM1_INSTR_R {
        SM1_INSTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sm1_instr(&mut self) -> SM1_INSTR_W {
        SM1_INSTR_W { w: self }
    }
}
