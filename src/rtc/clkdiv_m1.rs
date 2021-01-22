#[doc = "Reader of register CLKDIV_M1"]
pub type R = crate::R<u32, super::CLKDIV_M1>;
#[doc = "Writer for register CLKDIV_M1"]
pub type W = crate::W<u32, super::CLKDIV_M1>;
#[doc = "Register CLKDIV_M1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV_M1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKDIV_M1`"]
pub type CLKDIV_M1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLKDIV_M1`"]
pub struct CLKDIV_M1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_M1_W<'a> {
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
    pub fn clkdiv_m1(&self) -> CLKDIV_M1_R {
        CLKDIV_M1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn clkdiv_m1(&mut self) -> CLKDIV_M1_W {
        CLKDIV_M1_W { w: self }
    }
}
