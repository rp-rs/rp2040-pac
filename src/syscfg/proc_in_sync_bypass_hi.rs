#[doc = "Reader of register PROC_IN_SYNC_BYPASS_HI"]
pub type R = crate::R<u32, super::PROC_IN_SYNC_BYPASS_HI>;
#[doc = "Writer for register PROC_IN_SYNC_BYPASS_HI"]
pub type W = crate::W<u32, super::PROC_IN_SYNC_BYPASS_HI>;
#[doc = "Register PROC_IN_SYNC_BYPASS_HI `reset()`'s with value 0"]
impl crate::ResetValue for super::PROC_IN_SYNC_BYPASS_HI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROC_IN_SYNC_BYPASS_HI`"]
pub type PROC_IN_SYNC_BYPASS_HI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROC_IN_SYNC_BYPASS_HI`"]
pub struct PROC_IN_SYNC_BYPASS_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC_IN_SYNC_BYPASS_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn proc_in_sync_bypass_hi(&self) -> PROC_IN_SYNC_BYPASS_HI_R {
        PROC_IN_SYNC_BYPASS_HI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn proc_in_sync_bypass_hi(&mut self) -> PROC_IN_SYNC_BYPASS_HI_W {
        PROC_IN_SYNC_BYPASS_HI_W { w: self }
    }
}
