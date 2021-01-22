#[doc = "Reader of register PERFCTR2"]
pub type R = crate::R<u32, super::PERFCTR2>;
#[doc = "Writer for register PERFCTR2"]
pub type W = crate::W<u32, super::PERFCTR2>;
#[doc = "Register PERFCTR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERFCTR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERFCTR2`"]
pub type PERFCTR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERFCTR2`"]
pub struct PERFCTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFCTR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 2\\n Count some event signal from the busfabric arbiters.\\n Write any value to clear. Select an event to count using PERFSEL2"]
    #[inline(always)]
    pub fn perfctr2(&self) -> PERFCTR2_R {
        PERFCTR2_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 2\\n Count some event signal from the busfabric arbiters.\\n Write any value to clear. Select an event to count using PERFSEL2"]
    #[inline(always)]
    pub fn perfctr2(&mut self) -> PERFCTR2_W {
        PERFCTR2_W { w: self }
    }
}
