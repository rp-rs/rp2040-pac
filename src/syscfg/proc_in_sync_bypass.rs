#[doc = "Reader of register PROC_IN_SYNC_BYPASS"]
pub type R = crate::R<u32, super::PROC_IN_SYNC_BYPASS>;
#[doc = "Writer for register PROC_IN_SYNC_BYPASS"]
pub type W = crate::W<u32, super::PROC_IN_SYNC_BYPASS>;
#[doc = "Register PROC_IN_SYNC_BYPASS `reset()`'s with value 0"]
impl crate::ResetValue for super::PROC_IN_SYNC_BYPASS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROC_IN_SYNC_BYPASS`"]
pub type PROC_IN_SYNC_BYPASS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PROC_IN_SYNC_BYPASS`"]
pub struct PROC_IN_SYNC_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC_IN_SYNC_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn proc_in_sync_bypass(&self) -> PROC_IN_SYNC_BYPASS_R {
        PROC_IN_SYNC_BYPASS_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn proc_in_sync_bypass(&mut self) -> PROC_IN_SYNC_BYPASS_W {
        PROC_IN_SYNC_BYPASS_W { w: self }
    }
}
