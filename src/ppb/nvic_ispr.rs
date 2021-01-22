#[doc = "Reader of register NVIC_ISPR"]
pub type R = crate::R<u32, super::NVIC_ISPR>;
#[doc = "Writer for register NVIC_ISPR"]
pub type W = crate::W<u32, super::NVIC_ISPR>;
#[doc = "Register NVIC_ISPR `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETPEND`"]
pub type SETPEND_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SETPEND`"]
pub struct SETPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits.\\n Write:\\n 0 = No effect.\\n 1 = Changes interrupt state to pending.\\n Read:\\n 0 = Interrupt is not pending.\\n 1 = Interrupt is pending.\\n Note: Writing 1 to the NVIC_ISPR bit corresponding to:\\n An interrupt that is pending has no effect.\\n A disabled interrupt sets the state of that interrupt to pending."]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits.\\n Write:\\n 0 = No effect.\\n 1 = Changes interrupt state to pending.\\n Read:\\n 0 = Interrupt is not pending.\\n 1 = Interrupt is pending.\\n Note: Writing 1 to the NVIC_ISPR bit corresponding to:\\n An interrupt that is pending has no effect.\\n A disabled interrupt sets the state of that interrupt to pending."]
    #[inline(always)]
    pub fn setpend(&mut self) -> SETPEND_W {
        SETPEND_W { w: self }
    }
}
