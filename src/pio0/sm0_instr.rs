#[doc = "Reader of register SM0_INSTR"]
pub type R = crate::R<u32, super::SM0_INSTR>;
#[doc = "Writer for register SM0_INSTR"]
pub type W = crate::W<u32, super::SM0_INSTR>;
#[doc = "Register SM0_INSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0_INSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SM0_INSTR`"]
pub type SM0_INSTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SM0_INSTR`"]
pub struct SM0_INSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_INSTR_W<'a> {
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
    pub fn sm0_instr(&self) -> SM0_INSTR_R {
        SM0_INSTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sm0_instr(&mut self) -> SM0_INSTR_W {
        SM0_INSTR_W { w: self }
    }
}
