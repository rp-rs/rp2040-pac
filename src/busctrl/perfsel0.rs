#[doc = "Reader of register PERFSEL0"]
pub type R = crate::R<u32, super::PERFSEL0>;
#[doc = "Writer for register PERFSEL0"]
pub type W = crate::W<u32, super::PERFSEL0>;
#[doc = "Register PERFSEL0 `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PERFSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `PERFSEL0`"]
pub type PERFSEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERFSEL0`"]
pub struct PERFSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR0"]
    #[inline(always)]
    pub fn perfsel0(&self) -> PERFSEL0_R {
        PERFSEL0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select a performance event for PERFCTR0"]
    #[inline(always)]
    pub fn perfsel0(&mut self) -> PERFSEL0_W {
        PERFSEL0_W { w: self }
    }
}
