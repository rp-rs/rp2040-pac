#[doc = "Reader of register INTERP1_ACCUM1_ADD"]
pub type R = crate::R<u32, super::INTERP1_ACCUM1_ADD>;
#[doc = "Writer for register INTERP1_ACCUM1_ADD"]
pub type W = crate::W<u32, super::INTERP1_ACCUM1_ADD>;
#[doc = "Register INTERP1_ACCUM1_ADD `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERP1_ACCUM1_ADD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERP1_ACCUM1_ADD`"]
pub type INTERP1_ACCUM1_ADD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INTERP1_ACCUM1_ADD`"]
pub struct INTERP1_ACCUM1_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERP1_ACCUM1_ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn interp1_accum1_add(&self) -> INTERP1_ACCUM1_ADD_R {
        INTERP1_ACCUM1_ADD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn interp1_accum1_add(&mut self) -> INTERP1_ACCUM1_ADD_W {
        INTERP1_ACCUM1_ADD_W { w: self }
    }
}
