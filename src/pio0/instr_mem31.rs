#[doc = "Reader of register INSTR_MEM31"]
pub type R = crate::R<u32, super::INSTR_MEM31>;
#[doc = "Writer for register INSTR_MEM31"]
pub type W = crate::W<u32, super::INSTR_MEM31>;
#[doc = "Register INSTR_MEM31 `reset()`'s with value 0"]
impl crate::ResetValue for super::INSTR_MEM31 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR_MEM31`"]
pub type INSTR_MEM31_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INSTR_MEM31`"]
pub struct INSTR_MEM31_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR_MEM31_W<'a> {
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
    pub fn instr_mem31(&self) -> INSTR_MEM31_R {
        INSTR_MEM31_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn instr_mem31(&mut self) -> INSTR_MEM31_W {
        INSTR_MEM31_W { w: self }
    }
}
