#[doc = "Reader of register CH5_TOP"]
pub type R = crate::R<u32, super::CH5_TOP>;
#[doc = "Writer for register CH5_TOP"]
pub type W = crate::W<u32, super::CH5_TOP>;
#[doc = "Register CH5_TOP `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CH5_TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CH5_TOP`"]
pub type CH5_TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH5_TOP`"]
pub struct CH5_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_TOP_W<'a> {
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
    pub fn ch5_top(&self) -> CH5_TOP_R {
        CH5_TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch5_top(&mut self) -> CH5_TOP_W {
        CH5_TOP_W { w: self }
    }
}
