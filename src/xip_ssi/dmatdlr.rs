#[doc = "Reader of register DMATDLR"]
pub type R = crate::R<u32, super::DMATDLR>;
#[doc = "Writer for register DMATDLR"]
pub type W = crate::W<u32, super::DMATDLR>;
#[doc = "Register DMATDLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATDLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMATDL`"]
pub type DMATDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMATDL`"]
pub struct DMATDL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit data watermark level"]
    #[inline(always)]
    pub fn dmatdl(&self) -> DMATDL_R {
        DMATDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data watermark level"]
    #[inline(always)]
    pub fn dmatdl(&mut self) -> DMATDL_W {
        DMATDL_W { w: self }
    }
}
