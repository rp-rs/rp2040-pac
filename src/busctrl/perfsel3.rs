#[doc = "Reader of register PERFSEL3"]
pub type R = crate::R<u32, super::PERFSEL3>;
#[doc = "Writer for register PERFSEL3"]
pub type W = crate::W<u32, super::PERFSEL3>;
#[doc = "Register PERFSEL3 `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PERFSEL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `PERFSEL3`"]
pub type PERFSEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERFSEL3`"]
pub struct PERFSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR3"]
    #[inline(always)]
    pub fn perfsel3(&self) -> PERFSEL3_R {
        PERFSEL3_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR3"]
    #[inline(always)]
    pub fn perfsel3(&mut self) -> PERFSEL3_W {
        PERFSEL3_W { w: self }
    }
}
