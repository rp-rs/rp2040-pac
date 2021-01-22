#[doc = "Reader of register CH3_TOP"]
pub type R = crate::R<u32, super::CH3_TOP>;
#[doc = "Writer for register CH3_TOP"]
pub type W = crate::W<u32, super::CH3_TOP>;
#[doc = "Register CH3_TOP `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CH3_TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CH3_TOP`"]
pub type CH3_TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH3_TOP`"]
pub struct CH3_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TOP_W<'a> {
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
    pub fn ch3_top(&self) -> CH3_TOP_R {
        CH3_TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch3_top(&mut self) -> CH3_TOP_W {
        CH3_TOP_W { w: self }
    }
}
