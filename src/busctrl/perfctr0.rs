#[doc = "Reader of register PERFCTR0"]
pub type R = crate::R<u32, super::PERFCTR0>;
#[doc = "Writer for register PERFCTR0"]
pub type W = crate::W<u32, super::PERFCTR0>;
#[doc = "Register PERFCTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERFCTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERFCTR0`"]
pub type PERFCTR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERFCTR0`"]
pub struct PERFCTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFCTR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 0\\n Count some event signal from the busfabric arbiters.\\n Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    pub fn perfctr0(&self) -> PERFCTR0_R {
        PERFCTR0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 0\\n Count some event signal from the busfabric arbiters.\\n Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    pub fn perfctr0(&mut self) -> PERFCTR0_W {
        PERFCTR0_W { w: self }
    }
}
