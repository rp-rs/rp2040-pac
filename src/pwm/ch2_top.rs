#[doc = "Reader of register CH2_TOP"]
pub type R = crate::R<u32, super::CH2_TOP>;
#[doc = "Writer for register CH2_TOP"]
pub type W = crate::W<u32, super::CH2_TOP>;
#[doc = "Register CH2_TOP `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CH2_TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CH2_TOP`"]
pub type CH2_TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH2_TOP`"]
pub struct CH2_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TOP_W<'a> {
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
    pub fn ch2_top(&self) -> CH2_TOP_R {
        CH2_TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch2_top(&mut self) -> CH2_TOP_W {
        CH2_TOP_W { w: self }
    }
}
