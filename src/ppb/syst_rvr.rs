#[doc = "Reader of register SYST_RVR"]
pub type R = crate::R<u32, super::SYST_RVR>;
#[doc = "Writer for register SYST_RVR"]
pub type W = crate::W<u32, super::SYST_RVR>;
#[doc = "Register SYST_RVR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYST_RVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RELOAD`"]
pub type RELOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RELOAD`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0."]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}
