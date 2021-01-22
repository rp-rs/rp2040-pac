#[doc = "Reader of register CH0_TOP"]
pub type R = crate::R<u32, super::CH0_TOP>;
#[doc = "Writer for register CH0_TOP"]
pub type W = crate::W<u32, super::CH0_TOP>;
#[doc = "Register CH0_TOP `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CH0_TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CH0_TOP`"]
pub type CH0_TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH0_TOP`"]
pub struct CH0_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TOP_W<'a> {
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
    pub fn ch0_top(&self) -> CH0_TOP_R {
        CH0_TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch0_top(&mut self) -> CH0_TOP_W {
        CH0_TOP_W { w: self }
    }
}
