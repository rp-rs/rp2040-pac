#[doc = "Reader of register FC0_INTERVAL"]
pub type R = crate::R<u32, super::FC0_INTERVAL>;
#[doc = "Writer for register FC0_INTERVAL"]
pub type W = crate::W<u32, super::FC0_INTERVAL>;
#[doc = "Register FC0_INTERVAL `reset()`'s with value 0x08"]
impl crate::ResetValue for super::FC0_INTERVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `FC0_INTERVAL`"]
pub type FC0_INTERVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FC0_INTERVAL`"]
pub struct FC0_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn fc0_interval(&self) -> FC0_INTERVAL_R {
        FC0_INTERVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn fc0_interval(&mut self) -> FC0_INTERVAL_W {
        FC0_INTERVAL_W { w: self }
    }
}
