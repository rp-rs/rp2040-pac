#[doc = "Reader of register FBDIV_INT"]
pub type R = crate::R<u32, super::FBDIV_INT>;
#[doc = "Writer for register FBDIV_INT"]
pub type W = crate::W<u32, super::FBDIV_INT>;
#[doc = "Register FBDIV_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::FBDIV_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FBDIV_INT`"]
pub type FBDIV_INT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FBDIV_INT`"]
pub struct FBDIV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDIV_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - see ctrl reg description for constraints"]
    #[inline(always)]
    pub fn fbdiv_int(&self) -> FBDIV_INT_R {
        FBDIV_INT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - see ctrl reg description for constraints"]
    #[inline(always)]
    pub fn fbdiv_int(&mut self) -> FBDIV_INT_W {
        FBDIV_INT_W { w: self }
    }
}
