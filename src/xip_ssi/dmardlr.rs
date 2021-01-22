#[doc = "Reader of register DMARDLR"]
pub type R = crate::R<u32, super::DMARDLR>;
#[doc = "Writer for register DMARDLR"]
pub type W = crate::W<u32, super::DMARDLR>;
#[doc = "Register DMARDLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARDLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMARDL`"]
pub type DMARDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMARDL`"]
pub struct DMARDL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive data watermark level (DMARDLR+1)"]
    #[inline(always)]
    pub fn dmardl(&self) -> DMARDL_R {
        DMARDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive data watermark level (DMARDLR+1)"]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DMARDL_W {
        DMARDL_W { w: self }
    }
}
