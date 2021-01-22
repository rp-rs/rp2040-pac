#[doc = "Reader of register PERFSEL2"]
pub type R = crate::R<u32, super::PERFSEL2>;
#[doc = "Writer for register PERFSEL2"]
pub type W = crate::W<u32, super::PERFSEL2>;
#[doc = "Register PERFSEL2 `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PERFSEL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `PERFSEL2`"]
pub type PERFSEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERFSEL2`"]
pub struct PERFSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR2"]
    #[inline(always)]
    pub fn perfsel2(&self) -> PERFSEL2_R {
        PERFSEL2_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR2"]
    #[inline(always)]
    pub fn perfsel2(&mut self) -> PERFSEL2_W {
        PERFSEL2_W { w: self }
    }
}
