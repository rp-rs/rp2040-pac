#[doc = "Reader of register FC0_DELAY"]
pub type R = crate::R<u32, super::FC0_DELAY>;
#[doc = "Writer for register FC0_DELAY"]
pub type W = crate::W<u32, super::FC0_DELAY>;
#[doc = "Register FC0_DELAY `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FC0_DELAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `FC0_DELAY`"]
pub type FC0_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FC0_DELAY`"]
pub struct FC0_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fc0_delay(&self) -> FC0_DELAY_R {
        FC0_DELAY_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fc0_delay(&mut self) -> FC0_DELAY_W {
        FC0_DELAY_W { w: self }
    }
}
