#[doc = "Reader of register PERFCTR1"]
pub type R = crate::R<u32, super::PERFCTR1>;
#[doc = "Writer for register PERFCTR1"]
pub type W = crate::W<u32, super::PERFCTR1>;
#[doc = "Register PERFCTR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERFCTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERFCTR1`"]
pub type PERFCTR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERFCTR1`"]
pub struct PERFCTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFCTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 1\\n Count some event signal from the busfabric arbiters.\\n Write any value to clear. Select an event to count using PERFSEL1"]
    #[inline(always)]
    pub fn perfctr1(&self) -> PERFCTR1_R {
        PERFCTR1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 1\\n Count some event signal from the busfabric arbiters.\\n Write any value to clear. Select an event to count using PERFSEL1"]
    #[inline(always)]
    pub fn perfctr1(&mut self) -> PERFCTR1_W {
        PERFCTR1_W { w: self }
    }
}
