#[doc = "Reader of register INSTR_MEM24"]
pub type R = crate::R<u32, super::INSTR_MEM24>;
#[doc = "Writer for register INSTR_MEM24"]
pub type W = crate::W<u32, super::INSTR_MEM24>;
#[doc = "Register INSTR_MEM24 `reset()`'s with value 0"]
impl crate::ResetValue for super::INSTR_MEM24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR_MEM24`"]
pub type INSTR_MEM24_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INSTR_MEM24`"]
pub struct INSTR_MEM24_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR_MEM24_W<'a> {
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
    pub fn instr_mem24(&self) -> INSTR_MEM24_R {
        INSTR_MEM24_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn instr_mem24(&mut self) -> INSTR_MEM24_W {
        INSTR_MEM24_W { w: self }
    }
}
