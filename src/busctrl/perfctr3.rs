#[doc = "Reader of register PERFCTR3"]
pub type R = crate::R<u32, super::PERFCTR3>;
#[doc = "Writer for register PERFCTR3"]
pub type W = crate::W<u32, super::PERFCTR3>;
#[doc = "Register PERFCTR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERFCTR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERFCTR3`"]
pub type PERFCTR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERFCTR3`"]
pub struct PERFCTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFCTR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 3\\n Count some event signal from the busfabric arbiters.\\n Write any value to clear. Select an event to count using PERFSEL3"]
    #[inline(always)]
    pub fn perfctr3(&self) -> PERFCTR3_R {
        PERFCTR3_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 3\\n Count some event signal from the busfabric arbiters.\\n Write any value to clear. Select an event to count using PERFSEL3"]
    #[inline(always)]
    pub fn perfctr3(&mut self) -> PERFCTR3_W {
        PERFCTR3_W { w: self }
    }
}
