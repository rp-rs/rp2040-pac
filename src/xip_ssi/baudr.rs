#[doc = "Reader of register BAUDR"]
pub type R = crate::R<u32, super::BAUDR>;
#[doc = "Writer for register BAUDR"]
pub type W = crate::W<u32, super::BAUDR>;
#[doc = "Register BAUDR `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCKDV`"]
pub type SCKDV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCKDV`"]
pub struct SCKDV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKDV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SSI clock divider"]
    #[inline(always)]
    pub fn sckdv(&self) -> SCKDV_R {
        SCKDV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SSI clock divider"]
    #[inline(always)]
    pub fn sckdv(&mut self) -> SCKDV_W {
        SCKDV_W { w: self }
    }
}
