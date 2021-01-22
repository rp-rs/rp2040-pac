#[doc = "Reader of register TXD_DRIVE_EDGE"]
pub type R = crate::R<u32, super::TXD_DRIVE_EDGE>;
#[doc = "Writer for register TXD_DRIVE_EDGE"]
pub type W = crate::W<u32, super::TXD_DRIVE_EDGE>;
#[doc = "Register TXD_DRIVE_EDGE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXD_DRIVE_EDGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDE`"]
pub type TDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDE`"]
pub struct TDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TXD drive edge"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXD drive edge"]
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W {
        TDE_W { w: self }
    }
}
