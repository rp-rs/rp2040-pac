#[doc = "Reader of register CH4_DIV"]
pub type R = crate::R<u32, super::CH4_DIV>;
#[doc = "Writer for register CH4_DIV"]
pub type W = crate::W<u32, super::CH4_DIV>;
#[doc = "Register CH4_DIV `reset()`'s with value 0x10"]
impl crate::ResetValue for super::CH4_DIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `FRAC`"]
pub type FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRAC`"]
pub struct FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn frac(&mut self) -> FRAC_W {
        FRAC_W { w: self }
    }
}
