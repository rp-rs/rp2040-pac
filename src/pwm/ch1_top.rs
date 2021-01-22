#[doc = "Reader of register CH1_TOP"]
pub type R = crate::R<u32, super::CH1_TOP>;
#[doc = "Writer for register CH1_TOP"]
pub type W = crate::W<u32, super::CH1_TOP>;
#[doc = "Register CH1_TOP `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CH1_TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CH1_TOP`"]
pub type CH1_TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH1_TOP`"]
pub struct CH1_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TOP_W<'a> {
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
    pub fn ch1_top(&self) -> CH1_TOP_R {
        CH1_TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch1_top(&mut self) -> CH1_TOP_W {
        CH1_TOP_W { w: self }
    }
}
