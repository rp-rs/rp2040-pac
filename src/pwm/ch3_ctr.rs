#[doc = "Reader of register CH3_CTR"]
pub type R = crate::R<u32, super::CH3_CTR>;
#[doc = "Writer for register CH3_CTR"]
pub type W = crate::W<u32, super::CH3_CTR>;
#[doc = "Register CH3_CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3_CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH3_CTR`"]
pub type CH3_CTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH3_CTR`"]
pub struct CH3_CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_CTR_W<'a> {
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
    pub fn ch3_ctr(&self) -> CH3_CTR_R {
        CH3_CTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch3_ctr(&mut self) -> CH3_CTR_W {
        CH3_CTR_W { w: self }
    }
}
