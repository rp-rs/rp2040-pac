#[doc = "Reader of register PERFSEL1"]
pub type R = crate::R<u32, super::PERFSEL1>;
#[doc = "Writer for register PERFSEL1"]
pub type W = crate::W<u32, super::PERFSEL1>;
#[doc = "Register PERFSEL1 `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PERFSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `PERFSEL1`"]
pub type PERFSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERFSEL1`"]
pub struct PERFSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR1"]
    #[inline(always)]
    pub fn perfsel1(&self) -> PERFSEL1_R {
        PERFSEL1_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR1"]
    #[inline(always)]
    pub fn perfsel1(&mut self) -> PERFSEL1_W {
        PERFSEL1_W { w: self }
    }
}
