#[doc = "Reader of register FC0_MAX_KHZ"]
pub type R = crate::R<u32, super::FC0_MAX_KHZ>;
#[doc = "Writer for register FC0_MAX_KHZ"]
pub type W = crate::W<u32, super::FC0_MAX_KHZ>;
#[doc = "Register FC0_MAX_KHZ `reset()`'s with value 0x01ff_ffff"]
impl crate::ResetValue for super::FC0_MAX_KHZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01ff_ffff
    }
}
#[doc = "Reader of field `FC0_MAX_KHZ`"]
pub type FC0_MAX_KHZ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FC0_MAX_KHZ`"]
pub struct FC0_MAX_KHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_MAX_KHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn fc0_max_khz(&self) -> FC0_MAX_KHZ_R {
        FC0_MAX_KHZ_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn fc0_max_khz(&mut self) -> FC0_MAX_KHZ_W {
        FC0_MAX_KHZ_W { w: self }
    }
}
