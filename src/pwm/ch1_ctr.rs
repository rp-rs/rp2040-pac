#[doc = "Reader of register CH1_CTR"]
pub type R = crate::R<u32, super::CH1_CTR>;
#[doc = "Writer for register CH1_CTR"]
pub type W = crate::W<u32, super::CH1_CTR>;
#[doc = "Register CH1_CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1_CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH1_CTR`"]
pub type CH1_CTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH1_CTR`"]
pub struct CH1_CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_CTR_W<'a> {
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
    pub fn ch1_ctr(&self) -> CH1_CTR_R {
        CH1_CTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch1_ctr(&mut self) -> CH1_CTR_W {
        CH1_CTR_W { w: self }
    }
}
